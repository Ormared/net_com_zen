from __future__ import annotations

import re
from pathlib import Path
from typing import Literal

import yaml
from pydantic import BaseModel, Field, model_validator


class HopConfig(BaseModel):
    n_channels: int = Field(gt=1)
    hop_rate_hz: float = Field(gt=0)


class RadioProfile(BaseModel):
    freq_hz: float = Field(gt=0)
    bandwidth_hz: float = Field(gt=0)  # per hop channel
    tx_power_dbm: float = Field(le=30.0)  # SWaP cap: 1 W (README)
    antenna_gain_dbi: float = 0.0
    noise_figure_db: float = 7.0
    data_rate_bps: float = Field(gt=0)
    hop: HopConfig
    # FEC erasure-correction capability across interleaved dwells (0 = no FEC);
    # a packet survives jamming if <= floor(fec_fraction * dwells) are jammed
    fec_fraction: float = Field(ge=0.0, lt=1.0, default=0.0)


class NodeConfig(BaseModel):
    id: str
    waypoints: list[tuple[float, float]] = Field(min_length=1)
    speed_mps: float = Field(gt=0, default=5.0)
    role: Literal["vehicle", "command"] = "vehicle"  # M4: command = uplink sink
    # Which host this node runs on (substrate=lan only; ignored otherwise).
    # Must match a key in Scenario.hosts.  Default "local" keeps every
    # non-lan node on the local machine without requiring YAML changes.
    host: str = "local"


class FoliageRect(BaseModel):
    x_min: float
    x_max: float
    y_min: float
    y_max: float

    @model_validator(mode="after")
    def _ordered(self):
        if self.x_min >= self.x_max or self.y_min >= self.y_max:
            raise ValueError("foliage rect min must be < max")
        return self


class EnvironmentConfig(BaseModel):
    heightmap: Path | None = None  # .npy of heights (m), row=y, col=x
    extent_m: tuple[float, float] = (1000.0, 1000.0)
    foliage: list[FoliageRect] = []
    # Pathloss backend (M5.3): composite = analytical Friis/two-ray +
    # knife-edge + Weissberger; sionna = ray-traced grid, precomputed offline
    # with `pixi run -e sionna sionna-precompute` (foliage stays Weissberger).
    pathloss: Literal["composite", "sionna"] = "composite"
    sionna_grid: Path | None = None  # .npz written by sionna_precompute

    @model_validator(mode="after")
    def _sionna_needs_grid(self):
        if self.pathloss == "sionna" and self.sionna_grid is None:
            raise ValueError(
                "environment.pathloss=sionna requires environment.sionna_grid "
                "(run `pixi run -e sionna sionna-precompute <scenario> -o <grid>`)")
        return self


class JammerConfig(BaseModel):
    id: str
    kind: Literal["barrage", "spot", "sweep", "reactive"]
    position: tuple[float, float]
    tx_power_dbm: float
    start_s: float = 0.0
    stop_s: float | None = None
    channels: list[int] = []          # spot: jammed hop-channel indices
    bandwidth_hz: float | None = None  # barrage: total jammed bandwidth
    # reactive (follower) jammer: senses the active transmission and retunes to
    # jam its channel. It only lands on a dwell if it can lock on before the
    # transmitter hops away, so its effect is a per-dwell lock probability
    # (models.md): detection_prob * max(0, (T_dwell - react_latency_s)/T_dwell).
    react_latency_s: float | None = None  # sense + retune latency
    detection_prob: float = Field(default=1.0, ge=0.0, le=1.0)  # sensing reliability
    sense_threshold_db: float = 0.0       # min SINR at the jammer to detect a tx

    @model_validator(mode="after")
    def _kind_params(self):
        if self.kind == "spot" and not self.channels:
            raise ValueError("spot jammer requires channels")
        if self.kind == "barrage" and not self.bandwidth_hz:
            raise ValueError("barrage jammer requires bandwidth_hz")
        if self.kind == "reactive" and self.react_latency_s is None:
            raise ValueError("reactive jammer requires react_latency_s")
        return self


