from __future__ import annotations

import math

import rclpy
from builtin_interfaces.msg import Time
from geometry_msgs.msg import Point, TransformStamped
from rclpy.node import Node
from rosgraph_msgs.msg import Clock
from std_msgs.msg import ColorRGBA
from tf2_msgs.msg import TFMessage
from visualization_msgs.msg import Marker, MarkerArray

from ..channel.linkstate import LinkState
from ..ew import Jammer
from ..mobility import Pose
from ..terrain import Terrain

# packet length the link colouring assumes: one state-sync snapshot
# (the project's resilient config, docs/results/sync-delta-vs-state.md)
NOMINAL_LEN_BYTES = 255
MARKER_Z = 1.5  # lift vehicles/links off the terrain so lines don't clip


def _stamp(t: float) -> Time:
    return Time(sec=int(t), nanosec=int((t - int(t)) * 1e9))


def _link_color(st: LinkState, p: float) -> ColorRGBA:
    if st.medium == "sat":
        return ColorRGBA(r=0.2, g=0.4, b=1.0, a=0.9)
    # green -> red as delivery probability falls; near-dead links fade out
    return ColorRGBA(r=1.0 - p, g=p, b=0.0, a=0.9 if p > 0.02 else 0.25)


class RosVizBridge:
    """Publishes /clock (sim time -- the orchestrator stays clock master,
    ADR-0006), /tf vehicle frames, and /netcom/markers for RViz."""

    def __init__(self, terrain: Terrain, frame: str = "map",
                 node_name: str = "netcom_zen"):
        self.terrain = terrain
        self.frame = frame
        if not rclpy.ok():
            rclpy.init()
        self.node = Node(node_name)
        self._clock = self.node.create_publisher(Clock, "/clock", 10)
        self._tf = self.node.create_publisher(TFMessage, "/tf", 10)
        self._markers = self.node.create_publisher(
            MarkerArray, "/netcom/markers", 10)

    def close(self) -> None:
        self.node.destroy_node()

    def publish_tick(self, t: float, poses: dict[str, Pose],
                     table: dict[tuple[str, str], LinkState],
                     jammers: list[Jammer]) -> None:
        stamp = _stamp(t)
        self._clock.publish(Clock(clock=stamp))
        self._tf.publish(TFMessage(transforms=[
            self._transform(stamp, nid, p) for nid, p in poses.items()]))
        arr = MarkerArray()
        for i, (nid, p) in enumerate(sorted(poses.items())):
            arr.markers += self._vehicle_markers(stamp, i, nid, p)
        arr.markers.append(self._links_marker(stamp, poses, table))
        for i, j in enumerate(jammers):
            arr.markers += self._jammer_markers(stamp, i, j, t)
        self._markers.publish(arr)

    def _ground(self, x: float, y: float) -> float:
        return self.terrain.height(x, y) + MARKER_Z

    def _marker(self, stamp: Time, ns: str, mid: int, mtype: int) -> Marker:
        m = Marker()
        m.header.frame_id = self.frame
        m.header.stamp = stamp
        m.ns, m.id, m.type, m.action = ns, mid, mtype, Marker.ADD
        m.pose.orientation.w = 1.0
        return m

    def _transform(self, stamp: Time, nid: str, p: Pose) -> TransformStamped:
        tr = TransformStamped()
        tr.header.frame_id = self.frame
        tr.header.stamp = stamp
        tr.child_frame_id = nid
        tr.transform.translation.x = p.x
        tr.transform.translation.y = p.y
        tr.transform.translation.z = self._ground(p.x, p.y)
        tr.transform.rotation.z = math.sin(p.heading / 2)
        tr.transform.rotation.w = math.cos(p.heading / 2)
        return tr

    def _vehicle_markers(self, stamp: Time, i: int, nid: str,
                         p: Pose) -> list[Marker]:
        m = self._marker(stamp, "vehicles", i, Marker.SPHERE)
        m.pose.position.x, m.pose.position.y = p.x, p.y
        m.pose.position.z = self._ground(p.x, p.y)
        m.scale.x = m.scale.y = m.scale.z = 8.0
        m.color = ColorRGBA(r=0.1, g=0.7, b=1.0, a=1.0)
        label = self._marker(stamp, "labels", i, Marker.TEXT_VIEW_FACING)
        label.pose.position.x, label.pose.position.y = p.x, p.y
        label.pose.position.z = self._ground(p.x, p.y) + 12.0
        label.scale.z = 14.0
        label.color = ColorRGBA(r=1.0, g=1.0, b=1.0, a=1.0)
        label.text = nid
        return [m, label]

    def _links_marker(self, stamp: Time, poses: dict[str, Pose],
                      table: dict[tuple[str, str], LinkState]) -> Marker:
        m = self._marker(stamp, "links", 0, Marker.LINE_LIST)
        m.scale.x = 2.0  # line width, metres
        for (src, dst), st in table.items():
            if src > dst:  # one line per pair; quality = worse direction
                continue
            rev = table.get((dst, src))
            p = min(st.delivery_prob(NOMINAL_LEN_BYTES),
                    rev.delivery_prob(NOMINAL_LEN_BYTES) if rev else 1.0)
            color = _link_color(st, p)
            for nid in (src, dst):
                pt = poses[nid]
                m.points.append(self._point(pt.x, pt.y))
                m.colors.append(color)
        return m

    def _jammer_markers(self, stamp: Time, i: int, j: Jammer,
                        t: float) -> list[Marker]:
        x, y = j.position
        on = j.active(t)
        body = self._marker(stamp, "jammers", i, Marker.CYLINDER)
        body.pose.position.x, body.pose.position.y = float(x), float(y)
        body.pose.position.z = self._ground(x, y)
        body.scale.x = body.scale.y = 16.0
        body.scale.z = 6.0
        body.color = (ColorRGBA(r=1.0, g=0.1, b=0.1, a=1.0) if on
                      else ColorRGBA(r=0.5, g=0.5, b=0.5, a=0.5))
        label = self._marker(stamp, "jammer_labels", i, Marker.TEXT_VIEW_FACING)
        label.pose.position.x, label.pose.position.y = float(x), float(y)
        label.pose.position.z = self._ground(x, y) + 14.0
        label.scale.z = 14.0
        label.color = ColorRGBA(r=1.0, g=0.6, b=0.6, a=1.0)
        state = "ON" if on else "off"
        label.text = f"{j.cfg.id} {j.cfg.kind} {j.tx_power_dbm:.0f}dBm {state}"
        return [body, label]

    def _point(self, x: float, y: float) -> Point:
        return Point(x=x, y=y, z=self._ground(x, y))
