"""Kinematic stepper backend: WaypointVehicle math substepped at physics_dt.
Identical wire behavior to the Isaac backend, no GPU/Isaac install — the test
double for IsaacMobilityProvider and a dry-run mode for the stepper CLI."""
from __future__ import annotations

from ..mobility import WaypointVehicle


class KinematicBackend:
    def __init__(self, physics_dt: float, vehicles: dict, extent=None):
        self.dt = physics_dt  # extent is ground-visual only; no scene here
        self._v = {nid: WaypointVehicle(cfg["waypoints"], cfg["speed_mps"])
                   for nid, cfg in vehicles.items()}

    def poses(self) -> dict[str, list[float]]:
        return {nid: [v.x, v.y, v.heading] for nid, v in self._v.items()}

    def step(self, frames: int) -> dict[str, list[float]]:
        for _ in range(frames):
            for v in self._v.values():
                v.step(self.dt)
        return self.poses()

    def close(self) -> None:
        pass
