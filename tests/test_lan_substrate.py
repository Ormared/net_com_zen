"""Tests for the 'lan' substrate: config validation, XML generators with
interface-pinning args, _lan_node_cmd command structure, _lan_launcher_script
content, bridge_run_metrics mesh denominator, and smoke-YAML parse.

All tests run in the default pixi env with NO network access — no SSH, no
Docker, no real DDS processes.  Everything exercised here is pure Python:
config schema parsing, XML string generators, argv/script construction, and
in-process report metric calculation against synthetic fixtures.
"""
from __future__ import annotations

import json
import shlex
from pathlib import Path

import pyarrow as pa
import pyarrow.parquet as pq
import pytest
import yaml
from pydantic import ValidationError

from netcom_zen.config import LanHostConfig, Scenario
from netcom_zen.harness.report import bridge_run_metrics
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
# d1,d2 on local; d3,d4,d5 on mini — used by launcher-script tests to exercise
# a host with 3 remote nodes (enough to verify 3 export blocks + 2 sleeps).
_NODES_5 = [
    {"id": "d1", "waypoints": [[0, 0]], "host": "local"},
    {"id": "d2", "waypoints": [[0, 0]], "host": "local"},
    {"id": "d3", "waypoints": [[0, 0]], "host": "mini"},
    {"id": "d4", "waypoints": [[0, 0]], "host": "mini"},
    {"id": "d5", "waypoints": [[0, 0]], "host": "mini"},
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
    """Tests for _lan_node_cmd argv structure.

    The remote command is now a 5-element list:
        ["ssh", "-o", "BatchMode=yes", target, <one shlex-joined string>]
    where cmd[-1] is the shlex.join of the entire docker-exec argv.  Tests
    that inspect the remote argv must parse cmd[-1] with shlex.split to
    reconstruct the individual tokens — doing `"docker" in cmd` would fail
    because "docker" lives inside the joined string, not as its own element.
    """

    @pytest.fixture
    def engine(self, tmp_path):
        """ScenarioEngine initialised with the two-host lan scenario, no run."""
        s = Scenario.model_validate(_lan_dict())
        return ScenarioEngine(s, out_dir=tmp_path / "run_smoke")

    # ---- local node tests (cmd is a flat argv list, no shlex wrapping) ------

    def test_local_node_has_no_ssh_in_argv(self, engine):
        """Local node command must not contain 'ssh' anywhere in argv."""
        cmd = engine._lan_node_cmd("d1", "local")
        assert "ssh" not in cmd

    def test_local_node_starts_with_python_module(self, engine):
        """Local node argv: [sys.executable, '-m', 'netcom_zen.ros2_workload', ...]"""
        cmd = engine._lan_node_cmd("d1", "local")
        assert "-m" in cmd
        assert "netcom_zen.ros2_workload" in cmd

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

    # ---- remote node tests (cmd[-1] is the shlex-joined remote command) -----

    def test_remote_node_outer_shape(self, engine):
        """Remote argv outer shape: exactly ["ssh", "-o", "BatchMode=yes",
        target, <one joined string>] — 5 elements total."""
        cmd = engine._lan_node_cmd("d3", "mini")
        assert cmd[0] == "ssh"
        assert cmd[1] == "-o"
        assert cmd[2] == "BatchMode=yes"
        assert cmd[3] == engine.scenario.hosts["mini"].ssh
        assert len(cmd) == 5   # cmd[4] is the single shlex-joined string

    def test_remote_node_inner_starts_with_docker_exec(self, engine):
        """Parsed remote argv must start with ['docker', 'exec']."""
        cmd = engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        assert inner[0] == "docker"
        assert inner[1] == "exec"

    def test_remote_node_inner_contains_container_name(self, engine):
        """Parsed remote argv must contain the configured container name."""
        cmd = engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        container = engine.scenario.hosts["mini"].container
        assert container in inner

    def test_remote_node_inner_contains_remote_metrics_path(self, engine):
        """Parsed remote argv must contain the container-side metrics path."""
        cmd = engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        h = engine.scenario.hosts["mini"]
        run_name = engine.out_dir.name
        expected = f"{h.workdir}/results/lan/{run_name}/agent_d3.jsonl"
        assert expected in inner

    def test_peers_exclude_self_remote(self, engine):
        """--peers for a remote node must not include the node itself."""
        cmd = engine._lan_node_cmd("d3", "mini")
        inner = shlex.split(cmd[-1])
        peers_val = inner[inner.index("--peers") + 1]
        peers = peers_val.split(",")
        assert "d3" not in peers
        assert set(peers) == {"d1", "d2", "d4"}

    def test_extra_env_embedded_as_e_flags(self, engine):
        """extra_env dict must appear as -e KEY=VAL pairs inside the joined string."""
        cmd = engine._lan_node_cmd("d3", "mini",
                                   extra_env={"FOO": "bar", "BAZ": "qux"})
        inner = shlex.split(cmd[-1])
        # Collect all -e values
        e_vals = {inner[i + 1]
                  for i, tok in enumerate(inner)
                  if tok == "-e" and i + 1 < len(inner)}
        assert "FOO=bar" in e_vals
        assert "BAZ=qux" in e_vals

    def test_remote_env_shlex_roundtrip_preserves_bracketed_quotes(self, engine):
        """ZENOH_CONFIG_OVERRIDE value with bracketed endpoint list must survive
        the shlex.join / shlex.split round-trip that ssh uses for remote commands.

        This is the regression guard for the bug where the remote shell stripped
        double-quotes from `["tcp/127.0.0.1:7447"]`, leaving `[tcp/127.0.0.1:7447]`
        and causing zenoh to reject the config with Json5Err.
        """
        zenoh_val = 'connect/endpoints=["tcp/127.0.0.1:7447"]'
        cmd = engine._lan_node_cmd("d3", "mini", extra_env={
            "ZENOH_CONFIG_OVERRIDE": zenoh_val,
            "RMW_IMPLEMENTATION": "rmw_zenoh_cpp",
        })
        # cmd[-1] is the single string ssh hands to the remote shell.
        # Parsing it with shlex.split simulates what the remote shell does.
        inner = shlex.split(cmd[-1])
        recovered: str | None = None
        for i, tok in enumerate(inner):
            if tok == "-e" and i + 1 < len(inner):
                kv = inner[i + 1]
                if kv.startswith("ZENOH_CONFIG_OVERRIDE="):
                    # split on first "=" only; the value may contain "="
                    recovered = kv.split("=", 1)[1]
                    break
        assert recovered is not None, (
            "ZENOH_CONFIG_OVERRIDE -e flag not found in parsed remote argv")
        assert recovered == zenoh_val, (
            f"double-quotes were lost through shlex round-trip: "
            f"got {recovered!r}, expected {zenoh_val!r}")


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


# ---------------------------------------------------------------------------
# _lan_launcher_script — POSIX sh script content
# ---------------------------------------------------------------------------

def _lan_dict_5(**ros2_overrides) -> dict:
    """Like _lan_dict but with _NODES_5 (3 remote nodes on 'mini')."""
    d = _lan_dict(**ros2_overrides)
    d["nodes"] = _NODES_5
    return d


class TestLanLauncherScript:
    """Tests for _lan_launcher_script — the per-host POSIX sh launcher.

    All tests are purely structural (string inspection): no subprocess, no
    ssh, no Docker.  The method is pure Python, constructing the script from
    self.scenario and the node_envs dict argument.
    """

    @pytest.fixture
    def engine_3r(self, tmp_path):
        """Engine with 5-node scenario: d1/d2 local, d3/d4/d5 on 'mini'."""
        s = Scenario.model_validate(_lan_dict_5())
        return ScenarioEngine(s, out_dir=tmp_path / "run_launcher")

    def _mini_envs(self, zenoh_val: str | None = None) -> dict[str, dict]:
        """Minimal node_envs for the three nodes on 'mini'."""
        extra = ({"ZENOH_CONFIG_OVERRIDE": zenoh_val} if zenoh_val else {})
        return {nid: {"RMW_IMPLEMENTATION": "rmw_zenoh_cpp", **extra}
                for nid in ("d3", "d4", "d5")}

    def test_script_has_three_export_blocks(self, engine_3r):
        """Must contain exactly 3 backgrounded subshells (one per remote node)."""
        script = engine_3r._lan_launcher_script("mini", self._mini_envs())
        assert script.count("( export") == 3

    def test_script_ends_with_wait(self, engine_3r):
        """'wait' must be the last non-empty line (joins all background jobs)."""
        script = engine_3r._lan_launcher_script("mini", self._mini_envs())
        assert script.strip().endswith("wait")

    def test_script_contains_exits_txt_per_node(self, engine_3r):
        """Each subshell block must append to exits_mini.txt (3 references)."""
        script = engine_3r._lan_launcher_script("mini", self._mini_envs())
        # host-specific filename avoids collisions when two remote hosts rsync
        # back into the same local out_dir.
        assert script.count("exits_mini.txt") == 3

    def test_rmw_implementation_in_every_export_block(self, engine_3r):
        """RMW_IMPLEMENTATION must appear in every export block (3 occurrences)."""
        script = engine_3r._lan_launcher_script("mini", self._mini_envs())
        assert script.count("RMW_IMPLEMENTATION") == 3

    def test_zenoh_value_shlex_quoted_in_script(self, engine_3r):
        """ZENOH_CONFIG_OVERRIDE value must appear as shlex.quote(value) in the
        script — bare double-quotes in the value survive the remote shell only
        when the value is wrapped in single quotes by shlex.quote."""
        zenoh_val = 'connect/endpoints=["tcp/127.0.0.1:7447"]'
        script = engine_3r._lan_launcher_script("mini", self._mini_envs(zenoh_val))
        assert shlex.quote(zenoh_val) in script

    def test_zenoh_value_roundtrips_via_shlex_split(self, engine_3r):
        """shlex.split of the single-quoted assignment must recover the original
        value unchanged — regression guard for the double-quote-stripping bug."""
        zenoh_val = 'connect/endpoints=["tcp/127.0.0.1:7447"]'
        script = engine_3r._lan_launcher_script("mini", self._mini_envs(zenoh_val))
        quoted = shlex.quote(zenoh_val)
        # Simulate the remote shell parsing: wrap in a synthetic assignment and
        # split it back to verify the value is reconstructed identically.
        recovered = shlex.split(f"x={quoted}")[0].split("=", 1)[1]
        assert recovered == zenoh_val

    def test_stagger_sleep_lines_when_nonzero(self, tmp_path):
        """spawn_stagger_ms=100 → 2 sleep lines (between 3 nodes, not after last)."""
        s = Scenario.model_validate(_lan_dict_5(spawn_stagger_ms=100.0))
        engine = ScenarioEngine(s, out_dir=tmp_path / "run_stagger")
        script = engine._lan_launcher_script("mini", self._mini_envs())
        # 3 nodes → 2 gaps → exactly 2 sleep lines
        assert script.count("\nsleep ") == 2

    def test_no_sleep_when_stagger_zero(self, engine_3r):
        """Default spawn_stagger_ms=0 must produce no sleep lines."""
        script = engine_3r._lan_launcher_script("mini", self._mini_envs())
        assert "sleep" not in script


# ---------------------------------------------------------------------------
# bridge_run_metrics — manifest n_nodes as mesh denominator (Fix 2)
# ---------------------------------------------------------------------------

def _write_jsonl(path: Path, events: list[dict]) -> None:
    """Compact JSON lines matching the format written by ros2_workload/node.py."""
    path.write_text(
        "\n".join(json.dumps(e, separators=(",", ":")) for e in events) + "\n"
    )


class TestReportDenominator:
    """bridge_run_metrics must use manifest n_nodes (not JSONL file count) as
    the mesh_completeness denominator so missing/never-spawned nodes drive the
    metric toward 0 instead of being silently excluded (which inflated mesh
    to 1.0 when 6 nodes failed to spawn in the first cross-host sweep).
    """

    @pytest.fixture
    def sparse_run(self, tmp_path):
        """4-node manifest but only d1 and d2 have agent files (d3/d4 never
        spawned).  Connected pairs: d1←d2 and d2←d1 (2 out of 4*3=12 total).
        """
        # d1: starts, publishes once, receives once from d2
        _write_jsonl(tmp_path / "agent_d1.jsonl", [
            {"type": "start", "id": "d1", "ts_us": 1_000_000},
            {"type": "pub",   "id": "d1", "seq": 1, "kind": "snap",
             "ts_us": 1_100_000, "bytes": 50},
            {"type": "recv",  "id": "d1", "from": "d2",
             "peer_seq": 1, "peer_ts_us": 1_100_000,
             "ts_us": 1_200_000, "bytes": 50},
        ])
        # d2: starts, publishes once, receives once from d1
        _write_jsonl(tmp_path / "agent_d2.jsonl", [
            {"type": "start", "id": "d2", "ts_us": 1_000_000},
            {"type": "pub",   "id": "d2", "seq": 1, "kind": "snap",
             "ts_us": 1_100_000, "bytes": 50},
            {"type": "recv",  "id": "d2", "from": "d1",
             "peer_seq": 1, "peer_ts_us": 1_100_000,
             "ts_us": 1_200_000, "bytes": 50},
        ])
        # d3 and d4 have no agent files (they never spawned).

        # Minimal resources.parquet (bridge_run_metrics reads this regardless).
        pq.write_table(pa.table({
            "t":              pa.array([1.0, 1.0], type=pa.float64()),
            "proc":           pa.array(["d1", "d2"]),
            "cpu_pct":        pa.array([10.0, 10.0], type=pa.float64()),
            "rss_bytes":      pa.array([50_000_000, 50_000_000], type=pa.int64()),
            "host_cpu_pct":   pa.array([5.0, 5.0], type=pa.float64()),
            "host_mem_used":  pa.array([4_000_000_000, 4_000_000_000],
                                       type=pa.int64()),
            "host_mem_avail": pa.array([4_000_000_000, 4_000_000_000],
                                       type=pa.int64()),
            "host_swap_used": pa.array([0, 0], type=pa.int64()),
        }), tmp_path / "resources.parquet")

        (tmp_path / "manifest.json").write_text(json.dumps({
            "substrate": "bridge",
            "rmw": "fastrtps",
            "n_nodes": 4,   # 4 declared; only 2 reported — the fix's scenario
            "resource_samples": 1,
            "peak_host_mem_used_bytes": 4_000_000_000,
            "min_host_mem_avail_bytes": 4_000_000_000,
            "peak_host_swap_used_bytes": 0,
            "agent_exit_codes": {"d1": 0, "d2": 0},
        }))
        return tmp_path

    def test_mesh_denominator_uses_manifest_n_nodes(self, sparse_run):
        """mesh_completeness denominator must be n_manifest*(n_manifest-1)=12,
        not n_reporting*(n_reporting-1)=2, which would yield a falsely-perfect
        value of 1.0 (2/2) instead of the correct 2/12."""
        m = bridge_run_metrics(sparse_run)
        # 2 connected ordered pairs (d1→d2 and d2→d1) / (4*3 = 12 total)
        assert m["mesh_completeness"] == pytest.approx(2 / 12, abs=1e-9)

    def test_nodes_reporting_equals_file_count(self, sparse_run):
        """nodes_reporting must equal the count of agent_*.jsonl files present."""
        m = bridge_run_metrics(sparse_run)
        assert m["nodes_reporting"] == 2

    def test_nodes_reporting_less_than_n_nodes(self, sparse_run):
        """Sanity: nodes_reporting < n_nodes confirms the fix scenario is set up
        correctly (missing nodes are not silently included in reporting count)."""
        m = bridge_run_metrics(sparse_run)
        assert m["nodes_reporting"] < m["n_nodes"]
