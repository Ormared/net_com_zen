import argparse
import asyncio
from pathlib import Path

from .config import load_scenario
from .orchestrator import ScenarioEngine


def main() -> None:
    ap = argparse.ArgumentParser(description="Run a net_com_zen scenario (needs root)")
    ap.add_argument("scenario")
    ap.add_argument("-o", "--out", default="results/latest")
    args = ap.parse_args()
    engine = ScenarioEngine(load_scenario(args.scenario), Path(args.out))
    asyncio.run(engine.run())


if __name__ == "__main__":
    main()
