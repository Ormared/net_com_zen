from __future__ import annotations

import asyncio
import json
import os
import pwd
import shlex
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

import numpy as np
import psutil

from .channel.forwarder import ChannelForwarder
from .channel.linkstate import LOG_REF_LENGTH_BYTES, build_table, table_rows
from .config import Scenario
from .ew import Jammer
from .metrics import PacketLog
from .mobility import MobilityProvider, make_mobility
from .netns import NetnsTopology, netem_args
from .propagation import CompositePathloss
from .propagation.sionna_grid import SionnaGrid, SionnaGridPathloss
from .terrain import FoliageRegion, Terrain


# Host kernel socket-buffer ceilings raised when socket_buffer_bytes>0. A DDS
# SocketReceiveBufferSize request is silently clamped to net.core.rmem_max (and
# Cyclone treats its `min` as a HARD floor — it refuses to start the domain if
# the kernel ceiling is below the request), so the ceiling MUST be raised first.
# Root (the orchestrator runs under sudo) writes /proc/sys directly; the `sysctl`
# binary isn't on the NOPASSWD list. These are global (not netns-scoped).
_KERNEL_BUF_SYSCTLS = ("rmem_max", "wmem_max", "rmem_default", "wmem_default")

# ARP neighbor-table GC ceilings raised for bridge swarms large enough to
# exhaust the host-wide default (128/512/1024). See _apply_neigh_thresholds
# for the root-cause discovery note (2026-07-03).
_NEIGH_BASE = "/proc/sys/net/ipv4/neigh/default"


def _cluster_of(idx: int, n: int, k: int) -> int:
    """Contiguous-block cluster index for a node at position idx (0-based, in
    scenario/spawn order) among n nodes split into k clusters.

    Formula: idx * k // n.  Contiguous (not modulo) so cluster membership
    matches spawn and stagger order: nodes 0..⌈n/k⌉-1 form cluster 0, the
    next block forms cluster 1, …, and node n-1 belongs to cluster k-1.
    Modulo assignment would interleave clusters and break the stagger-order
    assumption used by the SPDP discovery-storm hypothesis experiments."""
    return idx * k // n


def _cyclonedds_xml(socket_buffer_bytes: int = 0, iface_name: str = "",
                    internal: dict[str, str] | None = None,
                    peers: list[str] | None = None,
                    allow_multicast: bool = True) -> str:
    """Cyclone DDS config for the bridge or lan substrate: SHM/Iceoryx off
    (force real UDP, no same-host shortcut) and multicast on so native
    discovery floods across the bridge. Domain id="any" applies under any
    ROS_DOMAIN_ID.

    socket_buffer_bytes>0 adds Internal/SocketReceiveBufferSize (the buffer
    lever); the host rmem_max must already be >= it or Cyclone fails to start.

    iface_name (lan substrate): when set, replaces autodetermine=true with an
    explicit NIC name.  Mandatory on hosts with noise interfaces (zerotier,
    tailscale, docker bridges) because Cyclone announces locators on every
    interface it discovers; an unreachable locator causes silent discovery
    failure at swarm scale.  When empty, autodetermine picks the single non-lo
    interface in each netns (bridge substrate behaviour, unchanged)."""
    # One combined <Internal> block: the socket-buffer lever plus any extra
    # tuning elements from ros2.cyclonedds_internal (P5 lossy-link knobs, e.g.
    # NackDelay / RetransmitMerging). Keys are validated in config.py to be
    # bare element names, so rendering them verbatim cannot break the XML.
    internal_elems = ""
    if socket_buffer_bytes > 0:
        internal_elems += (f'      <SocketReceiveBufferSize '
                           f'min="{socket_buffer_bytes} B"/>\n')
    for key, value in (internal or {}).items():
        internal_elems += f'      <{key}>{value}</{key}>\n'
    buf = (f'    <Internal>\n{internal_elems}    </Internal>\n'
           if internal_elems else "")
    # Interface element: explicit NIC name when pinning is requested, otherwise
    # autodetermine (safe in netns where the only non-lo interface is the veth).
    multicast = str(allow_multicast).lower()
    if iface_name:
        iface_elem = (f'        <NetworkInterface name="{iface_name}" '
                      f'multicast="{multicast}"/>\n')
    else:
        iface_elem = (f'        <NetworkInterface autodetermine="true" '
                      f'multicast="{multicast}"/>\n')
    peer_xml = "".join(
        f'        <Peer Address="{addr}"/>\n' for addr in (peers or []))
    peers_block = (
        f'      <Peers>\n{peer_xml}      </Peers>\n'
        if peers is not None else "")
    return ('<?xml version="1.0" encoding="UTF-8" ?>\n'
            '<CycloneDDS xmlns="https://cdds.io/config"\n'
            '    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"\n'
            '    xsi:schemaLocation="https://cdds.io/config '
            'https://raw.githubusercontent.com/eclipse-cyclonedds/cyclonedds/'
            'master/etc/cyclonedds.xsd">\n'
            '  <Domain id="any">\n'
            '    <General>\n'
            '      <Interfaces>\n'
            f'{iface_elem}'
            '      </Interfaces>\n'
            f'      <AllowMulticast>{multicast}</AllowMulticast>\n'
            '    </General>\n'
            '    <SharedMemory>\n'
            '      <Enable>false</Enable>\n'
            '    </SharedMemory>\n'
            f'{buf}'
            # Cyclone assigns each same-IP participant an index (its unicast
            # port slot); past the default ceiling rmw_create_node dies with
            # "failed to create domain".  On lan, many nodes share one host IP
            # (P4 star: 96 spokes on one rig, spokes 33+ crashed), so size the
            # ceiling above the substrate cap (128).  Netns substrates give
            # every node its own IP (index 0) and never hit this.
            '    <Discovery>\n'
            '      <MaxAutoParticipantIndex>150</MaxAutoParticipantIndex>\n'
            f'{peers_block}'
            '    </Discovery>\n'
            '  </Domain>\n'
            '</CycloneDDS>\n')


def _zenoh_bridge_env() -> dict[str, str]:
    """Env every zenoh-bridge-ros2dds process needs, on any substrate.

    ROS_DISTRO: the bridge reads it to pick the `ros_discovery_info` wire format,
    which changed across distros. Unset, it logs "Assuming 'iron'" and can then
    misread Jazzy discovery data. sudo strips the pixi activation that would
    normally provide it, so it has to be passed explicitly.

    RUST_LOG is forwarded when set so a run can be re-executed with bridge-side
    DDS tracing without an engine edit."""
    env = {"ROS_DISTRO": "jazzy"}
    if "RUST_LOG" in os.environ:
        env["RUST_LOG"] = os.environ["RUST_LOG"]
    return env


def _zenoh_bridge_version(flavor: str) -> str | None:
    """Version string of the vendored bridge, for the result manifest.

    Bridge behaviour is version-sensitive and the binary is fetched out-of-band
    rather than pinned in pixi.lock, so the manifest is the only place a run's
    bridge version is recorded. Returns None when the flavour does not use it,
    and on any read failure -- a missing version string must not abort a run
    that has already produced its data."""
    if flavor != "bridge":
        return None
    try:
        return (_zenoh_bridge_bin().parent / "VERSION").read_text().strip()
    except Exception:
        return None


def _zenoh_bridge_bin() -> Path:
    """Resolve the vendored zenoh-bridge-ros2dds binary.

    Not a pixi dependency: zenoh-plugin-ros2dds is absent from conda-forge and
    robostack-jazzy, so scripts/fetch_zenoh_bridge.sh pulls the upstream release
    into vendor/.  That path is repo-relative on purpose -- dds_lan_deploy.sh
    rsyncs the repo and the container bind-mounts it at /work, so both hosts run
    the same bit-for-bit binary without an image rebuild.

    The binary is self-contained (libc/libm/libgcc only) and embeds its own
    CycloneDDS; it does NOT link the ros2 pixi env's DDS libraries."""
    # orchestrator.py lives at sim/netcom_zen/, so the repo root is 2 up.
    root = Path(__file__).resolve().parents[2]
    binary = root / "vendor" / "zenoh-bridge-ros2dds" / "zenoh-bridge-ros2dds"
    if not binary.exists():
        raise RuntimeError(
            f"{binary} not found: zenoh_flavor='bridge' needs the vendored "
            "zenoh-bridge-ros2dds. Run scripts/fetch_zenoh_bridge.sh")
    return binary


def _fastdds_profiles_xml(socket_buffer_bytes: int = 0,
                          allocation_participants: int = 0,
                          whitelist_addr: str = "") -> str:
    """Fast DDS XML profiles defining a single UDPv4 transport, set as the
    default participant profile. useBuiltinTransports false means ONLY this
    transport is active — which also keeps SHM off (same isolation as the
    FASTDDS_BUILTIN_TRANSPORTS=UDPv4 env-var route), so the profiles route is a
    drop-in replacement for the env-var route. Needs the <profiles> wrapper
    (Fast DDS 8.x) or the parser rejects transport_descriptors.

    socket_buffer_bytes > 0: emit <receiveBufferSize> / <sendBufferSize> in the
    transport descriptor. 0: omit both (descriptor + useBuiltinTransports=false
    stay, so SHM remains off — required isolation on this rig).

    allocation_participants > 0 (A): emit an <allocation> block inside <rtps>
    preallocating total_participants/readers/writers to A with increment=0 (fully
    preallocated, no reallocation during discovery bursts), and also
    <maxInitialPeersRange>A inside the transport_descriptor. This is the P1 lever
    for the ~33-participant Fast DDS clique cap (docs/dds-topology-plan.md).

    whitelist_addr (lan substrate): when set, emit <interfaceWhiteList> pinning
    Fast DDS to one NIC IP.  Mandatory on hosts with noise interfaces (zerotier,
    tailscale, docker bridges) because Fast DDS announces locators on every
    interface it binds to; an unreachable locator causes silent discovery failure.

    XSD element order matters to Fast DDS 8.x: within <rtps>, the sequence must
    be <userTransports>, <useBuiltinTransports>, <allocation>; within
    <transport_descriptor>, buffer sizes precede <interfaceWhiteList> which
    precedes <maxInitialPeersRange>."""
    # transport descriptor: buffer sizes (optional) then whitelist (optional)
    # then peer range (optional) — XSD order is send/receive, whitelist, peers.
    buf_elems = (
        f'        <receiveBufferSize>{socket_buffer_bytes}</receiveBufferSize>\n'
        f'        <sendBufferSize>{socket_buffer_bytes}</sendBufferSize>\n'
        if socket_buffer_bytes > 0 else '')
    whitelist_elem = (
        f'        <interfaceWhiteList>'
        f'<address>{whitelist_addr}</address>'
        f'</interfaceWhiteList>\n'
        if whitelist_addr else '')
    peers_elem = (
        f'        <maxInitialPeersRange>{allocation_participants}'
        f'</maxInitialPeersRange>\n'
        if allocation_participants > 0 else '')
    # allocation block inside <rtps>: initial==maximum==A, increment 0 means fully
    # preallocated — no runtime realloc when all A participants discover at once.
    alloc_block = (
        '        <allocation>\n'
        '          <remote_locators>\n'
        '            <max_unicast_locators>4</max_unicast_locators>\n'
        '            <max_multicast_locators>1</max_multicast_locators>\n'
        '          </remote_locators>\n'
        f'          <total_participants>\n'
        f'            <initial>{allocation_participants}</initial>\n'
        f'            <maximum>{allocation_participants}</maximum>\n'
        f'            <increment>0</increment>\n'
        f'          </total_participants>\n'
        f'          <total_readers>\n'
        f'            <initial>{allocation_participants}</initial>\n'
        f'            <maximum>{allocation_participants}</maximum>\n'
        f'            <increment>0</increment>\n'
        f'          </total_readers>\n'
        f'          <total_writers>\n'
        f'            <initial>{allocation_participants}</initial>\n'
        f'            <maximum>{allocation_participants}</maximum>\n'
        f'            <increment>0</increment>\n'
        f'          </total_writers>\n'
        '        </allocation>\n'
        if allocation_participants > 0 else '')
    return ('<?xml version="1.0" encoding="UTF-8" ?>\n'
            '<dds xmlns="http://www.eprosima.com/XMLSchemas/fastRTPS_Profiles">\n'
            '  <profiles>\n'
            '    <transport_descriptors>\n'
            '      <transport_descriptor>\n'
            '        <transport_id>udp_big</transport_id>\n'
            '        <type>UDPv4</type>\n'
            f'{buf_elems}'
            f'{whitelist_elem}'
            f'{peers_elem}'
            '      </transport_descriptor>\n'
            '    </transport_descriptors>\n'
            '    <participant profile_name="big" is_default_profile="true">\n'
            '      <rtps>\n'
            '        <userTransports><transport_id>udp_big</transport_id>'
            '</userTransports>\n'
            '        <useBuiltinTransports>false</useBuiltinTransports>\n'
            f'{alloc_block}'
            '      </rtps>\n'
            '    </participant>\n'
            '  </profiles>\n'
            '</dds>\n')


def _git_hash() -> str:
    try:
        return subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True,
                              text=True, check=True).stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "unknown"


