"""Real Sionna RT smoke (M5.3 exit gate): runs only in the sionna pixi env
(`pixi run -e sionna sionna-smoke`); skipped wherever sionna is absent.
The careful RT-vs-analytical comparison lives in validation/ against
committed grids — this only proves the trace pipeline end-to-end."""
import math

import numpy as np
import pytest

pytest.importorskip("sionna.rt")

from netcom_zen.config import Scenario  # noqa: E402
from netcom_zen.propagation.freespace import baseline_db  # noqa: E402
from netcom_zen.propagation.sionna_grid import CAP_DB  # noqa: E402
from netcom_zen.propagation.sionna_precompute import _trace, cell_axes  # noqa: E402
from netcom_zen.terrain import Terrain  # noqa: E402

FREQ = 915e6
EXTENT = 200.0


def test_flat_scene_trace_matches_analytic_ballpark():
    scenario = Scenario(
        name="smoke", duration_s=1.0,
        radio=dict(freq_hz=FREQ, bandwidth_hz=1e6, tx_power_dbm=10.0,
                   data_rate_bps=250e3, hop=dict(n_channels=10, hop_rate_hz=50.0)),
        nodes=[dict(id="a", waypoints=[[0.0, 0.0]]),
               dict(id="b", waypoints=[[50.0, 0.0]])])
    terrain = Terrain((EXTENT, EXTENT))
    tx_ax = np.linspace(0.0, EXTENT, 2)
    n_cells = 20
    pl = _trace(scenario, terrain, tx_ax, tx_ax, n_cells, h_ant=1.5,
                max_depth=2, margin_m=50.0, mesh_cells=32)
    assert pl.shape == (2, 2, n_cells, n_cells)
    hit = pl < CAP_DB
    assert hit.mean() > 0.95, "flat scene: every cell has at least a LoS path"

    # tx (0,0); compare mid-range cells (past the two-ray crossover, ~19 m at
    # 915 MHz) against the analytical baseline. PathSolver is deterministic,
    # so the tolerance is physics (finite-permittivity ground), not noise.
    cx, cy = cell_axes((EXTENT, EXTENT), n_cells)
    errs = []
    for ci, cj in [(2, 2), (5, 5), (8, 3), (3, 8), (10, 10), (15, 6)]:
        d = math.hypot(cx[cj], cy[ci])
        errs.append(abs(pl[0, 0, ci, cj] - baseline_db(d, FREQ, 1.5, 1.5)))
    assert np.median(errs) < 4.0
