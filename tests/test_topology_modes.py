"""Topology mode plumbing (docs/dds-topology-plan.md P4 / D4).

Covers the config schema and argv construction for mesh / shared / star
endpoint topologies.  No rclpy, no subprocess, no root — config validation
and pure command-list construction only.
"""
from __future__ import annotations

import shlex
from types import SimpleNamespace

import pytest
from pydantic import ValidationError

from netcom_zen.config import Ros2WorkloadConfig, Scenario
from netcom_zen.orchestrator import ScenarioEngine

# ---------------------------------------------------------------------------
# Shared test fixtures (minimal valid scenario dicts)
# ---------------------------------------------------------------------------

_RADIO = {
    "freq_hz": 433e6,
    "bandwidth_hz": 250000.0,
    "tx_power_dbm": 27.0,
    "data_rate_bps": 250000.0,
    "hop": {"n_channels": 50, "hop_rate_hz": 100.0},
}

_NODES_4 = [
    {"id": "d1", "waypoints": [[0, 0]]},
    {"id": "d2", "waypoints": [[0, 0]]},
    {"id": "d3", "waypoints": [[0, 0]]},
    {"id": "d4", "waypoints": [[0, 0]]},
]


def _bridge_dict(**ros2_overrides) -> dict:
    """Minimal valid bridge scenario dict; override ros2 keys as needed."""
    d: dict = {
        "name": "test-topology",
        "duration_s": 10.0,
        "substrate": "bridge",
        "radio": _RADIO,
        "nodes": _NODES_4,
        "workload": "ros2",
    }
    if ros2_overrides:
        d["ros2"] = ros2_overrides
    return d


def _make_bridge_engine(scenario: Scenario, tmp_path) -> ScenarioEngine:
    """Build a ScenarioEngine and stub out topo so _workload_cmd can be
    called without running NetnsTopology.setup() (which needs root)."""
    e = ScenarioEngine(scenario, out_dir=tmp_path / "run")
    # _workload_cmd only needs topo.nodes (list) and topo.ns_names (dict);
    # SimpleNamespace is simpler than NetnsTopology.__new__ + manual attr set.
    nids = [n.id for n in scenario.nodes]
    e.topo = SimpleNamespace(
        nodes=nids,
        ns_names={nid: f"ncz_{nid}" for nid in nids},
    )
    return e


# ---------------------------------------------------------------------------
# Ros2WorkloadConfig — standalone field validation (no Scenario context needed)
# ---------------------------------------------------------------------------

class TestRos2WorkloadConfigTopologyFields:
    def test_topology_defaults_to_mesh(self):
        c = Ros2WorkloadConfig()
        assert c.topology == "mesh"
        assert c.hub_id == ""

    @pytest.mark.parametrize("topo", ["mesh", "shared", "star"])
    def test_all_topology_literals_accepted(self, topo: str):
        c = Ros2WorkloadConfig(topology=topo)
        assert c.topology == topo

    def test_invalid_topology_literal_rejected(self):
        """A topology value outside the allowed Literal must raise ValidationError."""
        with pytest.raises(ValidationError):
            Ros2WorkloadConfig(topology="ring")

    def test_hub_id_round_trips_as_string(self):
        c = Ros2WorkloadConfig(topology="star", hub_id="d1")
        assert c.hub_id == "d1"


# ---------------------------------------------------------------------------
# Scenario-level validator: topology × hub_id cross-field rules
# ---------------------------------------------------------------------------

