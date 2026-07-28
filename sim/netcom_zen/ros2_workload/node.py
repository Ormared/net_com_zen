"""The telemetry node process. Run via `python -m netcom_zen.ros2_workload`
inside a vehicle netns with RMW_IMPLEMENTATION=rmw_zenoh_cpp."""
from __future__ import annotations

import argparse
import copy
import json
import os
import time

import rclpy
from rclpy.duration import Duration
from rclpy.node import Node
from rclpy.qos import (
    DurabilityPolicy,
    HistoryPolicy,
    LivelinessPolicy,
    QoSProfile,
    ReliabilityPolicy,
)
from std_msgs.msg import String

from .payload import pack, unpack


def now_us() -> int:
    return int(time.time() * 1e6)


def _dur(ms: float) -> Duration:
    """Map a millisecond knob to an rmw Duration. <=0 means 'leave at the QoS
    default' (infinite / disabled) — same convention DDS uses for an unset
    deadline / lifespan / liveliness lease."""
    return Duration(nanoseconds=int(ms * 1e6)) if ms > 0 else Duration()


def build_qos(args: argparse.Namespace) -> QoSProfile:
    """Assemble the full DDS QoS contract from CLI knobs. This is the axis under
    study (dds-rmw-qos-plane): every policy that can plausibly affect how a stack
    scales to many participants is exposed here so the sweep can move it.

    Pub and sub share one profile so the offered/requested contract always
    matches (a mismatch would silently leave endpoints unpaired and look like a
    scaling failure when it is really a QoS-incompatibility)."""
    q = QoSProfile(
        depth=args.depth,
        history=(HistoryPolicy.KEEP_ALL if args.history == "keep_all"
                 else HistoryPolicy.KEEP_LAST),
        reliability=(ReliabilityPolicy.RELIABLE if args.reliability == "reliable"
                     else ReliabilityPolicy.BEST_EFFORT),
        durability=(DurabilityPolicy.TRANSIENT_LOCAL
                    if args.durability == "transient_local"
                    else DurabilityPolicy.VOLATILE),
        liveliness=(LivelinessPolicy.MANUAL_BY_TOPIC
                    if args.liveliness == "manual_by_topic"
                    else LivelinessPolicy.AUTOMATIC),
        deadline=_dur(args.deadline_ms),
        lifespan=_dur(args.lifespan_ms),
        liveliness_lease_duration=_dur(args.liveliness_lease_ms),
    )
    return q