class OutageWindow(BaseModel):
    start_s: float = Field(ge=0)
    stop_s: float | None = None  # None = until end of run

    @model_validator(mode="after")
    def _ordered(self):
        if self.stop_s is not None and self.stop_s <= self.start_s:
            raise ValueError("outage stop_s must be > start_s")
        return self


class SatelliteConfig(BaseModel):
    """Bent-pipe uplink from every vehicle to the command node (M4).

    While up, vehicle<->command links use these (low-latency) parameters,
    overriding the RF mesh; during an outage window the link falls back to RF.
    """
    enabled: bool = False
    delay_ms: float = Field(gt=0, default=40.0)   # LEO ~40 ms RTT default
    bandwidth_bps: float = Field(gt=0, default=2.0e6)
    loss: float = Field(ge=0, le=1, default=0.0)  # clear-sky packet loss
    outages: list[OutageWindow] = []

    def active(self, t: float) -> bool:
        return not any(
            o.start_s <= t and (o.stop_s is None or t < o.stop_s)
            for o in self.outages)


class AgentConfig(BaseModel):
    enabled: bool = False
    period_ms: int = Field(gt=0, default=500)
    port: int = 7447
    full_every: int = Field(gt=0, default=10)  # snapshot cadence = loss-heal latency
    mls: bool = True
    transport: Literal["tcp", "udp"] = "tcp"  # zenoh link scheme (A/B under loss)
    sync_mode: Literal["delta", "state"] = "delta"  # CRDT sync strategy (A/B)


class CentralizedWorkloadConfig(BaseModel):
    """Mixed client/server workload used by the centralized RMW study.

    ``server_ids`` names ordinary Scenario nodes; every remaining node is a
    client.  Middleware discovery/router processes are infrastructure and are
    deliberately kept separate from these application-server roles.
    """
    server_ids: list[str] = Field(default_factory=list, max_length=2)
    profile_label: Literal["stock", "candidate", "tuned"] = "tuned"
    mode: Literal["single", "active_active", "active_passive", "sharded"] = "single"
    primary_server_id: str = ""
    failure_at_s: float | None = Field(default=None, gt=0)
    discovery_mode: Literal["native", "centralized"] = "centralized"
    telemetry_period_ms: int = Field(gt=0, default=200)
    telemetry_payload_bytes: int = Field(gt=0, default=255)
    telemetry_reliability: Literal["reliable", "best_effort"] = "best_effort"
    command_period_ms: int = Field(gt=0, default=1000)
    command_payload_bytes: int = Field(gt=0, default=255)
    rpc_period_ms: int = Field(gt=0, default=1000)
    rpc_timeout_ms: int = Field(gt=0, default=2000)
    # Active-passive keeps the standby subscribed to telemetry but creates its
    # command publisher and service only after this scenario-relative time.
    standby_activate_after_s: float | None = Field(default=None, ge=0)

    @model_validator(mode="after")
    def _shape(self):
        expected = 1 if self.mode == "single" else 2
        if len(self.server_ids) != expected:
            raise ValueError(
                f"centralized mode={self.mode!r} requires exactly {expected} "
                f"server_ids (got {len(self.server_ids)})")
        if len(set(self.server_ids)) != len(self.server_ids):
            raise ValueError("centralized server_ids must be unique")
        if self.primary_server_id not in self.server_ids:
            raise ValueError(
                "centralized primary_server_id must name one of server_ids")
        if self.mode == "single":
            if self.failure_at_s is not None:
                raise ValueError(
                    "centralized single mode does not support failure_at_s")
            if self.standby_activate_after_s is not None:
                raise ValueError(
                    "centralized single mode has no standby")
        elif self.failure_at_s is None and self.mode in {
                "active_active", "active_passive"}:
            raise ValueError(
                f"centralized mode={self.mode!r} requires failure_at_s")
        if self.mode == "active_passive":
            if self.standby_activate_after_s is None:
                self.standby_activate_after_s = self.failure_at_s
        elif self.standby_activate_after_s is not None:
            raise ValueError(
                "standby_activate_after_s is active_passive only")
        return self


