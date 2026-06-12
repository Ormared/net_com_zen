import math


def dwells_per_packet(length_bytes: int, data_rate_bps: float,
                      hop_rate_hz: float) -> int:
    t_pkt = 8 * length_bytes / data_rate_bps
    return max(1, math.ceil(t_pkt * hop_rate_hz))


def packet_loss_prob(per_clear: float, rho: float,
                     dwell_lost_if_jammed: float, k: int) -> float:
    """models.md: PER_total = 1 - (1-PER_thermal) * (1-rho_eff)^k."""
    p_dwell_bad = rho * dwell_lost_if_jammed
    return 1.0 - (1.0 - per_clear) * (1.0 - p_dwell_bad) ** k
