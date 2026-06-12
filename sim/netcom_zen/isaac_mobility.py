"""Isaac Sim as MobilityProvider (plan R4, ADR-0006).

The stepper process (``netcom_zen.isaac_stepper``, isaac pixi env) owns the
headless SimulationApp; this client drives it in lockstep over a
newline-delimited-JSON unix socket — the "leaner IPC" path from the plan,
chosen over ROS 2 Simulation Control services so the orchestrator (root, any
env) needs neither rclpy nor simulation_interfaces, and so tests can swap in
a kinematic fake behind the identical protocol.

Protocol (one JSON object per line, client speaks first):
    -> {"cmd": "init", "physics_dt": 0.0166,
        "vehicles": {id: {"waypoints": [[x, y], ...], "speed_mps": 5.0}}}
    <- {"ok": true, "poses": {id: [x, y, heading]}}
    -> {"cmd": "step", "frames": 6}
    <- {"ok": true, "poses": {id: [x, y, heading]}}
    -> {"cmd": "close"}
    <- {"ok": true}            # stepper keeps serving (sweeps reuse it)
"""
from __future__ import annotations

import json
import socket
import time
from pathlib import Path

from .mobility import Pose

CONNECT_TIMEOUT_S = 60.0   # Isaac startup is slow; the stepper may still load
STEP_TIMEOUT_S = 30.0


class IsaacStepperError(RuntimeError):
    pass


class IsaacMobilityProvider:
    name = "isaac"
    seed_exact = False  # PhysX-integrated trajectories (ADR-0006 consequence)

    def __init__(self, nodes, socket_path: str | Path,
                 physics_hz: float = 60.0):
        self.physics_hz = physics_hz
        self._sock = self._connect(Path(socket_path))
        self._rx = self._sock.makefile("r", encoding="utf-8")
        reply = self._request({
            "cmd": "init",
            "physics_dt": 1.0 / physics_hz,
            "vehicles": {n.id: {"waypoints": [list(w) for w in n.waypoints],
                                "speed_mps": n.speed_mps}
                         for n in nodes}})
        self._poses = self._parse(reply)

    @staticmethod
    def _connect(path: Path) -> socket.socket:
        deadline = time.monotonic() + CONNECT_TIMEOUT_S
        while True:
            try:
                s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                s.connect(str(path))
                s.settimeout(STEP_TIMEOUT_S)
                return s
            except (FileNotFoundError, ConnectionRefusedError):
                if time.monotonic() > deadline:
                    raise IsaacStepperError(
                        f"no Isaac stepper listening on {path} — start it "
                        "first: pixi run -e isaac isaac-stepper") from None
                time.sleep(0.5)

    def _request(self, msg: dict) -> dict:
        self._sock.sendall((json.dumps(msg) + "\n").encode())
        line = self._rx.readline()
        if not line:
            raise IsaacStepperError("Isaac stepper closed the connection")
        reply = json.loads(line)
        if not reply.get("ok"):
            raise IsaacStepperError(reply.get("error", "stepper error"))
        return reply

    @staticmethod
    def _parse(reply: dict) -> dict[str, Pose]:
        return {nid: Pose(x, y, heading)
                for nid, (x, y, heading) in reply["poses"].items()}

    def poses(self) -> dict[str, Pose]:
        return dict(self._poses)

    def step(self, dt: float) -> dict[str, Pose]:
        frames = round(dt * self.physics_hz)
        reply = self._request({"cmd": "step", "frames": frames})
        self._poses = self._parse(reply)
        return dict(self._poses)

    def close(self) -> None:
        try:
            self._request({"cmd": "close"})
        except (OSError, IsaacStepperError):
            pass  # already gone; nothing to clean up on our side
        # both must close: makefile() dups the fd, and a half-open socket
        # would keep the stepper blocked on this client instead of accepting
        # the next run
        self._rx.close()
        self._sock.close()
