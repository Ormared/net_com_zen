"""Ray-traced pathloss served from a precomputed Sionna RT grid (plan M5.3).

The heavy ray tracing happens offline in the standalone `sionna` pixi env
(`sionna_precompute`); this module only interpolates the resulting .npz, so
the orchestrator keeps running in the default env with no GPU and no sionna
import.

Grid layout: coarse transmitter positions (tx_y x tx_x axes), each holding a
fine receiver map (cell_y x cell_x of cell centres at antenna height above
the terrain). ``pl_db[iy, ix, cy, cx]`` is the ray-traced pathloss in dB from
tx node (ix, iy) to rx cell (cx, cy) at ``freq_hz``, capped at CAP_DB where
the tracer found no path.
"""
from __future__ import annotations

import json
import math
from dataclasses import dataclass
from pathlib import Path

import numpy as np

from ..terrain import Terrain
from . import PathlossBreakdown
from .foliage import weissberger_db
from .freespace import baseline_db

CAP_DB = 250.0  # "no path found" sentinel; far below any usable link budget


@dataclass(frozen=True)
class SionnaGrid:
    freq_hz: float
    h_ant_m: float
    tx_x: np.ndarray   # (NX,) coarse tx x positions, ascending
    tx_y: np.ndarray   # (NY,)
    cell_x: np.ndarray  # (CX,) rx cell-centre x positions, ascending
    cell_y: np.ndarray  # (CY,)
    pl_db: np.ndarray  # (NY, NX, CY, CX) float32
    meta: dict

    def save(self, path: str | Path) -> None:
        np.savez_compressed(
            path, freq_hz=self.freq_hz, h_ant_m=self.h_ant_m,
            tx_x=self.tx_x, tx_y=self.tx_y,
            cell_x=self.cell_x, cell_y=self.cell_y,
            pl_db=self.pl_db.astype(np.float32),
            meta=json.dumps(self.meta))

    @classmethod
    def load(cls, path: str | Path) -> "SionnaGrid":
        z = np.load(path)
        return cls(freq_hz=float(z["freq_hz"]), h_ant_m=float(z["h_ant_m"]),
                   tx_x=z["tx_x"], tx_y=z["tx_y"],
                   cell_x=z["cell_x"], cell_y=z["cell_y"],
                   pl_db=z["pl_db"], meta=json.loads(str(z["meta"])))

    def lookup_db(self, tx: tuple[float, float], rx: tuple[float, float]) -> float:
        """Quadrilinear interpolation in dB space, clipped to grid edges."""
        ix0, ix1, wx = _axis_weights(self.tx_x, tx[0])
        iy0, iy1, wy = _axis_weights(self.tx_y, tx[1])
        cx0, cx1, ux = _axis_weights(self.cell_x, rx[0])
        cy0, cy1, uy = _axis_weights(self.cell_y, rx[1])
        # rows: the 4 surrounding tx nodes; cols: the 4 surrounding rx cells
        m = self.pl_db[[iy0, iy0, iy1, iy1], [ix0, ix1, ix0, ix1]][
            :, [cy0, cy0, cy1, cy1], [cx0, cx1, cx0, cx1]]
        txw = np.array([(1 - wy) * (1 - wx), (1 - wy) * wx,
                        wy * (1 - wx), wy * wx])
        rxw = np.array([(1 - uy) * (1 - ux), (1 - uy) * ux,
                        uy * (1 - ux), uy * ux])
        return float(txw @ m @ rxw)


def _axis_weights(axis: np.ndarray, v: float) -> tuple[int, int, float]:
    if v <= axis[0]:
        return 0, 0, 0.0
    if v >= axis[-1]:
        n = len(axis) - 1
        return n, n, 0.0
    i1 = int(np.searchsorted(axis, v))
    i0 = i1 - 1
    return i0, i1, (v - axis[i0]) / (axis[i1] - axis[i0])


class SionnaGridPathloss:
    """PathlossProvider (ADR-0001 shape) backed by a SionnaGrid.

    Breakdown semantics: ``fspl_db`` stays the analytical baseline
    (Friis/two-ray) so cross-model comparisons line up; ``terrain_db`` is the
    ray-traced excess over that baseline (negative = constructive multipath);
    foliage stays analytical Weissberger — the RT scene carries no vegetation.
    ``total_db`` is therefore exactly ray-traced loss + Weissberger.

    The lookup is symmetrised (mean of tx->rx and rx->tx in dB): physics is
    reciprocal, and averaging halves the coarse-tx-grid interpolation error.
    """

    def __init__(self, grid: SionnaGrid, terrain: Terrain):
        self.grid = grid
        self.terrain = terrain
        self.h_ant = grid.h_ant_m
        # below the rx-map cell pitch the grid cannot resolve the geometry
        self._min_d = float(min(np.diff(grid.cell_x).min(),
                                np.diff(grid.cell_y).min()))

    def loss(self, tx: tuple[float, float], rx: tuple[float, float],
             f_hz: float) -> PathlossBreakdown:
        if abs(f_hz - self.grid.freq_hz) > 0.01 * self.grid.freq_hz:
            raise ValueError(
                f"sionna grid was traced at {self.grid.freq_hz/1e6:.1f} MHz, "
                f"scenario asks for {f_hz/1e6:.1f} MHz — re-run precompute")
        d = math.hypot(rx[0] - tx[0], rx[1] - tx[1])
        base = baseline_db(d, f_hz, self.h_ant, self.h_ant)
        fol = weissberger_db(f_hz, self.terrain.foliage_depth(tx, rx))
        if d < self._min_d:
            return PathlossBreakdown(base, 0.0, fol)
        rt = 0.5 * (self.grid.lookup_db(tx, rx) + self.grid.lookup_db(rx, tx))
        return PathlossBreakdown(base, rt - base, fol)
