"""Tests for the 'lan' substrate: config validation, XML generators with
interface-pinning args, _lan_node_cmd command structure, and smoke-YAML parse.

All tests run in the default pixi env with NO network access — no SSH, no
Docker, no real DDS processes.  Everything exercised here is pure Python:
config schema parsing, XML string generators, and argv construction.
"""
from __future__ import annotations

from pathlib import Path

import pytest
import yaml
from pydantic import ValidationError

from netcom_zen.config import LanHostConfig, Scenario
from netcom_zen.orchestrator import (
    ScenarioEngine,
    _cyclonedds_xml,
    _fastdds_profiles_xml,
)

# ---------------------------------------------------------------------------
# Reusable minimal-valid lan scenario dict
# ---------------------------------------------------------------------------

_RADIO = {
    "freq_hz": 433e6,
    "bandwidth_hz": 250000.0,
    "tx_power_dbm": 27.0,
    "data_rate_bps": 250000.0,
    "hop": {"n_channels": 50, "hop_rate_hz": 100.0},
}
_HOSTS = {
    "local": {"addr": "192.168.1.6", "iface": "wlp130s0f0"},
    "mini": {
        "ssh": "orm_small_nix@192.168.1.9",
        "addr": "192.168.1.9",
        "iface": "wlp2s0",
    },
}
# d1,d2 on local; d3,d4 on mini — mirrors the smoke YAML layout.
_NODES_4 = [
    {"id": "d1", "waypoints": [[0, 0]], "host": "local"},
    {"id": "d2", "waypoints": [[0, 0]], "host": "local"},
    {"id": "d3", "waypoints": [[0, 0]], "host": "mini"},
    {"id": "d4", "waypoints": [[0, 0]], "host": "mini"},
]


def _lan_dict(**ros2_overrides) -> dict:
    """Return a minimal valid lan scenario dict; override ros2 keys as needed."""
    d: dict = {
        "name": "test-lan",
        "duration_s": 30.0,
        "substrate": "lan",
        "radio": _RADIO,
        "hosts": _HOSTS,
        "nodes": _NODES_4,
        "workload": "ros2",
    }
    if ros2_overrides:
        d["ros2"] = ros2_overrides
    return d


# ---------------------------------------------------------------------------
# Config validation — substrate=lan
# ---------------------------------------------------------------------------

