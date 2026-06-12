"""Stock-ROS2 telemetry workload (R3 wiring B, ADR-0006).

One instance per vehicle netns, peering over rmw_zenoh through the emulated
channel. Emits the agent's JSONL metrics schema (pub/recv/final_state) so the
existing AoI/report harness consumes ROS2 runs unchanged.

`payload` is importable everywhere; `node` needs rclpy (ros2 pixi env).
"""
