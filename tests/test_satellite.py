import pytest

from netcom_zen.channel.linkstate import LinkState, build_table
from netcom_zen.config import (JammerConfig, OutageWindow, RadioProfile,
                               SatelliteConfig, Scenario)
from netcom_zen.ew import Jammer
from netcom_zen.propagation import CompositePathloss
from netcom_zen.terrain import Terrain

RADIO = RadioProfile(freq_hz=433e6, bandwidth_hz=250e3, tx_power_dbm=27,
                     data_rate_bps=250e3, hop={"n_channels": 50, "hop_rate_hz": 100})


def test_outage_window_active():
    sat = SatelliteConfig(enabled=True, outages=[OutageWindow(start_s=20, stop_s=30)])
    assert sat.active(10) and sat.active(19.9)
    assert not sat.active(20) and not sat.active(29.9)
    assert sat.active(30)  # recovered


def test_outage_open_ended():
    sat = SatelliteConfig(enabled=True, outages=[OutageWindow(start_s=20)])
    assert sat.active(19) and not sat.active(20) and not sat.active(1000)


def test_sat_link_verdict():
    st = LinkState(src="v1", dst="cmd", prx_dbm=0, noise_dbm=0, rho=0.0,
                   jam_inchannel_dbm=None, data_rate_bps=2e6, hop_rate_hz=1,
                   prop_delay_s=0.04, foliage_db=0, terrain_db=0,
                   medium="sat", sat_loss=0.0)
    assert st.verdict(64, 0.5, 0.5) == "deliver"        # lossless sat
    lossy = LinkState(**{**st.__dict__, "sat_loss": 0.3})
    assert lossy.verdict(64, 0.1, 0.5) == "satloss"     # u < loss
    assert lossy.verdict(64, 0.5, 0.5) == "deliver"     # u >= loss


def test_sat_immune_to_jammer():
    # a barrage jammer that would obliterate RF must not affect the sat link
    pl = CompositePathloss(Terrain(extent_m=(1000, 1000)))
    jam = Jammer(JammerConfig(id="j", kind="barrage", position=(500, 500),
                              tx_power_dbm=80, bandwidth_hz=12.5e6))
    sat = SatelliteConfig(enabled=True)
    pos = {"v1": (150, 500), "cmd": (520, 500)}
    table = build_table(pos, [jam], 0.0, RADIO, pl, command_id="cmd", satellite=sat)
    st = table[("v1", "cmd")]
    assert st.medium == "sat"
    assert st.verdict(64, 0.99, 0.0) == "deliver"  # jammer ignored on sat


def test_failover_to_rf_during_outage():
    pl = CompositePathloss(Terrain(extent_m=(1000, 1000)))
    sat = SatelliteConfig(enabled=True, outages=[OutageWindow(start_s=20)])
    pos = {"v1": (150, 500), "cmd": (520, 500)}
    up = build_table(pos, [], 10.0, RADIO, pl, command_id="cmd", satellite=sat)
    down = build_table(pos, [], 25.0, RADIO, pl, command_id="cmd", satellite=sat)
    assert up[("v1", "cmd")].medium == "sat"
    assert down[("v1", "cmd")].medium == "rf"   # fell back to RF mesh
    # vehicle<->vehicle links are always RF, never satellite
    assert up[("v1", "cmd")].medium == "sat" and ("v1", "v1") not in up


def test_scenario_command_validation():
    base = dict(name="t", duration_s=10,
                radio=RADIO.model_dump(),
                nodes=[{"id": "v1", "waypoints": [[0, 0]]},
                       {"id": "cmd", "role": "command", "waypoints": [[1, 1]]}])
    Scenario.model_validate(base)  # ok
    # satellite enabled but no command -> error
    with pytest.raises(Exception):
        Scenario.model_validate({**base,
            "nodes": [{"id": "v1", "waypoints": [[0, 0]]},
                      {"id": "v2", "waypoints": [[1, 1]]}],
            "satellite": {"enabled": True}})
