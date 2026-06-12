"""Socket server side of the isaac_mobility protocol (see that module's
docstring for the wire format). Backend-agnostic: a backend is anything with
poses() -> {id: [x, y, heading]}, step(frames) -> poses, close().

One client at a time (the orchestrator); the server outlives runs so a sweep
reuses a single Isaac process — init rebuilds the scene, close tears it down.
"""
from __future__ import annotations

import json
import socket
from pathlib import Path


class StepperServer:
    def __init__(self, socket_path: str | Path, backend_factory):
        self.path = Path(socket_path)
        self.backend_factory = backend_factory  # (physics_dt, vehicles) -> backend
        self.backend = None

    def serve_forever(self) -> None:
        self.path.unlink(missing_ok=True)
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as srv:
            srv.bind(str(self.path))
            self.path.chmod(0o666)  # root orchestrator vs user-owned stepper
            srv.listen(1)
            print(f"stepper: listening on {self.path}", flush=True)
            while True:
                conn, _ = srv.accept()
                self._serve_client(conn)

    def _serve_client(self, conn: socket.socket) -> None:
        with conn, conn.makefile("r", encoding="utf-8") as rx:
            for line in rx:
                try:
                    reply = self._handle(json.loads(line))
                except Exception as e:  # a bad run must not kill the stepper
                    reply = {"ok": False, "error": f"{type(e).__name__}: {e}"}
                conn.sendall((json.dumps(reply) + "\n").encode())

    def _handle(self, msg: dict) -> dict:
        cmd = msg.get("cmd")
        if cmd == "init":
            if self.backend is not None:
                self.backend.close()
            self.backend = self.backend_factory(msg["physics_dt"],
                                                msg["vehicles"])
            return {"ok": True, "poses": self.backend.poses()}
        if cmd == "step":
            return {"ok": True, "poses": self.backend.step(int(msg["frames"]))}
        if cmd == "close":
            if self.backend is not None:
                self.backend.close()
                self.backend = None
            return {"ok": True}
        raise ValueError(f"unknown cmd {cmd!r}")
