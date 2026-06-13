import argparse
from functools import partial

from .server import StepperServer


def main() -> None:
    ap = argparse.ArgumentParser(
        description="Lockstep mobility stepper for mobility.provider=isaac "
                    "(start before the orchestrator; needs the isaac pixi env "
                    "unless --backend kinematic)")
    ap.add_argument("--socket", default="/tmp/ncz_isaac.sock")
    ap.add_argument("--backend", choices=["isaac", "kinematic"],
                    default="isaac")
    ap.add_argument("--gui", action="store_true",
                    help="render the Isaac viewport to watch it live (needs a "
                         "display); default is headless")
    args = ap.parse_args()

    idle_callback = None
    if args.backend == "isaac":
        # SimulationApp must exist before any isaacsim.core import; one app
        # per process, scenes rebuilt per run by the backend
        from isaacsim import SimulationApp
        app = SimulationApp({"headless": not args.gui})
        from .isaac_backend import IsaacBackend
        factory = partial(IsaacBackend, render=args.gui)
        if args.gui:
            # keep the viewport responsive in the gaps between lockstep ticks
            idle_callback = app.update
    else:
        from .kinematic import KinematicBackend
        factory = KinematicBackend

    try:
        StepperServer(args.socket, factory,
                      idle_callback=idle_callback).serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        if args.backend == "isaac":
            app.close()


if __name__ == "__main__":
    main()
