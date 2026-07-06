"""P5 lossy-link / router-topology knobs (docs/dds-topology-plan.md P5).

Two levers added for the fixed-rig re-measures: the zenoh router-graph shape
(mesh vs star — the hypothesized fix for the full-router-mesh collapse at
N=96) and passthrough CycloneDDS <Internal> tuning elements (retransmit /
pacing behaviour on netem-lossy links). Pure config/XML: runs in default env.
"""
import pytest
from pydantic import ValidationError

from netcom_zen.config import Ros2WorkloadConfig, Scenario
from netcom_zen.harness.dds_scenarios import bridge_scenario
from netcom_zen.orchestrator import _cyclonedds_xml


def test_zenoh_router_topology_default_and_zenoh_only():
    assert Ros2WorkloadConfig().zenoh_router_topology == "mesh"
    c = Ros2WorkloadConfig(rmw="zenoh", zenoh_router_topology="star")
    assert c.zenoh_router_topology == "star"
    # routerless RMWs have no router graph to shape
    for rmw in ("fastrtps", "cyclonedds"):
        with pytest.raises(ValidationError, match="zenoh only"):
            Ros2WorkloadConfig(rmw=rmw, zenoh_router_topology="star")


def test_zenoh_router_star_requires_bridge():
    d = bridge_scenario(4, "zenoh")
    d["ros2"]["zenoh_router_topology"] = "star"
    s = Scenario.model_validate(d)  # bridge: fine
    assert s.ros2.zenoh_router_topology == "star"
    d["substrate"] = "channel"
    del d["nodes"][2:]  # channel cap is lower; 2 nodes suffice
    with pytest.raises(ValidationError, match="substrate='bridge'"):
        Scenario.model_validate(d)


def test_cyclonedds_internal_cyclone_only_and_key_shape():
    c = Ros2WorkloadConfig(
        rmw="cyclonedds",
        cyclonedds_internal={"NackDelay": "100 ms",
                             "RetransmitMerging": "always"})
    assert c.cyclonedds_internal["NackDelay"] == "100 ms"
    with pytest.raises(ValidationError, match="CycloneDDS only"):
        Ros2WorkloadConfig(rmw="zenoh",
                           cyclonedds_internal={"NackDelay": "100 ms"})
    # keys must be bare element names — no XML structure injection
    with pytest.raises(ValidationError, match="bare XML element"):
        Ros2WorkloadConfig(rmw="cyclonedds",
                           cyclonedds_internal={"a><b": "x"})


def test_cyclonedds_xml_internal_elements():
    xml = _cyclonedds_xml(internal={"NackDelay": "100 ms"})
    assert "<NackDelay>100 ms</NackDelay>" in xml
    assert "<Internal>" in xml
    # composes with the buffer lever inside ONE <Internal> block
    xml = _cyclonedds_xml(4 << 20, internal={"RetransmitMerging": "always"})
    assert xml.count("<Internal>") == 1
    assert 'SocketReceiveBufferSize min="4194304 B"' in xml
    assert "<RetransmitMerging>always</RetransmitMerging>" in xml
    # no knobs -> no <Internal> block at all (stock XML unchanged)
    assert "<Internal>" not in _cyclonedds_xml(0)