class TestLanConfigValidation:
    def test_valid_two_host_scenario_validates(self):
        """The canonical two-host layout must parse without error."""
        s = Scenario.model_validate(_lan_dict())
        assert s.substrate == "lan"
        assert len(s.nodes) == 4
        assert "local" in s.hosts
        assert "mini" in s.hosts
        # Verify host assignment round-trips through the model
        assert s.nodes[0].host == "local"
        assert s.nodes[2].host == "mini"

    def test_lan_host_config_model_direct(self):
        """LanHostConfig itself must validate and expose its fields."""
        h = LanHostConfig(addr="10.0.0.1", iface="eth0")
        assert h.ssh == ""         # local machine by default
        assert h.container == "dds-lab"
        assert h.workdir == "/work"
        assert h.max_nodes == 24

    def test_missing_local_key_rejected(self):
        """substrate=lan requires a 'local' host entry (ssh='')."""
        d = _lan_dict()
        # Replace hosts with a dict that has no "local" key.
        d["hosts"] = {"remote_only": dict(_HOSTS["mini"])}
        d["nodes"] = [
            {"id": "d1", "waypoints": [[0, 0]], "host": "remote_only"},
            {"id": "d2", "waypoints": [[0, 0]], "host": "remote_only"},
        ]
        with pytest.raises(ValidationError, match="local"):
            Scenario.model_validate(d)

    def test_node_host_not_in_hosts_rejected(self):
        """node.host must resolve to a key in Scenario.hosts."""
        d = _lan_dict()
        d["nodes"] = list(_NODES_4) + [
            {"id": "d5", "waypoints": [[0, 0]], "host": "ghost_host"}
        ]
        with pytest.raises(ValidationError, match="ghost_host"):
            Scenario.model_validate(d)

    def test_host_over_max_nodes_rejected(self):
        """Per-host node count must not exceed LanHostConfig.max_nodes."""
        d = _lan_dict()
        # Set max_nodes=1 for "local" but assign 2 nodes there.
        d["hosts"] = {
            "local": {"addr": "192.168.1.6", "iface": "wlp130s0f0",
                      "max_nodes": 1},
            "mini": dict(_HOSTS["mini"]),
        }
        with pytest.raises(ValidationError, match="max_nodes"):
            Scenario.model_validate(d)

    def test_non_lan_substrate_rejects_non_local_host(self):
        """substrate=bridge must reject a node with host != 'local'."""
        d = {
            "name": "test-bridge-bad",
            "duration_s": 30.0,
            "substrate": "bridge",
            "radio": _RADIO,
            "nodes": [
                {"id": "d1", "waypoints": [[0, 0]], "host": "local"},
                {"id": "d2", "waypoints": [[0, 0]], "host": "mini"},
            ],
            "workload": "ros2",
        }
        with pytest.raises(ValidationError, match="host="):
            Scenario.model_validate(d)

    def test_lan_rejects_socket_buffer_bytes_gt_zero(self):
        """substrate=lan rejects socket_buffer_bytes>0 (kernel knob, needs sudo remotely)."""
        with pytest.raises(ValidationError, match="socket_buffer_bytes"):
            Scenario.model_validate(
                _lan_dict(rmw="fastrtps", socket_buffer_bytes=4096))

    def test_lan_rejects_discovery_server(self):
        """substrate=lan rejects discovery_server=True (DDS broker not started on lan path)."""
        with pytest.raises(ValidationError, match="discovery_server"):
            Scenario.model_validate(
                _lan_dict(rmw="fastrtps", discovery_server=True))

    def test_channel_substrate_preserves_existing_node_default(self):
        """Existing channel scenarios (no 'host' key) must still validate;
        NodeConfig.host='local' default keeps them compatible."""
        d = {
            "name": "compat-check",
            "duration_s": 10.0,
            "substrate": "channel",
            "radio": _RADIO,
            "nodes": [
                {"id": "v1", "waypoints": [[0, 0]]},   # no 'host' key
                {"id": "v2", "waypoints": [[10, 10]]},
            ],
        }
        s = Scenario.model_validate(d)
        assert all(n.host == "local" for n in s.nodes)


# ---------------------------------------------------------------------------
# XML generator — Cyclone DDS interface pinning
# ---------------------------------------------------------------------------

class TestCycloneXmlIfacePinning:
    def test_iface_name_replaces_autodetermine(self):
        """When iface_name is set, the XML must use name=... instead of
        autodetermine=true — cross-host must not let Cyclone pick the wrong NIC."""
        xml = _cyclonedds_xml(0, iface_name="wlp2s0")
        assert 'name="wlp2s0"' in xml
        assert "autodetermine" not in xml
        assert 'multicast="true"' in xml

    def test_shm_always_off_with_iface_name(self):
        """SHM must remain off even when iface pinning is active."""
        xml = _cyclonedds_xml(0, iface_name="eth0")
        assert "<Enable>false</Enable>" in xml

    def test_old_no_arg_call_unchanged(self):
        """Regression: _cyclonedds_xml() with no args keeps autodetermine."""
        xml = _cyclonedds_xml(0)
        assert 'autodetermine="true"' in xml
        assert "wlp2s0" not in xml

    def test_old_buffer_only_call_unchanged(self):
        """Regression: single positional arg still produces autodetermine + buffer."""
        xml = _cyclonedds_xml(4 << 20)
        assert 'autodetermine="true"' in xml
        assert "SocketReceiveBufferSize" in xml

    def test_iface_name_with_buffer(self):
        """iface_name and socket_buffer_bytes can coexist in the same config."""
        xml = _cyclonedds_xml(4 << 20, iface_name="eth0")
        assert 'name="eth0"' in xml
        assert "autodetermine" not in xml
        assert "SocketReceiveBufferSize" in xml


