import math

C = 299_792_458.0


def friis_db(d_m: float, f_hz: float) -> float:
    d = max(d_m, 1.0)
    return 20 * math.log10(d) + 20 * math.log10(f_hz) - 147.55


def two_ray_db(d_m: float, h_tx_m: float, h_rx_m: float) -> float:
    d = max(d_m, 1.0)
    return 40 * math.log10(d) - 20 * math.log10(h_tx_m * h_rx_m)


def crossover_m(f_hz: float, h_tx_m: float, h_rx_m: float) -> float:
    return 4 * math.pi * h_tx_m * h_rx_m * f_hz / C


def baseline_db(d_m: float, f_hz: float, h_tx_m: float = 1.5, h_rx_m: float = 1.5) -> float:
    if d_m < crossover_m(f_hz, h_tx_m, h_rx_m):
        return friis_db(d_m, f_hz)
    return two_ray_db(d_m, h_tx_m, h_rx_m)
