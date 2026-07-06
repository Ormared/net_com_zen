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


def test_reactive_requires_react_latency():
    with pytest.raises(ValueError, match="react_latency_s"):
        mk("reactive")


def test_reactive_occupancy_tracks_hop_rate():
    j = mk("reactive", react_latency_s=0.002)  # 2 ms sense+tune
    # slow hop (10 ms dwell): catches 80% of each dwell, full power on it
    rho, frac = j.occupancy(50, 250e3, hop_rate_hz=100.0)
    assert rho == pytest.approx(0.8) and frac == 1.0
    # fast hop (2 ms dwell): tx hops away before the jammer locks -> no hit
    assert j.occupancy(50, 250e3, hop_rate_hz=500.0)[0] == pytest.approx(0.0)


def test_reactive_occupancy_needs_hop_rate():
    j = mk("reactive", react_latency_s=0.001)
    with pytest.raises(ValueError, match="hop_rate_hz"):
        j.occupancy(50, 250e3)
