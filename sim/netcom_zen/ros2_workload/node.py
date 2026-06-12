"""The telemetry node process. Run via `python -m netcom_zen.ros2_workload`
inside a vehicle netns with RMW_IMPLEMENTATION=rmw_zenoh_cpp."""
from __future__ import annotations

import argparse
import json
import time

import rclpy
from rclpy.node import Node
from rclpy.qos import QoSProfile, ReliabilityPolicy
from std_msgs.msg import String

from .payload import pack, unpack


def now_us() -> int:
    return int(time.time() * 1e6)


class TelemetryNode(Node):
    def __init__(self, args: argparse.Namespace):
        super().__init__(f"telemetry_{args.id}")
        self.args = args
        self.seq = 0
        self.metrics = open(args.metrics, "a", buffering=1)
        qos = QoSProfile(
            depth=10,
            reliability=(ReliabilityPolicy.RELIABLE
                         if args.reliability == "reliable"
                         else ReliabilityPolicy.BEST_EFFORT))
        self.pub = self.create_publisher(
            String, f"/swarm/{args.id}/telemetry", qos)
        for peer in args.peers.split(","):
            self.create_subscription(
                String, f"/swarm/{peer}/telemetry", self._on_msg, qos)
        self.create_timer(args.period_ms / 1e3, self._tick)
        self._log({"type": "start", "id": args.id, "ts_us": now_us()})

    def _log(self, ev: dict) -> None:
        # compact separators: the harness greps for '"type":"pub"' (report.py)
        self.metrics.write(json.dumps(ev, separators=(",", ":")) + "\n")

    def _tick(self) -> None:
        self.seq += 1
        data = pack(self.args.id, self.seq, now_us(), self.args.payload_bytes)
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
    ap.add_argument("--duration-s", type=float, required=True)
    ap.add_argument("--metrics", required=True)
    args = ap.parse_args()

    rclpy.init()
    node = TelemetryNode(args)
    end = time.time() + args.duration_s
    while time.time() < end:
        rclpy.spin_once(node, timeout_sec=0.2)
    node._log({"type": "final_state", "id": args.id, "ts_us": now_us()})
    node.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
