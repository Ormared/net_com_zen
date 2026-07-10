"""Deterministic heightmap for the Sionna-vs-analytical terrain case (M5.3):
a Gaussian ridge running north-south across the middle of a 1000 x 1000 m map.

    pixi run python scripts/make_hill_heightmap.py

Writes scenarios/data/hill_ridge_128.npy (float32, row=y, col=x — the
Terrain convention).
"""
from pathlib import Path

import numpy as np

N = 128
EXTENT = 1000.0
RIDGE_X = 500.0
HEIGHT_M = 45.0
SIGMA_M = 60.0

x = np.linspace(0.0, EXTENT, N)
ridge = HEIGHT_M * np.exp(-((x - RIDGE_X) ** 2) / (2 * SIGMA_M**2))
hm = np.tile(ridge.astype(np.float32), (N, 1))  # constant along y

out = Path(__file__).resolve().parent.parent / "scenarios" / "data" / "hill_ridge_128.npy"
out.parent.mkdir(exist_ok=True)
np.save(out, hm)
print(f"wrote {out} max={hm.max():.1f} m")
