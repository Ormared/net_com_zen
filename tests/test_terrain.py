import numpy as np
import pytest

from netcom_zen.terrain import FoliageRegion, Terrain


def test_flat_terrain_height_zero():
    t = Terrain(extent_m=(1000, 1000))
    assert t.height(500, 500) == 0.0


def test_heightmap_lookup():
    hm = np.array([[0.0, 0.0], [0.0, 50.0]])  # row=y, col=x
    t = Terrain(extent_m=(100, 100), heightmap=hm)
    assert t.height(99, 99) == 50.0
    assert t.height(0, 0) == 0.0


def test_profile_endpoints():
    t = Terrain(extent_m=(1000, 1000))
    prof = t.profile((0, 0), (1000, 1000), n=16)
    assert len(prof) == 16 and prof[0] == 0.0


def test_foliage_depth_full_crossing():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=400, x_max=600, y_min=0, y_max=1000)])
    assert t.foliage_depth((0, 500), (1000, 500)) == pytest.approx(200.0)


def test_foliage_depth_miss():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=400, x_max=600, y_min=0, y_max=100)])
    assert t.foliage_depth((0, 500), (1000, 500)) == 0.0


def test_foliage_depth_zero_length_segment():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=0, x_max=1000, y_min=0, y_max=1000)])
    assert t.foliage_depth((500, 500), (500, 500)) == 0.0