class TestTopologyValidator:
    def test_star_with_known_hub_id_validates(self):
        """star + hub_id equal to a real node id must produce a valid Scenario."""
        s = Scenario.model_validate(_bridge_dict(topology="star", hub_id="d1"))
        assert s.ros2.topology == "star"
        assert s.ros2.hub_id == "d1"

    def test_star_with_empty_hub_id_raises(self):
        """star without hub_id (empty string) must raise (hub is required for star)."""
        with pytest.raises(ValidationError, match="hub_id"):
            Scenario.model_validate(_bridge_dict(topology="star", hub_id=""))

    def test_star_with_unknown_hub_id_raises(self):
        """star with hub_id not matching any node id must raise."""
        with pytest.raises(ValidationError, match="hub_id"):
            Scenario.model_validate(
                _bridge_dict(topology="star", hub_id="ghost"))

    def test_mesh_with_hub_id_raises(self):
        """Non-star topology with hub_id set must raise (stray hub_id)."""
        with pytest.raises(ValidationError, match="hub_id"):
            Scenario.model_validate(_bridge_dict(topology="mesh", hub_id="d1"))

    def test_shared_with_empty_hub_id_validates(self):
        """shared topology with hub_id='' (default) must pass."""
        s = Scenario.model_validate(_bridge_dict(topology="shared"))
        assert s.ros2.topology == "shared"
        assert s.ros2.hub_id == ""

    def test_shared_with_hub_id_raises(self):
        """shared topology with hub_id set must raise (stray hub_id)."""
        with pytest.raises(ValidationError, match="hub_id"):
            Scenario.model_validate(
                _bridge_dict(topology="shared", hub_id="d1"))

    def test_mesh_default_validates_without_hub_id(self):
        """Default mesh scenario (no ros2 block at all) must validate cleanly."""
        s = Scenario.model_validate(_bridge_dict())
        assert s.ros2.topology == "mesh"
        assert s.ros2.hub_id == ""

    def test_non_ros2_workload_skips_topology_validator(self):
        """topology validator must be a no-op when workload != 'ros2'; the
        agent track has no hub concept and must not be affected by this rule."""
        d = _bridge_dict()
        d["workload"] = "agent"
        s = Scenario.model_validate(d)
        assert s.workload == "agent"


# ---------------------------------------------------------------------------
# _workload_cmd — bridge substrate: topology and role flags in the argv
# ---------------------------------------------------------------------------

class TestWorkloadCmdTopology:
    """_workload_cmd builds the `ip netns exec ... python -m netcom_zen.ros2_workload`
    argv for the bridge substrate.  Tests here check only the argv shape
    (no subprocess launched, no netns, no root)."""

    @pytest.fixture
    def star_engine(self, tmp_path):
        """Engine with topology=star, hub_id=d1.  topo stub added — no setup()."""
        s = Scenario.model_validate(_bridge_dict(topology="star", hub_id="d1"))
        return _make_bridge_engine(s, tmp_path)

    def test_hub_node_gets_role_hub(self, star_engine):
        """`_workload_cmd` for the hub node must include `--role hub`."""
        cmd = star_engine._workload_cmd("d1")
        assert "--role" in cmd
        assert cmd[cmd.index("--role") + 1] == "hub"

    def test_spoke_node_gets_role_spoke(self, star_engine):
        """`_workload_cmd` for a spoke node must include `--role spoke`."""
        cmd = star_engine._workload_cmd("d2")
        assert "--role" in cmd
        assert cmd[cmd.index("--role") + 1] == "spoke"

    @pytest.mark.parametrize("nid", ["d1", "d2"])
    def test_topology_flag_is_star(self, star_engine, nid: str):
        """Both hub and spoke argv must carry `--topology star`."""
        cmd = star_engine._workload_cmd(nid)
        assert "--topology" in cmd
        assert cmd[cmd.index("--topology") + 1] == "star"

    @pytest.mark.parametrize("nid", ["d1", "d2"])
    def test_peers_empty_for_star(self, star_engine, nid: str):
        """star topology has no per-peer subscriptions; --peers must be empty."""
        cmd = star_engine._workload_cmd(nid)
        peers_val = cmd[cmd.index("--peers") + 1]
        assert peers_val == ""

    @pytest.fixture
    def mesh_engine(self, tmp_path):
        """Engine with default mesh topology (no topology key in ros2 block)."""
        s = Scenario.model_validate(_bridge_dict())
        return _make_bridge_engine(s, tmp_path)

    def test_mesh_topology_flag(self, mesh_engine):
        """mesh (default) must include `--topology mesh` in the argv."""
        cmd = mesh_engine._workload_cmd("d1")
        assert "--topology" in cmd
        assert cmd[cmd.index("--topology") + 1] == "mesh"

    def test_mesh_peers_non_empty(self, mesh_engine):
        """mesh --peers must list all other nodes (non-empty)."""
        cmd = mesh_engine._workload_cmd("d1")
        peers_val = cmd[cmd.index("--peers") + 1]
        assert peers_val != "", "--peers must not be empty for mesh topology"
        assert "d1" not in peers_val.split(","), "node must not be in its own peer list"

    def test_mesh_role_defaults_to_spoke(self, mesh_engine):
        """mesh scenario has hub_id=''; every node's role must be 'spoke'."""
        for nid in ("d1", "d2", "d3", "d4"):
            cmd = mesh_engine._workload_cmd(nid)
            assert cmd[cmd.index("--role") + 1] == "spoke"

    @pytest.fixture
    def shared_engine(self, tmp_path):
        """Engine with topology=shared (one aggregation topic, O(N) endpoints)."""
        s = Scenario.model_validate(_bridge_dict(topology="shared"))
        return _make_bridge_engine(s, tmp_path)

    def test_shared_topology_flag(self, shared_engine):
        """shared must include `--topology shared` in the argv."""
        cmd = shared_engine._workload_cmd("d1")
        assert "--topology" in cmd
        assert cmd[cmd.index("--topology") + 1] == "shared"

    def test_shared_peers_empty(self, shared_engine):
        """shared topology: no per-peer subs, so --peers must be empty."""
        cmd = shared_engine._workload_cmd("d1")
        assert cmd[cmd.index("--peers") + 1] == ""