class Ros2WorkloadConfig(BaseModel):
    """R3 wiring B (ADR-0006): stock ROS 2 telemetry over a configurable RMW.
    On substrate=channel it peers across the emulated channel via one zenoh
    router per netns; on substrate=bridge each RMW uses its native discovery
    over the kernel bridge (DDS benchmark track)."""
    # RMW under test (DDS benchmark axis). Maps to the rmw_*_cpp impl name and
    # the per-RMW spawn strategy: zenoh = router-per-netns; fastrtps/cyclonedds
    # = routerless, native multicast discovery on the bridge.
    rmw: Literal["zenoh", "fastrtps", "cyclonedds"] = "zenoh"
    period_ms: int = Field(gt=0, default=500)
    payload_bytes: int = Field(gt=0, default=255)  # ~ one state-sync snapshot
    reliability: Literal["reliable", "best_effort"] = "reliable"  # stock default
    # Full DDS QoS contract under study (dds-rmw-qos-plane). Pub and sub use the
    # same profile so offered==requested always. Stock-ROS2 defaults here.
    durability: Literal["volatile", "transient_local"] = "volatile"
    history: Literal["keep_last", "keep_all"] = "keep_last"
    depth: int = Field(gt=0, default=10)  # KEEP_LAST queue depth
    deadline_ms: float = Field(ge=0, default=0.0)  # 0 = infinite (off)
    lifespan_ms: float = Field(ge=0, default=0.0)  # 0 = samples never expire
    liveliness: Literal["automatic", "manual_by_topic"] = "automatic"
    liveliness_lease_ms: float = Field(ge=0, default=0.0)  # 0 = default lease
    # Transport buffer tuning (the "increase the buffer" lever). 0 = leave the
    # RMW + kernel at their defaults; >0 requests this many bytes for the DDS
    # socket receive buffer AND raises the host net.core.{r,w}mem_max ceiling so
    # the request isn't silently clamped. Applies to fastrtps/cyclonedds (UDP);
    # zenoh is TCP-meshed so it's a no-op there.
    socket_buffer_bytes: int = Field(ge=0, default=0)
    # Stagger participant joins by this many ms (sleep between spawns). 0 = all
    # nodes start simultaneously (the SPDP-storm worst case). The hypothesis for
    # Fast DDS's ~33-participant discovery ceiling is that simultaneous startup
    # collapses discovery into a clique; staggering lets it settle incrementally.
    spawn_stagger_ms: float = Field(ge=0, default=0.0)
    port: int = 7447  # router port inside each netns (zenoh only)
    # Fast DDS Discovery Server (tuning experiment, dds-rmw-tuning.md #4): when
    # true, run one `fast-discovery-server` broker on the bridge and point every
    # node at it (client-server discovery) instead of distributed SPDP/SEDP
    # multicast — turns the O(N^2) participant mesh into O(N). Fast DDS ONLY:
    # Cyclone has no broker equivalent (its analog is unicast peers + buffers)
    # and zenoh is already router-brokered, so this flag is rejected for them.
    discovery_server: bool = False
    # Fast DDS participant allocation preallocation (docs/dds-topology-plan.md
    # P1). 0 = stock allocation defaults (Fast DDS chooses expansion-based limits
    # at runtime); >0 preallocates the participant's discovery resource limits —
    # total_participants, total_readers, total_writers, and maxInitialPeersRange
    # — to this fixed capacity (initial=maximum=A, increment=0). The hypothesis
    # is that the ~33-participant clique cap at N=96 is a default allocation
    # ceiling binding under simultaneous-startup load; preallocating removes the
    # need for runtime reallocation during discovery bursts. Fast DDS ONLY:
    # cyclonedds and zenoh have no equivalent attribute.
    fastdds_allocation_participants: int = Field(ge=0, default=0)
    # Endpoint topology (docs/dds-topology-plan.md P4/D4). mesh = current
    # all-to-all (O(N²) SEDP endpoint matrix, the proven scaling wall at N≥48);
    # shared = one aggregation topic /swarm/telemetry that collapses the SEDP
    # matrix to O(N); star = one hub node + N-1 spokes with 2(N-1) directed
    # links, also O(N).  Validated against hub_id at the Scenario level below.
    topology: Literal["mesh", "shared", "star", "centralized"] = "mesh"
    # Hub node id for topology=star.  The hub subscribes to /swarm/telemetry
    # (spoke→hub direction) and publishes /swarm/command (hub→spoke direction).
    # Must equal one of the Scenario node ids when topology='star'; must be
    # empty for topology='mesh'/'shared' (a stray hub_id silently means nothing
    # and almost certainly indicates a misconfigured scenario).
    hub_id: str = ""
    # Populated only for topology=centralized.  None for every historical
    # workload so old scenarios and result manifests retain their meaning.
    centralized: CentralizedWorkloadConfig | None = None
    # ROS_DOMAIN_ID cluster partitioning (docs/dds-topology-plan.md P5,
    # implication #1 from the DDS benchmark results). 0 = single domain
    # (default; all existing behavior is unchanged).  K>0 = split the N nodes
    # into K contiguous blocks; block j runs on ROS_DOMAIN_ID j.  The SPDP
    # participant-discovery cap is per-domain (~33 for Fast DDS, storm-prone
    # for Cyclone under all-to-all SPDP at N≥48); ~24 nodes per cluster keeps
    # each block under the N=24-healthy / N=48-collapsed knee measured in the
    # benchmark.  V1 has NO cross-cluster relay — cross-cluster pairs see 0
    # discovered peers and the metric of interest is intra-cluster mesh
    # completeness, not cross-cluster connectivity.
    cluster_domains: int = Field(ge=0, le=32, default=0)
    # Zenoh ROUTER graph topology (docs/dds-topology-plan.md P5). Distinct from
    # `topology` above, which shapes the ROS endpoint graph: zenoh connectivity
    # rides its router links, and the full lower-index router mesh (N(N-1)/2 TCP
    # links) overloads zenoh's own control plane at N=96 on the bridge
    # ("Unable to push non droppable network message ... Closing transport!",
    # dds-neighbor-table.md). star = every router connects only to the first
    # node's router (N-1 links) — the hypothesized fix, measured in P5.
    # zenoh + bridge only: fastrtps/cyclonedds have no routers, and the lan
    # substrate already runs one router per HOST (its graph is per-host).
    zenoh_router_topology: Literal["mesh", "star"] = "mesh"
    # Extra CycloneDDS <Internal> tuning elements (docs/dds-topology-plan.md
    # P5: retransmit/pacing behaviour on lossy links — e.g. NackDelay,
    # RetransmitMerging, MaxQueuedRexmitBytes). Rendered verbatim as
    # <Key>value</Key> inside <Internal>; keys are restricted to bare element
    # names so a config value can't inject XML structure. cyclonedds only.
    cyclonedds_internal: dict[str, str] = Field(default_factory=dict)

    @model_validator(mode="after")
    def _discovery_server_is_fastrtps_only(self):
        if self.discovery_server and self.rmw != "fastrtps":
            raise ValueError(
                f"discovery_server=True is Fast DDS only, not rmw={self.rmw!r} "
                "(cyclonedds has no broker; zenoh is already router-brokered)")
        if self.fastdds_allocation_participants > 0 and self.rmw != "fastrtps":
            raise ValueError(
                f"fastdds_allocation_participants={self.fastdds_allocation_participants}"
                f" is Fast DDS only, not rmw={self.rmw!r} "
                "(cyclonedds and zenoh have no equivalent allocation attribute)")
        if self.zenoh_router_topology != "mesh" and self.rmw != "zenoh":
            raise ValueError(
                f"zenoh_router_topology={self.zenoh_router_topology!r} is zenoh "
                f"only, not rmw={self.rmw!r} (fastrtps/cyclonedds are routerless)")
        if self.cyclonedds_internal and self.rmw != "cyclonedds":
            raise ValueError(
                f"cyclonedds_internal={sorted(self.cyclonedds_internal)} is "
                f"CycloneDDS only, not rmw={self.rmw!r}")
        for k in self.cyclonedds_internal:
            if not re.fullmatch(r"[A-Za-z][A-Za-z0-9]*", k):
                raise ValueError(
                    f"cyclonedds_internal key {k!r} is not a bare XML element "
                    "name ([A-Za-z][A-Za-z0-9]*)")
        return self


