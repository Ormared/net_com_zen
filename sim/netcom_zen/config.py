from __future__ import annotations

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


class JammerConfig(BaseModel):
    id: str
    kind: Literal["barrage", "spot", "sweep"]
    position: tuple[float, float]
    tx_power_dbm: float
    start_s: float = 0.0
    stop_s: float | None = None
    channels: list[int] = []          # spot: jammed hop-channel indices
    bandwidth_hz: float | None = None  # barrage: total jammed bandwidth

    @model_validator(mode="after")
    def _kind_params(self):
        if self.kind == "spot" and not self.channels:
            raise ValueError("spot jammer requires channels")
        if self.kind == "barrage" and not self.bandwidth_hz:
            raise ValueError("barrage jammer requires bandwidth_hz")
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
        return self


class MobilityConfig(BaseModel):
    """Plan R4 (ADR-0006): poses from closed-form waypoint kinematics or from
    a headless Isaac Sim stepper driven in lockstep by the orchestrator."""
    provider: Literal["waypoint", "isaac"] = "waypoint"
    # isaac: unix socket where the stepper (netcom_zen.isaac_stepper, isaac
    # pixi env, started by the user -- it owns the GPU process) listens
    socket: Path = Path("/tmp/ncz_isaac.sock")
    physics_hz: float = Field(gt=0, default=60.0)  # PhysX frames per sim second


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
# lan = real NICs on two machines; 96 is the same stress ceiling as the original
# bridge target, and cross-host means the bottleneck is the Wi-Fi link not the rig.
_SUBSTRATE_MAX_NODES = {"channel": 8, "bridge": 256, "lan": 96}


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

    @property
    def command_id(self) -> str | None:
        return next((n.id for n in self.nodes if n.role == "command"), None)


def load_scenario(path: str | Path) -> Scenario:
    return Scenario.model_validate(yaml.safe_load(Path(path).read_text()))
