"""The telemetry node process. Run via `python -m netcom_zen.ros2_workload`
inside a vehicle netns with RMW_IMPLEMENTATION=rmw_zenoh_cpp."""
from __future__ import annotations

import argparse
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
        self.metrics = open(args.metrics, "a", buffering=1)
        qos = build_qos(args)
        self._manual_liveliness = args.liveliness == "manual_by_topic"
        self.pub = self.create_publisher(
            String, f"/swarm/{args.id}/telemetry", qos)
        for peer in args.peers.split(","):
            self.create_subscription(
                String, f"/swarm/{peer}/telemetry", self._on_msg, qos)
        self.create_timer(args.period_ms / 1e3, self._tick)
        # record the effective QoS so each run's metrics are self-describing
        self._log({"type": "start", "id": args.id, "ts_us": now_us(),
                   "qos": {"reliability": args.reliability,
                           "durability": args.durability,
                           "history": args.history, "depth": args.depth,
                           "deadline_ms": args.deadline_ms,
                           "lifespan_ms": args.lifespan_ms,
                           "liveliness": args.liveliness,
                           "liveliness_lease_ms": args.liveliness_lease_ms}})

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
        self._log({"type": "recv", "id": self.args.id, "from": peer,
                   "kind": "snap", "peer_seq": seq, "peer_ts_us": ts_us,
                   "ts_us": recv_us, "bytes": len(msg.data)})


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--id", required=True)
    ap.add_argument("--peers", required=True, help="comma-separated peer ids")
    ap.add_argument("--period-ms", type=int, default=500)
    ap.add_argument("--payload-bytes", type=int, default=255)
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