# ---------------------------------------------------------------------------
# XML generator — Fast DDS interface whitelist
# ---------------------------------------------------------------------------

class TestFastDDSWhitelistPinning:
    def test_whitelist_present_when_addr_set(self):
        """whitelist_addr must produce <interfaceWhiteList><address>...</address>."""
        xml = _fastdds_profiles_xml(0, 0, whitelist_addr="192.168.1.6")
        assert "<interfaceWhiteList>" in xml
        assert "<address>192.168.1.6</address>" in xml

    def test_whitelist_absent_by_default(self):
        """Default call (no whitelist_addr) must not emit interfaceWhiteList."""
        assert "interfaceWhiteList" not in _fastdds_profiles_xml()

    def test_whitelist_absent_with_two_args(self):
        """Regression: two-arg call (buf, alloc) must not emit interfaceWhiteList."""
        xml = _fastdds_profiles_xml(8 << 20, 0)
        assert "interfaceWhiteList" not in xml

    def test_whitelist_element_order_in_descriptor(self):
        """XSD order: sendBufferSize < interfaceWhiteList < maxInitialPeersRange."""
        xml = _fastdds_profiles_xml(4096, 64, whitelist_addr="10.0.0.1")
        assert xml.index("sendBufferSize") < xml.index("interfaceWhiteList")
        assert xml.index("interfaceWhiteList") < xml.index("maxInitialPeersRange")

    def test_whitelist_before_closing_transport_descriptor(self):
        """interfaceWhiteList must appear inside <transport_descriptor>, not after."""
        xml = _fastdds_profiles_xml(0, 0, whitelist_addr="192.168.1.6")
        assert xml.index("interfaceWhiteList") < xml.index("</transport_descriptor>")

    def test_existing_allocation_order_unchanged(self):
        """Regression: existing XSD order tests must pass (buf+alloc, no whitelist)."""
        xml = _fastdds_profiles_xml(4096, 64)
        assert xml.index("userTransports") < xml.index("useBuiltinTransports")
        assert xml.index("useBuiltinTransports") < xml.index("<allocation>")
        assert xml.index("maxInitialPeersRange") < xml.index("</transport_descriptor>")
        assert "interfaceWhiteList" not in xml


# ---------------------------------------------------------------------------
# _lan_node_cmd — argv structure (no subprocesses started)
# ---------------------------------------------------------------------------

class TestLanNodeCmd:
    @pytest.fixture
    def engine(self, tmp_path):
        """ScenarioEngine initialised with the two-host lan scenario, no run."""
        s = Scenario.model_validate(_lan_dict())
        return ScenarioEngine(s, out_dir=tmp_path / "run_smoke")

    def test_local_node_has_no_ssh_in_argv(self, engine):
        """Local node command must not contain 'ssh' anywhere in argv."""
        cmd = engine._lan_node_cmd("d1", "local")
        assert "ssh" not in cmd

    def test_local_node_starts_with_python_module(self, engine):
        """Local node argv: [sys.executable, '-m', 'netcom_zen.ros2_workload', ...]"""
        cmd = engine._lan_node_cmd("d1", "local")
        assert "-m" in cmd
        assert "netcom_zen.ros2_workload" in cmd

    def test_remote_node_starts_with_ssh_batchmode(self, engine):
        """Remote node command must start with ssh -o BatchMode=yes <target>."""
        cmd = engine._lan_node_cmd("d3", "mini")
        assert cmd[0] == "ssh"
        assert "-o" in cmd
        ssh_o_idx = cmd.index("-o")
        assert cmd[ssh_o_idx + 1] == "BatchMode=yes"

    def test_remote_node_contains_docker_exec(self, engine):
        """Remote command must have 'docker' followed eventually by 'exec'."""
        cmd = engine._lan_node_cmd("d3", "mini")
        assert "docker" in cmd
        docker_idx = cmd.index("docker")
        exec_idx = cmd.index("exec")
        assert docker_idx < exec_idx

    def test_remote_node_contains_container_name(self, engine):
        """Remote command must reference the configured container name."""
        cmd = engine._lan_node_cmd("d3", "mini")
        container = engine.scenario.hosts["mini"].container
        assert container in cmd

    def test_remote_node_metrics_path_is_remote(self, engine):
        """Remote node --metrics must point into the container workdir, not out_dir."""
        cmd = engine._lan_node_cmd("d3", "mini")
        h = engine.scenario.hosts["mini"]
        run_name = engine.out_dir.name
        expected = f"{h.workdir}/results/lan/{run_name}/agent_d3.jsonl"
        assert expected in cmd

    def test_local_node_metrics_is_in_out_dir(self, engine):
        """Local node --metrics must point into the local out_dir."""
        cmd = engine._lan_node_cmd("d1", "local")
        expected = str(engine.out_dir / "agent_d1.jsonl")
        assert expected in cmd

    def test_peers_exclude_self_local(self, engine):
        """--peers for a local node must not include the node itself."""
        cmd = engine._lan_node_cmd("d1", "local")
        peers_val = cmd[cmd.index("--peers") + 1]
        peers = peers_val.split(",")
        assert "d1" not in peers
        assert set(peers) == {"d2", "d3", "d4"}

    def test_peers_exclude_self_remote(self, engine):
        """--peers for a remote node must not include the node itself."""
        cmd = engine._lan_node_cmd("d3", "mini")
        peers_val = cmd[cmd.index("--peers") + 1]
        peers = peers_val.split(",")
        assert "d3" not in peers
        assert set(peers) == {"d1", "d2", "d4"}


