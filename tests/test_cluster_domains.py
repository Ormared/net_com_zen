"""ROS_DOMAIN_ID cluster partitioning (docs/dds-topology-plan.md P5,
implication #1 of the DDS benchmark results).

The DDS benchmark proved that Fast DDS caps at ~33 mutually-discovered
participants per domain (SPDP-level, immovable) and Cyclone storms under
all-to-all SPDP at N≥48.  Splitting N nodes into K clusters on separate
ROS_DOMAIN_IDs (~N/K participants each) keeps each block below the knee.

Covers:
  - _cluster_of: contiguous-block formula and edge cases
  - Ros2WorkloadConfig.cluster_domains: field bounds
  - Scenario._cluster_domains_valid: accepted and rejected combinations
  - _domain_of: per-node domain lookup (via stubbed ScenarioEngine)
  - _workload_cmd: peer filtering restricted to same-domain nodes

No rclpy, no subprocess, no root — config validation and pure command-list
construction only.  _run_bridge / _run_lan are not called (they need root).
"""
from __future__ import annotations

from types import SimpleNamespace

import pytest
from pydantic import ValidationError

from netcom_zen.config import Ros2WorkloadConfig, Scenario
from netcom_zen.orchestrator import ScenarioEngine, _cluster_of

# ---------------------------------------------------------------------------
# Shared helpers
# ---------------------------------------------------------------------------

_RADIO = {
    "freq_hz": 433e6,
    "bandwidth_hz": 250000.0,
    "tx_power_dbm": 27.0,
    "data_rate_bps": 250000.0,
    "hop": {"n_channels": 50, "hop_rate_hz": 100.0},
}


def _nodes(n: int) -> list[dict]:
    """N minimal node dicts with ids d1..dN, all at origin."""
    return [{"id": f"d{i + 1}", "waypoints": [[0, 0]]} for i in range(n)]


def _bridge_dict(n: int = 8, **ros2_overrides) -> dict:
    """Minimal valid bridge+mesh+ros2 scenario dict; override ros2 keys as
    needed to exercise the cluster validator in isolation."""
    d: dict = {
        "name": "test-clusters",
        "duration_s": 10.0,
        "substrate": "bridge",
        "radio": _RADIO,
        "nodes": _nodes(n),
        "workload": "ros2",
    }
    if ros2_overrides:
        d["ros2"] = ros2_overrides
    return d


def _make_engine(scenario: Scenario, tmp_path) -> ScenarioEngine:
    """Build a ScenarioEngine with a stubbed topo so _workload_cmd and
    _domain_of work without NetnsTopology.setup() (which needs root).

    Mirrors the pattern used by test_topology_modes.py: SimpleNamespace with
    just the two attributes that _workload_cmd reads from topo."""
    e = ScenarioEngine(scenario, out_dir=tmp_path / "run")
    nids = [n.id for n in scenario.nodes]
    e.topo = SimpleNamespace(
        nodes=nids,
        ns_names={nid: f"ncz_{nid}" for nid in nids},
    )
    return e


# ---------------------------------------------------------------------------
# _cluster_of: contiguous-block formula
# ---------------------------------------------------------------------------

class TestClusterOf:
    """_cluster_of(idx, n, k) = idx*k//n.  Contiguous (not modulo) so cluster
    membership matches scenario/spawn order."""

    def test_n96_k4_first_block_is_0(self):
        """Nodes 0..23 all land in cluster 0 (the first quarter of 96)."""
        for idx in range(24):
            assert _cluster_of(idx, 96, 4) == 0, f"idx={idx} should be cluster 0"

    def test_n96_k4_second_block_starts_at_24(self):
        """Node 24 is the first member of cluster 1 (boundary of the 2nd block)."""
        assert _cluster_of(24, 96, 4) == 1

    def test_n96_k4_node47_in_cluster1(self):
        """Node 47 (last of the second quarter) is still cluster 1."""
        assert _cluster_of(47, 96, 4) == 1

    def test_n96_k4_last_node_in_cluster3(self):
        """Node 95 (last of 96) belongs to cluster 3 (the last quarter)."""
        # 95*4 = 380, 380//96 = 3  (3*96=288 < 380 < 4*96=384)
        assert _cluster_of(95, 96, 4) == 3

    def test_k1_all_nodes_in_cluster0(self):
        """k=1: single-cluster mode; every node maps to cluster 0."""
        for n in (4, 8, 96):
            for idx in range(n):
                assert _cluster_of(idx, n, 1) == 0, f"n={n} idx={idx}"

    def test_k_equals_n_each_node_own_cluster(self):
        """k==n: one node per cluster; node idx maps to cluster idx."""
        for n in (4, 8):
            for idx in range(n):
                assert _cluster_of(idx, n, n) == idx, f"n={n} idx={idx}"