# ---------------------------------------------------------------------------
# _lan_node_cmd — lan substrate: topology and role carried through both
# local (flat argv) and remote (shlex.split(cmd[-1]) inner argv) forms
# ---------------------------------------------------------------------------

_LAN_HOSTS = {
    "local": {"addr": "192.168.1.1", "iface": "eth0"},
    "mini": {
        "ssh": "user@192.168.1.2",
        "addr": "192.168.1.2",
        "iface": "eth1",
    },
}
_LAN_NODES_4 = [
    {"id": "d1", "waypoints": [[0, 0]], "host": "local"},
    {"id": "d2", "waypoints": [[0, 0]], "host": "local"},
    {"id": "d3", "waypoints": [[0, 0]], "host": "mini"},
    {"id": "d4", "waypoints": [[0, 0]], "host": "mini"},
]


class TestLanNodeCmdTopology:
    """_lan_node_cmd returns either a flat argv list (local node) or the 5-element
    ssh wrapper ["ssh", "-o", "BatchMode=yes", target, <shlex-joined string>]
    (remote node).  Topology / role checks parse the inner argv via shlex.split
    for remote nodes — the same round-trip ssh uses when invoking the remote shell."""

    @pytest.fixture
    def star_lan_engine(self, tmp_path):
        """Engine with star topology (hub=d1) on the lan substrate."""
        d = {
            "name": "test-lan-star",
            "duration_s": 10.0,
            "substrate": "lan",
            "radio": _RADIO,
            "hosts": _LAN_HOSTS,
            "nodes": _LAN_NODES_4,
            "workload": "ros2",
            "ros2": {"topology": "star", "hub_id": "d1"},
        }
        s = Scenario.model_validate(d)
        return ScenarioEngine(s, out_dir=tmp_path / "run_lan_star")

    # ---- local nodes (flat argv, no shlex wrapping) -------------------------

    def test_local_hub_role_hub(self, star_lan_engine):
        """Local hub node (d1) argv must carry `--role hub`."""
        cmd = star_lan_engine._lan_node_cmd("d1", "local")
        assert "--role" in cmd
        assert cmd[cmd.index("--role") + 1] == "hub"

    def test_local_spoke_role_spoke(self, star_lan_engine):
        """Local spoke node (d2) argv must carry `--role spoke`."""
        cmd = star_lan_engine._lan_node_cmd("d2", "local")
        assert "--role" in cmd
        assert cmd[cmd.index("--role") + 1] == "spoke"

    def test_local_topology_flag(self, star_lan_engine):
        """Local node argv must carry `--topology star`."""
        cmd = star_lan_engine._lan_node_cmd("d1", "local")
        assert "--topology" in cmd
        assert cmd[cmd.index("--topology") + 1] == "star"

    def test_local_peers_empty_for_star(self, star_lan_engine):
        """star topology: local node --peers must be empty."""
        cmd = star_lan_engine._lan_node_cmd("d1", "local")
        assert cmd[cmd.index("--peers") + 1] == ""

    # ---- remote nodes (inner argv extracted via shlex.split(cmd[-1])) -------

    def test_remote_spoke_inner_has_topology_star(self, star_lan_engine):
        """Remote spoke (d3, on mini) inner argv must carry `--topology star`."""
        cmd = star_lan_engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        assert "--topology" in inner
        assert inner[inner.index("--topology") + 1] == "star"

    def test_remote_spoke_inner_has_role_spoke(self, star_lan_engine):
        """Remote spoke (d3) inner argv must carry `--role spoke`."""
        cmd = star_lan_engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        assert "--role" in inner
        assert inner[inner.index("--role") + 1] == "spoke"

    def test_remote_peers_empty_for_star(self, star_lan_engine):
        """Remote star node inner argv --peers must be empty."""
        cmd = star_lan_engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        assert inner[inner.index("--peers") + 1] == ""
