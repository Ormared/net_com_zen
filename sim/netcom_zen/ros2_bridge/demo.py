"""Root-free RViz demo (R2 exit criterion): drives mobility + link-state at
real time and publishes through the bridge. No netns, no agents, no root --
only the dataplane needs CAP_NET_ADMIN; the world model does not.

    pixi run -e ros2 viz-demo scenarios/resilience_4node.yaml
"""
import argparse
import time

from ..channel.linkstate import build_table
from ..config import load_scenario
from ..orchestrator import build_world
from .bridge import RosVizBridge


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("scenario")
    ap.add_argument("--speed", type=float, default=1.0,
                    help="real-time multiplier (2.0 = twice as fast)")
    args = ap.parse_args()
    sc = load_scenario(args.scenario)
    world = build_world(sc)
    bridge = RosVizBridge(world.terrain)
    dt = 1.0 / sc.tick_hz
    t = 0.0
    print(f"publishing {sc.name!r} to RViz at {args.speed}x "
          f"({sc.duration_s}s sim) -- ctrl-c to stop")
    try:
        while t < sc.duration_s:
            poses = world.mobility.step(dt)
            positions = {nid: (p.x, p.y) for nid, p in poses.items()}
            table = build_table(
                positions, world.jammers, t, sc.radio, world.pathloss,
                command_id=sc.command_id, satellite=sc.satellite)
            bridge.publish_tick(t, poses, table, world.jammers)
            time.sleep(dt / args.speed)
            t += dt
    except KeyboardInterrupt:
        pass
    finally:
        bridge.close()
        world.mobility.close()


if __name__ == "__main__":
    main()
