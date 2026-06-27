from __future__ import annotations

import asyncio
import json
import os
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path

import numpy as np
import psutil

from .channel.forwarder import ChannelForwarder
from .channel.linkstate import build_table
from .config import Scenario
from .ew import Jammer
from .metrics import PacketLog
from .mobility import MobilityProvider, make_mobility
from .netns import NetnsTopology
from .propagation import CompositePathloss
from .terrain import FoliageRegion, Terrain


# Host kernel socket-buffer ceilings raised when socket_buffer_bytes>0. A DDS
# SocketReceiveBufferSize request is silently clamped to net.core.rmem_max (and
# Cyclone treats its `min` as a HARD floor — it refuses to start the domain if
# the kernel ceiling is below the request), so the ceiling MUST be raised first.
# Root (the orchestrator runs under sudo) writes /proc/sys directly; the `sysctl`
# binary isn't on the NOPASSWD list. These are global (not netns-scoped).
_KERNEL_BUF_SYSCTLS = ("rmem_max", "wmem_max", "rmem_default", "wmem_default")


def _cyclonedds_xml(socket_buffer_bytes: int = 0) -> str:
    """Cyclone DDS config for the bridge substrate: SHM/Iceoryx off (force real
    UDP over the veth, no same-host shortcut) and multicast on so native
    discovery floods across the bridge. autodetermine picks the single non-lo
    veth in each netns. Domain id="any" applies under any ROS_DOMAIN_ID.

    socket_buffer_bytes>0 adds Internal/SocketReceiveBufferSize (the buffer
    lever); the host rmem_max must already be >= it or Cyclone fails to start."""
    buf = (f'    <Internal><SocketReceiveBufferSize min="{socket_buffer_bytes} B"/>'
           f'</Internal>\n' if socket_buffer_bytes > 0 else "")
    return ('<?xml version="1.0" encoding="UTF-8" ?>\n'
            '<CycloneDDS xmlns="https://cdds.io/config"\n'
            '    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"\n'
            '    xsi:schemaLocation="https://cdds.io/config '
            'https://raw.githubusercontent.com/eclipse-cyclonedds/cyclonedds/'
            'master/etc/cyclonedds.xsd">\n'
            '  <Domain id="any">\n'
            '    <General>\n'
            '      <Interfaces>\n'
            '        <NetworkInterface autodetermine="true" multicast="true"/>\n'
            '      </Interfaces>\n'
            '      <AllowMulticast>true</AllowMulticast>\n'
            '    </General>\n'
            '    <SharedMemory>\n'
            '      <Enable>false</Enable>\n'
            '    </SharedMemory>\n'
            f'{buf}'
            '  </Domain>\n'
            '</CycloneDDS>\n')


