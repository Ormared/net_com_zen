"""Standalone real-time Isaac preview: boot the GUI, build a scenario's
vehicles on flat ground, and run the waypoint controllers in real time so you
can watch them move in the Isaac viewport.

This is a visual sanity check, not a metrics run — no channel, no netns, no
lockstep to the orchestrator (use the socket stepper + a scenario run for
that). One terminal, isaac pixi env:

    pixi run -e isaac isaac-preview scenarios/isaac_smoke_4node.yaml
"""
from __future__ import annotations

import argparse
import time


def main() -> None:
    ap = argparse.ArgumentParser(description="Watch Isaac drive a scenario's "
                                 "vehicles in real time (GUI).")
    ap.add_argument("scenario")
    ap.add_argument("--physics-hz", type=float, default=60.0)
    args = ap.parse_args()

    # Load + validate the scenario before the ~10s Isaac boot: fail fast on a
    # bad path, and import pydantic in the clean outer interpreter (Isaac's kit
    # Python re-checks site at boot; the isaac env sets PYTHONNOUSERSITE so the
    # pinned pydantic wins either way, but importing here keeps it cached).
    from netcom_zen.config import load_scenario
    sc = load_scenario(args.scenario)
    vehicles = {n.id: {"waypoints": [list(w) for w in n.waypoints],
                       "speed_mps": n.speed_mps} for n in sc.nodes}

    # SimulationApp must come up before any isaacsim.core import
    from isaacsim import SimulationApp
    app = SimulationApp({"headless": False})
    from .isaac_backend import IsaacBackend
    backend = IsaacBackend(1.0 / args.physics_hz, vehicles,
                           extent=sc.environment.extent_m, render=True)
    frames = round(args.physics_hz / sc.tick_hz)  # PhysX frames per scenario tick
    dt = 1.0 / sc.tick_hz
    print(f"previewing {sc.name!r}: {len(vehicles)} vehicles at "
          f"{sc.tick_hz:.0f} Hz — close the window or ctrl-c to stop",
          flush=True)
    try:
        while app.is_running():
            t0 = time.monotonic()
            backend.step(frames)
            time.sleep(max(0.0, dt - (time.monotonic() - t0)))  # real-time pace
    except KeyboardInterrupt:
        pass
    finally:
        backend.close()
        app.close()


if __name__ == "__main__":
    main()
