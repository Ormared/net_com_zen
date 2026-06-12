from __future__ import annotations

from .config import JammerConfig


class Jammer:
    """Position-aware jammer. occupancy() returns (rho, in_channel_power_fraction):
    rho = fraction of hop channels affected at any instant;
    fraction = share of the jammer's TX power landing inside one affected channel.
    Sweep is modeled statistically as one uniformly-random jammed channel (models.md).
    """

    def __init__(self, cfg: JammerConfig):
        self.cfg = cfg
        self.position = cfg.position
        self.tx_power_dbm = cfg.tx_power_dbm

    def active(self, t: float) -> bool:
        return self.cfg.start_s <= t and (self.cfg.stop_s is None or t < self.cfg.stop_s)

    def occupancy(self, n_channels: int, channel_bw_hz: float) -> tuple[float, float]:
        c = self.cfg
        if c.kind == "spot":
            in_band = {ch for ch in c.channels if 0 <= ch < n_channels}
            return len(in_band) / n_channels, 1.0 / max(len(c.channels), 1)
        if c.kind == "barrage":
            return 1.0, min(1.0, channel_bw_hz / c.bandwidth_hz)
        # sweep
        return 1.0 / n_channels, 1.0
