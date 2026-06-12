from __future__ import annotations

import math
import zlib
from dataclasses import dataclass

import numpy as np

from ..config import RadioProfile
from ..ew import Jammer
from ..propagation import CompositePathloss
from .fhss import dwells_per_packet
from .linkbudget import dbm_to_mw, fsk_per, noise_dbm, per_threshold_sinr_db, sinr_db


def link_rng(seed: int, src: str, dst: str) -> np.random.Generator:
    """Deterministic per-link RNG stream (replay mechanism, benchmarking.md)."""
    return np.random.default_rng([seed, zlib.crc32(f"{src}>{dst}".encode())])


@dataclass(frozen=True)
class LinkState:
    src: str
    dst: str
    prx_dbm: float
    noise_dbm: float
    rho: float                       # fraction of hop channels jammed
    jam_inchannel_dbm: float | None  # received jammer power in a jammed channel
    data_rate_bps: float
    hop_rate_hz: float
    prop_delay_s: float
    foliage_db: float
    terrain_db: float
    medium: str = "rf"        # "rf" | "sat" (M4 satellite overlay)
    sat_loss: float = 0.0     # clear-sky packet loss for a satellite link

    def verdict(self, length_bytes: int, u_thermal: float, u_jam: float) -> str:
        """'deliver' or drop cause. u_*: uniforms from the seeded per-link stream."""
        if self.medium == "sat":
            # bent-pipe uplink: flat loss, immune to the RF jammer (different band)
            return "deliver" if u_thermal >= self.sat_loss else "satloss"
        s_clear = sinr_db(self.prx_dbm, self.noise_dbm)
        if u_thermal < fsk_per(s_clear, length_bytes):
            return "foliage" if self.foliage_db > max(self.terrain_db, 3.0) else "range"
        if self.rho > 0 and self.jam_inchannel_dbm is not None:
            s_jam = sinr_db(self.prx_dbm, self.noise_dbm, [self.jam_inchannel_dbm])
            lost = 1.0 if s_jam < per_threshold_sinr_db(length_bytes) else 0.0
            k = dwells_per_packet(length_bytes, self.data_rate_bps, self.hop_rate_hz)
            if u_jam < 1.0 - (1.0 - self.rho * lost) ** k:
                return "jam"
        return "deliver"


def _sat_link(src: str, dst: str, sat) -> LinkState:
    """A satellite (bent-pipe) link overriding the RF model on this pair."""
    return LinkState(
        src=src, dst=dst, prx_dbm=0.0, noise_dbm=0.0, rho=0.0,
        jam_inchannel_dbm=None, data_rate_bps=sat.bandwidth_bps,
        hop_rate_hz=1.0, prop_delay_s=sat.delay_ms / 1e3,
        foliage_db=0.0, terrain_db=0.0, medium="sat", sat_loss=sat.loss)


def build_table(positions: dict[str, tuple[float, float]], jammers: list[Jammer],
                t: float, radio: RadioProfile, pathloss: CompositePathloss,
                command_id: str | None = None, satellite=None
                ) -> dict[tuple[str, str], LinkState]:
    """Per-tick directed link-state table. M1 simplification: the single
    highest-impact jammer per link is applied (documented in models.md).

    M4: while the satellite is up, vehicle<->command pairs use the satellite
    overlay (low latency, jammer-immune); during an outage they fall back to the
    RF mesh computed here, exposing them to range/foliage/jamming."""
    n0 = noise_dbm(radio.bandwidth_hz, radio.noise_figure_db)
    active = [j for j in jammers if j.active(t)]
    sat_up = satellite is not None and satellite.enabled and satellite.active(t)
    table: dict[tuple[str, str], LinkState] = {}
    for src, sp in positions.items():
        for dst, dp in positions.items():
            if src == dst:
                continue
            if sat_up and command_id in (src, dst):
                table[(src, dst)] = _sat_link(src, dst, satellite)
                continue
            bd = pathloss.loss(sp, dp, radio.freq_hz)
            prx = radio.tx_power_dbm + 2 * radio.antenna_gain_dbi - bd.total_db
            rho, jam_dbm, impact = 0.0, None, 0.0
            for j in active:
                r, frac = j.occupancy(radio.hop.n_channels, radio.bandwidth_hz)
                jl = pathloss.loss(j.position, dp, radio.freq_hz)
                p = j.tx_power_dbm - jl.total_db + 10 * math.log10(frac)
                if r * dbm_to_mw(p) > impact:
                    rho, jam_dbm, impact = r, p, r * dbm_to_mw(p)
            d = math.hypot(dp[0] - sp[0], dp[1] - sp[1])
            table[(src, dst)] = LinkState(
                src=src, dst=dst, prx_dbm=prx, noise_dbm=n0, rho=rho,
                jam_inchannel_dbm=jam_dbm, data_rate_bps=radio.data_rate_bps,
                hop_rate_hz=radio.hop.hop_rate_hz, prop_delay_s=d / 3e8,
                foliage_db=bd.foliage_db, terrain_db=bd.terrain_db)
    return table
