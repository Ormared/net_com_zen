"""R1 smoke test: ROS 2 pub/sub round-trip in the pixi `ros2` env (ADR-0006).

Runs only where rclpy is importable (the ros2 environment); the default env
skips. Each RMW gets a fresh child process because the RMW implementation is
fixed at rclpy.init() time.
"""
import os
import subprocess
import sys
import textwrap

import pytest

rclpy_available = bool(
    subprocess.run(
        [sys.executable, "-c", "import rclpy"], capture_output=True
    ).returncode == 0
)

pytestmark = pytest.mark.skipif(
    not rclpy_available, reason="rclpy not in this environment (use `pixi run -e ros2`)"
)

ROUNDTRIP = textwrap.dedent("""
    import rclpy
    from rclpy.node import Node
    from std_msgs.msg import String

    rclpy.init()
    node = Node("smoke")
    got = []
    node.create_subscription(String, "/smoke", lambda m: got.append(m.data), 10)
    pub = node.create_publisher(String, "/smoke", 10)

    deadline = node.get_clock().now().nanoseconds + 15_000_000_000
    while not got and node.get_clock().now().nanoseconds < deadline:
        msg = String()
        msg.data = "ping"
        pub.publish(msg)
        rclpy.spin_once(node, timeout_sec=0.2)

    node.destroy_node()
    rclpy.shutdown()
    assert got and got[0] == "ping", f"no round-trip: {got}"
""")


def _run_roundtrip(rmw: str, extra_env: dict) -> None:
    env = {**os.environ, "RMW_IMPLEMENTATION": rmw, **extra_env}
    proc = subprocess.run(
        [sys.executable, "-c", ROUNDTRIP],
        env=env, capture_output=True, text=True, timeout=60,
    )
    assert proc.returncode == 0, (
        f"{rmw} round-trip failed\nstdout: {proc.stdout}\nstderr: {proc.stderr}"
    )


def test_roundtrip_fastdds():
    _run_roundtrip("rmw_fastrtps_cpp", {})


def test_roundtrip_zenoh():
    # rmw_zenoh needs its router; give it a non-default port to avoid
    # colliding with any zenoh session the agent side may have open.
    env = {**os.environ, "RMW_IMPLEMENTATION": "rmw_zenoh_cpp"}
    router = subprocess.Popen(
        ["ros2", "run", "rmw_zenoh_cpp", "rmw_zenohd"],
        env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )
    try:
        _run_roundtrip("rmw_zenoh_cpp", {})
    finally:
        router.terminate()
        router.wait(timeout=10)
