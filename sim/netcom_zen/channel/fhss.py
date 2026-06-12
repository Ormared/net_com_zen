import math


def dwells_per_packet(length_bytes: int, data_rate_bps: float,
                      hop_rate_hz: float) -> int:
    t_pkt = 8 * length_bytes / data_rate_bps
    return max(1, math.ceil(t_pkt * hop_rate_hz))


def jam_survival_prob(p_dwell_bad: float, k: int,
                      correctable_fraction: float = 0.0) -> float:
    """P(packet survives jamming) over k interleaved FHSS dwells.

    Each dwell is independently "bad" (jammed below threshold) with probability
    p_dwell_bad. With FEC + interleaving the code recovers as long as no more
    than floor(correctable_fraction * k) dwells are erased, so survival is the
    binomial CDF P(B <= e), B ~ Binomial(k, p), e = floor(f*k).

    f = 0 reduces to "every dwell must be clean" = (1-p)^k (no FEC). As k grows
    with f > 0, survival -> 1 whenever the jammed fraction p < f: this is why
    FEC + fast hopping beats slow hopping against partial-band jamming.
    """
    p = p_dwell_bad
    if p <= 0.0:
        return 1.0
    e = min(int(correctable_fraction * k), k)
    if e >= k:
        return 1.0
    if p >= 1.0:
        return 0.0  # every dwell bad; only survives if e >= k (handled above)
    # stable iterative binomial CDF: pmf(0)=(1-p)^k, pmf(b)/pmf(b-1)=(k-b+1)/b*p/(1-p)
    pmf = (1.0 - p) ** k
    cdf = pmf
    ratio = p / (1.0 - p)
    for b in range(1, e + 1):
        pmf *= (k - b + 1) / b * ratio
        cdf += pmf
    return min(cdf, 1.0)


def packet_loss_prob(per_clear: float, rho: float, dwell_lost_if_jammed: float,
                     k: int, correctable_fraction: float = 0.0) -> float:
    """models.md: PER_total = 1 - (1-PER_thermal) * P(survive jamming).

    correctable_fraction is the FEC erasure-correction capability (0 = no FEC)."""
    survive = jam_survival_prob(rho * dwell_lost_if_jammed, k, correctable_fraction)
    return 1.0 - (1.0 - per_clear) * survive