# ---------------------------------------------------------------------------
# Smoke YAML — parse the committed files
# ---------------------------------------------------------------------------

_SCENARIOS_DDS = Path("scenarios/dds")


class TestLanSmokeYamls:
    @pytest.fixture(autouse=True)
    def _require_scenarios(self):
        if not _SCENARIOS_DDS.exists():
            pytest.skip("scenarios/dds/ not present")

    @pytest.mark.parametrize("rmw", ["fastrtps", "cyclonedds", "zenoh"])
    def test_smoke_yaml_parses(self, rmw):
        """Each smoke YAML must round-trip through Scenario.model_validate."""
        path = _SCENARIOS_DDS / f"lan_smoke_n4_{rmw}.yaml"
        assert path.exists(), f"missing {path}"
        s = Scenario.model_validate(yaml.safe_load(path.read_text()))
        assert s.substrate == "lan"
        assert s.ros2.rmw == rmw
        assert len(s.nodes) == 4
        assert "local" in s.hosts
        assert "mini" in s.hosts

    @pytest.mark.parametrize("rmw", ["fastrtps", "cyclonedds", "zenoh"])
    def test_smoke_yaml_node_host_placement(self, rmw):
        """d1,d2 must be on 'local'; d3,d4 must be on 'mini'."""
        path = _SCENARIOS_DDS / f"lan_smoke_n4_{rmw}.yaml"
        s = Scenario.model_validate(yaml.safe_load(path.read_text()))
        node_hosts = {n.id: n.host for n in s.nodes}
        assert node_hosts["d1"] == "local"
        assert node_hosts["d2"] == "local"
        assert node_hosts["d3"] == "mini"
        assert node_hosts["d4"] == "mini"

    @pytest.mark.parametrize("rmw", ["fastrtps", "cyclonedds", "zenoh"])
    def test_smoke_yaml_host_fields(self, rmw):
        """hosts 'local' and 'mini' must have the correct addr/iface/ssh values."""
        path = _SCENARIOS_DDS / f"lan_smoke_n4_{rmw}.yaml"
        s = Scenario.model_validate(yaml.safe_load(path.read_text()))
        assert s.hosts["local"].addr == "192.168.1.6"
        assert s.hosts["local"].iface == "wlp130s0f0"
        assert s.hosts["local"].ssh == ""
        assert s.hosts["mini"].addr == "192.168.1.9"
        assert s.hosts["mini"].iface == "wlp2s0"
        assert "192.168.1.9" in s.hosts["mini"].ssh
