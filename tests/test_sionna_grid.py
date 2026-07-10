"""Sionna grid provider + precompute helpers (M5.3) — default env, no sionna
import: everything here is the numpy-only side of the pipeline."""
import math

import numpy as np
import pytest
from pydantic import ValidationError

from netcom_zen.config import EnvironmentConfig
from netcom_zen.propagation import CompositePathloss
from netcom_zen.propagation.foliage import weissberger_db
from netcom_zen.propagation.freespace import baseline_db
from netcom_zen.propagation.sionna_grid import (CAP_DB, SionnaGrid,
                                                SionnaGridPathloss,
                                                _axis_weights)
from netcom_zen.propagation.sionna_precompute import (cell_axes,
                                                      gain_to_pl_db,
                                                      grid_mesh, write_ply)
from netcom_zen.terrain import FoliageRegion, Terrain

FREQ = 915e6


def linear_grid(f=FREQ, h=1.5, n_tx=3, n_cell=10, extent=100.0):
    """pl(tx, rx) = 60 + 0.1*tx_x + 0.2*tx_y + 0.3*rx_x + 0.4*rx_y — affine,
    so quadrilinear interpolation must reproduce it exactly."""
    tx = np.linspace(0.0, extent, n_tx)
    cx, cy = cell_axes((extent, extent), n_cell)
    pl = (60.0 + 0.1 * tx[None, :, None, None] + 0.2 * tx[:, None, None, None]
          + 0.3 * cx[None, None, None, :] + 0.4 * cy[None, None, :, None])
    return SionnaGrid(freq_hz=f, h_ant_m=h, tx_x=tx, tx_y=tx,
                      cell_x=cx, cell_y=cy,
                      pl_db=np.broadcast_to(pl, (n_tx, n_tx, n_cell, n_cell)).astype(np.float32),
                      meta={})


def analytic(grid, tx, rx):
    return (60.0 + 0.1 * tx[0] + 0.2 * tx[1] + 0.3 * rx[0] + 0.4 * rx[1])


def test_axis_weights_interior_and_clip():
    ax = np.array([0.0, 10.0, 30.0])
    assert _axis_weights(ax, -5.0) == (0, 0, 0.0)
    assert _axis_weights(ax, 50.0) == (2, 2, 0.0)
    i0, i1, w = _axis_weights(ax, 15.0)
    assert (i0, i1) == (1, 2) and math.isclose(w, 0.25)


def test_lookup_reproduces_affine_field():
    g = linear_grid()
    for tx, rx in [((5.0, 5.0), (95.0, 5.0)), ((37.0, 81.0), (12.0, 44.0)),
                   ((50.0, 50.0), (50.0, 95.0))]:
        # inside the cell-centre hull so no edge clipping perturbs the value
        assert g.lookup_db(tx, rx) == pytest.approx(analytic(g, tx, rx), abs=1e-4)


def test_save_load_roundtrip(tmp_path):
    g = linear_grid()
    g.meta["samples"] = 123
    g.save(tmp_path / "g.npz")
    g2 = SionnaGrid.load(tmp_path / "g.npz")
    assert g2.freq_hz == g.freq_hz and g2.meta["samples"] == 123
    np.testing.assert_allclose(g2.pl_db, g.pl_db)
    assert g2.lookup_db((10, 20), (80, 70)) == pytest.approx(
        g.lookup_db((10, 20), (80, 70)))


def test_provider_breakdown_composes_rt_plus_weissberger():
    g = linear_grid()
    fol = [FoliageRegion(40.0, 60.0, 0.0, 100.0)]
    terrain = Terrain((100.0, 100.0), None, fol)
    p = SionnaGridPathloss(g, terrain)
    tx, rx = (10.0, 50.0), (90.0, 50.0)
    bd = p.loss(tx, rx, FREQ)
    rt = 0.5 * (g.lookup_db(tx, rx) + g.lookup_db(rx, tx))
    base = baseline_db(80.0, FREQ, 1.5, 1.5)
    assert bd.fspl_db == pytest.approx(base)
    assert bd.terrain_db == pytest.approx(rt - base)
    assert bd.foliage_db == pytest.approx(weissberger_db(FREQ, 20.0))
    assert bd.total_db == pytest.approx(rt + bd.foliage_db)