class TelemetryNode(Node):
    def __init__(self, args: argparse.Namespace):
        super().__init__(f"telemetry_{args.id}")
        self.args = args
        self.seq = 0
        self._started_mono = time.monotonic()
        self.metrics = open(args.metrics, "a", buffering=1)
        qos = build_qos(args)
        self._manual_liveliness = args.liveliness == "manual_by_topic"

        # ---- topology-dependent pub / sub wiring (docs/dds-topology-plan.md P4/D4) ----
        # mesh (unchanged, default): O(N²) endpoint matrix — each node keeps one
        # per-id publisher and N-1 per-peer subscriptions (the proven scaling wall
        # at N≥48 in the DDS benchmark).
        # shared: one aggregation topic (/swarm/telemetry) collapses SEDP to O(N)
        # endpoints; self-drop in _on_msg prevents logging own delivery as recv.
        # star: O(N) directed links — spokes publish up on /swarm/telemetry, the
        # hub fans commands back down on /swarm/command (2(N-1) endpoint pairs).
        topo = args.topology
        if topo == "mesh":
            # Fail loudly on an empty peer list — an empty string looks like a
            # valid value but means the node would run with zero subscriptions,
            # which is indistinguishable from successful discovery to the user.
            if not args.peers:
                raise ValueError(
                    "--topology mesh requires --peers (non-empty comma-separated "
                    "peer ids); shared/star topologies leave --peers unused")
            self.pub = self.create_publisher(
                String, f"/swarm/{args.id}/telemetry", qos)
            for peer in args.peers.split(","):
                self.create_subscription(
                    String, f"/swarm/{peer}/telemetry", self._on_msg, qos)
        elif topo == "shared":
            # One shared topic per swarm: every node publishes and subscribes to
            # the same /swarm/telemetry topic.  ROS 2 delivers a node's own
            # publications to its own subscription; _on_msg drops them before the
            # recv log line so self-messages never appear in analytics.
            self.pub = self.create_publisher(String, "/swarm/telemetry", qos)
            self.create_subscription(
                String, "/swarm/telemetry", self._on_msg, qos)
        elif topo == "star":
            if args.role == "spoke":
                # Spoke: publishes telemetry up to the hub; listens for hub
                # command fan-out (the return path of the star).
                self.pub = self.create_publisher(
                    String, "/swarm/telemetry", qos)
                self.create_subscription(
                    String, "/swarm/command", self._on_msg, qos)
            else:  # hub
                # Hub: collects telemetry from all spokes; its _tick publishes
                # command messages back down on /swarm/command.
                self.pub = self.create_publisher(
                    String, "/swarm/command", qos)
                self.create_subscription(
                    String, "/swarm/telemetry", self._on_msg, qos)
        elif topo == "centralized":
            self._setup_centralized()

        if topo != "centralized":
            self.create_timer(args.period_ms / 1e3, self._tick)
        # record the effective QoS + topology so each run's metrics are self-describing
        self._log({"type": "start", "id": args.id, "ts_us": now_us(),
                   "topology": args.topology, "role": args.role,
                   "qos": {"reliability": args.reliability,
                           "durability": args.durability,
                           "history": args.history, "depth": args.depth,
                           "deadline_ms": args.deadline_ms,
                           "lifespan_ms": args.lifespan_ms,
                           "liveliness": args.liveliness,
                           "liveliness_lease_ms": args.liveliness_lease_ms}})

    def _qos_with_reliability(self, reliability: str) -> QoSProfile:
        ns = copy.copy(self.args)
        ns.reliability = reliability
        return build_qos(ns)

    def _setup_centralized(self) -> None:
        """Wire the mixed telemetry + command + service workload."""
        from std_srvs.srv import Trigger

        args = self.args
        self._Trigger = Trigger
        self._central_active = args.role == "server"
        self._central_activated = False
        self._pending_rpc: dict[int, tuple[int, object]] = {}
        self._rpc_seq = 0
        self._seen_commands: set[int] = set()
        self._server_rpc_count = 0

        shard = args.central_mode == "sharded"
        target = args.assigned_server
        telemetry_topic = (
            f"/central/{target}/telemetry" if shard and args.role == "client"
            else f"/central/{args.id}/telemetry" if shard
            else "/central/telemetry")
        command_topic = (
            f"/central/{target}/command" if shard and args.role == "client"
            else f"/central/{args.id}/command" if shard
            else "/central/command")
        service_name = (
            f"/central/{target}/request" if shard and args.role == "client"
            else f"/central/{args.id}/request" if shard
            else "/central/request")
        self._central_command_topic = command_topic
        self._central_service_name = service_name

        telemetry_qos = self._qos_with_reliability(
            args.telemetry_reliability)
        command_qos = self._qos_with_reliability("reliable")

        if args.role == "client":
            self.telemetry_pub = self.create_publisher(
                String, telemetry_topic, telemetry_qos)
            self.create_subscription(
                String, command_topic, self._on_command, command_qos)
            self.rpc_client = self.create_client(Trigger, service_name)
            self.create_timer(
                args.telemetry_period_ms / 1e3, self._telemetry_tick)
            self.create_timer(args.rpc_period_ms / 1e3, self._rpc_tick)
            self.create_timer(0.05, self._expire_rpc)
        else:
            self.create_subscription(
                String, telemetry_topic, self._on_telemetry, telemetry_qos)
            if self._central_active:
                self._activate_server()
            else:
                self.create_timer(0.05, self._activation_check)

    def _activate_server(self) -> None:
        if self._central_activated:
            return
        self._central_activated = True
        command_qos = self._qos_with_reliability("reliable")
        self.command_pub = self.create_publisher(
            String, self._central_command_topic, command_qos)
        self.rpc_service = self.create_service(
            self._Trigger, self._central_service_name, self._on_rpc)
        self.create_timer(
            self.args.command_period_ms / 1e3, self._command_tick)
        self._log({"type": "server_active", "id": self.args.id,
                   "ts_us": now_us()})

    def _activation_check(self) -> None:
        activate_at = self.args.activate_after_s
        if (activate_at >= 0
                and time.monotonic() - self._started_mono >= activate_at):
            self._activate_server()

    def _telemetry_tick(self) -> None:
        self.seq += 1
        ts_us = now_us()
        data = pack(
            self.args.id, self.seq, ts_us,
            self.args.telemetry_payload_bytes)
        self.telemetry_pub.publish(String(data=data))
        self._log({"type": "telemetry_pub", "id": self.args.id,
                   "seq": self.seq, "ts_us": ts_us, "bytes": len(data)})

    def _on_telemetry(self, msg: String) -> None:
        peer, seq, ts_us = unpack(msg.data)
        self._log({"type": "telemetry_recv", "id": self.args.id,
                   "from": peer, "peer_seq": seq, "peer_ts_us": ts_us,
                   "ts_us": now_us(), "bytes": len(msg.data)})

    def _command_tick(self) -> None:
        self.seq += 1
        ts_us = now_us()
        data = pack(
            self.args.id, self.seq, ts_us,
            self.args.command_payload_bytes)
        self.command_pub.publish(String(data=data))
        self._log({"type": "command_pub", "id": self.args.id,
                   "seq": self.seq, "ts_us": ts_us, "bytes": len(data)})

    def _on_command(self, msg: String) -> None:
        server, seq, ts_us = unpack(msg.data)
        duplicate = seq in self._seen_commands
        self._seen_commands.add(seq)
        self._log({"type": "command_duplicate" if duplicate
                   else "command_recv",
                   "id": self.args.id, "from": server, "peer_seq": seq,
                   "peer_ts_us": ts_us, "ts_us": now_us(),
                   "bytes": len(msg.data)})

    def _rpc_tick(self) -> None:
        self._rpc_seq += 1
        seq = self._rpc_seq
        started_us = now_us()
        if not self.rpc_client.service_is_ready():
            self._log({"type": "rpc_unavailable", "id": self.args.id,
                       "seq": seq, "ts_us": started_us})
            return
        future = self.rpc_client.call_async(self._Trigger.Request())
        self._pending_rpc[seq] = (started_us, future)
        self._log({"type": "rpc_start", "id": self.args.id,
                   "seq": seq, "ts_us": started_us})

        def done(fut, request_seq=seq):
            pending = self._pending_rpc.pop(request_seq, None)
            if pending is None:
                return
            try:
                response = fut.result()
                self._log({
                    "type": "rpc_success", "id": self.args.id,
                    "seq": request_seq, "server": response.message,
                    "started_us": pending[0], "ts_us": now_us(),
                })
            except Exception as exc:
                self._log({"type": "rpc_error", "id": self.args.id,
                           "seq": request_seq, "error": type(exc).__name__,
                           "started_us": pending[0], "ts_us": now_us()})

        future.add_done_callback(done)

    def _expire_rpc(self, force: bool = False) -> None:
        now = now_us()
        timeout_us = self.args.rpc_timeout_ms * 1000
        for seq, (started, future) in list(self._pending_rpc.items()):
            if not force and now - started < timeout_us:
                continue
            self._pending_rpc.pop(seq, None)
            future.cancel()
            self._log({"type": "rpc_timeout", "id": self.args.id,
                       "seq": seq, "started_us": started, "ts_us": now})

    def _on_rpc(self, request, response):
        self._server_rpc_count += 1
        response.success = True
        response.message = f"{self.args.id}:{self._server_rpc_count}"
        self._log({"type": "rpc_served", "id": self.args.id,
                   "seq": self._server_rpc_count, "ts_us": now_us()})
        return response

    def _log(self, ev: dict) -> None:
        # compact separators: the harness greps for '"type":"pub"' (report.py)
        self.metrics.write(json.dumps(ev, separators=(",", ":")) + "\n")

    def _tick(self) -> None:
        self.seq += 1
        data = pack(self.args.id, self.seq, now_us(), self.args.payload_bytes)
        # MANUAL_BY_TOPIC: the writer must assert liveliness itself. Publishing a
        # sample asserts it implicitly, but assert explicitly too so a stalled
        # publish can't silently drop the writer out of its readers' liveliness.
        if self._manual_liveliness:
            self.pub.assert_liveliness()
        self.pub.publish(String(data=data))
        self._log({"type": "pub", "id": self.args.id, "seq": self.seq,
                   "kind": "snap", "ts_us": now_us(), "bytes": len(data)})

    def _on_msg(self, msg: String) -> None:
        recv_us = now_us()
        peer, seq, ts_us = unpack(msg.data)
        # shared topic: ROS 2 delivers a node's own publications to its own
        # matching subscription (unlike per-peer mesh topics where you never
        # subscribe to your own topic).  Drop before logging — a self-message
        # is not a connected pair and must not inflate recv counts or appear
        # in latency distributions.
        if self.args.topology == "shared" and peer == self.args.id:
            return
        self._log({"type": "recv", "id": self.args.id, "from": peer,
                   "kind": "snap", "peer_seq": seq, "peer_ts_us": ts_us,
                   "ts_us": recv_us, "bytes": len(msg.data)})


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--id", required=True)
    ap.add_argument("--peers", required=False, default="",
                    help="comma-separated peer ids (mesh only; unused for shared/star)")
    ap.add_argument("--topology",
                    choices=["mesh", "shared", "star", "centralized"],
                    default="mesh",
                    help="endpoint topology: mesh=O(N²) all-to-all (default); "
                         "shared=one aggregation topic O(N); star=hub+spokes O(N)")
    ap.add_argument("--role",
                    choices=["spoke", "hub", "client", "server", "standby"],
                    default="spoke",
                    help="star role: spoke publishes telemetry up, hub fans commands "
                         "down (ignored for mesh/shared)")
    ap.add_argument("--period-ms", type=int, default=500)
    ap.add_argument("--payload-bytes", type=int, default=255)
    ap.add_argument("--central-mode",
                    choices=["single", "active_active", "active_passive",
                             "sharded"],
                    default="single")
    ap.add_argument("--server-ids", default="")
    ap.add_argument("--assigned-server", default="")
    ap.add_argument("--activate-after-s", type=float, default=-1.0)
    ap.add_argument("--telemetry-period-ms", type=int, default=200)
    ap.add_argument("--telemetry-payload-bytes", type=int, default=255)
    ap.add_argument("--telemetry-reliability",
                    choices=["reliable", "best_effort"],
                    default="best_effort")
    ap.add_argument("--command-period-ms", type=int, default=1000)
    ap.add_argument("--command-payload-bytes", type=int, default=255)
    ap.add_argument("--rpc-period-ms", type=int, default=1000)
    ap.add_argument("--rpc-timeout-ms", type=int, default=2000)
    ap.add_argument("--reliability", choices=["reliable", "best_effort"],
                    default="reliable")
    ap.add_argument("--durability", choices=["volatile", "transient_local"],
                    default="volatile")
    ap.add_argument("--history", choices=["keep_last", "keep_all"],
                    default="keep_last")
    ap.add_argument("--depth", type=int, default=10)
    ap.add_argument("--deadline-ms", type=float, default=0.0,
                    help="0 = no deadline (infinite)")
    ap.add_argument("--lifespan-ms", type=float, default=0.0,
                    help="0 = no lifespan (samples never expire)")
    ap.add_argument("--liveliness", choices=["automatic", "manual_by_topic"],
                    default="automatic")
    ap.add_argument("--liveliness-lease-ms", type=float, default=0.0,
                    help="0 = default lease (infinite)")
    ap.add_argument("--duration-s", type=float, required=True)
    ap.add_argument("--metrics", required=True)
    args = ap.parse_args()

    rclpy.init()
    node = TelemetryNode(args)
    end = time.time() + args.duration_s
    while time.time() < end:
        rclpy.spin_once(node, timeout_sec=0.2)
    if args.topology == "centralized" and args.role == "client":
        node._expire_rpc(force=True)
    node._log({"type": "final_state", "id": args.id, "ts_us": now_us()})
    node.metrics.flush()
    # rmw_cyclonedds_cpp (and occasionally fastrtps) can hang for seconds in
    # rclpy.shutdown() draining DDS threads. The metrics file is line-buffered
    # so all events are already on disk; a clean teardown buys us nothing (the
    # netns is about to be destroyed, SHM is off) and would stall every run to
    # the orchestrator's grace deadline. Exit hard instead.
    os._exit(0)


if __name__ == "__main__":
    main()
