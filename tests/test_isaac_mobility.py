"""IsaacMobilityProvider against the stepper protocol (plan R4).

The kinematic stepper backend stands in for Isaac: same server, same wire
format, no GPU — so the orchestrator-side client is fully exercised in the
default env. The real-Isaac smoke lives in test_isaac_real.py.
"""
import math
import subprocess
import sys

import pytest

from netcom_zen.config import load_scenario
from netcom_zen.isaac_mobility import IsaacMobilityProvider, IsaacStepperError
from netcom_zen.mobility import Pose, WaypointMobility, make_mobility


class Node:
    def __init__(self, id, waypoints, speed_mps=10.0):
        self.id, self.waypoints, self.speed_mps = id, waypoints, speed_mps


NODES = [Node("v1", [(0, 0), (100, 0)]), Node("v2", [(50, 50)], speed_mps=0.1)]


@pytest.fixture
def stepper(tmp_path):
    sock = tmp_path / "stepper.sock"
    proc = subprocess.Popen(
        [sys.executable, "-m", "netcom_zen.isaac_stepper",
         "--backend", "kinematic", "--socket", str(sock)])
    yield sock
    proc.terminate()
    proc.wait(timeout=5)


def test_init_poses_at_first_waypoint(stepper):
    prov = IsaacMobilityProvider(NODES, stepper, physics_hz=60.0)
    poses = prov.poses()
    assert set(poses) == {"v1", "v2"}
    assert isinstance(poses["v1"], Pose)
    assert poses["v1"].x == 0.0 and poses["v1"].y == 0.0
    assert poses["v2"].x == 50.0 and poses["v2"].y == 50.0
    prov.close()


def test_step_advances_toward_waypoint(stepper):
    prov = IsaacMobilityProvider(NODES, stepper, physics_hz=60.0)
    p = prov.step(0.1)["v1"]  # 6 physics frames @ 60 Hz
    assert math.isclose(p.x, 1.0, rel_tol=0.05)  # 10 m/s * 0.1 s, eastbound
    assert abs(p.y) < 1e-6
    # ten more ticks: cumulative progress, poses() reflects the last step
    for _ in range(10):
        prov.step(0.1)
    assert math.isclose(prov.poses()["v1"].x, 11.0, rel_tol=0.05)
    prov.close()


def test_stepper_survives_reinit(stepper):
    # a sweep reuses one stepper process across runs: close, then init again
    for _ in range(2):
        prov = IsaacMobilityProvider(NODES, stepper, physics_hz=60.0)
        assert prov.step(0.1)["v1"].x > 0.0
        prov.close()


def test_missing_stepper_raises(tmp_path, monkeypatch):
    monkeypatch.setattr("netcom_zen.isaac_mobility.CONNECT_TIMEOUT_S", 0.2)
    with pytest.raises(IsaacStepperError, match="no Isaac stepper"):
        IsaacMobilityProvider(NODES, tmp_path / "nope.sock")


def test_make_mobility_selects_provider():
    sc = load_scenario("scenarios/smoke_2node.yaml")
    prov = make_mobility(sc)
    assert isinstance(prov, WaypointMobility)
    assert prov.name == "waypoint" and prov.seed_exact is True
    assert IsaacMobilityProvider.seed_exact is False  # manifest contract


def test_physics_hz_must_divide_tick():
    sc = load_scenario("scenarios/smoke_2node.yaml")
    cfg = sc.model_dump(mode="json")
    cfg["mobility"] = {"provider": "isaac", "physics_hz": 25.0}  # 25/10 = 2.5
    with pytest.raises(ValueError, match="integer multiple"):
        type(sc).model_validate(cfg)
