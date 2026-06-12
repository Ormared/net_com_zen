"""FHSS statistical model cross-checked against direct dwell-level simulation."""
import numpy as np
import pytest

from netcom_zen.channel.fhss import packet_loss_prob


def test_formula_matches_dwell_simulation():
    rng = np.random.default_rng(7)
    rho, k, trials = 0.15, 4, 200_000
    hits = rng.random((trials, k)) < rho
    sim = float(np.mean(hits.any(axis=1)))
    formula = packet_loss_prob(0.0, rho, 1.0, k)
    assert sim == pytest.approx(formula, abs=3 * np.sqrt(formula * (1 - formula) / trials))


def test_thermal_and_jamming_compose():
    # independent loss processes: survival multiplies
    per_clear, rho, k = 0.2, 0.1, 3
    total = packet_loss_prob(per_clear, rho, 1.0, k)
    assert total == pytest.approx(1 - (1 - per_clear) * (1 - rho) ** k)
    assert total > per_clear  # jamming can only make things worse
