import argparse

from .server import StepperServer


def main() -> None:
    ap = argparse.ArgumentParser(
        description="Lockstep mobility stepper for mobility.provider=isaac "
                    "(start before the orchestrator; needs the isaac pixi env "
                    "unless --backend kinematic)")
    ap.add_argument("--socket", default="/tmp/ncz_isaac.sock")
    ap.add_argument("--backend", choices=["isaac", "kinematic"],
                    default="isaac")
    args = ap.parse_args()

    if args.backend == "isaac":
        # SimulationApp must exist before any isaacsim.core import; one app
        # per process, scenes rebuilt per run by the backend
        from isaacsim import SimulationApp
        app = SimulationApp({"headless": True})
        from .isaac_backend import IsaacBackend
        factory = IsaacBackend
    else:
        from .kinematic import KinematicBackend
        factory = KinematicBackend

    try:
        StepperServer(args.socket, factory).serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        if args.backend == "isaac":
            app.close()


if __name__ == "__main__":
    main()