class MobilityConfig(BaseModel):
    """Plan R4 (ADR-0006): poses from closed-form waypoint kinematics or from
    a headless Isaac Sim stepper driven in lockstep by the orchestrator."""
    provider: Literal["waypoint", "isaac"] = "waypoint"
    # isaac: unix socket where the stepper (netcom_zen.isaac_stepper, isaac
    # pixi env, started by the user -- it owns the GPU process) listens
    socket: Path = Path("/tmp/ncz_isaac.sock")
    physics_hz: float = Field(gt=0, default=60.0)  # PhysX frames per sim second


class LinkstateLogConfig(BaseModel):
    """Per-tick ground-truth link-state logging (EW backlog): persist the
    build_table() output to linkstate.parquet so the dashboard can show the
    true jammer footprint / link quality even on idle links (the packet-based
    reconstruction only colours links that carried traffic). Off by default —
    the table is O(N^2) rows per logged tick; every_n_ticks decimates (link
    state changes slowly relative to the tick rate)."""
    enabled: bool = False
    every_n_ticks: int = Field(ge=1, default=1)


class NetemConfig(BaseModel):
    """Per-link tc netem emulation on the bridge substrate (P5,
    docs/dds-topology-plan.md). Applied to each node's egress (the veth inside
    its netns), so a one-way path costs delay_ms once and pair RTT is
    2*delay_ms. 0 disables that knob."""
    delay_ms: float = Field(ge=0, default=0.0)
    jitter_ms: float = Field(ge=0, default=0.0)   # requires delay_ms > 0
    loss_pct: float = Field(ge=0, le=100, default=0.0)
    rate_mbit: float = Field(ge=0, default=0.0)   # 0 = unlimited


