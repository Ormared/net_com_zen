"""Real Isaac Sim stepper smoke (plan R4 exit): runs only in the isaac pixi
env (`pixi run -e isaac isaac-smoke`); skipped wherever isaacsim is absent.
First run downloads extensions and takes ~a minute to boot."""
import math
import subprocess
import sys

import pytest

pytest.importorskip("isaacsim")

from netcom_zen.isaac_mobility import IsaacMobilityProvider  # noqa: E402
from netcom_zen.mobility import WaypointVehicle  # noqa: E402


class Node:
    def __init__(self, id, waypoints, speed_mps=10.0):
        self.id, self.waypoints, self.speed_mps = id, waypoints, speed_mps


def test_isaac_poses_track_waypoint_kinematics(tmp_path):
    sock = tmp_path / "isaac.sock"
    proc = subprocess.Popen(
        [sys.executable, "-m", "netcom_zen.isaac_stepper",
         "--backend", "isaac", "--socket", str(sock)])
    try:
        nodes = [Node("v1", [(0, 0), (200, 0)]),
                 Node("v2", [(0, 20), (200, 20)])]
        prov = IsaacMobilityProvider(nodes, sock, physics_hz=60.0)
        ref = WaypointVehicle([(0, 0), (200, 0)], speed_mps=10.0)
        for _ in range(50):  # 5 sim-seconds
            poses = prov.step(0.1)
            ref.step(0.1)
        # exit criterion: equivalent dynamics at matched trajectories — PhysX
        # integration (friction, contacts) may lag ideal kinematics a little,
        # but must track within a couple of metres over 50 m
        assert math.hypot(poses["v1"].x - ref.x, poses["v1"].y - ref.y) < 3.0
        assert poses["v2"].x > 30.0  # second body moves independently
        prov.close()
    finally:
        proc.terminate()
        proc.wait(timeout=30)
