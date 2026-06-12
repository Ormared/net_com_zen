import math


def noise_dbm(bandwidth_hz: float, noise_figure_db: float) -> float:
    return -174.0 + 10 * math.log10(bandwidth_hz) + noise_figure_db


def dbm_to_mw(dbm: float) -> float:
    return 10 ** (dbm / 10)


def mw_to_dbm(mw: float) -> float:
    return 10 * math.log10(mw)


def sinr_db(prx_dbm: float, noise_dbm_val: float,
            interferer_dbm: list[float] = ()) -> float:
    denom = dbm_to_mw(noise_dbm_val) + sum(dbm_to_mw(i) for i in interferer_dbm)
    return prx_dbm - mw_to_dbm(denom)


def fsk_per(sinr_db_val: float, length_bytes: int) -> float:
    """Noncoherent BFSK: BER = 0.5*exp(-SNR/2); PER over 8L bits (models.md)."""
    snr = 10 ** (sinr_db_val / 10)
    ber = 0.5 * math.exp(-snr / 2)
    return 1.0 - (1.0 - ber) ** (8 * length_bytes)


def per_threshold_sinr_db(length_bytes: int, target_per: float = 0.5) -> float:
    """SINR at which PER == target (closed-form inverse of fsk_per)."""
    ber = 1.0 - (1.0 - target_per) ** (1.0 / (8 * length_bytes))
    snr = -2.0 * math.log(2.0 * ber)
    return 10 * math.log10(snr)
