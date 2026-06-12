"""Sweep runner: cartesian product of parameter axes × seeds over a base
scenario (benchmarking.md). Each cell runs the full engine; paired seeds mean
every configuration sees the identical channel realization per seed.

Needs root (netns):
    sudo $(which python) -m netcom_zen.harness.sweep scenarios/sweep_X.yaml -o results/X
"""
from __future__ import annotations

import argparse
import asyncio
import copy
import itertools
import json
from pathlib import Path

import yaml

from ..config import Scenario
from ..orchestrator import ScenarioEngine


def apply_override(cfg: dict, dotted: str, value) -> None:
    """Set a dotted path like 'radio.hop.hop_rate_hz' or 'jammers.0.tx_power_dbm'."""
    keys = dotted.split(".")
    node = cfg
    for k in keys[:-1]:
        node = node[int(k)] if isinstance(node, list) else node[k]
    last = keys[-1]
    if isinstance(node, list):
        node[int(last)] = value
    else:
        node[last] = value


def cell_name(overrides: dict[str, object], seed: int) -> str:
    parts = [f"{k.split('.')[-1]}={v}" for k, v in overrides.items()]
    return "__".join(parts + [f"seed={seed}"]).replace(" ", "")


def expand(sweep: dict) -> list[tuple[dict[str, object], int]]:
    axes: dict[str, list] = sweep.get("axes", {})
    seeds: list[int] = sweep.get("seeds", [0])
    cells = []
    for combo in itertools.product(*axes.values()) if axes else [()]:
        overrides = dict(zip(axes.keys(), combo))
        for seed in seeds:
            cells.append((overrides, seed))
    return cells


async def run_sweep(sweep_path: str | Path, out_root: str | Path) -> Path:
    sweep_path = Path(sweep_path)
    sweep = yaml.safe_load(sweep_path.read_text())
    base = yaml.safe_load((sweep_path.parent / sweep["base"]).read_text())
    out_root = Path(out_root)
    out_root.mkdir(parents=True, exist_ok=True)

    cells = expand(sweep)
    done = []
    for i, (overrides, seed) in enumerate(cells, 1):
        cfg = copy.deepcopy(base)
        for k, v in overrides.items():
            apply_override(cfg, k, v)
        cfg["seed"] = seed
        scenario = Scenario.model_validate(cfg)
        name = cell_name(overrides, seed)
        run_dir = out_root / name
        if (run_dir / "manifest.json").exists():
            print(f"[{i}/{len(cells)}] {name} — exists, skipping")
            done.append({"name": name, "overrides": overrides, "seed": seed})
            continue
        print(f"[{i}/{len(cells)}] {name} ({scenario.duration_s:.0f}s)")
        engine = ScenarioEngine(scenario, run_dir)
        await engine.run()
        done.append({"name": name, "overrides": overrides, "seed": seed})

    (out_root / "sweep_manifest.json").write_text(json.dumps({
        "sweep": sweep, "cells": done}, indent=2))
    return out_root


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("sweep")
    ap.add_argument("-o", "--out", required=True)
    args = ap.parse_args()
    asyncio.run(run_sweep(args.sweep, args.out))


if __name__ == "__main__":
    main()
