import pytest

from netcom_zen.config import JammerConfig
from netcom_zen.ew import Jammer


def mk(kind, **kw):
    base = {"id": "j1", "kind": kind, "position": (0, 0), "tx_power_dbm": 30}
    return Jammer(JammerConfig(**base, **kw))


def test_active_window():
    j = mk("spot", channels=[0, 1], start_s=10, stop_s=20)
    assert not j.active(5) and j.active(10) and j.active(19.9) and not j.active(20)


def test_spot_occupancy():
    j = mk("spot", channels=[0, 1, 2, 3])
    rho, frac = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == pytest.approx(4 / 50)
    assert frac == pytest.approx(1 / 4)  # power split across its 4 channels


def test_spot_ignores_out_of_band_channels():
    j = mk("spot", channels=[0, 999])
    rho, _ = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == pytest.approx(1 / 50)


def test_barrage_occupancy():
    j = mk("barrage", bandwidth_hz=12.5e6)
    rho, frac = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == 1.0
    assert frac == pytest.approx(250e3 / 12.5e6)  # in-channel share of psd


def test_sweep_occupancy():
    j = mk("sweep")
    rho, frac = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == pytest.approx(1 / 50) and frac == 1.0
