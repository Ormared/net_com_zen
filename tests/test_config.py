import pytest
from pydantic import ValidationError

from netcom_zen.config import Scenario, load_scenario

MINIMAL = """
name: t
duration_s: 5
seed: 42
radio:
  freq_hz: 433.0e6
  bandwidth_hz: 250.0e3
  tx_power_dbm: 27.0
  data_rate_bps: 250.0e3
  hop: {n_channels: 50, hop_rate_hz: 100.0}
nodes:
  - {id: v1, waypoints: [[100, 500]]}
  - {id: v2, waypoints: [[200, 500]]}
"""


def test_minimal_scenario_parses(tmp_path):
    p = tmp_path / "s.yaml"
    p.write_text(MINIMAL)
    s = load_scenario(p)
    assert s.tick_hz == 10.0 and s.radio.noise_figure_db == 7.0
    assert s.nodes[0].speed_mps == 5.0


def test_tx_power_swap_cap():
    with pytest.raises(ValidationError):
        Scenario.model_validate(
            {"name": "t", "duration_s": 1,
             "radio": {"freq_hz": 1, "bandwidth_hz": 1, "tx_power_dbm": 40,
                       "data_rate_bps": 1, "hop": {"n_channels": 2, "hop_rate_hz": 1}},
             "nodes": [{"id": "a", "waypoints": [[0, 0]]},
                       {"id": "b", "waypoints": [[1, 1]]}]})


def test_duplicate_node_ids_rejected(tmp_path):
    p = tmp_path / "s.yaml"
    p.write_text(MINIMAL.replace("id: v2", "id: v1"))
    with pytest.raises(ValidationError):
        load_scenario(p)


def test_discovery_server_is_fastrtps_only():
    from netcom_zen.config import Ros2WorkloadConfig
    # Fast DDS Discovery Server is a Fast DDS feature; cyclonedds has no broker
    # equivalent and zenoh is already router-brokered (dds-rmw-tuning.md §8.1).
    assert Ros2WorkloadConfig(rmw="fastrtps", discovery_server=True).discovery_server
    assert not Ros2WorkloadConfig(rmw="zenoh").discovery_server  # default off
    for bad in ("zenoh", "cyclonedds"):
        with pytest.raises(ValidationError):
            Ros2WorkloadConfig(rmw=bad, discovery_server=True)


def test_jammer_kind_params():
    base = {"id": "j", "position": (0, 0), "tx_power_dbm": 30}
    from netcom_zen.config import JammerConfig
    with pytest.raises(ValidationError):
        JammerConfig(**base, kind="spot")  # missing channels
    with pytest.raises(ValidationError):
        JammerConfig(**base, kind="barrage")  # missing bandwidth_hz
    JammerConfig(**base, kind="sweep")


def test_linkstate_log_requires_channel(tmp_path):
    p = tmp_path / "s.yaml"
    p.write_text(MINIMAL + "linkstate_log: {enabled: true}\n")
    s = load_scenario(p)  # channel substrate: fine
    assert s.linkstate_log.enabled and s.linkstate_log.every_n_ticks == 1
    p.write_text(MINIMAL + "substrate: bridge\nlinkstate_log: {enabled: true}\n")
    with pytest.raises(ValidationError, match="substrate='channel'"):
        load_scenario(p)
