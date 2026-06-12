import math

import numpy as np

from .freespace import C


def knife_edge_loss_db(profile: np.ndarray, d_m: float,
                       h_tx: float, h_rx: float, f_hz: float) -> float:
    """Single dominant knife-edge (ITU-R P.526 approximation J(v)).

    profile: terrain heights along the path, endpoints included.
    Antennas sit h_tx/h_rx above the endpoint terrain heights.
    """
    n = len(profile)
    if n < 3 or d_m <= 0:
        return 0.0
    lam = C / f_hz
    z_tx = float(profile[0]) + h_tx
    z_rx = float(profile[-1]) + h_rx
    v_max = -math.inf
    for i in range(1, n - 1):
        d1 = d_m * i / (n - 1)
        d2 = d_m - d1
        los = z_tx + (z_rx - z_tx) * i / (n - 1)
        h = float(profile[i]) - los
        v_max = max(v_max, h * math.sqrt(2 * d_m / (lam * d1 * d2)))
    if v_max <= -0.78:
        return 0.0
    return 6.9 + 20 * math.log10(math.sqrt((v_max - 0.1) ** 2 + 1) + v_max - 0.1)
