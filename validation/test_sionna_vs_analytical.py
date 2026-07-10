"""Ray-traced (Sionna RT) vs analytical pathloss cross-checks (M5.3 exit).

Runs in the default env against the committed grids in grids/ (deterministic
PathSolver output, so the assertions are exact reruns, not statistics).
Regenerate with:

    pixi run -e sionna sionna-precompute scenarios/resilience_4node.yaml \
        -o grids/resilience_4node_433.npz --tx-grid 7 --rx-cells 100
    pixi run -e sionna sionna-precompute scenarios/sionna_hill_4node.yaml \
        -o grids/sionna_hill_433.npz --tx-grid 7 --rx-cells 100

Agreement validates the pipeline where the models must coincide (flat earth
-> two-ray); the divergences are the documented fidelity envelope
(docs/results/sionna-vs-analytical.md): RT adds ridge-echo multipath the
analytical model lacks, but loses terrain diffraction — sionna-rt only
diffracts around sharp wedges (interior dihedral ~<=90 deg), so a smooth
45 m ridge casts a hard shadow where knife-edge keeps a finite ~30 dB field.
"""
from pathlib import Path

import numpy as np
import pytest

from netcom_zen.propagation import CompositePathloss
from netcom_zen.propagation.freespace import baseline_db, crossover_m
from netcom_zen.propagation.sionna_grid import (CAP_DB, SionnaGrid,
                                                SionnaGridPathloss)
from netcom_zen.terrain import Terrain

ROOT = Path(__file__).resolve().parent.parent
FLAT_GRID = ROOT / "grids" / "resilience_4node_433.npz"
HILL_GRID = ROOT / "grids" / "sionna_hill_433.npz"
HILL_HM = ROOT / "scenarios" / "data" / "hill_ridge_128.npy"

pytestmark = pytest.mark.skipif(
    not (FLAT_GRID.exists() and HILL_GRID.exists()),
    reason="sionna grids not present (see module docstring to regenerate)")


@pytest.fixture(scope="module")
def flat():
    g = SionnaGrid.load(FLAT_GRID)
    return g, SionnaGridPathloss(g, Terrain((1000.0, 1000.0)))


@pytest.fixture(scope="module")
def hill():
    g = SionnaGrid.load(HILL_GRID)
    t = Terrain((1000.0, 1000.0), np.load(HILL_HM))
    return g, t, SionnaGridPathloss(g, t)


def _rt_db(provider, a, b, f):
    bd = provider.loss(a, b, f)
    return bd.fspl_db + bd.terrain_db


def test_flat_scene_fully_reachable(flat):
    g, _ = flat
    assert float((g.pl_db >= CAP_DB).mean()) == 0.0


def test_flat_far_field_matches_two_ray(flat):
    """Where the models must agree: flat earth beyond the two-ray crossover.
    The residual is physics, not error — RT reflects off finite-permittivity
    ground (|Gamma| < 1), the analytic asymptote assumes a perfect mirror."""
    g, p = flat
    xover = crossover_m(g.freq_hz, 1.5, 1.5)
    diffs = []
    rng = np.random.default_rng(0)
    while len(diffs) < 300:
        a = tuple(rng.uniform(20, 980, 2))
        b = tuple(rng.uniform(20, 980, 2))
        d = float(np.hypot(b[0] - a[0], b[1] - a[1]))
        if d < max(60.0, xover * 1.5):
            continue
        diffs.append(_rt_db(p, a, b, g.freq_hz) - baseline_db(d, g.freq_hz, 1.5, 1.5))
    diffs = np.abs(diffs)
    assert float(np.median(diffs)) < 3.0
    assert float(np.percentile(diffs, 90)) < 4.0


def test_hill_shadow_is_hard_where_knife_edge_is_finite(hill):
    """The divergence: cross-ridge pairs. Geometric-optics RT finds no path
    over the smooth crest (sionna diffracts only around sharp wedges), while
    the analytical knife-edge keeps a finite ~25-32 dB diffraction loss."""
    g, t, p = hill
    comp = CompositePathloss(t)
    for a, b in [((300.0, 400.0), (700.0, 400.0)),
                 ((350.0, 600.0), (650.0, 600.0)),
                 ((200.0, 500.0), (800.0, 500.0))]:
        rt_excess = p.loss(a, b, g.freq_hz).terrain_db
        ke_excess = comp.loss(a, b, g.freq_hz).terrain_db
        assert rt_excess > 60.0, "RT should cast a hard shadow"
        assert 20.0 < ke_excess < 40.0, "knife-edge stays finite"


def test_hill_ridge_echo_fills_two_ray_nulls(hill):
    """Same-side (unblocked) pairs near the ridge: the west-facing slope is a
    huge reflector whose echo dominates once the direct two-ray pair nearly
    cancels — RT reads systematically BELOW the flat-earth baseline. This is
    added fidelity, not error (verified against direct per-path solves)."""
    g, t, p = hill
    rng = np.random.default_rng(3)
    diffs = []
    while len(diffs) < 200:
        a = (rng.uniform(20, 300), rng.uniform(50, 950))
        b = (rng.uniform(20, 300), rng.uniform(50, 950))
        if np.hypot(b[0] - a[0], b[1] - a[1]) < 60:
            continue
        diffs.append(p.loss(a, b, g.freq_hz).terrain_db)
    med = float(np.median(diffs))
    assert -25.0 < med < -5.0, f"ridge echo should fill the nulls (median {med:.1f})"
