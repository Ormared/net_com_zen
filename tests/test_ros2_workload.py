"""R3 wiring B: payload codec, config plumbing (default env), and the
end-to-end netns integration smoke (sudo + ros2 env only)."""
import asyncio
import json
import sys
from pathlib import Path

import pytest

from netcom_zen.config import Scenario, load_scenario
from netcom_zen.ros2_workload.payload import pack, unpack

# --- payload codec (any env) ---


def test_payload_roundtrip_and_size():
    data = pack("v1", 42, 1_700_000_000_000_000, 255)
    assert len(data) == 255
    assert unpack(data) == ("v1", 42, 1_700_000_000_000_000)


def test_payload_never_truncates_header():
    data = pack("vehicle-with-long-id", 1, 0, 10)  # target smaller than header
    assert unpack(data)[0] == "vehicle-with-long-id"


# --- config plumbing (any env) ---


def test_workload_defaults_agent():
    sc = load_scenario("scenarios/smoke_2node.yaml")
    assert sc.workload == "agent"
    assert sc.ros2.reliability == "reliable"


def test_workload_ros2_parses():
    sc = load_scenario("scenarios/ros2_vs_agent_4node.yaml")
    cfg = sc.model_dump()
    cfg["workload"] = "ros2"
    sc2 = Scenario.model_validate(cfg)
    assert sc2.workload == "ros2" and sc2.ros2.payload_bytes == 255


def test_invalid_workload_rejected():
    sc = load_scenario("scenarios/smoke_2node.yaml").model_dump()
    sc["workload"] = "dds"
    with pytest.raises(ValueError):
        Scenario.model_validate(sc)


# --- integration: ROS2 telemetry through the emulated channel (sudo + ros2 env)


ZENOHD = Path(sys.executable).resolve().parents[1] / "lib/rmw_zenoh_cpp/rmw_zenohd"


@pytest.mark.sudo
@pytest.mark.skipif(not ZENOHD.exists(),
                    reason="rmw_zenohd not in this env (use the ros2 pixi env)")
def test_ros2_pubsub_through_channel(tmp_path):
    from netcom_zen.harness.aoi import aoi_summary
    from netcom_zen.orchestrator import ScenarioEngine

    cfg = load_scenario("scenarios/smoke_2node.yaml").model_dump()
    cfg.update(workload="ros2", duration_s=15)
    engine = ScenarioEngine(Scenario.model_validate(cfg), out_dir=tmp_path)
    asyncio.run(engine.run())

    manifest = json.loads((tmp_path / "manifest.json").read_text())
    assert all(code == 0 for code in manifest["agent_exit_codes"].values())

    summary = aoi_summary(tmp_path)
    for obs, peer in (("v1", "v2"), ("v2", "v1")):
        assert peer in summary[obs], f"{obs} never heard {peer}"
        s = summary[obs][peer]
        assert s["n_updates"] >= 5, f"{obs}<-{peer}: {s['n_updates']} updates"
        assert s["mean_s"] < 2.0, f"{obs}<-{peer} mean AoI {s['mean_s']:.2f}s"
