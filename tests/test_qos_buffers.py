"""QoS + buffer-tuning plumbing (dds-rmw-qos-plane study).

Covers the config schema, the RMW config-file generators, and the kernel-buffer
helper's restore bookkeeping. The DDS XML generators are pure strings (no rclpy),
so these run in the default env.
"""
import pytest
from pydantic import ValidationError

from netcom_zen.config import Ros2WorkloadConfig
from netcom_zen.orchestrator import (
    ScenarioEngine,
    _cyclonedds_xml,
    _fastdds_profiles_xml,
)


def test_qos_fields_parse_and_validate():
    c = Ros2WorkloadConfig(
        rmw="cyclonedds", durability="transient_local", history="keep_all",
        depth=50, deadline_ms=1000, lifespan_ms=2000,
        liveliness="manual_by_topic", liveliness_lease_ms=3000,
        socket_buffer_bytes=4 << 20, spawn_stagger_ms=200)
    assert c.history == "keep_all" and c.depth == 50
    assert c.socket_buffer_bytes == 4 << 20
    assert c.spawn_stagger_ms == 200
    # bounds: depth>0, the duration/buffer knobs are >=0
    with pytest.raises(ValidationError):
        Ros2WorkloadConfig(depth=0)
    with pytest.raises(ValidationError):
        Ros2WorkloadConfig(socket_buffer_bytes=-1)
    with pytest.raises(ValidationError):
        Ros2WorkloadConfig(deadline_ms=-5)


def test_cyclonedds_xml_buffer_toggle():
    # off: no SocketReceiveBufferSize element at all (stock behaviour preserved)
    assert "SocketReceiveBufferSize" not in _cyclonedds_xml(0)
    # on: Cyclone wants the size with a unit; bytes are emitted as "<n> B"
    xml = _cyclonedds_xml(4 << 20)
    assert 'SocketReceiveBufferSize min="4194304 B"' in xml
    assert "<Enable>false</Enable>" in xml  # SHM still forced off


def test_fastdds_profiles_xml_buffer():
    xml = _fastdds_profiles_xml(8 << 20)
    # the <profiles> wrapper is required or Fast DDS 8.x rejects the file
    assert "<profiles>" in xml
    assert "<receiveBufferSize>8388608</receiveBufferSize>" in xml
    assert "<sendBufferSize>8388608</sendBufferSize>" in xml
    # builtin transports off -> SHM off, only the enlarged UDPv4 transport
    assert "<useBuiltinTransports>false</useBuiltinTransports>" in xml


def test_kernel_buffer_apply_is_noop_when_zero():
    # nbytes<=0 must not touch /proc/sys at all (returns empty -> nothing to
    # restore), so a stock run never perturbs host kernel state.
    assert ScenarioEngine._apply_kernel_buffers(0) == {}
    ScenarioEngine._restore_kernel_buffers({})  # must not raise
