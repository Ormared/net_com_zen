from __future__ import annotations

import asyncio
import json
import subprocess
import time
from pathlib import Path

import numpy as np

from .channel.forwarder import ChannelForwarder
from .channel.linkstate import build_table
from .config import Scenario
from .ew import Jammer
from .metrics import PacketLog
from .mobility import WaypointVehicle
from .netns import NetnsTopology
from .propagation import CompositePathloss
from .terrain import FoliageRegion, Terrain


def _git_hash() -> str:
    try:
        return subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True,
                              text=True, check=True).stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "unknown"


class ScenarioEngine:
    """Owns the single clock: tick loop driving mobility -> pathloss ->
    link-state -> forwarder (architecture.md)."""

    def __init__(self, scenario: Scenario, out_dir: str | Path):
        self.scenario = scenario
        self.out_dir = Path(out_dir)
        self.ready = asyncio.Event()
        self.topo: NetnsTopology | None = None

    def _build(self) -> None:
        env = self.scenario.environment
        hm = np.load(env.heightmap) if env.heightmap else None
        terrain = Terrain(env.extent_m, hm,
                          [FoliageRegion(**f.model_dump()) for f in env.foliage])
        self.pathloss = CompositePathloss(terrain)
        self.vehicles = {n.id: WaypointVehicle(n.waypoints, n.speed_mps)
                         for n in self.scenario.nodes}
        self.jammers = [Jammer(j) for j in self.scenario.jammers]

    async def run(self) -> None:
        self._build()
        self.topo = NetnsTopology([n.id for n in self.scenario.nodes])
        self.topo.setup()
        log = PacketLog()
        fwd = ChannelForwarder(self.topo, self.scenario.seed, log)
        max_tick_lag = 0.0  # timing integrity (architecture.md)
        dt = 1.0 / self.scenario.tick_hz
        try:
            fwd.start()
            self.ready.set()
            t = 0.0
            wall0 = time.monotonic()
            while t < self.scenario.duration_s:
                poses = {nid: v.step(dt) for nid, v in self.vehicles.items()}
                positions = {nid: (p.x, p.y) for nid, p in poses.items()}
                fwd.update_links(build_table(positions, self.jammers, t,
                                             self.scenario.radio, self.pathloss))
                await asyncio.sleep(dt)
                t += dt
                max_tick_lag = max(max_tick_lag, (time.monotonic() - wall0) - t)
            fwd.stop()
        finally:
            self.topo.teardown()
        self.out_dir.mkdir(parents=True, exist_ok=True)
        log.to_parquet(self.out_dir / "packets.parquet")
        (self.out_dir / "manifest.json").write_text(json.dumps({
            "scenario": self.scenario.model_dump(mode="json"),
            "seed": self.scenario.seed,
            "git_hash": _git_hash(),
            "max_tick_lag_s": max_tick_lag,
            "timing_ok": max_tick_lag < dt,
        }, indent=2))
