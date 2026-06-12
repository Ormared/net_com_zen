from __future__ import annotations

import math
from dataclasses import dataclass

import numpy as np


@dataclass(frozen=True)
class FoliageRegion:
    x_min: float
    x_max: float
    y_min: float
    y_max: float


class Terrain:
    """Heightmap + land-cover. Flat ground (z=0) when no heightmap given."""

    def __init__(self, extent_m: tuple[float, float],
                 heightmap: np.ndarray | None = None,
                 foliage: list[FoliageRegion] | None = None):
        self.extent = extent_m
        self.heightmap = heightmap
        self.foliage = list(foliage or [])

    def height(self, x: float, y: float) -> float:
        if self.heightmap is None:
            return 0.0
        h, w = self.heightmap.shape
        i = min(max(int(round(y / self.extent[1] * (h - 1))), 0), h - 1)
        j = min(max(int(round(x / self.extent[0] * (w - 1))), 0), w - 1)
        return float(self.heightmap[i, j])

    def profile(self, p1: tuple[float, float], p2: tuple[float, float],
                n: int = 64) -> np.ndarray:
        xs = np.linspace(p1[0], p2[0], n)
        ys = np.linspace(p1[1], p2[1], n)
        return np.array([self.height(x, y) for x, y in zip(xs, ys)])

    def foliage_depth(self, p1: tuple[float, float], p2: tuple[float, float]) -> float:
        """Total length of segment p1->p2 inside foliage regions (Liang-Barsky)."""
        dx, dy = p2[0] - p1[0], p2[1] - p1[1]
        seg = math.hypot(dx, dy)
        if seg == 0:
            return 0.0
        total = 0.0
        for r in self.foliage:
            t0, t1 = 0.0, 1.0
            inside = True
            for p, q in ((-dx, p1[0] - r.x_min), (dx, r.x_max - p1[0]),
                         (-dy, p1[1] - r.y_min), (dy, r.y_max - p1[1])):
                if p == 0:
                    if q < 0:
                        inside = False
                        break
                else:
                    t = q / p
                    if p < 0:
                        t0 = max(t0, t)
                    else:
                        t1 = min(t1, t)
            if inside and t0 < t1:
                total += (t1 - t0) * seg
        return total