def _ssh_base() -> list[str]:
    """SSH transport, reusing the invoking user's credentials under sudo.

    LAN orchestration needs root for local network setup, but home-mini trusts
    the interactive user's SSH key and known_hosts file, not /root/.ssh.
    Referencing those files in place avoids copying private credentials.
    """
    cmd = ["ssh", "-o", "BatchMode=yes"]
    sudo_user = os.environ.get("SUDO_USER")
    if os.geteuid() == 0 and sudo_user and sudo_user != "root":
        home = Path(pwd.getpwnam(sudo_user).pw_dir)
        known_hosts = home / ".ssh" / "known_hosts"
        if known_hosts.exists():
            cmd += ["-o", f"UserKnownHostsFile={known_hosts}"]
        for name in ("id_ed25519", "id_rsa", "id_ecdsa"):
            identity = home / ".ssh" / name
            if identity.exists():
                cmd += ["-o", "IdentitiesOnly=yes", "-i", str(identity)]
                break
    return cmd


def _ssh_cmd(target: str, remote_argv: list[str]) -> list[str]:
    """Build an ssh argv that reconstructs the remote command exactly.

    ssh(1) joins all arguments after the destination with spaces and passes
    the result to the remote login shell as a single string.  If remote_argv
    is passed as separate list elements, the remote shell re-parses them and
    loses quoting:
      - A `sh -c 'cat > /path'` arg becomes `sh -c cat` with `> /path` as a
        REDIRECT ON THE REMOTE HOST (not inside the container).
      - A `-e KEY=val["with","brackets"]` element has its quotes stripped,
        producing `[with,brackets]` instead of `["with","brackets"]`.

    shlex.join() produces one properly-quoted string; the remote shell splits
    it back into exactly the original tokens.  Equivalent to writing:
        ssh target 'docker exec -e "K=v" container cmd arg'
    but constructed programmatically without manual quoting.
    """
    return [*_ssh_base(), target, shlex.join(remote_argv)]


@dataclass
class World:
    """Scenario-derived simulation state, buildable without root (the netns
    dataplane is separate). Shared by the engine and the ROS 2 viz demo."""
    terrain: Terrain
    pathloss: CompositePathloss | SionnaGridPathloss
    mobility: MobilityProvider
    jammers: list[Jammer]


def build_world(scenario: Scenario) -> World:
    env = scenario.environment
    hm = np.load(env.heightmap) if env.heightmap else None
    terrain = Terrain(env.extent_m, hm,
                      [FoliageRegion(**f.model_dump()) for f in env.foliage])
    if env.pathloss == "sionna":
        pathloss = SionnaGridPathloss(SionnaGrid.load(env.sionna_grid), terrain)
    else:
        pathloss = CompositePathloss(terrain)
    return World(
        terrain=terrain,
        pathloss=pathloss,
        mobility=make_mobility(scenario),
        jammers=[Jammer(j) for j in scenario.jammers])


