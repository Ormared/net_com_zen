from __future__ import annotations

import math
from dataclasses import dataclass
from typing import TYPE_CHECKING, Protocol

if TYPE_CHECKING:
    from .config import Scenario


@dataclass
class Pose:
    x: float
    y: float
    heading: float  # radians, CCW from +x


class MobilityProvider(Protocol):
    """World-level mobility seam (ADR-0002/0006): one step per scenario tick
    advances every vehicle and returns the new poses."""

    name: str
    seed_exact: bool  # same seed => bit-identical trajectories?

    def poses(self) -> dict[str, Pose]:
        """Current poses without advancing (initial link table needs them)."""
        ...

    def step(self, dt: float) -> dict[str, Pose]: ...

    def close(self) -> None: ...


class WaypointVehicle:
    """Turn-rate-limited unicycle following waypoints in a loop.

    MobilityProvider seam (ADR-0002): anything with step(dt) -> Pose fits.
    """

    ARRIVE_M = 2.0

    def __init__(self, waypoints, speed_mps: float = 5.0,
                 max_turn_rate: float = math.radians(60)):
        self.waypoints = [tuple(map(float, w)) for w in waypoints]
        self.speed = speed_mps
        self.max_turn = max_turn_rate
        self.x, self.y = self.waypoints[0]
        self._target = 1 % len(self.waypoints)
        self.heading = self._bearing_to(self.waypoints[self._target])

    def _bearing_to(self, wp) -> float:
        return math.atan2(wp[1] - self.y, wp[0] - self.x)

    def step(self, dt: float) -> Pose:
        wp = self.waypoints[self._target]
        if math.hypot(wp[0] - self.x, wp[1] - self.y) < self.ARRIVE_M:
            self._target = (self._target + 1) % len(self.waypoints)
            wp = self.waypoints[self._target]
        err = (self._bearing_to(wp) - self.heading + math.pi) % (2 * math.pi) - math.pi
        lim = self.max_turn * dt
        self.heading += max(-lim, min(lim, err))
        self.x += self.speed * dt * math.cos(self.heading)
        self.y += self.speed * dt * math.sin(self.heading)
        return Pose(self.x, self.y, self.heading)


class WaypointMobility:
    """Default MobilityProvider: one WaypointVehicle per node, closed-form
    kinematics, seed-exact (trajectories depend only on the scenario)."""

    name = "waypoint"
    seed_exact = True

    def __init__(self, nodes):
        self._vehicles = {n.id: WaypointVehicle(n.waypoints, n.speed_mps)
                          for n in nodes}

    def poses(self) -> dict[str, Pose]:
        return {nid: Pose(v.x, v.y, v.heading)
                for nid, v in self._vehicles.items()}

    def step(self, dt: float) -> dict[str, Pose]:
        return {nid: v.step(dt) for nid, v in self._vehicles.items()}

    def close(self) -> None:
        pass


def make_mobility(scenario: "Scenario") -> MobilityProvider:
    if scenario.mobility.provider == "isaac":
        from .isaac_mobility import IsaacMobilityProvider
        return IsaacMobilityProvider(
            scenario.nodes, socket_path=scenario.mobility.socket,
            physics_hz=scenario.mobility.physics_hz,
            extent=scenario.environment.extent_m)
    return WaypointMobility(scenario.nodes)
