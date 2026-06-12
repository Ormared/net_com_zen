from __future__ import annotations

import math
from dataclasses import dataclass


@dataclass
class Pose:
    x: float
    y: float
    heading: float  # radians, CCW from +x


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