class LanHostConfig(BaseModel):
    """One physical machine participating in a substrate=lan run.

    The "local" host (ssh="") is the machine running the orchestrator.
    Remote hosts are reached via SSH and nodes are started inside the
    long-lived dds-lab container (see docker/dds-lab/Dockerfile and
    scripts/dds_lan_deploy.sh).

    Interface pinning is MANDATORY on hosts that have noise interfaces
    (zerotier, tailscale, docker bridges) because DDS announces locators
    on every interface it finds; an unreachable locator causes silent
    discovery failure at swarm scale.
    """
    ssh: str = ""            # SSH target ("user@host"); "" = the local machine
    addr: str                # LAN IP on the shared segment (used for DDS pinning)
    iface: str               # NIC name to pin DDS / zenoh to (e.g. wlp130s0f0)
    container: str = "dds-lab"   # container name created by dds_lan_deploy.sh
    workdir: str = "/work"       # bind-mount point inside the container
    max_nodes: int = Field(gt=0, default=24)  # per-host node ceiling


# Per-substrate node ceilings. channel = AF_PACKET forwarder (single-thread
# asyncio, O(N^2) broadcast fan-out) genuinely tops out low — keep the historic
# cap so the EW track's assumptions are untouched. bridge = kernel L2 forwarding,
# pushed to swarm scale for the DDS benchmark (96 was the original stress point;
# raised to 256 for the QoS-plane ceiling probe that pushes each RMW past 96).
# lan = real NICs on two machines; 96 was the original stress ceiling, raised to
# 128 so the P4 star archetype fits 96 spokes + 1 hub (cross-host means the
# bottleneck is the Wi-Fi link not the rig).
_SUBSTRATE_MAX_NODES = {"channel": 8, "bridge": 256, "lan": 128}


