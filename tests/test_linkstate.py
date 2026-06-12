import pytest

from netcom_zen.channel.linkstate import LinkState, build_table, link_rng
from netcom_zen.config import JammerConfig, RadioProfile
from netcom_zen.ew import Jammer
from netcom_zen.propagation import CompositePathloss
from netcom_zen.terrain import Terrain

RADIO = RadioProfile(freq_hz=433e6, bandwidth_hz=250e3, tx_power_dbm=27,
                     data_rate_bps=250e3,
                     hop={"n_channels": 50, "hop_rate_hz": 100})


def make_state(**kw):
    base = dict(src="a", dst="b", prx_dbm=-80, noise_dbm=-113, rho=0.0,
                jam_inchannel_dbm=None, data_rate_bps=250e3, hop_rate_hz=100,
                prop_delay_s=1e-6, foliage_db=0.0, terrain_db=0.0)
    base.update(kw)
    return LinkState(**base)


def test_strong_link_delivers():
    assert make_state().verdict(64, 0.999, 0.999) == "deliver"


def test_weak_link_drops_range():
    assert make_state(prx_dbm=-120).verdict(64, 0.5, 0.5) == "range"


def test_foliage_attribution():
    assert make_state(prx_dbm=-120, foliage_db=25.0).verdict(64, 0.5, 0.5) == "foliage"


def test_strong_jammer_drops_jam():
    st = make_state(rho=1.0, jam_inchannel_dbm=-60)
    assert st.verdict(64, 0.999, 0.5) == "jam"


def test_weak_jammer_passes():
    # jammer 40 dB below signal: SINR barely moves, dwell never lost
    st = make_state(rho=1.0, jam_inchannel_dbm=-120)
    assert st.verdict(64, 0.999, 0.001) == "deliver"


def test_build_table_directed_pairs():
    pl = CompositePathloss(Terrain(extent_m=(1000, 1000)))
    jam = Jammer(JammerConfig(id="j", kind="spot", position=(500, 0),
                              tx_power_dbm=30, channels=[0, 1]))
    table = build_table({"a": (0, 0), "b": (100, 0)}, [jam], t=0.0,
                        radio=RADIO, pathloss=pl)
    assert set(table) == {("a", "b"), ("b", "a")}
    st = table[("a", "b")]
    assert st.rho == pytest.approx(2 / 50)
    assert st.jam_inchannel_dbm is not None


def test_replay_determinism():
    st = make_state(prx_dbm=-105, rho=0.3, jam_inchannel_dbm=-95)

    def run():
        rng = link_rng(seed=42, src="a", dst="b")
        return [st.verdict(64, rng.random(), rng.random()) for _ in range(500)]

    assert run() == run()