def _fastdds_profiles_xml(socket_buffer_bytes: int) -> str:
    """Fast DDS XML profiles defining a single UDPv4 transport with enlarged
    socket buffers, set as the default participant profile. useBuiltinTransports
    false means ONLY this transport is used — which also keeps SHM off (the same
    isolation FASTDDS_BUILTIN_TRANSPORTS=UDPv4 gives), so the buffer route is a
    drop-in replacement for the env-var route. Needs the <profiles> wrapper
    (Fast DDS 8.x) or the parser rejects transport_descriptors."""
    return ('<?xml version="1.0" encoding="UTF-8" ?>\n'
            '<dds xmlns="http://www.eprosima.com/XMLSchemas/fastRTPS_Profiles">\n'
            '  <profiles>\n'
            '    <transport_descriptors>\n'
            '      <transport_descriptor>\n'
            '        <transport_id>udp_big</transport_id>\n'
            '        <type>UDPv4</type>\n'
            f'        <receiveBufferSize>{socket_buffer_bytes}</receiveBufferSize>\n'
            f'        <sendBufferSize>{socket_buffer_bytes}</sendBufferSize>\n'
            '      </transport_descriptor>\n'
            '    </transport_descriptors>\n'
            '    <participant profile_name="big" is_default_profile="true">\n'
            '      <rtps>\n'
            '        <userTransports><transport_id>udp_big</transport_id>'
            '</userTransports>\n'
            '        <useBuiltinTransports>false</useBuiltinTransports>\n'
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


@dataclass
class World:
    """Scenario-derived simulation state, buildable without root (the netns
    dataplane is separate). Shared by the engine and the ROS 2 viz demo."""
    terrain: Terrain
    pathloss: CompositePathloss
    mobility: MobilityProvider
    jammers: list[Jammer]


def build_world(scenario: Scenario) -> World:
    env = scenario.environment
    hm = np.load(env.heightmap) if env.heightmap else None
    terrain = Terrain(env.extent_m, hm,
                      [FoliageRegion(**f.model_dump()) for f in env.foliage])
    return World(
        terrain=terrain,
        pathloss=CompositePathloss(terrain),
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

    def _workload_cmd(self, nid: str) -> list[str]:
        cfg = self.scenario.ros2
        return ["ip", "netns", "exec", self.topo.ns_names[nid],
                sys.executable, "-m", "netcom_zen.ros2_workload",
                "--id", nid,
                "--peers", ",".join(p for p in self.topo.nodes if p != nid),
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
            return self._spawn_zenoh(log_dir)
        buf = self.scenario.ros2.socket_buffer_bytes
        if rmw == "fastrtps":
            if self.scenario.ros2.discovery_server:
                return self._spawn_fastdds_server(log_dir)
            if buf > 0:
                # profiles XML: a UDPv4-only transport with enlarged buffers.
                # useBuiltinTransports=false keeps SHM off, same as the env route.
                prof = self.out_dir / "fastdds_profiles.xml"
                prof.write_text(_fastdds_profiles_xml(buf))
                return self._spawn_routerless(
                    "rmw_fastrtps_cpp", log_dir,
                    lambda nid: {"FASTDDS_DEFAULT_PROFILES_FILE": str(prof)})
            # UDPv4 builtin transport drops the default SHM transport, forcing
            # real UDP over the veth; native SPDP multicast handles discovery.
            return self._spawn_routerless(
                "rmw_fastrtps_cpp", log_dir,
                lambda nid: {"FASTDDS_BUILTIN_TRANSPORTS": "UDPv4"})
        if rmw == "cyclonedds":
            xml = self.out_dir / "cyclonedds.xml"
            xml.write_text(_cyclonedds_xml(buf))
            return self._spawn_routerless(
                "rmw_cyclonedds_cpp", log_dir,
                lambda nid: {"CYCLONEDDS_URI": f"file://{xml}"})
        raise RuntimeError(f"unknown rmw {rmw!r}")

    def _spawn_routerless(self, rmw_impl: str, log_dir: Path,
                          extra_env) -> dict[str, subprocess.Popen]:
        """fastrtps / cyclonedds: no router, one workload node per netns, native
        multicast discovery flooded by the bridge (mcast_snooping off)."""
        base = self._ros2_base_env(rmw_impl, log_dir)
        stagger_s = self.scenario.ros2.spawn_stagger_ms / 1e3
        agents: dict[str, subprocess.Popen] = {}
        for i, nid in enumerate(self.topo.nodes):
            agents[nid] = subprocess.Popen(self._workload_cmd(nid),
                                           env={**base, **extra_env(nid)})
            # stagger participant joins to defuse the simultaneous-startup SPDP
            # storm (tests the Fast DDS discovery-ceiling hypothesis)
            if stagger_s and i < len(self.topo.nodes) - 1:
                time.sleep(stagger_s)
        return agents

    # Fast DDS Discovery Server port (the impl's well-known default). The server
    # binds this on node 0's bridge address; every client unicasts discovery to
    # it instead of flooding SPDP multicast.
    _DS_PORT = 11811

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
            env = {**base_env, "ZENOH_CONFIG_OVERRIDE":
                   f'connect/endpoints=["tcp/127.0.0.1:{cfg.port}"]'}
            agents[nid] = subprocess.Popen(self._workload_cmd(nid), env=env)
        return agents

    async def run(self) -> None:
        self.out_dir.mkdir(parents=True, exist_ok=True)
        if self.scenario.substrate == "bridge":
            await self._run_bridge()
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
        }, indent=2))

    async def _run_bridge(self) -> None:
        """DDS benchmark substrate (plan: substrate=bridge). No RF model, no
        forwarder, no mobility: netns + kernel bridge carry the traffic, each
        RMW uses native discovery. Spawn the workload, then sample per-PID +
        host CPU/mem/swap for duration_s. Host-level sampling is what lets the
        write-up tell 'RMW degraded' from 'rig saturated' at swarm scale (96
        nodes is the deliberate stress point). Metrics otherwise come from the
        workload JSONL; there is no packets.parquet."""
        self.topo = NetnsTopology([n.id for n in self.scenario.nodes],
                                  bridge=True)
        self.topo.setup()
        self._routers = {}
        agents: dict[str, subprocess.Popen] = {}
        agent_exit: dict[str, int | None] = {}
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
            # keep sampling until duration elapsed AND agents exited (a dead
            # agent mid-run is a logged event, not an abort), capped by grace
            while True:
                elapsed = time.monotonic() - wall0
                alive = any(p.poll() is None for p in agents.values())
                if (elapsed >= duration and not alive) or elapsed >= grace:
                    break
                samples.append(self._sample_resources(round(elapsed, 3), meters))
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
            "resource_samples": len(samples),
            "peak_host_mem_used_bytes": peak_mem,
            "min_host_mem_avail_bytes": min_avail,
            "peak_host_swap_used_bytes": peak_swap,
            "socket_buffer_bytes": self.scenario.ros2.socket_buffer_bytes,
            "agent_exit_codes": agent_exit,
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
