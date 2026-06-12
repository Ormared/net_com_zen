"""ROS 2 observability bridge (R2, ADR-0006).

Publishes live scenario state -- sim time, vehicle poses, link quality,
jammer activity -- for RViz. Importable only in the `ros2` pixi environment;
the orchestrator imports it lazily behind --ros2-viz.
"""
from .bridge import RosVizBridge

__all__ = ["RosVizBridge"]