class Scenario(BaseModel):
    name: str
    duration_s: float = Field(gt=0)
    tick_hz: float = 10.0
    seed: int = 0
    radio: RadioProfile
    # Dataplane: channel = netns + AF_PACKET RF forwarder (EW track); bridge =
    # netns + veth into a Linux kernel bridge, no RF model (DDS benchmark track);
    # lan = plain processes on real machines' NICs, no netns, no root required.
    substrate: Literal["channel", "bridge", "lan"] = "channel"
    # Upper bound is the largest substrate ceiling; the exact cap is enforced
    # per-substrate in _nodes_fit_substrate below.
    nodes: list[NodeConfig] = Field(min_length=2, max_length=256)
    # Host map for substrate=lan: keys are logical names (e.g. "local", "mini"),
    # values are LanHostConfig.  Must contain "local" (ssh="").  Ignored when
    # substrate != "lan".
    hosts: dict[str, LanHostConfig] = {}
    environment: EnvironmentConfig = EnvironmentConfig()
    jammers: list[JammerConfig] = []
    workload: Literal["agent", "ros2"] = "agent"  # what crosses the channel
    mobility: MobilityConfig = MobilityConfig()
    agent: AgentConfig = AgentConfig()
    ros2: Ros2WorkloadConfig = Ros2WorkloadConfig()
    satellite: SatelliteConfig = SatelliteConfig()
    # Per-link tc netem emulation (P5, docs/dds-topology-plan.md). None = no
    # qdisc, ideal-link baseline. Only valid on substrate=bridge — enforced by
    # _netem_requires_bridge below.
    netem: NetemConfig | None = None
    # Ground-truth per-tick link-state logging -> linkstate.parquet (EW
    # backlog). channel substrate only: the other substrates have no RF model.
    linkstate_log: LinkstateLogConfig = LinkstateLogConfig()

    @model_validator(mode="after")
    def _unique_ids(self):
        ids = [n.id for n in self.nodes]
        if len(set(ids)) != len(ids):
            raise ValueError("duplicate node ids")
        return self

    @model_validator(mode="after")
    def _nodes_fit_substrate(self):
        cap = _SUBSTRATE_MAX_NODES[self.substrate]
        if len(self.nodes) > cap:
            hint = ("; the channel forwarder can't model more, "
                    "use substrate=bridge for swarm scale"
                    if self.substrate == "channel" else "")
            raise ValueError(
                f"substrate={self.substrate!r} supports at most {cap} nodes "
                f"(got {len(self.nodes)}){hint}")
        return self

    @model_validator(mode="after")
    def _physics_divides_tick(self):
        # lockstep needs a whole number of physics frames per scenario tick
        frames = self.mobility.physics_hz / self.tick_hz
        if self.mobility.provider == "isaac" and abs(frames - round(frames)) > 1e-9:
            raise ValueError(
                f"mobility.physics_hz ({self.mobility.physics_hz}) must be an "
                f"integer multiple of tick_hz ({self.tick_hz})")
        return self

    @model_validator(mode="after")
    def _command_requires_sat(self):
        commands = [n.id for n in self.nodes if n.role == "command"]
        if len(commands) > 1:
            raise ValueError("at most one command node")
        if self.satellite.enabled and not commands:
            raise ValueError("satellite.enabled requires a command node")
        return self

    @model_validator(mode="after")
    def _lan_config_valid(self):
        """Validate lan-specific constraints and reject invalid host placements
        on non-lan substrates.

        For substrate=lan:
          - hosts must be non-empty and contain a "local" key (ssh="").
          - every node.host must resolve to a key in hosts.
          - per-host node count must not exceed LanHostConfig.max_nodes.
          - socket_buffer_bytes and discovery_server are rejected: v1 keeps
            kernel buffer knobs and DDS broker knobs off the lan path to
            avoid confounding the baseline cross-host measurement.

        For other substrates:
          - node.host != "local" is rejected (no remote spawning available).
        """
        if self.substrate == "lan":
            if not self.hosts:
                raise ValueError(
                    "substrate=lan requires at least one host in hosts; "
                    'add a "local" entry (ssh="") for the orchestrator machine')
            if "local" not in self.hosts:
                raise ValueError(
                    'substrate=lan requires a "local" key in hosts (ssh=""); '
                    "the orchestrator spawns local nodes without SSH")
            # Validate per-node host references
            for node in self.nodes:
                if node.host not in self.hosts:
                    raise ValueError(
                        f"node {node.id!r} has host={node.host!r} which is not "
                        f"a key in hosts (defined hosts: "
                        f"{list(self.hosts.keys())})")
            # Per-host ceiling: each host has its own max_nodes budget
            from collections import Counter
            counts: Counter = Counter(n.host for n in self.nodes)
            for host_name, count in counts.items():
                max_n = self.hosts[host_name].max_nodes
                if count > max_n:
                    raise ValueError(
                        f"host {host_name!r} has {count} nodes assigned but "
                        f"max_nodes={max_n}; raise LanHostConfig.max_nodes or "
                        "redistribute nodes")
            # v1 keeps kernel knobs off the lan path: socket_buffer_bytes>0
            # would require sudo on the remote host (we never use sudo remotely)
            # and cross-host buffer tuning is a separate study.
            if self.ros2.socket_buffer_bytes > 0:
                raise ValueError(
                    "substrate=lan rejects ros2.socket_buffer_bytes > 0 (v1 "
                    "keeps kernel buffer knobs off the lan path; remote sudo "
                    "is not available)")
            # discovery_server requires a local broker process that the lan path
            # does not start; reject early with a clear message.
            if self.ros2.discovery_server:
                raise ValueError(
                    "substrate=lan rejects ros2.discovery_server=True (v1 "
                    "keeps the DDS broker knob off the lan path)")
        else:
            # Non-lan substrates: remote host placement makes no sense since
            # _run_channel and _run_bridge only work locally.
            for node in self.nodes:
                if node.host != "local":
                    raise ValueError(
                        f"node {node.id!r} has host={node.host!r} but "
                        f"substrate={self.substrate!r} only supports "
                        "host='local' (remote spawning requires substrate=lan)")
        return self

    @model_validator(mode="after")
    def _netem_requires_bridge(self):
        """netem lives on the bridge substrate only: channel has its own
        RF/pathloss model that already controls link quality; lan runs on real
        NICs where tc is not managed by the orchestrator (no root on remote
        hosts). Also enforce tc's own constraint: jitter is a perturbation on
        top of a base delay, so delay_ms must be > 0 when jitter_ms > 0.
        Putting this cross-field check here (not in NetemConfig) keeps
        NetemConfig a plain value object and groups all Scenario-level cross-
        field rules in the same place."""
        if self.netem is None:
            return self
        if self.substrate != "bridge":
            raise ValueError(
                f"netem requires substrate='bridge' "
                f"(channel has its own RF model; lan is real hardware); "
                f"got substrate={self.substrate!r}")
        if self.netem.jitter_ms > 0 and self.netem.delay_ms == 0:
            raise ValueError(
                "netem.jitter_ms requires netem.delay_ms > 0 "
                "(tc netem: jitter is a perturbation on top of a base delay; "
                "setting jitter without delay is a tc error)")
        return self

    @model_validator(mode="after")
    def _linkstate_log_requires_channel(self):
        # the link-state table exists only where the RF model runs
        if self.linkstate_log.enabled and self.substrate != "channel":
            raise ValueError(
                f"linkstate_log requires substrate='channel' (bridge/lan have "
                f"no RF link-state table); got substrate={self.substrate!r}")
        return self

    @model_validator(mode="after")
    def _zenoh_router_star_requires_bridge(self):
        """zenoh_router_topology='star' rewires the per-netns router graph,
        which only exists on the bridge substrate. channel pre-wires the same
        per-netns mesh but its RF model is the object under test there; lan
        runs ONE router per host (the graph is per-host, not per-node)."""
        if (self.workload == "ros2"
                and self.ros2.zenoh_router_topology != "mesh"
                and self.substrate != "bridge"):
            raise ValueError(
                f"zenoh_router_topology="
                f"{self.ros2.zenoh_router_topology!r} requires "
                f"substrate='bridge'; got substrate={self.substrate!r}")
        return self

    @model_validator(mode="after")
    def _topology_hub_id_valid(self):
        """For topology=star, hub_id must name a real scenario node (the hub
        subscribes to /swarm/telemetry and fans out on /swarm/command).  For
        any other topology, hub_id must be empty — a non-empty hub_id when
        topology≠star is almost certainly a misconfigured scenario and would
        silently mean nothing at runtime.  Skipped for workload!='ros2'
        (hub_id is a ros2-workload concept with no meaning in the agent track).
        """
        if self.workload != "ros2":
            return self
        topo = self.ros2.topology
        hub = self.ros2.hub_id
        ids = {n.id for n in self.nodes}
        if topo == "star":
            if not hub:
                raise ValueError(
                    "ros2.topology='star' requires ros2.hub_id (the hub node id); "
                    "set hub_id to one of the scenario node ids")
            if hub not in ids:
                raise ValueError(
                    f"ros2.hub_id={hub!r} is not a scenario node id "
                    f"(valid ids: {sorted(ids)})")
        elif hub:
            raise ValueError(
                f"ros2.hub_id={hub!r} requires ros2.topology='star' "
                f"(got topology={topo!r}); clear hub_id or set topology='star'")
        central = self.ros2.centralized
        if topo == "centralized":
            if central is None:
                raise ValueError(
                    "ros2.topology='centralized' requires ros2.centralized")
            missing = sorted(set(central.server_ids) - ids)
            if missing:
                raise ValueError(
                    f"centralized server_ids are not scenario nodes: {missing}")
            if len(ids - set(central.server_ids)) < 1:
                raise ValueError(
                    "centralized workload requires at least one client node")
            if (central.failure_at_s is not None
                    and central.failure_at_s >= self.duration_s):
                raise ValueError(
                    "centralized failure_at_s must be before scenario duration_s")
            if (central.standby_activate_after_s is not None
                    and central.standby_activate_after_s >= self.duration_s):
                raise ValueError(
                    "centralized standby activation must be before duration_s")
        elif central is not None:
            raise ValueError(
                "ros2.centralized requires ros2.topology='centralized'")
        return self

    @model_validator(mode="after")
    def _cluster_domains_valid(self):
        """cluster_domains > 0 requires a specific combination of substrate,
        workload, and topology; validate every cross-field constraint here so
        the error surfaces at parse time rather than at spawn time.

        Substrate: lan uses real NICs with no orchestrator-managed netns env
        injection; bridge is the only substrate where per-netns ROS_DOMAIN_ID
        is controlled by the orchestrator.
        Workload: only the ros2 track uses ROS_DOMAIN_ID; the agent track is
        unaffected (it has no DDS discovery).
        Topology: shared/star collapse the endpoint matrix by different means
        (a future combination); mesh is the proven O(N²) SPDP wall this lever
        targets.  Reject now rather than silently misbehave with shared/star.
        Discovery Server: _spawn_fastdds_server does not inject ROS_DOMAIN_ID
        (it is left untouched); combining DS + clusters would give every node
        the same server address but different domain IDs, breaking client
        registration silently.  Reject early with a clear message.
        Node count: cluster_domains > len(nodes) leaves the last clusters empty
        (the contiguous-block formula gives 0 members), which is nonsensical.
        """
        k = self.ros2.cluster_domains
        if k == 0:
            return self  # fast path: clustering off, nothing to validate
        if self.workload != "ros2":
            raise ValueError(
                f"cluster_domains={k} requires workload='ros2' "
                f"(got workload={self.workload!r}; only the ros2 track uses "
                "ROS_DOMAIN_ID-based DDS domain partitioning)")
        if self.substrate != "bridge":
            raise ValueError(
                f"cluster_domains={k} requires substrate='bridge' "
                f"(got substrate={self.substrate!r}); the bridge substrate is "
                "the only path where the orchestrator injects per-netns "
                "ROS_DOMAIN_ID; lan runs plain processes with no env injection")
        if self.ros2.topology != "mesh":
            raise ValueError(
                f"cluster_domains={k} requires ros2.topology='mesh' "
                f"(got topology={self.ros2.topology!r}); shared/star + clusters "
                "is a later combination — not implemented in v1")
        if self.ros2.discovery_server:
            raise ValueError(
                f"cluster_domains={k} is incompatible with "
                "ros2.discovery_server=True: the Discovery Server spawn path "
                "does not inject ROS_DOMAIN_ID per node, so combining DS and "
                "cluster partitioning would silently break client registration")
        if k > len(self.nodes):
            raise ValueError(
                f"cluster_domains={k} > len(nodes)={len(self.nodes)}: some "
                "clusters would be empty (reduce cluster_domains or add nodes)")
        return self

    @property
    def command_id(self) -> str | None:
        return next((n.id for n in self.nodes if n.role == "command"), None)


def load_scenario(path: str | Path) -> Scenario:
    return Scenario.model_validate(yaml.safe_load(Path(path).read_text()))
