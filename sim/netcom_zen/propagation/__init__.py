from __future__ import annotations

import math
from dataclasses import dataclass

from ..terrain import Terrain
from .diffraction import knife_edge_loss_db
from .foliage import weissberger_db
from .freespace import baseline_db


@dataclass(frozen=True)
class PathlossBreakdown:
    fspl_db: float
    terrain_db: float
    foliage_db: float

    @property
    def total_db(self) -> float:
        return self.fspl_db + self.terrain_db + self.foliage_db


class CompositePathloss:
    """PathlossProvider: (tx_xy, rx_xy, freq_hz) -> breakdown. EMANE-shaped (ADR-0001)."""

    def __init__(self, terrain: Terrain, antenna_height_m: float = 1.5):
        self.terrain = terrain
        self.h_ant = antenna_height_m

    def loss(self, tx: tuple[float, float], rx: tuple[float, float],
             f_hz: float) -> PathlossBreakdown:
        d = math.hypot(rx[0] - tx[0], rx[1] - tx[1])
        fspl = baseline_db(d, f_hz, self.h_ant, self.h_ant)
        prof = self.terrain.profile(tx, rx)
        terr = knife_edge_loss_db(prof, d, self.h_ant, self.h_ant, f_hz)
        fol = weissberger_db(f_hz, self.terrain.foliage_depth(tx, rx))
        return PathlossBreakdown(fspl, terr, fol)
