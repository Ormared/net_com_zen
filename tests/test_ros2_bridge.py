"""R2: the viz bridge publishes /clock, /tf and /netcom/markers correctly.

Runs only in the `ros2` pixi environment (rclpy importable); skips elsewhere.
"""
import math

import pytest

rclpy = pytest.importorskip("rclpy")

from rclpy.node import Node  # noqa: E402
from rosgraph_msgs.msg import Clock  # noqa: E402
from tf2_msgs.msg import TFMessage  # noqa: E402
from visualization_msgs.msg import MarkerArray  # noqa: E402

from netcom_zen.channel.linkstate import LinkState  # noqa: E402
from netcom_zen.config import JammerConfig  # noqa: E402
from netcom_zen.ew import Jammer  # noqa: E402
from netcom_zen.mobility import Pose  # noqa: E402
from netcom_zen.ros2_bridge import RosVizBridge  # noqa: E402
from netcom_zen.terrain import Terrain  # noqa: E402

POSES = {"v1": Pose(100.0, 200.0, 0.0), "v2": Pose(300.0, 200.0, math.pi)}


def _link(src, dst, prx=-80.0):
    return LinkState(src=src, dst=dst, prx_dbm=prx, noise_dbm=-113.0, rho=0.0,
                     jam_inchannel_dbm=None, data_rate_bps=250e3,
                     hop_rate_hz=100.0, prop_delay_s=1e-6,
                     foliage_db=0.0, terrain_db=0.0)


TABLE = {("v1", "v2"): _link("v1", "v2"), ("v2", "v1"): _link("v2", "v1")}
JAMMER = Jammer(JammerConfig(id="j1", kind="barrage", position=(500.0, 900.0),
                             tx_power_dbm=40.0, bandwidth_hz=12.5e6,
                             start_s=20.0))


@pytest.fixture
def received():
    bridge = RosVizBridge(Terrain(extent_m=(1000, 1000)))
    sub = Node("listener")
    got: dict[str, object] = {}
    sub.create_subscription(Clock, "/clock", lambda m: got.update(clock=m), 10)
    sub.create_subscription(TFMessage, "/tf", lambda m: got.update(tf=m), 10)
    sub.create_subscription(MarkerArray, "/netcom/markers",
                            lambda m: got.update(markers=m), 10)
    try:
        for _ in range(100):  # publish until discovery completes + all arrive
            bridge.publish_tick(30.5, POSES, TABLE, [JAMMER])
            rclpy.spin_once(sub, timeout_sec=0.05)
            if {"clock", "tf", "markers"} <= got.keys():
                break
        assert {"clock", "tf", "markers"} <= got.keys(), f"missing: {got.keys()}"
        yield got
    finally:
        sub.destroy_node()
        bridge.close()


def test_clock_is_sim_time(received):
    clk = received["clock"].clock
    assert clk.sec == 30 and clk.nanosec == pytest.approx(5e8, rel=1e-6)


def test_tf_frames(received):
    tfs = {tr.child_frame_id: tr for tr in received["tf"].transforms}
    assert set(tfs) == {"v1", "v2"}
    assert tfs["v1"].header.frame_id == "map"
    assert tfs["v1"].transform.translation.x == pytest.approx(100.0)
    # heading pi -> yaw quaternion (0, 0, 1, 0)
    assert tfs["v2"].transform.rotation.z == pytest.approx(1.0)


def test_marker_namespaces(received):
    markers = received["markers"].markers
    ns = {m.ns for m in markers}
    assert {"vehicles", "labels", "links", "jammers", "jammer_labels"} <= ns
    links = next(m for m in markers if m.ns == "links")
    assert len(links.points) == 2  # one undirected pair = one segment
    # strong clear link renders green
    assert links.colors[0].g > 0.9 and links.colors[0].r < 0.1
    jammer = next(m for m in markers if m.ns == "jammers")
    assert jammer.color.r == pytest.approx(1.0)  # t=30.5 > start_s=20: active
    label = next(m for m in markers if m.ns == "jammer_labels")
    assert "j1" in label.text and "ON" in label.text
