from __future__ import annotations

from .config import JammerConfig


class Jammer:
    """Position-aware jammer. occupancy() returns (rho, in_channel_power_fraction):
    rho = per-dwell probability the jammer's energy lands on the active channel;
    fraction = share of the jammer's TX power landing inside one affected channel.
    Sweep is modeled statistically as one uniformly-random jammed channel (models.md).

    For barrage/spot/sweep rho is the geometric overlap |J ∩ hopset| / N (a random
    dwell falls in the jammed set with this probability). For a reactive follower
    it is instead the lock probability: the jammer is always on the *right* channel
    when it locks, but only catches a dwell if it retunes before the transmitter
    hops away — so rho depends on the hop rate and the sense+tune latency.
    """

    def __init__(self, cfg: JammerConfig):
        self.cfg = cfg
        self.position = cfg.position
        self.tx_power_dbm = cfg.tx_power_dbm

    def active(self, t: float) -> bool:
        return self.cfg.start_s <= t and (self.cfg.stop_s is None or t < self.cfg.stop_s)

    def dwell_exposure(self, hop_rate_hz: float) -> float:
        """Fraction of a dwell exposed to a reactive jammer after its sense+tune
        latency: max(0, (T_dwell - react_latency_s) / T_dwell). Fast hopping
        drives this to 0 (the tx has hopped away before the jammer locks on),
        which is why hop rate is the lever against a follower jammer."""
        if hop_rate_hz <= 0:
            return 0.0
        t_dwell = 1.0 / hop_rate_hz
        return max(0.0, (t_dwell - self.cfg.react_latency_s) / t_dwell)

    def occupancy(self, n_channels: int, channel_bw_hz: float,
                  hop_rate_hz: float | None = None) -> tuple[float, float]:
        c = self.cfg
        if c.kind == "spot":
            in_band = {ch for ch in c.channels if 0 <= ch < n_channels}
            return len(in_band) / n_channels, 1.0 / max(len(c.channels), 1)
        if c.kind == "barrage":
            return 1.0, min(1.0, channel_bw_hz / c.bandwidth_hz)
        if c.kind == "reactive":
            if hop_rate_hz is None:
                raise ValueError("reactive occupancy needs hop_rate_hz")
            # full power on the single active channel; lock probability folds in
            # sensing reliability and the sub-dwell timing race. The source-
            # sensing geometry gate is applied by the caller (build_table).
            return c.detection_prob * self.dwell_exposure(hop_rate_hz), 1.0
        # sweep
        return 1.0 / n_channels, 1.0
