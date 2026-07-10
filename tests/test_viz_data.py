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


def test_pdr_series_aggregates_over_links(tmp_path):
    run = _make_run(tmp_path, positions=[], agents={}, packets=[
        (0.1, "v1", "cmd", 64, "delivered", 0.01),
        (0.2, "v1", "v2", 64, "jam", 0.0),
        (0.3, "v2", "cmd", 64, "delivered", 0.01),
        (1.1, "v1", "cmd", 64, "jam", 0.0),
    ])
    pdr = D.pdr_series(run, window_s=1.0)
    b0 = pdr[pdr["t"] == 0.0].iloc[0]
    assert b0["attempts"] == 3 and abs(b0["pdr"] - 2 / 3) < 1e-9
    b1 = pdr[pdr["t"] == 1.0].iloc[0]
    assert b1["pdr"] == 0.0


def test_update_delivery_counts_landed_publishes(tmp_path):
    # v1 publishes seq 1,2,3; cmd receives 1 and 3 (2 lost). 2/3 delivery.
    run = _make_run(tmp_path, positions=[], packets=[], agents={
        "v1": [
            {"type": "start", "id": "v1", "ts_us": 0},
            {"type": "pub", "id": "v1", "seq": 1, "ts_us": 1_000_000},
            {"type": "pub", "id": "v1", "seq": 2, "ts_us": 1_500_000},
            {"type": "pub", "id": "v1", "seq": 3, "ts_us": 1_800_000},
        ],
        "cmd": [
            {"type": "start", "id": "cmd", "ts_us": 0},
            {"type": "recv", "from": "v1", "peer_seq": 1,
             "peer_ts_us": 1_000_000, "ts_us": 1_100_000},
            {"type": "recv", "from": "v1", "peer_seq": 3,
             "peer_ts_us": 1_800_000, "ts_us": 1_900_000},
        ],
    })
    upd = D.update_delivery_series(run, window_s=10.0)
    row = upd.iloc[0]
    assert row["published"] == 3 and row["delivered"] == 2
    assert abs(row["delivery"] - 2 / 3) < 1e-9


def test_aoi_mean_time_average(tmp_path):
    # single reset at t=1 (aoi 0.1) over a 10s run: area under sawtooth / 10.
    run = _make_run(tmp_path, positions=[], packets=[], agents={"cmd": [
        {"type": "start", "id": "cmd", "ts_us": 0},
        {"type": "recv", "from": "v1", "peer_ts_us": 900_000, "ts_us": 1_000_000},
    ]})
    m = D.aoi_mean(run, "cmd")
    assert m is not None and m > 0


def test_export_run_is_json_serialisable(tmp_path):
    run = _make_run(tmp_path,
                    positions=[(0.0, "v1", 1.0, 2.0, 0.0),
                               (0.0, "cmd", 5.0, 6.0, 0.0),
                               (0.1, "v1", 1.5, 2.0, 0.1),
                               (0.1, "cmd", 5.0, 6.0, 0.0)],
                    packets=[(0.1, "v1", "cmd", 64, "delivered", 0.01)],
                    scenario_extra={"jammers": [
                        {"id": "j1", "kind": "barrage", "tx_power_dbm": 40,
                         "position": [10.0, 20.0], "start_s": 0.0,
                         "stop_s": None}]},
                    agents={"v1": [{"type": "start", "id": "v1", "ts_us": 0},
                                   {"type": "pub", "id": "v1", "seq": 1,
                                    "ts_us": 500_000}]})
    bundle = D.export_run(run, stride=1)
    json.dumps(bundle)  # must not raise
    assert bundle["extent"] == [1000, 1000]
    assert len(bundle["frames"]) == 2
    f0 = bundle["frames"][0]
    assert {n["id"] for n in f0["nodes"]} == {"v1", "cmd"}
    assert f0["jammers"][0]["active"] is True
    assert bundle["drop_attribution"] == {"delivered": 1}


def _write_linkstate(tmp_path, rows):
    cols = list(zip(*rows))
    pq.write_table(pa.table({
        "t": cols[0], "src": cols[1], "dst": cols[2], "prx_dbm": cols[3],
        "noise_dbm": cols[4], "rho": cols[5], "jam_inchannel_dbm": cols[6],
        "delivery_prob": cols[7], "foliage_db": cols[8], "terrain_db": cols[9],
        "medium": cols[10]}), tmp_path / "linkstate.parquet")


def test_link_state_truth_bins_and_covers_idle_links(tmp_path):
    run = _make_run(tmp_path, positions=[], packets=[], agents={})
    # v1->cmd carries no packets at all, but the truth log still rates it
    _write_linkstate(tmp_path, [
        (0.1, "v1", "cmd", -80.0, -113.0, 0.0, float("nan"), 1.0, 0.0, 0.0, "rf"),
        (0.2, "v1", "cmd", -80.0, -113.0, 1.0, -60.0, 0.5, 0.0, 0.0, "rf"),
        (1.1, "v1", "cmd", -80.0, -113.0, 1.0, -60.0, 0.0, 0.0, 0.0, "rf"),
    ])
    run = D.load_run(tmp_path)
    truth = D.link_state_truth(run, window_s=1.0)
    b0 = truth[truth["bin"] == 0.0].iloc[0]
    assert b0["pdr"] == 0.75 and b0["attempts"] == 2  # mean over the window
    assert b0["delivered"] == 1.5  # pdr*attempts: weighted aggs stay correct
    assert truth[truth["bin"] == 1.0].iloc[0]["pdr"] == 0.0
    assert b0["rho"] == 0.5


def test_link_quality_best_prefers_truth_falls_back(tmp_path):
    # no linkstate.parquet -> packet reconstruction
    run = _make_run(tmp_path, positions=[], agents={}, packets=[
        (0.1, "v1", "cmd", 64, "delivered", 0.01)])
    assert D.link_quality_best(run).iloc[0]["pdr"] == 1.0
    # truth log present -> it wins even where packets disagree
    _write_linkstate(tmp_path, [
        (0.1, "v1", "cmd", -80.0, -113.0, 0.0, float("nan"), 0.25, 0.0, 0.0, "rf")])
    run = D.load_run(tmp_path)
    assert D.link_quality_best(run).iloc[0]["pdr"] == 0.25
