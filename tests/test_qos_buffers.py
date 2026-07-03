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


def test_fastdds_allocation_xml():
    # allocation-only path (P1 lever): must carry the allocation block and
    # maxInitialPeersRange, but NO buffer elements (the descriptor + builtin-
    # transports=false still appear — SHM stays off even without buffer tuning).
    xml_alloc = _fastdds_profiles_xml(0, 128)
    assert "<allocation>" in xml_alloc
    assert "<initial>128</initial>" in xml_alloc
    assert "<maximum>128</maximum>" in xml_alloc
    assert "<maxInitialPeersRange>128</maxInitialPeersRange>" in xml_alloc
    assert "receiveBufferSize" not in xml_alloc
    assert "<profiles>" in xml_alloc
    assert "<useBuiltinTransports>false</useBuiltinTransports>" in xml_alloc

    # buffer-only path (old call shape — positional args still work after the
    # signature gained a default second param): must carry buffer elements, no
    # allocation block.
    xml_buf = _fastdds_profiles_xml(8 << 20, 0)
    assert "<receiveBufferSize>8388608</receiveBufferSize>" in xml_buf
    assert "<sendBufferSize>8388608</sendBufferSize>" in xml_buf
    assert "<allocation>" not in xml_buf
    assert "<profiles>" in xml_buf
    assert "<useBuiltinTransports>false</useBuiltinTransports>" in xml_buf


def test_fastdds_allocation_xml_order():
    # Fast DDS 8.x XSD enforces element order inside <rtps> and
    # <transport_descriptor>. Verify the generated string respects the required
    # sequences: userTransports < useBuiltinTransports < allocation (in rtps),
    # and maxInitialPeersRange appears before </transport_descriptor>.
    xml = _fastdds_profiles_xml(4096, 64)
    assert xml.index("userTransports") < xml.index("useBuiltinTransports")
    assert xml.index("useBuiltinTransports") < xml.index("<allocation>")
    assert xml.index("maxInitialPeersRange") < xml.index("</transport_descriptor>")


def test_allocation_field_is_fastrtps_only():
    # fastrtps accepts any non-negative value
    c = Ros2WorkloadConfig(rmw="fastrtps", fastdds_allocation_participants=128)
    assert c.fastdds_allocation_participants == 128
    # cyclonedds must reject a non-zero value (no equivalent attribute)
    with pytest.raises(ValidationError):
        Ros2WorkloadConfig(rmw="cyclonedds", fastdds_allocation_participants=128)
    # negative value must always fail regardless of rmw (ge=0 field constraint)
    with pytest.raises(ValidationError):
        Ros2WorkloadConfig(fastdds_allocation_participants=-1)
    # zenoh at 0 (default) must be fine — the validator only fires when >0
    c0 = Ros2WorkloadConfig(rmw="zenoh")
    assert c0.fastdds_allocation_participants == 0