# ---------------------------------------------------------------------------
# Ros2WorkloadConfig.cluster_domains field bounds
# ---------------------------------------------------------------------------

class TestClusterDomainsField:
    def test_default_is_0(self):
        """cluster_domains must default to 0 (single-domain, off)."""
        assert Ros2WorkloadConfig().cluster_domains == 0

    @pytest.mark.parametrize("k", [0, 1, 4, 32])
    def test_valid_range_accepted(self, k: int):
        """ge=0, le=32 constraint; boundary and interior values must parse."""
        c = Ros2WorkloadConfig(cluster_domains=k)
        assert c.cluster_domains == k

    def test_negative_rejected(self):
        """Negative cluster count is nonsensical and must be rejected (ge=0)."""
        with pytest.raises(ValidationError):
            Ros2WorkloadConfig(cluster_domains=-1)

    def test_exceeds_max_rejected(self):
        """33 exceeds le=32 (32 domains is already far beyond any sane split)."""
        with pytest.raises(ValidationError):
            Ros2WorkloadConfig(cluster_domains=33)


# ---------------------------------------------------------------------------
# Scenario-level validator: accepted and rejected combinations
# ---------------------------------------------------------------------------

class TestClusterDomainsValidator:
    def test_bridge_mesh_ros2_k4_validates(self):
        """The canonical combination (bridge + mesh + ros2 + k≤N) must pass."""
        s = Scenario.model_validate(_bridge_dict(n=8, cluster_domains=4))
        assert s.ros2.cluster_domains == 4

    def test_k0_default_validates_everywhere(self):
        """k=0 (off) must not trigger the cluster validator in any context."""
        # default — no ros2 block at all
        Scenario.model_validate(_bridge_dict(n=4))
        # explicit zero
        Scenario.model_validate(_bridge_dict(n=4, cluster_domains=0))

    def test_k_equals_n_validates(self):
        """cluster_domains == len(nodes) is valid (one node per cluster)."""
        s = Scenario.model_validate(_bridge_dict(n=4, cluster_domains=4))
        assert s.ros2.cluster_domains == 4

    def test_lan_substrate_raises(self):
        """cluster_domains > 0 must be rejected on substrate=lan: lan has no
        per-netns env injection and cluster partitioning is a bridge knob."""
        d = {
            "name": "test-lan-clusters",
            "duration_s": 10.0,
            "substrate": "lan",
            "radio": _RADIO,
            "nodes": [
                {"id": "d1", "waypoints": [[0, 0]], "host": "local"},
                {"id": "d2", "waypoints": [[0, 0]], "host": "local"},
            ],
            "hosts": {"local": {"addr": "192.168.1.1", "iface": "eth0"}},
            "workload": "ros2",
            "ros2": {"cluster_domains": 2},
        }
        with pytest.raises(ValidationError, match="cluster_domains"):
            Scenario.model_validate(d)

    def test_topology_shared_raises(self):
        """shared + clusters is a later combination; v1 must reject it."""
        with pytest.raises(ValidationError, match="cluster_domains"):
            Scenario.model_validate(
                _bridge_dict(n=4, cluster_domains=2, topology="shared"))

    def test_topology_star_raises(self):
        """star + clusters is a later combination; v1 must reject it.
        hub_id='d1' satisfies _topology_hub_id_valid so the cluster validator
        is the one that raises (topology != 'mesh')."""
        with pytest.raises(ValidationError, match="cluster_domains"):
            Scenario.model_validate(
                _bridge_dict(n=4, cluster_domains=2, topology="star",
                             hub_id="d1"))

    def test_discovery_server_raises(self):
        """cluster_domains + discovery_server=True must raise: the DS spawn
        path does not inject ROS_DOMAIN_ID, so the combination silently
        breaks client registration."""
        with pytest.raises(ValidationError, match="cluster_domains"):
            Scenario.model_validate(
                _bridge_dict(n=4, cluster_domains=2,
                             rmw="fastrtps", discovery_server=True))

    def test_k_greater_than_n_raises(self):
        """cluster_domains > len(nodes) would leave the last clusters empty;
        reject with a clear message so the user fixes the scenario YAML."""
        with pytest.raises(ValidationError, match="cluster_domains"):
            Scenario.model_validate(_bridge_dict(n=4, cluster_domains=5))


