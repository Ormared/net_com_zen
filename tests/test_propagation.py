import numpy as np
import pytest

from netcom_zen.propagation import CompositePathloss, PathlossBreakdown
from netcom_zen.propagation.diffraction import knife_edge_loss_db
from netcom_zen.propagation.foliage import weissberger_db
from netcom_zen.propagation.freespace import (baseline_db, crossover_m, friis_db,
                                              two_ray_db)
from netcom_zen.terrain import FoliageRegion, Terrain


def test_friis_known_value():
    # 1 km @ 2.4 GHz: 20log10(1000)+20log10(2.4e9)-147.55 = 100.05 dB
    assert friis_db(1000, 2.4e9) == pytest.approx(100.05, abs=0.02)


def test_friis_monotonic_and_guarded():
    assert friis_db(0.1, 2.4e9) == friis_db(1.0, 2.4e9)  # d clamped to 1 m
    assert friis_db(2000, 2.4e9) > friis_db(1000, 2.4e9)


def test_two_ray_known_value():
    # 40log10(1000) - 20log10(1.5*1.5) = 120 - 7.04 = 112.96 dB
    assert two_ray_db(1000, 1.5, 1.5) == pytest.approx(112.96, abs=0.02)


def test_baseline_switches_at_crossover():
    f, h = 2.4e9, 1.5
    dc = crossover_m(f, h, h)
    assert baseline_db(dc * 0.5, f, h, h) == friis_db(dc * 0.5, f)
    assert baseline_db(dc * 2.0, f, h, h) == two_ray_db(dc * 2.0, h, h)


def test_weissberger_regimes():
    assert weissberger_db(900e6, 0.0) == 0.0
    # d<=14 m: 0.45 * f_GHz^0.284 * d
    assert weissberger_db(900e6, 10.0) == pytest.approx(4.368, abs=0.01)
    # 14<d<=400: 1.33 * f_GHz^0.284 * d^0.588
    assert weissberger_db(900e6, 100.0) == pytest.approx(19.36, abs=0.05)
    assert weissberger_db(900e6, 1000.0) == weissberger_db(900e6, 400.0)


def test_knife_edge_grazing_six_db():
    prof = np.array([0.0, 1.5, 0.0])
    loss = knife_edge_loss_db(prof, d_m=1000, h_tx=1.5, h_rx=1.5, f_hz=433e6)
    assert loss == pytest.approx(6.03, abs=0.02)


def test_knife_edge_deep_shadow():
    prof = np.array([0.0, 30.0, 0.0])
    assert knife_edge_loss_db(prof, 1000, 1.5, 1.5, 433e6) > 20.0


def test_knife_edge_clear_and_degenerate():
    assert 0.0 <= knife_edge_loss_db(np.zeros(16), 1000, 1.5, 1.5, 433e6) < 7.0
    assert knife_edge_loss_db(np.zeros(2), 100, 1.5, 1.5, 433e6) == 0.0
    assert knife_edge_loss_db(np.zeros(16), 0.0, 1.5, 1.5, 433e6) == 0.0


def test_composite_open_field():
    pl = CompositePathloss(Terrain(extent_m=(1000, 1000)))
    bd = pl.loss((0, 500), (300, 500), 433e6)
    assert isinstance(bd, PathlossBreakdown)
    assert bd.foliage_db == 0.0
    assert bd.fspl_db == pytest.approx(baseline_db(300, 433e6), abs=0.01)
    assert bd.total_db == bd.fspl_db + bd.terrain_db + bd.foliage_db


def test_composite_forest_adds_weissberger():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=100, x_max=200, y_min=0, y_max=1000)])
    pl = CompositePathloss(t)
    bd = pl.loss((0, 500), (300, 500), 433e6)
    assert bd.foliage_db == pytest.approx(weissberger_db(433e6, 100.0), abs=0.01)