def test_provider_is_symmetric():
    g = linear_grid()
    p = SionnaGridPathloss(g, Terrain((100.0, 100.0)))
    a = p.loss((10.0, 20.0), (80.0, 70.0), FREQ)
    b = p.loss((80.0, 70.0), (10.0, 20.0), FREQ)
    assert a.total_db == pytest.approx(b.total_db)


def test_provider_rejects_wrong_frequency():
    p = SionnaGridPathloss(linear_grid(), Terrain((100.0, 100.0)))
    with pytest.raises(ValueError, match="re-run precompute"):
        p.loss((0, 0), (50, 50), 2.4e9)


def test_short_links_fall_back_to_analytic():
    p = SionnaGridPathloss(linear_grid(), Terrain((100.0, 100.0)))
    bd = p.loss((50.0, 50.0), (52.0, 50.0), FREQ)  # < 10 m cell pitch
    assert bd.terrain_db == 0.0
    assert bd.fspl_db == pytest.approx(baseline_db(2.0, FREQ, 1.5, 1.5))


def test_config_sionna_requires_grid():
    with pytest.raises(ValidationError, match="sionna_grid"):
        EnvironmentConfig(pathloss="sionna")
    EnvironmentConfig(pathloss="sionna", sionna_grid="grids/x.npz")  # ok
    EnvironmentConfig()  # composite default untouched


def test_build_world_selects_grid_provider(tmp_path):
    from netcom_zen.config import load_scenario
    from netcom_zen.orchestrator import build_world
    import yaml
    linear_grid(extent=1000.0).save(tmp_path / "g.npz")
    spec = dict(
        name="t", duration_s=1.0,
        radio=dict(freq_hz=FREQ, bandwidth_hz=1e6, tx_power_dbm=10.0,
                   data_rate_bps=250e3, hop=dict(n_channels=10, hop_rate_hz=50.0)),
        nodes=[dict(id="a", waypoints=[[0, 0]]), dict(id="b", waypoints=[[9, 0]])],
        environment=dict(extent_m=[1000.0, 1000.0], pathloss="sionna",
                         sionna_grid=str(tmp_path / "g.npz")))
    path = tmp_path / "s.yaml"
    path.write_text(yaml.safe_dump(spec))
    w = build_world(load_scenario(path))
    assert isinstance(w.pathloss, SionnaGridPathloss)
    spec["environment"].pop("pathloss")
    spec["environment"].pop("sionna_grid")
    path.write_text(yaml.safe_dump(spec))
    assert isinstance(build_world(load_scenario(path)).pathloss,
                      CompositePathloss)


# --- precompute pure helpers -------------------------------------------------

def test_grid_mesh_heights_and_winding():
    hm = np.array([[0.0, 0.0], [10.0, 10.0]])  # ramp in y
    t = Terrain((100.0, 100.0), hm)
    x = np.linspace(0.0, 100.0, 3)
    verts, faces = grid_mesh(t, x, x, z_offset=1.5)
    assert verts.shape == (9, 3) and faces.shape == (8, 3)
    assert verts[0, 2] == pytest.approx(t.height(0, 0) + 1.5)
    assert verts[-1, 2] == pytest.approx(t.height(100, 100) + 1.5)
    v = verts[faces[0]]
    normal_z = np.cross(v[1] - v[0], v[2] - v[0])[2]
    assert normal_z > 0  # CCW from above


def test_write_ply_roundtrip(tmp_path):
    t = Terrain((10.0, 10.0))
    verts, faces = grid_mesh(t, np.linspace(0, 10, 2), np.linspace(0, 10, 2))
    write_ply(tmp_path / "m.ply", verts, faces)
    text = (tmp_path / "m.ply").read_text().splitlines()
    assert "element vertex 4" in text and "element face 2" in text
    assert text[text.index("end_header") + 1].split() == ["0.000", "0.000", "0.000"]


def test_gain_to_pl_db_caps_zeros():
    pl = gain_to_pl_db(np.array([[1e-10, 0.0]]))
    assert pl[0, 0] == pytest.approx(100.0)
    assert pl[0, 1] == CAP_DB