# ---------------------------------------------------------------------------
# _domain_of: per-node ROS_DOMAIN_ID lookup
# ---------------------------------------------------------------------------

class TestDomainOf:
    """Tests use a stubbed engine (SimpleNamespace topo) — no root, no
    subprocess, no NetnsTopology.setup() needed.  _domain_of is a pure
    in-memory computation over scenario.nodes list order."""

    @pytest.fixture
    def engine_k2(self, tmp_path):
        """N=8, k=2: d1-d4 → domain 0, d5-d8 → domain 1."""
        s = Scenario.model_validate(_bridge_dict(n=8, cluster_domains=2))
        return _make_engine(s, tmp_path)

    def test_first_cluster_domain(self, engine_k2):
        """d1..d4 (indices 0..3 of 8) must all be on domain 0."""
        for nid in ("d1", "d2", "d3", "d4"):
            assert engine_k2._domain_of(nid) == 0, f"{nid} should be domain 0"

    def test_second_cluster_domain(self, engine_k2):
        """d5..d8 (indices 4..7 of 8) must all be on domain 1."""
        for nid in ("d5", "d6", "d7", "d8"):
            assert engine_k2._domain_of(nid) == 1, f"{nid} should be domain 1"

    def test_k0_always_returns_0(self, tmp_path):
        """When clustering is off (k=0), _domain_of must return 0 for every
        node regardless of position (single-domain, backward compatible)."""
        s = Scenario.model_validate(_bridge_dict(n=4))
        e = _make_engine(s, tmp_path)
        for nid in ("d1", "d2", "d3", "d4"):
            assert e._domain_of(nid) == 0


# ---------------------------------------------------------------------------
# _workload_cmd: peer filtering restricted to same-domain nodes
# ---------------------------------------------------------------------------

class TestWorkloadCmdPeers:
    """N=8, k=2: cluster 0 = {d1..d4}, cluster 1 = {d5..d8}.

    When clustering is on, --peers must list intra-cluster peers only.
    Cross-cluster nodes are on a different ROS_DOMAIN_ID and cannot be
    discovered; including them in --peers would stall the peer-wait loop."""

    @pytest.fixture
    def engine_k2(self, tmp_path):
        s = Scenario.model_validate(_bridge_dict(n=8, cluster_domains=2))
        return _make_engine(s, tmp_path)

    def test_d1_peers_are_d2_to_d4_only(self, engine_k2):
        """d1 (cluster 0) must peer with d2, d3, d4 — no more, no less."""
        cmd = engine_k2._workload_cmd("d1")
        peers = set(cmd[cmd.index("--peers") + 1].split(","))
        assert peers == {"d2", "d3", "d4"}

    def test_d1_peers_exclude_cluster1(self, engine_k2):
        """d1 must NOT include any cluster-1 node (d5..d8) in --peers."""
        cmd = engine_k2._workload_cmd("d1")
        peers = cmd[cmd.index("--peers") + 1].split(",")
        for nid in ("d5", "d6", "d7", "d8"):
            assert nid not in peers, f"{nid} (domain 1) must not appear in d1's peers"

    def test_d1_not_in_own_peer_list(self, engine_k2):
        """A node must never appear in its own --peers (sanity check)."""
        cmd = engine_k2._workload_cmd("d1")
        peers = cmd[cmd.index("--peers") + 1].split(",")
        assert "d1" not in peers

    def test_d8_peers_are_d5_to_d7_only(self, engine_k2):
        """d8 (cluster 1, last node) must peer with d5, d6, d7 — no more."""
        cmd = engine_k2._workload_cmd("d8")
        peers = set(cmd[cmd.index("--peers") + 1].split(","))
        assert peers == {"d5", "d6", "d7"}

    def test_k0_mesh_peers_all_others(self, tmp_path):
        """Without clustering (k=0), mesh --peers must list ALL other nodes
        (existing behaviour unchanged; no regression)."""
        s = Scenario.model_validate(_bridge_dict(n=4))
        e = _make_engine(s, tmp_path)
        cmd = e._workload_cmd("d1")
        peers = set(cmd[cmd.index("--peers") + 1].split(","))
        assert peers == {"d2", "d3", "d4"}

    def test_non_mesh_topology_peers_empty_unaffected(self, tmp_path):
        """shared topology with k=0 must still return empty --peers (the
        non-mesh branch is not touched by the cluster change)."""
        s = Scenario.model_validate(_bridge_dict(n=4, topology="shared"))
        e = _make_engine(s, tmp_path)
        cmd = e._workload_cmd("d1")
        assert cmd[cmd.index("--peers") + 1] == ""