class ScenarioEngine:
    """Owns the single clock: tick loop driving mobility -> pathloss ->
    link-state -> forwarder (architecture.md)."""

    def __init__(self, scenario: Scenario, out_dir: str | Path,
                 ros2_viz: bool = False):
        self.scenario = scenario
        self.out_dir = Path(out_dir)
        self.ros2_viz = ros2_viz
        self.ready = asyncio.Event()
        self.topo: NetnsTopology | None = None

    def _build(self) -> None:
        w = build_world(self.scenario)
        self.terrain, self.pathloss = w.terrain, w.pathloss
        self.mobility, self.jammers = w.mobility, w.jammers

    def _spawn_agents(self) -> dict[str, subprocess.Popen]:
        cfg = self.scenario.agent
        bin_path = os.environ.get("NCZ_AGENT_BIN", "agent/target/release/ncz-agent")
        agents: dict[str, subprocess.Popen] = {}
        proto = cfg.transport  # "tcp" | "udp"
        for nid in self.topo.nodes:
            others = [f"{proto}/{self.topo.addrs[n]}:{cfg.port}"
                      for n in self.topo.nodes if n != nid]
            cmd = ["ip", "netns", "exec", self.topo.ns_names[nid], bin_path,
                   "--id", nid,
                   "--listen", f"{proto}/{self.topo.addrs[nid]}:{cfg.port}",
                   "--metrics", str(self.out_dir / f"agent_{nid}.jsonl"),
                   "--period-ms", str(cfg.period_ms),
                   "--duration-s", str(self.scenario.duration_s),
                   "--full-every", str(cfg.full_every),
                   "--wait-peers", str(len(self.topo.nodes) - 1),
                   "--members", ",".join(self.topo.nodes)]
            if not cfg.mls:
                cmd.append("--plaintext")
            # best-effort over UDP is the point of the A/B: avoid retransmit
            # stalls under loss (CRDT snapshots heal the gaps). TCP is reliable.
            cmd += ["--reliability",
                    "best-effort" if cfg.transport == "udp" else "reliable"]
            cmd += ["--sync-mode", cfg.sync_mode]
            # M4: vehicles track the command node's link; command tracks none
            cmd_id = self.scenario.command_id
            if cmd_id and nid != cmd_id:
                cmd += ["--command-id", cmd_id]
            for o in others:
                cmd += ["--connect", o]
            agents[nid] = subprocess.Popen(cmd)
        return agents

    def _ros2_base_env(self, rmw_impl: str, log_dir: Path) -> dict:
        """Minimal child env: sudo strips activation, so children need only
        these (verified against a bare `env -i` in R3)."""
        prefix = Path(sys.executable).resolve().parents[1]
        return {
            "AMENT_PREFIX_PATH": str(prefix),
            "ROS_LOG_DIR": str(log_dir),
            "RMW_IMPLEMENTATION": rmw_impl,
            "PYTHONNOUSERSITE": "1",
            "PATH": os.environ.get("PATH", "/usr/sbin:/usr/bin:/sbin:/bin"),
        }

    def _domain_of(self, nid: str) -> int:
        """ROS_DOMAIN_ID (cluster index) for node nid.

        Returns 0 when cluster_domains==0 (single domain, all existing
        behaviour unchanged).  Otherwise looks up nid's position in scenario
        node order — the canonical spawn/stagger order — and delegates to
        the module-level _cluster_of for the contiguous-block assignment.
        Unit-testable without root: no subprocess, no netns, no filesystem."""
        k = self.scenario.ros2.cluster_domains
        if k == 0:
            return 0
        nodes = [n.id for n in self.scenario.nodes]
        return _cluster_of(nodes.index(nid), len(nodes), k)

    def _workload_cmd(self, nid: str) -> list[str]:
        cfg = self.scenario.ros2
        # For shared/star topologies --peers is unused (O(N) endpoint wiring
        # has no per-peer subscriptions); pass empty string so node.py fails
        # loudly if it accidentally enters mesh mode with an empty peer list.
        # When cluster partitioning is on, restrict mesh peers to the same
        # ROS_DOMAIN_ID block only: cross-cluster pairs are on different domains
        # and cannot discover each other, so including them in --peers would
        # stall the workload's peer-wait loop indefinitely.
        if cfg.topology != "mesh":
            peers = ""
        elif cfg.cluster_domains > 0:
            my_domain = self._domain_of(nid)
            peers = ",".join(
                p for p in self.topo.nodes
                if p != nid and self._domain_of(p) == my_domain
            )
        else:
            peers = ",".join(p for p in self.topo.nodes if p != nid)
        role, assigned_server, activate_after = self._workload_role(nid)
        return ["ip", "netns", "exec", self.topo.ns_names[nid],
                sys.executable, "-m", "netcom_zen.ros2_workload",
                "--id", nid,
                "--peers", peers,
                "--topology", cfg.topology,
                "--role", role,
                *self._centralized_workload_args(
                    assigned_server, activate_after),
                "--period-ms", str(cfg.period_ms),
                "--payload-bytes", str(cfg.payload_bytes),
                "--reliability", cfg.reliability,
                "--durability", cfg.durability,
                "--history", cfg.history,
                "--depth", str(cfg.depth),
                "--deadline-ms", str(cfg.deadline_ms),
                "--lifespan-ms", str(cfg.lifespan_ms),
                "--liveliness", cfg.liveliness,
                "--liveliness-lease-ms", str(cfg.liveliness_lease_ms),
                "--duration-s", str(self.scenario.duration_s),
                "--metrics", str(self.out_dir / f"agent_{nid}.jsonl")]

    def _workload_role(self, nid: str) -> tuple[str, str, float]:
        """Return workload role, shard assignment, and standby activation."""
        cfg = self.scenario.ros2
        if cfg.topology != "centralized":
            return ("hub" if nid == cfg.hub_id else "spoke", "", -1.0)
        central = cfg.centralized
        assert central is not None
        if nid in central.server_ids:
            if (central.mode == "active_passive"
                    and nid != central.primary_server_id):
                return ("standby", nid,
                        float(central.standby_activate_after_s))
            return ("server", nid, -1.0)
        clients = [
            n.id for n in self.scenario.nodes
            if n.id not in central.server_ids
        ]
        assigned = central.server_ids[
            clients.index(nid) % len(central.server_ids)]
        return "client", assigned, -1.0

    def _centralized_workload_args(
            self, assigned_server: str, activate_after: float) -> list[str]:
        cfg = self.scenario.ros2
        if cfg.topology != "centralized":
            return []
        central = cfg.centralized
        assert central is not None
        return [
            "--central-mode", central.mode,
            "--server-ids", ",".join(central.server_ids),
            "--assigned-server", assigned_server,
            "--activate-after-s", str(activate_after),
            "--telemetry-period-ms", str(central.telemetry_period_ms),
            "--telemetry-payload-bytes",
            str(central.telemetry_payload_bytes),
            "--telemetry-reliability", central.telemetry_reliability,
            "--command-period-ms", str(central.command_period_ms),
            "--command-payload-bytes", str(central.command_payload_bytes),
            "--rpc-period-ms", str(central.rpc_period_ms),
            "--rpc-timeout-ms", str(central.rpc_timeout_ms),
        ]

    def _spawn_ros2(self) -> dict[str, subprocess.Popen]:
        """Stock-ROS2 telemetry workload under the configured RMW (DDS benchmark
        axis). zenoh is router-based (one rmw_zenohd per netns); fastrtps and
        cyclonedds are routerless and use native multicast discovery. The
        routerless multicast path only works on substrate=bridge (the channel
        forwarder can't carry multicast) — that asymmetry is inherent to the
        architectures (plan Q1), not a bug."""
        rmw = self.scenario.ros2.rmw
        if rmw != "zenoh" and self.scenario.substrate != "bridge":
            raise RuntimeError(
                f"rmw={rmw!r} needs substrate=bridge (native multicast "
                "discovery); only zenoh's explicit mesh works over substrate="
                "channel")
        log_dir = self.out_dir / "ros_logs"
        log_dir.mkdir(parents=True, exist_ok=True)
        if rmw == "zenoh":
            if self.scenario.ros2.zenoh_flavor == "bridge":
                return self._spawn_zenoh_bridge(log_dir)
            if self.scenario.ros2.topology == "centralized":
                return self._spawn_zenoh_centralized(log_dir)
            return self._spawn_zenoh(log_dir)
        buf = self.scenario.ros2.socket_buffer_bytes
        alloc = self.scenario.ros2.fastdds_allocation_participants
        if rmw == "fastrtps":
            if (self.scenario.ros2.topology == "centralized"
                    and self.scenario.ros2.centralized.discovery_mode
                    == "centralized"):
                return self._spawn_fastdds_servers(log_dir)
            if self.scenario.ros2.discovery_server:
                return self._spawn_fastdds_server(log_dir)
            if buf > 0 or alloc > 0:
                # profiles XML route: UDPv4-only transport (SHM off, same as the
                # env route) + optional enlarged buffers + optional allocation
                # preallocation. alloc>0 is the P1 hypothesis lever for the
                # ~33-participant clique cap (docs/dds-topology-plan.md).
                prof = self.out_dir / "fastdds_profiles.xml"
                prof.write_text(_fastdds_profiles_xml(buf, alloc))
                return self._spawn_routerless(
                    "rmw_fastrtps_cpp", log_dir,
                    lambda nid: {"FASTDDS_DEFAULT_PROFILES_FILE": str(prof)})
            # UDPv4 builtin transport drops the default SHM transport, forcing
            # real UDP over the veth; native SPDP multicast handles discovery.
            return self._spawn_routerless(
                "rmw_fastrtps_cpp", log_dir,
                lambda nid: {"FASTDDS_BUILTIN_TRANSPORTS": "UDPv4"})
        if rmw == "cyclonedds":
            if (self.scenario.ros2.topology == "centralized"
                    and self.scenario.ros2.centralized.discovery_mode
                    == "centralized"):
                return self._spawn_cyclone_centralized(log_dir)
            xml = self.out_dir / "cyclonedds.xml"
            xml.write_text(_cyclonedds_xml(
                buf, internal=self.scenario.ros2.cyclonedds_internal))
            return self._spawn_routerless(
                "rmw_cyclonedds_cpp", log_dir,
                lambda nid: {"CYCLONEDDS_URI": f"file://{xml}"})
        raise RuntimeError(f"unknown rmw {rmw!r}")

    def _spawn_cyclone_centralized(
            self, log_dir: Path) -> dict[str, subprocess.Popen]:
        central = self.scenario.ros2.centralized
        assert central is not None
        server_addrs = [self.topo.addrs[s] for s in central.server_ids]
        base = self._ros2_base_env("rmw_cyclonedds_cpp", log_dir)
        agents: dict[str, subprocess.Popen] = {}
        for nid in self.topo.nodes:
            xml = self.out_dir / f"cyclonedds_{nid}.xml"
            xml.write_text(_cyclonedds_xml(
                self.scenario.ros2.socket_buffer_bytes,
                internal=self.scenario.ros2.cyclonedds_internal,
                peers=[a for a in server_addrs if a != self.topo.addrs[nid]],
                allow_multicast=False))
            agents[nid] = subprocess.Popen(
                self._workload_cmd(nid),
                env={**base, "CYCLONEDDS_URI": f"file://{xml}"})
        return agents

    def _spawn_routerless(self, rmw_impl: str, log_dir: Path,
                          extra_env) -> dict[str, subprocess.Popen]:
        """fastrtps / cyclonedds: no router, one workload node per netns, native
        multicast discovery flooded by the bridge (mcast_snooping off)."""
        base = self._ros2_base_env(rmw_impl, log_dir)
        stagger_s = self.scenario.ros2.spawn_stagger_ms / 1e3
        agents: dict[str, subprocess.Popen] = {}
        for i, nid in enumerate(self.topo.nodes):
            # Per-node cluster domain: when partitioning is on, each node must
            # open the DDS domain matching its cluster so SPDP discovery stays
            # within the block and does not cross into adjacent clusters.
            domain_env = ({"ROS_DOMAIN_ID": str(self._domain_of(nid))}
                          if self.scenario.ros2.cluster_domains > 0 else {})
            agents[nid] = subprocess.Popen(
                self._workload_cmd(nid),
                env={**base, **extra_env(nid), **domain_env})
            # stagger participant joins to defuse the simultaneous-startup SPDP
            # storm (tests the Fast DDS discovery-ceiling hypothesis)
            if stagger_s and i < len(self.topo.nodes) - 1:
                time.sleep(stagger_s)
        return agents

    # Fast DDS Discovery Server port (the impl's well-known default). The server
    # binds this on node 0's bridge address; every client unicasts discovery to
    # it instead of flooding SPDP multicast.
    _DS_PORT = 11811

    def _spawn_fastdds_servers(
            self, log_dir: Path) -> dict[str, subprocess.Popen]:
        """One discovery server per application server for centralized runs."""
        central = self.scenario.ros2.centralized
        assert central is not None
        prefix = Path(sys.executable).resolve().parents[1]
        server_bin = prefix / "bin" / "fast-discovery-server"
        if not server_bin.exists():
            raise RuntimeError(
                f"{server_bin} not found: centralized Fast DDS needs the "
                "ros2 pixi environment")
        base = self._ros2_base_env("rmw_fastrtps_cpp", log_dir)
        endpoints: list[str] = []
        for i, nid in enumerate(central.server_ids):
            addr = self.topo.addrs[nid]
            port = self._DS_PORT + i
            endpoints.append(f"{addr}:{port}")
            self._routers[f"ds:{nid}"] = subprocess.Popen(
                ["ip", "netns", "exec", self.topo.ns_names[nid],
                 str(server_bin), "-i", str(i), "-l", addr,
                 "-p", str(port)],
                env=base,
                stdout=(self.out_dir / f"fastdds_ds_{nid}.log").open("w"),
                stderr=subprocess.STDOUT)
        time.sleep(1.0)
        client_env = {
            "FASTDDS_BUILTIN_TRANSPORTS": "UDPv4",
            "ROS_DISCOVERY_SERVER": ";".join(endpoints),
        }
        return {
            nid: subprocess.Popen(
                self._workload_cmd(nid), env={**base, **client_env})
            for nid in self.topo.nodes
        }

    def _spawn_fastdds_server(self, log_dir: Path) -> dict[str, subprocess.Popen]:
        """Fast DDS client-server discovery (tuning experiment, dds-rmw-tuning.md
        #4). One `fast-discovery-server` broker runs in node 0's netns bound to
        its bridge address; every workload node points at it via
        ROS_DISCOVERY_SERVER and joins as a discovery CLIENT. This replaces the
        distributed O(N^2) SPDP/SEDP multicast mesh with an O(N) star — each node
        keeps one discovery link to the server instead of 95 peer participants.

        The server lives in node 0's existing netns (reusing its veth on the
        bridge, no extra namespace) and is tracked in self._routers so the run
        lifecycle meters and tears it down exactly like a zenoh router. SHM is
        still forced off (FASTDDS_BUILTIN_TRANSPORTS=UDPv4) so user data is real
        UDP over the veth, identical to the routerless baseline — the ONLY thing
        that changes between the two fastrtps cells is the discovery mechanism."""
        prefix = Path(sys.executable).resolve().parents[1]
        server_bin = prefix / "bin" / "fast-discovery-server"
        if not server_bin.exists():
            raise RuntimeError(
                f"{server_bin} not found: discovery_server needs the ros2 pixi "
                "env (sudo .pixi/envs/ros2/bin/python ...)")
        nodes = list(self.topo.nodes)
        server_addr = self.topo.addrs[nodes[0]]
        base = self._ros2_base_env("rmw_fastrtps_cpp", log_dir)
        # server id 0 -> first (only) position in ROS_DISCOVERY_SERVER; clients
        # derive the expected server GUID from that position, so they must match.
        self._routers["ds"] = subprocess.Popen(
            ["ip", "netns", "exec", self.topo.ns_names[nodes[0]],
             str(server_bin), "-i", "0", "-l", server_addr,
             "-p", str(self._DS_PORT)],
            env=base,
            stdout=(self.out_dir / "fastdds_discovery_server.log").open("w"),
            stderr=subprocess.STDOUT)
        time.sleep(1.0)  # let the server bind+listen before clients dial in
        client_env = {
            "FASTDDS_BUILTIN_TRANSPORTS": "UDPv4",
            "ROS_DISCOVERY_SERVER": f"{server_addr}:{self._DS_PORT}",
        }
        agents: dict[str, subprocess.Popen] = {}
        for nid in nodes:
            agents[nid] = subprocess.Popen(self._workload_cmd(nid),
                                           env={**base, **client_env})
        return agents

    def _spawn_zenoh(self, log_dir: Path) -> dict[str, subprocess.Popen]:
        """Per netns one rmw_zenohd router + a telemetry node reaching it over
        loopback, wired into an explicit lower-index TCP mesh (one link per
        router pair).

        Q1 asymmetry (documented, not a bug): unlike Fast DDS / Cyclone, which
        are routerless and discover peers via native SPDP/scouting multicast,
        rmw_zenohd routers do NOT autoconnect to peer routers off multicast
        scouting alone (they hear each other on 224.0.0.224 but stay
        unconnected; the autoconnect/whatami override key is rejected by this
        zenoh build). So zenoh uses a pre-wired router mesh on BOTH substrates.
        Consequence: zenoh's discovery-time is effectively connect-time, not
        comparable to the DDS impls' native discovery — the write-up reports
        zenoh discovery separately. Latency / throughput / CPU / mem stay fair.
        On the channel substrate the explicit mesh is also mandatory (the
        forwarder can't carry multicast); on the bridge it is a fairness/
        reliability choice."""
        cfg = self.scenario.ros2
        prefix = Path(sys.executable).resolve().parents[1]
        zenohd = prefix / "lib" / "rmw_zenoh_cpp" / "rmw_zenohd"
        if not zenohd.exists():
            raise RuntimeError(
                f"{zenohd} not found: workload=ros2 must run with the ros2 "
                "pixi env python (sudo .pixi/envs/ros2/bin/python ...)")
        base_env = self._ros2_base_env("rmw_zenoh_cpp", log_dir)
        nodes = list(self.topo.nodes)
        for i, nid in enumerate(nodes):
            listen = (f'listen/endpoints=["tcp/{self.topo.addrs[nid]}:{cfg.port}",'
                      f'"tcp/127.0.0.1:{cfg.port}"]')
            if cfg.zenoh_router_topology == "star":
                # star: every router dials only the first node's router
                # (N-1 links total). The full mesh's N(N-1)/2 links overload
                # zenoh's own control plane at N>=96 (dds-neighbor-table.md);
                # this is the P5 lever that collapses the router graph to O(N).
                connect = ([f'"tcp/{self.topo.addrs[nodes[0]]}:{cfg.port}"']
                           if i > 0 else [])
            else:
                # lower-index mesh: exactly one router-router TCP link per pair
                connect = [f'"tcp/{self.topo.addrs[p]}:{cfg.port}"'
                           for p in nodes[:i]]
            override = listen + (f';connect/endpoints=[{",".join(connect)}]'
                                 if connect else '')
            self._routers[nid] = subprocess.Popen(
                ["ip", "netns", "exec", self.topo.ns_names[nid], str(zenohd)],
                env={**base_env, "ZENOH_CONFIG_OVERRIDE": override},
                stdout=(self.out_dir / f"router_{nid}.log").open("w"),
                stderr=subprocess.STDOUT)
        time.sleep(1.0)  # let routers accept before sessions dial in
        agents: dict[str, subprocess.Popen] = {}
        for nid in nodes:
            node_env = {**base_env, "ZENOH_CONFIG_OVERRIDE":
                        f'connect/endpoints=["tcp/127.0.0.1:{cfg.port}"]'}
            # zenoh routers are domain-agnostic (they mesh at the transport
            # layer regardless of ROS_DOMAIN_ID); only the ROS node env needs
            # the variable so the RMW opens the correct DDS participant domain.
            if self.scenario.ros2.cluster_domains > 0:
                node_env["ROS_DOMAIN_ID"] = str(self._domain_of(nid))
            agents[nid] = subprocess.Popen(self._workload_cmd(nid),
                                           env=node_env)
        return agents

    def _zenoh_bridge_local_dds_env(self) -> tuple[str, dict]:
        """(rmw_impl, env) for the LOCAL DDS side of zenoh_flavor='bridge'.

        The DDS domain is pinned to LOOPBACK. That is not tuning, it is the
        invariant that makes the experiment mean anything: substrate=bridge puts
        every netns on one L2 broadcast domain, so a DDS config that multicasts
        on the veth lets every node's "local" domain discover every other node's
        directly over SPDP. The bridges then route peer telemetry as LOCAL
        publishers and the traffic that is supposed to cross zenoh never does --
        a run that completes, reports healthy delivery, and measures nothing.
        (Observed exactly once, before this was pinned: bridge d1 created
        `Route Publisher (ROS:/swarm/d2/telemetry -> ...)` for all four nodes.)

        Loopback is the right boundary on both substrates because it follows the
        netns: on substrate=bridge each node has its own netns and therefore its
        own lo, giving one participant per domain; on substrate=lan the container
        is --network host, so lo is shared by that host's nodes and the domain is
        exactly one host's worth of participants.

        ROS_AUTOMATIC_DISCOVERY_RANGE is set as well, but it is belt-and-braces
        only -- it does NOT override an explicit CYCLONEDDS_URI, which is how the
        leak above got through. The XML/profile is what actually confines."""
        cfg = self.scenario.ros2
        if cfg.zenoh_bridge_local_dds == "fastrtps":
            # Confinement here rides on ROS_AUTOMATIC_DISCOVERY_RANGE, which
            # rmw_fastrtps DOES honour at the rcl layer -- unlike the Cyclone
            # arm, where an explicit CYCLONEDDS_URI silently overrides it.
            #
            # Deliberately NOT an interfaceWhiteList profile pinned to 127.0.0.1:
            # that confines Fast DDS so hard it stops announcing anywhere the
            # Cyclone-based bridge can hear it (measured: bridge discovered 0
            # participants over a full run). UDPv4-only still drops SHM so the
            # transport matches the routerless baseline.
            return "rmw_fastrtps_cpp", {
                "FASTDDS_BUILTIN_TRANSPORTS": "UDPv4",
                "ROS_AUTOMATIC_DISCOVERY_RANGE": "LOCALHOST",
            }
        # Cyclone: pin to lo, multicast off, single localhost peer. The
        # MaxAutoParticipantIndex=150 this helper emits also matters here -- the
        # bridge binary ships a built-in default of 32, the same per-IP ceiling
        # that killed same-IP participants 33+ in P4.
        xml = self.out_dir / "cyclonedds.xml"
        xml.write_text(_cyclonedds_xml(
            cfg.socket_buffer_bytes, iface_name="lo",
            internal=cfg.cyclonedds_internal,
            peers=["localhost"], allow_multicast=False))
        return "rmw_cyclonedds_cpp", {
            "CYCLONEDDS_URI": f"file://{xml}",
            "ROS_AUTOMATIC_DISCOVERY_RANGE": "LOCALHOST",
        }

    def _spawn_zenoh_bridge(self, log_dir: Path) -> dict[str, subprocess.Popen]:
        """zenoh_flavor='bridge' on substrate=bridge: per netns one
        zenoh-bridge-ros2dds plus a telemetry node that reaches it over the
        netns-local DDS domain, with the bridges wired into the same explicit
        TCP router graph rmw_zenohd uses.

        Router graph parity with _spawn_zenoh is deliberate: zenoh_router_topology
        stays the controlled variable so a bridge-vs-rmw delta is attributable to
        the flavour and not to a different link graph.

        EXPECTED DEGENERACY (state it in the write-up, do not present it as a
        finding): one node per netns means each local DDS domain has exactly one
        application participant, so the bridge has nothing to aggregate and this
        configuration is rmw_zenoh plus a DDS serialise/deserialise hop.  It is
        measured for completeness of the recommendation table; substrate=lan is
        where the flavour is architecturally distinct."""
        cfg = self.scenario.ros2
        binary = _zenoh_bridge_bin()
        nodes = list(self.topo.nodes)
        # The same confined DDS config goes to the nodes AND to the bridge
        # processes: the bridge is a DDS participant too, and leaving it on its
        # built-in default would both re-open multicast and reinstate its baked
        # in MaxAutoParticipantIndex=32.
        rmw_impl, local_env = self._zenoh_bridge_local_dds_env()

        for i, nid in enumerate(nodes):
            listen = (f'tcp/{self.topo.addrs[nid]}:{cfg.port}')
            if cfg.zenoh_router_topology == "star":
                connect = ([f"tcp/{self.topo.addrs[nodes[0]]}:{cfg.port}"]
                           if i > 0 else [])
            else:
                connect = [f"tcp/{self.topo.addrs[p]}:{cfg.port}"
                           for p in nodes[:i]]
            argv = ["ip", "netns", "exec", self.topo.ns_names[nid],
                    str(binary),
                    "-d", str(self._domain_of(nid)),
                    "--ros-automatic-discovery-range", "LOCALHOST",
                    "-l", listen, "-l", f"tcp/127.0.0.1:{cfg.port}"]
            for c in connect:
                argv += ["-e", c]
            bridge_env = {**self._ros2_base_env(rmw_impl, log_dir),
                          **local_env, **_zenoh_bridge_env()}
            # The bridge is a DDS participant in its own right; it must not be
            # told to speak zenoh as its RMW.
            bridge_env.pop("RMW_IMPLEMENTATION", None)
            self._routers[nid] = subprocess.Popen(
                argv, env=bridge_env,
                stdout=(self.out_dir / f"bridge_{nid}.log").open("w"),
                stderr=subprocess.STDOUT)
        # Bridges need longer than rmw_zenohd's 1.0 s: each one starts a DDS
        # participant and completes local SPDP before it can accept sessions.
        time.sleep(3.0)

        agents: dict[str, subprocess.Popen] = {}
        base_env = {**self._ros2_base_env(rmw_impl, log_dir), **local_env}
        for nid in nodes:
            node_env = dict(base_env)
            if cfg.cluster_domains > 0:
                node_env["ROS_DOMAIN_ID"] = str(self._domain_of(nid))
            agents[nid] = subprocess.Popen(
                self._workload_cmd(nid), env=node_env)
        return agents

    def _spawn_zenoh_centralized(
            self, log_dir: Path) -> dict[str, subprocess.Popen]:
        """Run one central router per application server, never per client."""
        cfg = self.scenario.ros2
        central = cfg.centralized
        assert central is not None
        prefix = Path(sys.executable).resolve().parents[1]
        zenohd = prefix / "lib" / "rmw_zenoh_cpp" / "rmw_zenohd"
        if not zenohd.exists():
            raise RuntimeError(
                f"{zenohd} not found: centralized zenoh needs the ros2 pixi env")
        base = self._ros2_base_env("rmw_zenoh_cpp", log_dir)
        endpoints = [
            f"tcp/{self.topo.addrs[nid]}:{cfg.port + i}"
            for i, nid in enumerate(central.server_ids)
        ]
        for i, nid in enumerate(central.server_ids):
            listen = (
                f'listen/endpoints=["{endpoints[i]}",'
                f'"tcp/127.0.0.1:{cfg.port + i}"]')
            connect = (
                f';connect/endpoints=["{endpoints[0]}"]' if i > 0 else "")
            routing = ";routing/router/peers_failover_brokering=true"
            self._routers[f"central:{nid}"] = subprocess.Popen(
                ["ip", "netns", "exec", self.topo.ns_names[nid], str(zenohd)],
                env={**base, "ZENOH_CONFIG_OVERRIDE":
                     listen + connect + routing},
                stdout=(self.out_dir / f"router_{nid}.log").open("w"),
                stderr=subprocess.STDOUT)
        time.sleep(1.0)
        connect = (
            f'mode="client";connect/endpoints=['
            f'{",".join(json.dumps(e) for e in endpoints)}]')
        return {
            nid: subprocess.Popen(
                self._workload_cmd(nid),
                env={**base, "ZENOH_CONFIG_OVERRIDE": connect})
            for nid in self.topo.nodes
        }

    async def run(self) -> None:
        self.out_dir.mkdir(parents=True, exist_ok=True)
        if self.scenario.substrate == "bridge":
            await self._run_bridge()
        elif self.scenario.substrate == "lan":
            await self._run_lan()
        else:
            await self._run_channel()

    async def _run_channel(self) -> None:
        self._build()
        self.topo = NetnsTopology([n.id for n in self.scenario.nodes])
        self.topo.setup()
        log = PacketLog()
        fwd = ChannelForwarder(self.topo, self.scenario.seed, log)
        max_tick_lag = 0.0  # timing integrity (architecture.md)
        late_ticks = 0
        n_ticks = 0
        dt = 1.0 / self.scenario.tick_hz
        agents: dict[str, subprocess.Popen] = {}
        agent_exit: dict[str, int | None] = {}
        self._routers: dict[str, subprocess.Popen] = {}
        track: list[tuple] = []  # per-tick (t, id, x, y, heading) for visualization
        ls_cfg = self.scenario.linkstate_log
        ls_rows: list[dict] = []  # ground-truth link-state log (opt-in)
        bridge = None
        if self.ros2_viz:
            # lazy: rclpy exists only in the ros2 pixi env (ADR-0006)
            from .ros2_bridge import RosVizBridge
            bridge = RosVizBridge(self.terrain)
        try:
            fwd.start()
            # install the initial link-state table BEFORE agents spawn, so their
            # first connection attempts don't die as no_link
            positions0 = {nid: (p.x, p.y)
                          for nid, p in self.mobility.poses().items()}
            fwd.update_links(build_table(
                positions0, self.jammers, 0.0, self.scenario.radio, self.pathloss,
                command_id=self.scenario.command_id,
                satellite=self.scenario.satellite))
            agents = (self._spawn_ros2() if self.scenario.workload == "ros2"
                      else self._spawn_agents()
                      if self.scenario.agent.enabled else {})
            self.ready.set()
            t = 0.0
            wall0 = time.monotonic()
            # keep the channel alive until duration elapsed AND agents exited
            # (a dead agent mid-run is a logged event, not an abort)
            grace = self.scenario.duration_s + 10.0
            while t < self.scenario.duration_s or (
                    t < grace and any(p.poll() is None for p in agents.values())):
                poses = self.mobility.step(dt)
                positions = {nid: (p.x, p.y) for nid, p in poses.items()}
                for nid, p in poses.items():
                    track.append((round(t, 3), nid, round(p.x, 2), round(p.y, 2),
                                  round(p.heading, 4)))
                table = build_table(
                    positions, self.jammers, t, self.scenario.radio, self.pathloss,
                    command_id=self.scenario.command_id,
                    satellite=self.scenario.satellite)
                fwd.update_links(table)
                if ls_cfg.enabled and n_ticks % ls_cfg.every_n_ticks == 0:
                    ls_rows.extend(table_rows(table, round(t, 3)))
                if bridge:
                    bridge.publish_tick(t, poses, table, self.jammers)
                t += dt
                # sleep to the absolute tick deadline, not for a flat dt: per-
                # tick work (mobility step, build_table, forwarder) must not
                # accumulate as wall-clock drift, or the timing monitor reports
                # scheduling slack as overruns. lag now measures genuine
                # inability to keep pace (a slow step or stall) and self-heals
                # once caught up.
                await asyncio.sleep(max(0.0, wall0 + t - time.monotonic()))
                lag = (time.monotonic() - wall0) - t
                n_ticks += 1
                late_ticks += lag > dt
                max_tick_lag = max(max_tick_lag, lag)
            for nid, p in agents.items():
                if p.poll() is None:
                    p.kill()
                agent_exit[nid] = p.wait(timeout=10)
            fwd.stop()
        finally:
            for p in list(agents.values()) + list(self._routers.values()):
                if p.poll() is None:
                    p.kill()
            if bridge:
                bridge.close()
            self.mobility.close()
            self.topo.teardown()
        log.to_parquet(self.out_dir / "packets.parquet")
        if track:
            import pyarrow as pa
            import pyarrow.parquet as pq
            cols = list(zip(*track))
            pq.write_table(pa.table({
                "t": cols[0], "id": cols[1], "x": cols[2], "y": cols[3],
                "heading": cols[4]}), self.out_dir / "positions.parquet")
        if ls_rows:
            import pyarrow as pa
            import pyarrow.parquet as pq
            pq.write_table(pa.table(
                {k: [r[k] for r in ls_rows] for k in ls_rows[0]}),
                self.out_dir / "linkstate.parquet")
        (self.out_dir / "manifest.json").write_text(json.dumps({
            "scenario": self.scenario.model_dump(mode="json"),
            "seed": self.scenario.seed,
            "git_hash": _git_hash(),
            "mobility": self.mobility.name,
            # ADR-0006: Isaac trajectories are PhysX-integrated, not seed-
            # exact; the channel RNG stays seeded/replayable regardless
            "trajectories_seed_exact": self.mobility.seed_exact,
            "max_tick_lag_s": max_tick_lag,
            "late_tick_fraction": late_ticks / max(n_ticks, 1),
            # ok = the loop kept pace statistically: <1% late ticks and no
            # stall longer than 5 tick periods (one OS hiccup is not a bad run)
            "timing_ok": late_ticks / max(n_ticks, 1) < 0.01
                         and max_tick_lag < 5 * dt,
            "agent_exit_codes": agent_exit,
            # delivery_prob in linkstate.parquet is computed at this payload
            **({"linkstate_ref_length_bytes": LOG_REF_LENGTH_BYTES}
               if ls_rows else {}),
        }, indent=2))

    async def _run_bridge(self) -> None:
        """DDS benchmark substrate (plan: substrate=bridge). No RF model, no
        forwarder, no mobility: netns + kernel bridge carry the traffic, each
        RMW uses native discovery. Spawn the workload, then sample per-PID +
        host CPU/mem/swap for duration_s. Host-level sampling is what lets the
        write-up tell 'RMW degraded' from 'rig saturated' at swarm scale (96
        nodes is the deliberate stress point). Metrics otherwise come from the
        workload JSONL; there is no packets.parquet."""
        # netem_args() translates the NetemConfig value object into the token
        # list that netns.py can apply without importing config.py (keeps the
        # dependency edge orchestrator→both, not config←netns).
        self.topo = NetnsTopology(
            [n.id for n in self.scenario.nodes],
            bridge=True,
            netem=(netem_args(**self.scenario.netem.model_dump())
                   if self.scenario.netem else None))
        # Size the ARP neighbor table BEFORE netns are created: namespaces
        # inherit the host's gc_thresh limits at creation time, so the raise
        # must precede topo.setup().  No-op when the current ceiling already
        # covers n*(n-1)*2 entries (never lowers an already-higher host).
        n_nodes = len(self.scenario.nodes)
        neigh_orig = self._apply_neigh_thresholds(n_nodes)
        # Capture the live kernel value right after the optional raise for the
        # manifest; records the true ceiling even when the write was a no-op
        # (host limit already sufficient) or blocked by PermissionError (CI).
        try:
            neigh_gc3_eff = int(
                Path(f"{_NEIGH_BASE}/gc_thresh3").read_text().strip())
        except OSError:
            # /proc unreadable (unusual sandbox): fall back to desired value.
            neigh_gc3_eff = self._neigh_thresholds_for(n_nodes)["gc_thresh3"]
        self.topo.setup()
        self._routers = {}
        agents: dict[str, subprocess.Popen] = {}
        agent_exit: dict[str, int | None] = {}
        fault_events: list[dict] = []
        samples: list[dict] = []
        sample_dt = 1.0  # resource sampling cadence (s)
        # raise host socket-buffer ceilings BEFORE any DDS proc starts, else a
        # SocketReceiveBufferSize request is clamped (and Cyclone refuses to
        # start). 0 -> no-op. Restored in finally.
        kbuf_orig = self._apply_kernel_buffers(
            self.scenario.ros2.socket_buffer_bytes)
        try:
            agents = (self._spawn_ros2() if self.scenario.workload == "ros2"
                      else self._spawn_agents()
                      if self.scenario.agent.enabled else {})
            self.ready.set()
            # track workload nodes + (zenoh) routers; engine runs as root under
            # sudo, so psutil can read these root-owned child PIDs
            tracked = {**agents,
                       **{f"router:{k}": v for k, v in self._routers.items()}}
            meters = {name: psutil.Process(p.pid)
                      for name, p in tracked.items() if p.poll() is None}
            for m in meters.values():
                m.cpu_percent(None)  # prime per-PID cpu deltas
            psutil.cpu_percent(None)  # prime host cpu delta
            wall0 = time.monotonic()
            duration = self.scenario.duration_s
            grace = duration + 10.0  # let agents finish past duration (logged)
            tick = 0
            fault_injected = False
            low_mem_since: float | None = None
            resource_abort_reason: str | None = None
            # keep sampling until duration elapsed AND agents exited (a dead
            # agent mid-run is a logged event, not an abort), capped by grace
            while True:
                elapsed = time.monotonic() - wall0
                central = self.scenario.ros2.centralized
                if (not fault_injected and central is not None
                        and central.failure_at_s is not None
                        and elapsed >= central.failure_at_s):
                    primary = central.primary_server_id
                    proc = agents.get(primary)
                    if proc is not None and proc.poll() is None:
                        proc.kill()
                    for key, infra in self._routers.items():
                        if primary in key and infra.poll() is None:
                            infra.kill()
                    fault_events.append({
                        "type": "primary_failure",
                        "server_id": primary,
                        "elapsed_s": round(elapsed, 3),
                    })
                    fault_injected = True
                alive = any(p.poll() is None for p in agents.values())
                if (elapsed >= duration and not alive) or elapsed >= grace:
                    break
                sample = self._sample_resources(round(elapsed, 3), meters)
                samples.append(sample)
                if central is not None:
                    floor = int(os.environ.get(
                        "NETCOM_CENTRAL_MEM_ABORT_BYTES",
                        str(6 * 1024**3)))
                    if sample["host_mem_avail"] < floor:
                        low_mem_since = low_mem_since or time.monotonic()
                        if time.monotonic() - low_mem_since >= 10:
                            resource_abort_reason = (
                                "available memory remained below "
                                f"{floor} bytes for 10 seconds")
                            break
                    else:
                        low_mem_since = None
                tick += 1
                # deadline-paced like the channel tick loop, so per-sample work
                # doesn't accumulate as drift (orchestrator-tick-pacing)
                await asyncio.sleep(max(0.0, wall0 + tick * sample_dt
                                        - time.monotonic()))
            for nid, p in agents.items():
                if p.poll() is None:
                    p.kill()
                agent_exit[nid] = p.wait(timeout=10)
        finally:
            for p in list(agents.values()) + list(self._routers.values()):
                if p.poll() is None:
                    p.kill()
            self.topo.teardown()
            self._restore_kernel_buffers(kbuf_orig)
            self._restore_neigh_thresholds(neigh_orig)
        self._write_resources(samples)
        # peak host pressure = the "was the rig saturated?" evidence
        peak_mem = max((s["host_mem_used"] for s in samples), default=0)
        min_avail = min((s["host_mem_avail"] for s in samples), default=0)
        peak_swap = max((s["host_swap_used"] for s in samples), default=0)
        (self.out_dir / "manifest.json").write_text(json.dumps({
            "scenario": self.scenario.model_dump(mode="json"),
            "seed": self.scenario.seed,
            "git_hash": _git_hash(),
            "substrate": "bridge",
            "rmw": self.scenario.ros2.rmw,
            "n_nodes": len(self.scenario.nodes),
            # topology determines the pair-count denominator for report.py:
            # mesh N(N-1); star 2(N-1); shared N(N-1) logical but O(N) SEDP endpoints
            "topology": self.scenario.ros2.topology,
            # zenoh router graph shape (mesh = full lower-index mesh); records
            # the P5 lever so a star-router run can't be mistaken for a mesh one
            "zenoh_router_topology": self.scenario.ros2.zenoh_router_topology,
            # Zenoh flavour axis. The full scenario dump above already carries
            # it, but the report groups on top-level manifest keys -- without
            # these a bridge-flavour run is indistinguishable from an
            # rmw_zenoh_cpp one at the grouping level, and the two are different
            # products with different profiles.
            "zenoh_flavor": self.scenario.ros2.zenoh_flavor,
            "zenoh_bridge_local_dds": (
                self.scenario.ros2.zenoh_bridge_local_dds
                if self.scenario.ros2.zenoh_flavor == "bridge" else None),
            "zenoh_bridge_version": _zenoh_bridge_version(self.scenario.ros2.zenoh_flavor),
            "cyclonedds_internal": self.scenario.ros2.cyclonedds_internal,
            "resource_samples": len(samples),
            "peak_host_mem_used_bytes": peak_mem,
            "min_host_mem_avail_bytes": min_avail,
            "peak_host_swap_used_bytes": peak_swap,
            "socket_buffer_bytes": self.scenario.ros2.socket_buffer_bytes,
            "fastdds_allocation_participants": self.scenario.ros2.fastdds_allocation_participants,
            # netem knobs active in this run; None = ideal link (no qdisc)
            "netem": (self.scenario.netem.model_dump()
                      if self.scenario.netem else None),
            # effective ARP neighbor-table ceiling after _apply_neigh_thresholds;
            # the root-cause artifact (swarm pairs silently capped at ~1022 when
            # N>=48) is absent when this value >= n*(n-1)*2.
            "neigh_gc_thresh3": neigh_gc3_eff,
            # cluster_of is non-empty only when cluster_domains>0, so existing
            # report.py scripts reading v0 manifests see an empty dict and are
            # unaffected; new scripts can split intra- vs cross-cluster pairs.
            "cluster_domains": self.scenario.ros2.cluster_domains,
            "cluster_of": (
                {n.id: self._domain_of(n.id) for n in self.scenario.nodes}
                if self.scenario.ros2.cluster_domains > 0 else {}
            ),
            "agent_exit_codes": agent_exit,
            "fault_events": fault_events,
            "resource_abort_reason": resource_abort_reason,
        }, indent=2))

    @staticmethod
    def _apply_kernel_buffers(nbytes: int) -> dict[str, str]:
        """Raise net.core.{r,w}mem_max/default to >= nbytes and netdev backlog,
        returning the original values for restore. No-op (returns {}) when
        nbytes<=0. Root writes /proc/sys directly (the orchestrator runs under
        sudo; the `sysctl` binary is not on the NOPASSWD list)."""
        if nbytes <= 0:
            return {}
        orig: dict[str, str] = {}
        try:
            for key in _KERNEL_BUF_SYSCTLS:
                path = f"/proc/sys/net/core/{key}"
                orig[path] = Path(path).read_text().strip()
                # only raise, never lower a generously-configured host
                if int(orig[path]) < nbytes:
                    Path(path).write_text(str(nbytes))
            blog = "/proc/sys/net/core/netdev_max_backlog"
            orig[blog] = Path(blog).read_text().strip()
            if int(orig[blog]) < 5000:
                Path(blog).write_text("5000")
        except (PermissionError, OSError):
            # not root / locked-down: leave whatever we managed to set, restore
            # only those. The DDS layer will surface a clamp if it matters.
            pass
        return orig

    @staticmethod
    def _restore_kernel_buffers(orig: dict[str, str]) -> None:
        for path, val in orig.items():
            try:
                Path(path).write_text(val)
            except (PermissionError, OSError):
                pass

    @staticmethod
    def _neigh_thresholds_for(n_nodes: int) -> dict[str, int]:
        """Pure computation: desired gc_thresh{1,2,3} for an n_nodes bridge
        swarm, returned as {filename: value}.

        needed = n*(n-1)*2 — the 2× factor gives slack for transient duplicate
        ARP entries produced by background discovery/router processes during
        the initial multicast burst.  gc_thresh2 = needed//2 (soft GC trigger),
        gc_thresh1 = needed//4 (free-slot floor below which GC always runs).

        LAN substrate is exempt: each physical host's neighbor table only holds
        its own N-1 peers; the cross-namespace accumulation artifact that fills
        the global table cannot arise there, so _run_lan does not call this."""
        needed = n_nodes * (n_nodes - 1) * 2
        return {
            "gc_thresh3": needed,
            "gc_thresh2": needed // 2,
            "gc_thresh1": needed // 4,
        }

    @staticmethod
    def _apply_neigh_thresholds(n_nodes: int) -> dict[str, str]:
        """Raise net.ipv4.neigh.default.gc_thresh{1,2,3} so the ARP table can
        hold all N*(N-1)*2 bridge-swarm entries without silent eviction.

        ROOT CAUSE (2026-07-03): the Linux ARP neighbor table's GC limits
        (defaults 128/512/1024) are accounted across ALL network namespaces.
        A bridge-substrate run with N nodes needs ~N*(N-1) entries host-wide
        (every node resolves every peer); at N>=48 the 1024 ceiling binds and
        silently caps connected pairs at ~1022 — this artifact masqueraded as
        "the Fast DDS ~33-participant clique cap" and "Cyclone's retransmit
        storm" through the entire benchmark track.  Empirical proof: raising to
        16384 took fastrtps N=96 from mesh 0.112 → 0.9945 and cyclonedds from
        0.057 → 1.000 (delivery 0.965).  This fix makes the rig correct BY
        DEFAULT for every bridge run.

        Modeled on _apply_kernel_buffers: no-op (returns {}) if the current
        gc_thresh3 already covers the swarm — never lower an already-higher
        host.  Root writes /proc/sys directly (sysctl not on NOPASSWD list).
        Returns original values keyed by /proc path for restore.

        Note: _run_lan does NOT call this.  On real NICs each host resolves
        only N-1 peers (its own broadcast domain); the cross-namespace global
        accumulation cannot occur there."""
        desired = ScenarioEngine._neigh_thresholds_for(n_nodes)
        thresh3_path = f"{_NEIGH_BASE}/gc_thresh3"
        orig: dict[str, str] = {}
        try:
            current3 = int(Path(thresh3_path).read_text().strip())
            if desired["gc_thresh3"] <= current3:
                return {}  # current ceiling sufficient; never lower
            # Save originals then raise all three in ascending order so the
            # kernel invariant thresh1 <= thresh2 <= thresh3 is never violated
            # even momentarily (thresh3 last → always the new maximum).
            for key in ("gc_thresh1", "gc_thresh2", "gc_thresh3"):
                path = f"{_NEIGH_BASE}/{key}"
                orig[path] = Path(path).read_text().strip()
            for key in ("gc_thresh1", "gc_thresh2", "gc_thresh3"):
                Path(f"{_NEIGH_BASE}/{key}").write_text(str(desired[key]))
        except (PermissionError, OSError):
            # not root / locked-down: leave whatever we managed to set, restore
            # only those.  The neigh eviction will surface at swarm scale if
            # it matters (same pattern as _apply_kernel_buffers).
            pass
        return orig

    @staticmethod
    def _restore_neigh_thresholds(orig: dict[str, str]) -> None:
        for path, val in orig.items():
            try:
                Path(path).write_text(val)
            except (PermissionError, OSError):
                pass

    def _sample_resources(self, t: float, meters: dict) -> dict:
        """One resource sample: per-PID cpu%/RSS plus host cpu/mem/swap."""
        procs: list[tuple] = []
        for name, m in meters.items():
            try:
                with m.oneshot():
                    procs.append((name, m.cpu_percent(None), m.memory_info().rss))
            except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
                procs.append((name, float("nan"), 0))  # exited mid-run
        vm = psutil.virtual_memory()
        return {
            "t": t,
            "host_cpu_pct": psutil.cpu_percent(None),
            "host_mem_used": vm.used,
            "host_mem_avail": vm.available,
            "host_swap_used": psutil.swap_memory().used,
            "procs": procs,
        }

    def _write_resources(self, samples: list[dict]) -> None:
        if not samples:
            return
        import pyarrow as pa
        import pyarrow.parquet as pq
        cols: dict[str, list] = {k: [] for k in (
            "t", "proc", "cpu_pct", "rss_bytes", "host_cpu_pct",
            "host_mem_used", "host_mem_avail", "host_swap_used")}
        for s in samples:
            for name, cpu, rss in s["procs"]:
                cols["t"].append(s["t"])
                cols["proc"].append(name)
                cols["cpu_pct"].append(cpu)
                cols["rss_bytes"].append(rss)
                cols["host_cpu_pct"].append(s["host_cpu_pct"])
                cols["host_mem_used"].append(s["host_mem_used"])
                cols["host_mem_avail"].append(s["host_mem_avail"])
                cols["host_swap_used"].append(s["host_swap_used"])
        pq.write_table(pa.table(cols), self.out_dir / "resources.parquet")

    # ------------------------------------------------------------------
    # LAN substrate helpers
    # ------------------------------------------------------------------

    @staticmethod
    def _ping_rtt(addr: str) -> float | None:
        """Probe one-way RTT to addr using `ping -c 3 -q`; returns the average
        ms from the summary line or None on any failure (host unreachable,
        tool not installed, etc.).  Informational only — written to the manifest
        so the report can flag runs where Wi-Fi was particularly jittery."""
        try:
            out = subprocess.run(
                ["ping", "-c", "3", "-q", addr],
                capture_output=True, text=True, timeout=15)
            # Linux ping summary: "rtt min/avg/max/mdev = 0.1/0.2/0.3/0.05 ms"
            for line in out.stdout.splitlines():
                if "rtt" in line and "/" in line:
                    # split on "=" then on "/" to reach the avg field (index 1)
                    return float(line.split("=")[1].strip().split("/")[1])
        except Exception:
            pass
        return None

    def _lan_node_cmd(self, nid: str, host_name: str,
                      extra_env: dict | None = None) -> list[str]:
        """Pure command-list builder for a single workload node in a lan run.

        Local host (ssh=""): returns [sys.executable, "-m", workload, ...args].
        env= for local processes is set by the caller via Popen(env=...) and is
        NOT embedded in the argv — extra_env is ignored for local nodes.

        Remote host: returns _ssh_cmd(target, docker_exec_argv) — a 5-element
        list ["ssh", "-o", "BatchMode=yes", target, <one shlex-joined string>].
        The shlex.join step is critical: ssh passes everything after the target
        to the remote login shell as a SINGLE string; without it the shell
        re-parses the tokens and:
          - Strips double-quotes from ZENOH_CONFIG_OVERRIDE endpoint lists
            (e.g. ["tcp/127.0.0.1:7447"] → [tcp/127.0.0.1:7447]), causing
            zenoh to reject the config with Json5Err.
          - Treats `sh -c 'cat > /path'` as `sh -c cat` with `> /path` as a
            redirect on the REMOTE HOST (not inside the container), so XML
            distribution fails with exit 1 because /work/ doesn't exist there.
        With shlex.join the remote shell sees one properly-quoted string and
        splits it back into exactly the original docker-exec argv.

        extra_env: environment variables embedded as `-e KEY=VAL` flags in the
        docker exec argv for remote nodes.  Caller constructs the complete env
        dict (base vars + RMW-specific extras) and passes it here.

        Unit-testable: no subprocess, no filesystem, no network I/O.
        """
        cfg = self.scenario.ros2
        h = self.scenario.hosts[host_name]
        all_ids = [n.id for n in self.scenario.nodes]
        # For shared/star topologies --peers is unused (O(N) endpoint wiring
        # has no per-peer subscriptions); pass empty string so node.py fails
        # loudly if it accidentally enters mesh mode with an empty peer list.
        peers = (",".join(p for p in all_ids if p != nid)
                 if cfg.topology == "mesh" else "")
        role, assigned_server, activate_after = self._workload_role(nid)

        # Workload arguments are identical across local and remote; only the
        # interpreter prefix, --metrics path, and env-injection method differ.
        workload_args = [
            "-m", "netcom_zen.ros2_workload",
            "--id", nid,
            "--peers", peers,
            "--topology", cfg.topology,
            "--role", role,
            *self._centralized_workload_args(
                assigned_server, activate_after),
            "--period-ms", str(cfg.period_ms),
            "--payload-bytes", str(cfg.payload_bytes),
            "--reliability", cfg.reliability,
            "--durability", cfg.durability,
            "--history", cfg.history,
            "--depth", str(cfg.depth),
            "--deadline-ms", str(cfg.deadline_ms),
            "--lifespan-ms", str(cfg.lifespan_ms),
            "--liveliness", cfg.liveliness,
            "--liveliness-lease-ms", str(cfg.liveliness_lease_ms),
            "--duration-s", str(self.scenario.duration_s),
        ]

        if h.ssh == "":
            # Local: interpreter runs directly; env= set by caller via Popen.
            metrics = str(self.out_dir / f"agent_{nid}.jsonl")
            return [sys.executable, *workload_args, "--metrics", metrics]

        # Remote: build docker exec argv with env vars embedded as -e flags,
        # then wrap in _ssh_cmd so the whole thing arrives at the remote shell
        # as one shlex-quoted string (see class docstring for the bug context).
        run_name = self.out_dir.name
        remote_metrics = f"{h.workdir}/results/lan/{run_name}/agent_{nid}.jsonl"
        e_flags = [
            flag
            for k, v in (extra_env or {}).items()
            for flag in ("-e", f"{k}={v}")
        ]
        remote_argv = [
            "docker", "exec",
            *e_flags,
            h.container,
            ".pixi/envs/ros2/bin/python",
            *workload_args,
            "--metrics", remote_metrics,
        ]
        return _ssh_cmd(h.ssh, remote_argv)

    def _lan_launcher_script(self, host_name: str,
                              node_envs: dict[str, dict]) -> str:
        """Generate a POSIX sh script that starts all nodes for host_name in
        one remote shell session, eliminating the per-node ssh handshake that
        trips sshd's MaxStartups limit (default 10:30:100) at 24+ concurrent
        remote nodes — each extra connection above the threshold is randomly
        delayed or rejected, silently leaving those nodes absent from the run.

        Each node gets a backgrounded subshell that:
          - exports its env with shlex-quoted values: shlex.quote wraps each
            value in single-quotes and escapes embedded single-quotes as '\\''
            — the only correct way to handle arbitrary shell values, including
            ZENOH_CONFIG_OVERRIDE which contains double-quoted endpoint lists
            like ["tcp/127.0.0.1:7447"] that an unquoted assignment mangles;
          - runs the workload (same args as _lan_node_cmd's remote path),
            redirecting stdout/stderr to a per-node log file;
          - appends "nid <exit_code>" to exits_<host_name>.txt after the
            workload exits, for collection by _run_lan step 6b.

        spawn_stagger_ms inserts sleep lines between background launches so
        participant joins are paced within this host (per-host stagger).
        Semantics differ from the old global-interleaved stagger on the bridge
        substrate: bridge: global order across all hosts; lan: per-host order
        within each launcher — cross-host ordering is determined by network
        latency and is not guaranteed.

        Returns the complete script text.
        Pure Python: no subprocess, no filesystem, no network I/O.
        Unit-testable without root or Docker.
        """
        cfg = self.scenario.ros2
        h = self.scenario.hosts[host_name]
        run_name = self.out_dir.name
        run_dir = f"{h.workdir}/results/lan/{run_name}"
        # Host-specific exits file avoids collisions when multiple remote hosts
        # rsync their results into the same local out_dir.
        exits_file = f"{run_dir}/exits_{host_name}.txt"

        # Nodes assigned to this host, in scenario order (spawn/stagger order).
        host_nodes = [n.id for n in self.scenario.nodes
                      if n.host == host_name]
        stagger_s = cfg.spawn_stagger_ms / 1e3

        lines: list[str] = [
            "#!/bin/sh",
            f"# Launcher for host {host_name!r}, run {run_name!r}.",
            "# Generated by ScenarioEngine._lan_launcher_script; do not edit.",
            "# Env values are single-quoted via shlex.quote so that values",
            "# containing double-quotes or semicolons (e.g. ZENOH_CONFIG_OVERRIDE)",
            "# survive the remote shell without mangling.",
        ]

        for i, nid in enumerate(host_nodes):
            env = node_envs.get(nid, {})
            # KEY=<shlex.quote(value)> pairs; shlex.quote('foo["bar"]') →
            # 'foo["bar"]' which the shell re-parses correctly as foo["bar"].
            export_pairs = " ".join(
                f"{k}={shlex.quote(str(v))}" for k, v in env.items()
            )

            # Workload args mirror _lan_node_cmd's remote argv (same CLI).
            all_ids = [n.id for n in self.scenario.nodes]
            peers = (",".join(p for p in all_ids if p != nid)
                     if cfg.topology == "mesh" else "")
            role, assigned_server, activate_after = self._workload_role(nid)
            remote_metrics = f"{run_dir}/agent_{nid}.jsonl"
            node_log = f"{run_dir}/node_{nid}.log"

            python_cmd = shlex.join([
                ".pixi/envs/ros2/bin/python", "-m", "netcom_zen.ros2_workload",
                "--id", nid,
                "--peers", peers,
                "--topology", cfg.topology,
                "--role", role,
                *self._centralized_workload_args(
                    assigned_server, activate_after),
                "--period-ms", str(cfg.period_ms),
                "--payload-bytes", str(cfg.payload_bytes),
                "--reliability", cfg.reliability,
                "--durability", cfg.durability,
                "--history", cfg.history,
                "--depth", str(cfg.depth),
                "--deadline-ms", str(cfg.deadline_ms),
                "--lifespan-ms", str(cfg.lifespan_ms),
                "--liveliness", cfg.liveliness,
                "--liveliness-lease-ms", str(cfg.liveliness_lease_ms),
                "--duration-s", str(self.scenario.duration_s),
                "--metrics", remote_metrics,
            ])

            # The subshell captures the workload's own exit code via $? and
            # always appends "nid code" to the exits file even on failure.
            # shlex.quote(nid) single-quotes the node id so node ids with
            # special characters (unlikely but defensive) survive the echo.
            lines.append(
                f"( export {export_pairs}; "
                f"{python_cmd} > {node_log} 2>&1; "
                f"echo {shlex.quote(nid)} $? >> {exits_file} ) &"
            )

            # Per-host stagger: sleep only between node launches, not after
            # the last one — two nodes → one sleep, three → two, etc.
            if stagger_s and i < len(host_nodes) - 1:
                lines.append(f"sleep {stagger_s}")

        # Wait for all background subshells.  The launcher ssh Popen in
        # _run_lan exits here; its returncode goes to remote_launcher_exit.
        lines.append("wait")
        return "\n".join(lines) + "\n"

    async def _run_lan(self) -> None:
        """Real-NIC multi-host substrate (substrate=lan).

        No netns, no veth, no kernel bridge, no root required.  Local nodes are
        plain subprocesses on this machine's real NIC; remote nodes run inside
        the long-lived dds-lab container over SSH.

        Structure mirrors _run_bridge: same deadline-paced sampling loop, same
        manifest keys, same _write_resources call.  Key differences:
          - No NetnsTopology, no _apply_kernel_buffers.
          - psutil meters LOCAL processes only (ssh Popen objects don't expose
            the remote PID tree).
          - Interface pinning is mandatory: every XML config includes either an
            explicit NIC name (Cyclone) or an interfaceWhiteList IP (Fast DDS)
            to avoid DDS announcing locators on noise interfaces.
          - zenoh uses ONE router per HOST (host-level lower-index TCP mesh),
            not one per node — cross-host router wiring happens at this level.
          - Remote results are rsync'd back after the run.
        """
        cfg = self.scenario.ros2
        rmw = cfg.rmw
        # zenoh_flavor='bridge' keeps a native DDS domain on each host and puts
        # zenoh only on the cross-link, so on this substrate the ROS nodes' RMW
        # is a DDS impl and the zenoh process is a bridge rather than a router.
        # substrate=lan is where the flavour is architecturally distinct: the
        # container runs --network host, so a host's nodes share one netns and
        # the local DDS domain actually has something to aggregate.
        zbridge = (rmw == "zenoh" and cfg.zenoh_flavor == "bridge")
        # What the workload nodes actually speak. Drives the per-host XML
        # config, RMW_IMPLEMENTATION, and the AMENT prefix derivation below.
        local_dds = cfg.zenoh_bridge_local_dds if zbridge else rmw
        central = (cfg.centralized
                   if cfg.topology == "centralized" else None)
        hosts = self.scenario.hosts          # dict[str, LanHostConfig]
        all_nodes = self.scenario.nodes
        node_ids = [n.id for n in all_nodes]
        host_of: dict[str, str] = {n.id: n.host for n in all_nodes}

        out_dir = self.out_dir
        out_dir.mkdir(parents=True, exist_ok=True)
        log_dir = out_dir / "ros_logs"
        log_dir.mkdir(parents=True, exist_ok=True)
        # out_dir.name is the run identifier used in remote metrics paths and rsync.
        run_name = out_dir.name

        remote_hosts = {name: h for name, h in hosts.items() if h.ssh}

        self._routers: dict[str, subprocess.Popen] = {}
        agents: dict[str, subprocess.Popen] = {}
        agent_exit: dict[str, int | None] = {}
        fault_events: list[dict] = []
        samples: list[dict] = []
        sample_dt = 1.0  # resource sampling cadence (s) — same as bridge

        # Probe RTTs before spawning so any latency is captured before the run
        # perturbs the link.  Wi-Fi RTTs are informational only (cross-host
        # clocks are not synced; per-node latency is meaningless cross-host).
        rtt_ms: dict[str, float | None] = {}
        for name, h in remote_hosts.items():
            rtt_ms[name] = self._ping_rtt(h.addr)

        # Container path for the ros2 pixi env (fixed by the deploy script).
        remote_ament = "/work/.pixi/envs/ros2"
        # RMW impl string used for AMENT_PREFIX_PATH derivation on local nodes
        # and RMW_IMPLEMENTATION env var for remote nodes.
        rmw_impl = {
            "zenoh": "rmw_zenoh_cpp",
            "fastrtps": "rmw_fastrtps_cpp",
            "cyclonedds": "rmw_cyclonedds_cpp",
        }[local_dds]
        local_base = self._ros2_base_env(rmw_impl, log_dir)

        # xml_env[host_name] = {ENV_KEY: "value"} that activates the per-host
        # pinned config on both local (Popen env=) and remote (docker exec -e).
        xml_env: dict[str, dict] = {}
        # One Popen per remote host: runs the generated launcher script over a
        # single ssh connection.  Declared outside try so step 6b can read
        # returncode even when the try block raised after the launcher started.
        launcher_procs: dict[str, subprocess.Popen] = {}
        # Populated in step 6b from the per-host exits_<host>.txt files;
        # values are the launcher sh process's exit codes (not per-node codes).
        remote_launcher_exit: dict[str, int | None] = {}

        try:
            # --- 1. Create remote result directories -------------------------
            # Must exist before any node tries to write its metrics JSONL.
            for name, h in remote_hosts.items():
                remote_res = f"{h.workdir}/results/lan/{run_name}"
                # rm -rf before mkdir -p: a re-run with the same run name
                # would otherwise rsync back stale agent JSONLs from the
                # previous attempt, corrupting mesh metrics with phantom data.
                subprocess.run(
                    _ssh_cmd(h.ssh,
                             ["docker", "exec", h.container,
                              "sh", "-c",
                              f"rm -rf {remote_res} && mkdir -p {remote_res}"]),
                    check=True)

            # --- 2. Generate and distribute per-host XML configs -------------
            # Interface pinning is done per-host (each host has one NIC / one
            # LAN IP).  Local files land in out_dir; remote files are streamed
            # into the container via stdin to avoid a separate scp step (the
            # bind mount makes them immediately visible to the container).
            if local_dds == "fastrtps":
                for name, h in hosts.items():
                    # Under zenoh_flavor='bridge' the local DDS domain must
                    # not leave the host: both hosts sit on one Wi-Fi subnet, so
                    # a LAN-IP whitelist would let the two "local" domains
                    # discover each other directly over SPDP and carry traffic
                    # that is supposed to cross zenoh.  Loopback-only is the
                    # same confinement ROS_LOCALHOST_ONLY applies.
                    xml_content = _fastdds_profiles_xml(
                        0, 0,
                        whitelist_addr="127.0.0.1" if zbridge else h.addr)
                    if h.ssh == "":
                        xml_path = out_dir / f"fastdds_profiles_{name}.xml"
                        xml_path.write_text(xml_content)
                        xml_env[name] = {
                            "FASTDDS_DEFAULT_PROFILES_FILE": str(xml_path)}
                    else:
                        remote_xml = (
                            f"{h.workdir}/results/lan/{run_name}/"
                            f"fastdds_profiles_{name}.xml")
                        # Pipe XML into the container via stdin.  `cat > path`
                        # is the sh -c argument; shlex-quoting inside _ssh_cmd
                        # keeps the redirect INSIDE the container shell, not on
                        # the remote host (the pre-fix bug: remote shell parsed
                        # `sh -c cat > /path` and ran the redirect locally).
                        subprocess.run(
                            _ssh_cmd(h.ssh,
                                     ["docker", "exec", "-i", h.container,
                                      "sh", "-c", f"cat > {remote_xml}"]),
                            input=xml_content.encode(), check=True)
                        xml_env[name] = {
                            "FASTDDS_DEFAULT_PROFILES_FILE": remote_xml}

            elif local_dds == "cyclonedds":
                central_peer_addrs = (
                    [hosts[host_of[s]].addr for s in central.server_ids]
                    if central is not None
                    and central.discovery_mode == "centralized" else None)
                for name, h in hosts.items():
                    # iface_name pins Cyclone to the correct NIC; without it,
                    # Cyclone announces locators on every interface (including
                    # zerotier and tailscale) which breaks cross-host discovery.
                    # zenoh_flavor='bridge': pin to lo with multicast off and
                    # a localhost peer, so the DDS domain is bounded by the host
                    # and every cross-host sample must traverse the bridge.
                    # Otherwise: pin to the real NIC and let SPDP multicast do
                    # native cross-host discovery (the routerless baseline).
                    xml_content = _cyclonedds_xml(
                        0,
                        iface_name="lo" if zbridge else h.iface,
                        internal=self.scenario.ros2.cyclonedds_internal,
                        peers=["localhost"] if zbridge else central_peer_addrs,
                        allow_multicast=(
                            False if zbridge
                            else central_peer_addrs is None))
                    if h.ssh == "":
                        xml_path = out_dir / f"cyclonedds_{name}.xml"
                        xml_path.write_text(xml_content)
                        xml_env[name] = {
                            "CYCLONEDDS_URI": f"file://{xml_path}"}
                    else:
                        remote_xml = (
                            f"{h.workdir}/results/lan/{run_name}/"
                            f"cyclonedds_{name}.xml")
                        subprocess.run(
                            _ssh_cmd(h.ssh,
                                     ["docker", "exec", "-i", h.container,
                                      "sh", "-c", f"cat > {remote_xml}"]),
                            input=xml_content.encode(), check=True)
                        xml_env[name] = {
                            "CYCLONEDDS_URI": f"file://{remote_xml}"}

            # Centralized Fast DDS uses one discovery server colocated with
            # each application server.  Native mode keeps the historical
            # multicast path.
            if (rmw == "fastrtps" and central is not None
                    and central.discovery_mode == "centralized"):
                prefix = Path(sys.executable).resolve().parents[1]
                ds_local = prefix / "bin" / "fast-discovery-server"
                ds_remote = f"{remote_ament}/bin/fast-discovery-server"
                ds_endpoints = []
                for i, sid in enumerate(central.server_ids):
                    host_name = host_of[sid]
                    h = hosts[host_name]
                    port = self._DS_PORT + i
                    ds_endpoints.append(f"{h.addr}:{port}")
                    log = (out_dir / f"fastdds_ds_{sid}.log").open("w")
                    argv = [
                        str(ds_local), "-i", str(i), "-l", h.addr,
                        "-p", str(port)]
                    if h.ssh:
                        argv = _ssh_cmd(h.ssh, [
                            "docker", "exec",
                            "-e", f"AMENT_PREFIX_PATH={remote_ament}",
                            h.container, ds_remote, "-i", str(i),
                            "-l", h.addr, "-p", str(port)])
                    self._routers[f"ds:{sid}"] = subprocess.Popen(
                        argv, env=(local_base if not h.ssh else None),
                        stdout=log, stderr=subprocess.STDOUT)
                ds_env = {
                    "FASTDDS_BUILTIN_TRANSPORTS": "UDPv4",
                    "ROS_DISCOVERY_SERVER": ";".join(ds_endpoints),
                }
                for name in hosts:
                    xml_env[name] = {**xml_env.get(name, {}), **ds_env}
                time.sleep(1.0)

            # --- 3. Spawn zenoh routers --------------------------------------
            # The host-level lower-index mesh mirrors _spawn_zenoh's node-level
            # mesh: host i connects to all hosts[0..i-1].  Every workload node
            # then connects to tcp/127.0.0.1:<port> on its own host's router
            # (the container uses --network host so loopback is shared).
            if zbridge:
                # One zenoh-bridge-ros2dds per HOST, wired in the same
                # lower-index host mesh the rmw_zenohd path uses so the link
                # graph stays the controlled variable across flavours.
                # /work is the container's bind-mount of the repo, so the
                # vendored binary is the same bytes on every host.
                bridge_local = _zenoh_bridge_bin()
                bridge_remote = "/work/vendor/zenoh-bridge-ros2dds/zenoh-bridge-ros2dds"
                host_names = list(hosts)
                for i, name in enumerate(host_names):
                    h = hosts[name]
                    argv = [
                        "-d", "0",
                        "--ros-automatic-discovery-range", "LOCALHOST",
                        "-l", f"tcp/{h.addr}:{cfg.port}",
                        "-l", f"tcp/127.0.0.1:{cfg.port}",
                    ]
                    for j in range(i):
                        argv += ["-e",
                                 f"tcp/{hosts[host_names[j]].addr}:{cfg.port}"]
                    blog = (out_dir / f"bridge_{name}.log").open("w")
                    if h.ssh == "":
                        benv = {**local_base, **xml_env.get(name, {}),
                                **_zenoh_bridge_env()}
                        # The bridge is a DDS participant, not a ROS node; an
                        # RMW_IMPLEMENTATION would be meaningless to it.
                        benv.pop("RMW_IMPLEMENTATION", None)
                        self._routers[name] = subprocess.Popen(
                            [str(bridge_local)] + argv, env=benv,
                            stdout=blog, stderr=subprocess.STDOUT)
                    else:
                        eflags = []
                        for k, v in xml_env.get(name, {}).items():
                            eflags += ["-e", f"{k}={v}"]
                        self._routers[name] = subprocess.Popen(
                            _ssh_cmd(h.ssh, [
                                "docker", "exec",
                                *eflags,
                                "-e", "PYTHONNOUSERSITE=1",
                                h.container, bridge_remote, *argv]),
                            stdout=blog, stderr=subprocess.STDOUT)
                # Bridges each stand up a DDS participant and settle local SPDP
                # before accepting sessions -- slower than rmw_zenohd's 1.0 s.
                time.sleep(3.0)
                zenoh_central_connect = ""
            elif rmw == "zenoh":
                if central is not None:
                    router_specs = [
                        (f"central:{sid}", host_of[sid],
                         cfg.port + i, sid)
                        for i, sid in enumerate(central.server_ids)]
                else:
                    router_specs = [
                        (name, name, cfg.port, "")
                        for name in hosts]
                prefix = Path(sys.executable).resolve().parents[1]
                zenohd_local = (
                    prefix / "lib" / "rmw_zenoh_cpp" / "rmw_zenohd")
                if not zenohd_local.exists():
                    raise RuntimeError(
                        f"{zenohd_local} not found: substrate=lan with "
                        "rmw=zenoh needs the ros2 pixi env")
                zenohd_remote = (
                    f"{remote_ament}/lib/rmw_zenoh_cpp/rmw_zenohd")
                endpoint_by_key = {
                    key: f"tcp/{hosts[host_name].addr}:{port}"
                    for key, host_name, port, _ in router_specs}
                for i, (key, host_name, port, sid) in enumerate(router_specs):
                    h = hosts[host_name]
                    listen = (
                        f'listen/endpoints=['
                        f'"tcp/{h.addr}:{port}",'
                        f'"tcp/127.0.0.1:{port}"]')
                    lower = [f'"{endpoint_by_key[router_specs[j][0]]}"'
                             for j in range(i)]
                    override = listen + (
                        f';connect/endpoints=[{",".join(lower)}]'
                        if lower else '')
                    if central is not None:
                        override += (
                            ";routing/router/"
                            "peers_failover_brokering=true")
                    safe_key = key.replace(":", "_")
                    router_log = (out_dir / f"router_{safe_key}.log").open("w")
                    if h.ssh == "":
                        self._routers[key] = subprocess.Popen(
                            [str(zenohd_local)],
                            env={**local_base,
                                 "ZENOH_CONFIG_OVERRIDE": override},
                            stdout=router_log, stderr=subprocess.STDOUT)
                    else:
                        # Remote router: env travels as docker exec -e flags.
                        # override contains quoted endpoint lists; _ssh_cmd's
                        # shlex.join keeps them intact through the remote shell.
                        self._routers[key] = subprocess.Popen(
                            _ssh_cmd(h.ssh, [
                                "docker", "exec",
                                "-e", f"ZENOH_CONFIG_OVERRIDE={override}",
                                "-e", f"AMENT_PREFIX_PATH={remote_ament}",
                                "-e", "PYTHONNOUSERSITE=1",
                                h.container, zenohd_remote,
                            ]),
                            stdout=router_log, stderr=subprocess.STDOUT)
                # Give all routers time to bind and accept before sessions
                # connect (same 1-second grace as _spawn_zenoh).
                time.sleep(1.0)
                zenoh_central_connect = (
                    f'mode="client";connect/endpoints=['
                    f'{",".join(json.dumps(v) for v in endpoint_by_key.values())}]'
                    if central is not None else "")

            # --- 4. Spawn workload nodes ----------------------------------------
            # Local nodes: one Popen per node (unchanged).
            # Remote nodes: ONE ssh Popen per HOST that runs a launcher script
            # containing all that host's nodes, avoiding sshd MaxStartups
            # throttling (default 10:30:100 drops connections 11+ in a burst,
            # silently leaving those nodes absent from the run at N≥24 remote).
            #
            # spawn_stagger_ms semantics (bridge: global order; lan: per-host):
            # on the bridge substrate the orchestrator sleeps between every
            # spawn regardless of host; here local nodes are staggered among
            # themselves and remote nodes are staggered within each launcher
            # script — cross-host ordering is not guaranteed.
            stagger_s = cfg.spawn_stagger_ms / 1e3

            # 4a. Local nodes — direct Popen per node, same as before.
            local_node_ids = [nid for nid in node_ids
                              if hosts[host_of[nid]].ssh == ""]
            for i, nid in enumerate(local_node_ids):
                host_name = host_of[nid]
                if zbridge:
                    # Bridge flavour: the node is a plain DDS participant. It
                    # never speaks zenoh and never learns the bridge's endpoint
                    # -- it just publishes into the host-local DDS domain and
                    # the bridge picks the sample up off SPDP.
                    node_extra = {**xml_env.get(host_name, {}),
                                  "ROS_AUTOMATIC_DISCOVERY_RANGE": "LOCALHOST"}
                elif rmw == "zenoh":
                    # All nodes connect to loopback; the router on each host
                    # exposes loopback + its LAN IP for cross-host transport.
                    node_extra = {"ZENOH_CONFIG_OVERRIDE":
                                  (zenoh_central_connect if central is not None
                                   else f'connect/endpoints=['
                                        f'"tcp/127.0.0.1:{cfg.port}"]')}
                else:
                    node_extra = xml_env.get(host_name, {})
                cmd = self._lan_node_cmd(nid, host_name)
                agents[nid] = subprocess.Popen(
                    cmd, env={**local_base, **node_extra})
                if stagger_s and i < len(local_node_ids) - 1:
                    time.sleep(stagger_s)

            # 4b. Remote nodes — one launcher Popen per host.
            for host_name, h in remote_hosts.items():
                remote_log_dir_h = f"{h.workdir}/results/lan/{run_name}"
                # Build the full env dict for each node on this host (same
                # contents as the old per-node remote_env, now passed to the
                # launcher script generator so it can emit export lines).
                node_envs: dict[str, dict] = {}
                for nid in node_ids:
                    if host_of[nid] != host_name:
                        continue
                    if zbridge:
                        node_extra = {
                            **xml_env.get(host_name, {}),
                            "ROS_AUTOMATIC_DISCOVERY_RANGE": "LOCALHOST"}
                    elif rmw == "zenoh":
                        node_extra = {"ZENOH_CONFIG_OVERRIDE":
                                      (zenoh_central_connect
                                       if central is not None
                                       else f'connect/endpoints=['
                                            f'"tcp/127.0.0.1:{cfg.port}"]')}
                    else:
                        node_extra = xml_env.get(host_name, {})
                    node_envs[nid] = {
                        "RMW_IMPLEMENTATION": rmw_impl,
                        "PYTHONNOUSERSITE": "1",
                        "ROS_LOG_DIR": remote_log_dir_h,
                        "AMENT_PREFIX_PATH": remote_ament,
                        **node_extra,
                    }

                # Generate the launcher script and stream it into the container
                # via stdin (same cat-redirect mechanism as XML in step 2).
                script_text = self._lan_launcher_script(host_name, node_envs)
                launcher_path = (f"{h.workdir}/results/lan/{run_name}/"
                                 f"launcher_{host_name}.sh")
                subprocess.run(
                    _ssh_cmd(h.ssh,
                             ["docker", "exec", "-i", h.container,
                              "sh", "-c", f"cat > {launcher_path}"]),
                    input=script_text.encode(), check=True)

                # Start all nodes on this host with ONE ssh connection.  The
                # launcher script backgrounds each node and ends with `wait`,
                # so this Popen exits only when every node on the host exits.
                launcher_procs[host_name] = subprocess.Popen(
                    _ssh_cmd(h.ssh,
                             ["docker", "exec", h.container,
                              "sh", launcher_path]))

                # Register the launcher Popen for every nid on this host.
                # The sampling loop's alive check and the kill step operate on
                # agents.values(); p.poll() / p.wait() are safe to call
                # multiple times (return the same cached exit code after first
                # completion — idempotent for the shared-Popen case).
                for nid in node_ids:
                    if host_of[nid] == host_name:
                        agents[nid] = launcher_procs[host_name]

            self.ready.set()

            # --- 5. Sampling loop (local processes only) ---------------------
            # Remote nodes are spawned as ssh Popen objects; their PIDs are on
            # the remote host and are not accessible to psutil here.  We still
            # sample host CPU / mem to detect local saturation and track local
            # workload + router processes.
            local_pids: dict[str, subprocess.Popen] = {
                nid: agents[nid]
                for nid in node_ids if hosts[host_of[nid]].ssh == ""}
            local_pids.update({
                f"router:{name}": p
                for name, p in self._routers.items()
                if hosts[
                    host_of[name.split(":", 1)[1]]
                    if ":" in name else name
                ].ssh == ""})
            meters = {
                name: psutil.Process(p.pid)
                for name, p in local_pids.items()
                if p.poll() is None}
            for m in meters.values():
                m.cpu_percent(None)   # prime per-PID CPU deltas
            psutil.cpu_percent(None)  # prime host CPU delta

            wall0 = time.monotonic()
            duration = self.scenario.duration_s
            grace = duration + 10.0
            tick = 0
            fault_injected = False
            low_mem_since: float | None = None
            resource_abort_reason: str | None = None
            # Run until duration elapsed AND all agents exited, or grace expired.
            # Remote nodes exit on --duration-s; the ssh Popen exits with them.
            while True:
                elapsed = time.monotonic() - wall0
                if (not fault_injected and central is not None
                        and central.failure_at_s is not None
                        and elapsed >= central.failure_at_s):
                    primary = central.primary_server_id
                    primary_host = hosts[host_of[primary]]
                    if primary_host.ssh == "":
                        proc = agents.get(primary)
                        if proc is not None and proc.poll() is None:
                            proc.kill()
                    else:
                        subprocess.run(
                            _ssh_cmd(primary_host.ssh, [
                                "docker", "exec", primary_host.container,
                                "pkill", "-f",
                                f"netcom_zen.ros2_workload.*--id {primary}"]),
                            capture_output=True)
                        # The local Popen for remote infrastructure is only an
                        # SSH channel; killing it does not terminate the
                        # container child.  A redundancy LAN scenario places
                        # one server per host, so removing both middleware
                        # processes on the primary host models that host loss.
                        for target in (
                                "rmw_zenohd", "fast-discovery-server",
                                "zenoh-bridge-ros2dds"):
                            subprocess.run(
                                _ssh_cmd(primary_host.ssh, [
                                    "docker", "exec", primary_host.container,
                                    "pkill", "-f", target]),
                                capture_output=True)
                    for key, proc in self._routers.items():
                        if primary in key and proc.poll() is None:
                            proc.kill()
                    fault_events.append({
                        "type": "primary_failure",
                        "server_id": primary,
                        "elapsed_s": round(elapsed, 3),
                    })
                    fault_injected = True
                alive = any(p.poll() is None for p in agents.values())
                if (elapsed >= duration and not alive) or elapsed >= grace:
                    break
                sample = self._sample_resources(round(elapsed, 3), meters)
                samples.append(sample)
                if central is not None:
                    floor = int(os.environ.get(
                        "NETCOM_CENTRAL_MEM_ABORT_BYTES",
                        str(6 * 1024**3)))
                    if sample["host_mem_avail"] < floor:
                        low_mem_since = low_mem_since or time.monotonic()
                        if time.monotonic() - low_mem_since >= 10:
                            resource_abort_reason = (
                                "available memory remained below "
                                f"{floor} bytes for 10 seconds")
                            break
                    else:
                        low_mem_since = None
                tick += 1
                # Deadline-paced (per orchestrator-tick-pacing): per-sample work
                # does not accumulate as drift.
                await asyncio.sleep(
                    max(0.0, wall0 + tick * sample_dt - time.monotonic()))

            for nid, p in agents.items():
                if p.poll() is None:
                    p.kill()
                agent_exit[nid] = p.wait(timeout=10)

        finally:
            # Kill local procs (direct Popens) and remote launcher ssh Popens.
            # Killing a launcher ssh Popen terminates the ssh channel but NOT
            # the background processes already backgrounded inside the container
            # — the pkill step below handles those.  set() deduplicates the
            # launcher Popen that appears once per nid on the same host.
            for p in set(agents.values()) | set(self._routers.values()):
                if p.poll() is None:
                    p.kill()
            # Kill remote workload processes and routers: pkill inside the
            # container is the only reliable way since killing the local ssh
            # Popen does not SIGKILL the remote child.  Ignore failures (the
            # process may have already exited normally).
            for name, h in remote_hosts.items():
                for target in (
                        "netcom_zen.ros2_workload", "rmw_zenohd",
                        "fast-discovery-server", "zenoh-bridge-ros2dds"):
                    subprocess.run(
                        _ssh_cmd(h.ssh,
                                 ["docker", "exec", h.container,
                                  "pkill", "-f", target]),
                        capture_output=True)   # ignore exit code

        # --- 6. Collect remote results via rsync ----------------------------
        # The bind mount (-v ~/net_com_zen:/work) makes the container's
        # /work/results/... identical to the host's ~/net_com_zen/results/...;
        # rsync uses the HOST path (ssh rsync works host-to-host, no docker).
        for name, h in remote_hosts.items():
            # Derive host path: container's /work == ~/net_com_zen on the host.
            host_results = h.workdir.replace("/work", "~/net_com_zen", 1)
            subprocess.run(
                ["rsync", "-a", "-e", shlex.join(_ssh_base()),
                 f"{h.ssh}:{host_results}/results/lan/{run_name}/",
                 str(out_dir) + "/"],
                check=True)

        # --- 6b. Collect per-node exit codes from exits_<host>.txt ----------
        # The launcher script appends "nid exitcode" to exits_<host>.txt after
        # each node's subshell completes; rsync (step 6) brings that file back.
        # This overwrites the launcher ssh-Popen exit code initially stored in
        # agent_exit for remote nids, replacing it with the actual workload
        # exit code.  Nodes absent from exits.txt (launcher killed before the
        # subshell finished appending) are set to None — unknown exit.
        for host_name, h in remote_hosts.items():
            remote_launcher_exit[host_name] = (
                launcher_procs[host_name].returncode
                if host_name in launcher_procs else None)
            host_node_ids = {nid for nid in node_ids
                             if host_of[nid] == host_name}
            # Pre-mark all remote nodes for this host as None; entries found
            # in exits.txt will overwrite with the real per-node exit code.
            for nid in host_node_ids:
                agent_exit[nid] = None
            exits_file = out_dir / f"exits_{host_name}.txt"
            if exits_file.exists():
                for line in exits_file.read_text().splitlines():
                    parts = line.split(maxsplit=1)
                    if len(parts) == 2 and parts[0] in host_node_ids:
                        try:
                            agent_exit[parts[0]] = int(parts[1])
                        except ValueError:
                            pass  # corrupted line; leave as None

        # --- 7. Write resource parquet and manifest -------------------------
        self._write_resources(samples)
        peak_mem = max((s["host_mem_used"] for s in samples), default=0)
        min_avail = min((s["host_mem_avail"] for s in samples), default=0)
        peak_swap = max((s["host_swap_used"] for s in samples), default=0)

        (out_dir / "manifest.json").write_text(json.dumps({
            "scenario": self.scenario.model_dump(mode="json"),
            "seed": self.scenario.seed,
            "git_hash": _git_hash(),
            "substrate": "lan",
            "rmw": rmw,
            # Zenoh flavour axis. The full scenario dump above already carries
            # it, but the report groups on top-level manifest keys -- without
            # these a bridge-flavour run is indistinguishable from an
            # rmw_zenoh_cpp one at the grouping level, and the two are different
            # products with different profiles.
            "zenoh_flavor": cfg.zenoh_flavor,
            "zenoh_bridge_local_dds": (
                cfg.zenoh_bridge_local_dds
                if cfg.zenoh_flavor == "bridge" else None),
            "zenoh_bridge_version": _zenoh_bridge_version(cfg.zenoh_flavor),
            "n_nodes": len(all_nodes),
            # topology determines the pair-count denominator for report.py:
            # mesh N(N-1); star 2(N-1); shared N(N-1) logical but O(N) SEDP endpoints
            "topology": cfg.topology,
            "resource_samples": len(samples),
            "peak_host_mem_used_bytes": peak_mem,
            "min_host_mem_avail_bytes": min_avail,
            "peak_host_swap_used_bytes": peak_swap,
            "socket_buffer_bytes": cfg.socket_buffer_bytes,
            # per-node host placement — the report uses this to split
            # same-host pairs (meaningful latency) from cross-host pairs
            # (clocks not synced; one-way latency is meaningless).
            "hosts": {n.id: n.host for n in all_nodes},
            "host_addrs": {name: h.addr for name, h in hosts.items()},
            # remote sampling not implemented in v1: a future iteration can
            # add a remote metrics collector (e.g. push psutil over ssh).
            "remote_sampled": False,
            # Wi-Fi RTT at run-start (informational; cross-host clocks not
            # synced so this is a link-quality indicator, not a latency datum).
            "rtt_ms": rtt_ms,
            "agent_exit_codes": agent_exit,
            # Per-host launcher sh exit codes (0 = all nodes on that host
            # exited before `wait` returned; non-zero = the launcher script
            # itself failed, e.g. docker exec error).  Distinct from per-node
            # codes in agent_exit_codes, which come from exits_<host>.txt.
            "remote_launcher_exit": remote_launcher_exit,
            "fault_events": fault_events,
            "resource_abort_reason": resource_abort_reason,
        }, indent=2))
