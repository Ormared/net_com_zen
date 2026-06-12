"""Tests for the dashboard data layer (the render-ready transforms)."""
import json

import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq

from netcom_zen.viz import data as D


def _make_run(tmp_path, *, positions, packets, agents, scenario_extra=None):
    scenario = {
        "name": "viz", "duration_s": 10.0,
        "radio": {"hop": {"hop_rate_hz": 100.0}},
        "environment": {"extent_m": [1000, 1000], "foliage": []},
        "nodes": [{"id": "v1", "role": "vehicle"},
                  {"id": "cmd", "role": "command"}],
        "agent": {"transport": "udp", "sync_mode": "state", "mls": True},
        "jammers": [], "satellite": {"enabled": False, "outages": []},
    }
    if scenario_extra:
        scenario.update(scenario_extra)
    (tmp_path / "manifest.json").write_text(json.dumps({"scenario": scenario}))
    if positions:
        cols = list(zip(*positions))
        pq.write_table(pa.table({"t": cols[0], "id": cols[1], "x": cols[2],
                                 "y": cols[3], "heading": cols[4]}),
                       tmp_path / "positions.parquet")
    if packets:
        cols = list(zip(*packets))
        pq.write_table(pa.table({"t": cols[0], "src": cols[1], "dst": cols[2],
                                 "length": cols[3], "verdict": cols[4],
                                 "delay_s": cols[5]}), tmp_path / "packets.parquet")
    for nid, events in agents.items():
        (tmp_path / f"agent_{nid}.jsonl").write_text(
            "\n".join(json.dumps(e) for e in events))
    return D.load_run(tmp_path)


def test_config_summary_and_geometry(tmp_path):
    run = _make_run(tmp_path, positions=[(0.0, "v1", 1.0, 2.0, 0.0)],
                    packets=[], agents={})
    assert run.config["transport"] == "udp" and run.config["sync_mode"] == "state"
    assert run.extent == (1000, 1000)
    assert run.roles == {"v1": "vehicle", "cmd": "command"}


def test_link_quality_bins(tmp_path):
    run = _make_run(tmp_path, positions=[], agents={}, packets=[
        (0.1, "v1", "cmd", 64, "delivered", 0.01),
        (0.4, "v1", "cmd", 64, "jam", 0.0),
        (0.9, "v1", "cmd", 64, "delivered", 0.01),
        (0.5, "v1", "cmd", 64, "no_link", 0.0),  # excluded
    ])
    lq = D.link_quality(run, window_s=1.0)
    row = lq[(lq["src"] == "v1") & (lq["dst"] == "cmd")].iloc[0]
    assert row["attempts"] == 3 and row["delivered"] == 2
    assert abs(row["pdr"] - 2 / 3) < 1e-9


def test_aoi_series_sawtooth_and_stale_skip(tmp_path):
    run = _make_run(tmp_path, positions=[], packets=[], agents={"cmd": [
        {"type": "start", "id": "cmd", "ts_us": 0},
        {"type": "recv", "from": "v1", "peer_ts_us": 900_000, "ts_us": 1_000_000},
        {"type": "recv", "from": "v1", "peer_ts_us": 800_000, "ts_us": 1_500_000},
        {"type": "recv", "from": "v1", "peer_ts_us": 1_900_000, "ts_us": 2_000_000},
    ]})
    df = D.aoi_series(run, "cmd")
    assert list(df["peer"]) == ["v1", "v1"]            # stale 800k skipped
    assert df.iloc[0]["aoi_s"] == 0.1 and df.iloc[0]["t"] == 1.0


def test_events_timeline_orders_jammer_and_links(tmp_path):
    run = _make_run(tmp_path, positions=[], packets=[],
                    scenario_extra={"jammers": [
                        {"id": "j1", "kind": "barrage", "tx_power_dbm": 40,
                         "start_s": 20.0, "stop_s": None}],
                        "satellite": {"enabled": True,
                                      "outages": [{"start_s": 15.0, "stop_s": None}]}},
                    agents={"v1": [
                        {"type": "start", "id": "v1", "ts_us": 0},
                        {"type": "link_down", "id": "v1", "ts_us": 22_000_000}]})
    ev = D.events_timeline(run)
    kinds = list(ev["kind"])
    assert "sat_outage" in kinds and "jammer_on" in kinds and "link_down" in kinds
    assert ev["t"].is_monotonic_increasing
