import json

import pytest

from netcom_zen.harness.failover import failover_times


def write_run(tmp_path, outage_s, command_events):
    (tmp_path / "manifest.json").write_text(json.dumps({
        "scenario": {"satellite": {"outages": [{"start_s": outage_s}]}}}))
    (tmp_path / "agent_cmd.jsonl").write_text(
        "\n".join(json.dumps(e) for e in command_events) + "\n")


def test_failover_time_computed(tmp_path):
    # command starts at t=0us; outage at 20s -> outage_us = 20_000_000
    # v1 update originating at 22s applied at 23s -> failover 3s
    # v2 update originating at 21s applied at 24.5s -> failover 4.5s
    write_run(tmp_path, 20.0, [
        {"type": "start", "id": "cmd", "ts_us": 0},
        # pre-outage updates (ignored)
        {"type": "recv", "from": "v1", "peer_ts_us": 5_000_000, "ts_us": 5_500_000},
        # post-outage recoveries
        {"type": "recv", "from": "v1", "peer_ts_us": 22_000_000, "ts_us": 23_000_000},
        {"type": "recv", "from": "v2", "peer_ts_us": 21_000_000, "ts_us": 24_500_000},
        {"type": "final_state", "ts_us": 45_000_000},
    ])
    r = failover_times(tmp_path, "cmd")
    assert r["per_vehicle_failover_s"] == {"v1": 3.0, "v2": 4.5}
    assert r["swarm_failover_s"] == 4.5
    assert r["vehicles_recovered"] == 2


def test_failover_no_recovery(tmp_path):
    write_run(tmp_path, 20.0, [
        {"type": "start", "id": "cmd", "ts_us": 0},
        {"type": "recv", "from": "v1", "peer_ts_us": 5_000_000, "ts_us": 5_500_000},
        {"type": "final_state", "ts_us": 45_000_000},
    ])
    r = failover_times(tmp_path, "cmd")
    assert r["vehicles_recovered"] == 0
    assert r["swarm_failover_s"] is None


def test_failover_requires_outage(tmp_path):
    (tmp_path / "manifest.json").write_text(json.dumps(
        {"scenario": {"satellite": {"outages": []}}}))
    (tmp_path / "agent_cmd.jsonl").write_text(
        json.dumps({"type": "start", "id": "cmd", "ts_us": 0}) + "\n")
    assert "error" in failover_times(tmp_path, "cmd")
