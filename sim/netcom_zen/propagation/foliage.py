def weissberger_db(f_hz: float, depth_m: float) -> float:
    """Weissberger Modified Exponential Decay model (foliage depth <= 400 m)."""
    if depth_m <= 0:
        return 0.0
    f_ghz = f_hz / 1e9
    d = min(depth_m, 400.0)
    if d <= 14.0:
        return 0.45 * f_ghz**0.284 * d
    return 1.33 * f_ghz**0.284 * d**0.588
