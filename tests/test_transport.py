import yaml

from netcom_zen.config import AgentConfig, Scenario


def test_transport_defaults_tcp():
    assert AgentConfig().transport == "tcp"


def test_transport_udp_parses():
    cfg = AgentConfig.model_validate({"transport": "udp"})
    assert cfg.transport == "udp"


def test_invalid_transport_rejected():
    import pytest
    with pytest.raises(Exception):
        AgentConfig.model_validate({"transport": "carrier-pigeon"})


def test_scenario_carries_transport():
    s = Scenario.model_validate({
        "name": "t", "duration_s": 5,
        "radio": {"freq_hz": 433e6, "bandwidth_hz": 250e3, "tx_power_dbm": 27,
                  "data_rate_bps": 250e3, "hop": {"n_channels": 50, "hop_rate_hz": 100}},
        "nodes": [{"id": "v1", "waypoints": [[0, 0]]},
                  {"id": "v2", "waypoints": [[1, 1]]}],
        "agent": {"enabled": True, "transport": "udp"}})
    assert s.agent.transport == "udp"
