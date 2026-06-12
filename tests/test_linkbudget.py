import pytest

from netcom_zen.channel.fhss import dwells_per_packet, packet_loss_prob
from netcom_zen.channel.linkbudget import (fsk_per, noise_dbm, per_threshold_sinr_db,
                                           sinr_db)


def test_noise_floor():
    # -174 + 10log10(250e3) + 7 = -113.02 dBm
    assert noise_dbm(250e3, 7.0) == pytest.approx(-113.02, abs=0.02)


def test_sinr_no_interference():
    assert sinr_db(-90, -113) == pytest.approx(23.0, abs=0.01)


def test_sinr_with_equal_interferer():
    # interferer equal to noise: -3.01 dB shift
    assert sinr_db(-90, -113, [-113]) == pytest.approx(19.99, abs=0.02)


def test_fsk_per_known_value():
    # SINR 10 dB, 32-byte packet: BER=0.5*exp(-5)=3.369e-3, PER=1-(1-BER)^256=0.5785
    assert fsk_per(10.0, 32) == pytest.approx(0.5785, abs=1e-3)


def test_per_threshold_inverse():
    thr = per_threshold_sinr_db(32, target_per=0.5)
    assert fsk_per(thr, 32) == pytest.approx(0.5, abs=1e-6)


def test_dwells_at_least_one():
    # 32B @ 250kbps = 1.024 ms; 100 hop/s dwell = 10 ms -> 1 dwell
    assert dwells_per_packet(32, 250e3, 100) == 1


def test_dwells_fast_hopping():
    # 1.024 ms packet, 1 ms dwell -> 2 dwells
    assert dwells_per_packet(32, 250e3, 1000) == 2


def test_fhss_loss_prob_formula():
    assert packet_loss_prob(0.0, 0.2, 1.0, 3) == pytest.approx(1 - 0.8**3)
    assert packet_loss_prob(0.1, 0.0, 1.0, 5) == pytest.approx(0.1)
