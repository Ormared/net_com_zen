import json
from pathlib import Path
from types import SimpleNamespace

import pytest
from pydantic import ValidationError

from netcom_zen.config import Scenario
from netcom_zen.harness.centralized_lab import (
    _profile_cells,
    centralized_lan_scenario,
    centralized_scenario,
    generate,
)
from netcom_zen.harness.report import centralized_run_metrics
from netcom_zen.harness.sweep import expand
from netcom_zen.orchestrator import (
    ScenarioEngine,
    _cyclonedds_xml,
)


def test_single_requires_one_known_server_and_client():
    scenario = Scenario.model_validate(centralized_scenario(2))
    assert scenario.ros2.centralized.server_ids == ["s1"]
    assert len(scenario.nodes) == 3

    bad = centralized_scenario(2)
    bad["ros2"]["centralized"]["server_ids"] = ["missing"]
    bad["ros2"]["centralized"]["primary_server_id"] = "missing"
    with pytest.raises(ValidationError, match="not scenario nodes"):
        Scenario.model_validate(bad)


def test_active_passive_defaults_activation_to_failure():
    scenario = Scenario.model_validate(
        centralized_scenario(2, mode="active_passive"))
    central = scenario.ros2.centralized
    assert central.failure_at_s == 45.0
    assert central.standby_activate_after_s == 45.0


def test_centralized_roles_and_shards(tmp_path):
    scenario = Scenario.model_validate(
        centralized_scenario(3, mode="sharded"))
    engine = ScenarioEngine(scenario, tmp_path)
    engine.topo = SimpleNamespace(
        nodes=[n.id for n in scenario.nodes],
        ns_names={n.id: f"ncz-{n.id}" for n in scenario.nodes},
    )
    assert engine._workload_role("s1")[0] == "server"
    assert engine._workload_role("s2")[0] == "server"
    assert engine._workload_role("c1")[1] == "s1"
    assert engine._workload_role("c2")[1] == "s2"
    assert engine._workload_role("c3")[1] == "s1"
    cmd = engine._workload_cmd("c2")
    assert cmd[cmd.index("--topology") + 1] == "centralized"
    assert cmd[cmd.index("--assigned-server") + 1] == "s2"
    lan_scenario = Scenario.model_validate(
        centralized_lan_scenario(3, mode="sharded", inverted=True))
    lan_engine = ScenarioEngine(lan_scenario, tmp_path)
    script = lan_engine._lan_launcher_script("mini", {
        node.id: {} for node in lan_scenario.nodes})
    assert "--role server" in script
    assert "--role client" in script
    assert "--central-mode sharded" in script
    assert "--assigned-server s2" in script


def test_cyclone_static_peers_disable_multicast():
    xml = _cyclonedds_xml(
        peers=["10.99.0.1", "10.99.0.2"], allow_multicast=False)
    assert "<AllowMulticast>false</AllowMulticast>" in xml
    assert '<Peer Address="10.99.0.1"/>' in xml
    assert '<Peer Address="10.99.0.2"/>' in xml


def test_conditional_profiles_and_zenoh_tuning_are_not_cross_products(
        tmp_path):
    cells = _profile_cells(None)
    assert len(cells) == 6
    assert {(c["ros2.rmw"], c["ros2.centralized.profile_label"])
            for c in cells} == {
                (rmw, label)
                for rmw in ("fastrtps", "cyclonedds", "zenoh")
                for label in ("stock", "tuned")}

    generate(tmp_path)
    tune = __import__("yaml").safe_load(
        (tmp_path / "tune_zenoh_n32_sweep.yaml").read_text())
    expanded = expand(tune)
    assert len(expanded) == 6
    assert all("ros2.centralized.discovery_mode" not in overrides
               for overrides, _ in expanded)


def _write_events(path: Path, rows: list[dict]) -> None:
    path.write_text("".join(json.dumps(row) + "\n" for row in rows))


def test_centralized_metrics_keep_rtt_and_failover_local(tmp_path):
    scenario = centralized_scenario(
        2, mode="active_active", duration_s=90, failure_at_s=45)
    (tmp_path / "manifest.json").write_text(json.dumps({
        "scenario": scenario,
        "substrate": "bridge",
        "topology": "centralized",
        "rmw": "zenoh",
    }))
    _write_events(tmp_path / "agent_s1.jsonl", [
        {"type": "start", "id": "s1", "ts_us": 0},
        {"type": "telemetry_recv", "id": "s1", "from": "c1",
         "peer_seq": 1, "peer_ts_us": 1, "ts_us": 2, "bytes": 1},
        {"type": "telemetry_recv", "id": "s1", "from": "c2",
         "peer_seq": 1, "peer_ts_us": 1, "ts_us": 2, "bytes": 1},
        {"type": "command_pub", "id": "s1", "seq": 1,
         "ts_us": 1, "bytes": 1},
    ])
    _write_events(tmp_path / "agent_s2.jsonl", [
        {"type": "start", "id": "s2", "ts_us": 0},
        {"type": "telemetry_recv", "id": "s2", "from": "c1",
         "peer_seq": 1, "peer_ts_us": 1, "ts_us": 2, "bytes": 1},
        {"type": "telemetry_recv", "id": "s2", "from": "c2",
         "peer_seq": 1, "peer_ts_us": 1, "ts_us": 2, "bytes": 1},
        {"type": "command_pub", "id": "s2", "seq": 1,
         "ts_us": 1, "bytes": 1},
    ])
    for client in ("c1", "c2"):
        _write_events(tmp_path / f"agent_{client}.jsonl", [
            {"type": "start", "id": client, "ts_us": 0},
            {"type": "telemetry_pub", "id": client, "seq": 1,
             "ts_us": 1, "bytes": 1},
            {"type": "command_recv", "id": client, "from": "s1",
             "peer_seq": 1, "peer_ts_us": 1, "ts_us": 40_000_000,
             "bytes": 1},
            {"type": "command_duplicate", "id": client, "from": "s2",
             "peer_seq": 1, "peer_ts_us": 1, "ts_us": 40_001_000,
             "bytes": 1},
            {"type": "command_recv", "id": client, "from": "s2",
             "peer_seq": 2, "peer_ts_us": 1, "ts_us": 46_000_000,
             "bytes": 1},
            {"type": "rpc_start", "id": client, "seq": 1,
             "ts_us": 10_000},
            {"type": "rpc_success", "id": client, "seq": 1,
             "server": "s2:1", "started_us": 10_000, "ts_us": 20_000},
        ])
    metrics = centralized_run_metrics(tmp_path)
    assert metrics["telemetry_delivery_ratio"] == 1.0
    assert metrics["command_client_coverage"] == 1.0
    assert metrics["command_duplicate_ratio"] == pytest.approx(1 / 3)
    assert metrics["rpc_success_ratio"] == 1.0
    assert metrics["rpc_rtt_p50_ms"] == 10.0
    assert metrics["command_failover_gap_s"] == pytest.approx(5.999)
    assert metrics["discovery_mode"] == "centralized"
    assert metrics["telemetry_reliability"] == "best_effort"
    assert metrics["socket_buffer_bytes"] == 0
