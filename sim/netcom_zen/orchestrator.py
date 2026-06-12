from __future__ import annotations

import asyncio
import json
import os
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

    def _spawn_agents(self) -> dict[str, subprocess.Popen]:
        cfg = self.scenario.agent
        bin_path = os.environ.get("NCZ_AGENT_BIN", "agent/target/release/ncz-agent")
        agents: dict[str, subprocess.Popen] = {}
        for nid in self.topo.nodes:
            others = [f"tcp/{self.topo.addrs[n]}:{cfg.port}"
                      for n in self.topo.nodes if n != nid]
            cmd = ["ip", "netns", "exec", self.topo.ns_names[nid], bin_path,
                   "--id", nid,
                   "--listen", f"tcp/{self.topo.addrs[nid]}:{cfg.port}",
                   "--metrics", str(self.out_dir / f"agent_{nid}.jsonl"),
                   "--period-ms", str(cfg.period_ms),
                   "--duration-s", str(self.scenario.duration_s)]
            for o in others:
                cmd += ["--connect", o]
            agents[nid] = subprocess.Popen(cmd)
        return agents

    async def run(self) -> None:
        self._build()
        self.out_dir.mkdir(parents=True, exist_ok=True)
        self.topo = NetnsTopology([n.id for n in self.scenario.nodes])
        self.topo.setup()
        log = PacketLog()
        fwd = ChannelForwarder(self.topo, self.scenario.seed, log)
        max_tick_lag = 0.0  # timing integrity (architecture.md)
        dt = 1.0 / self.scenario.tick_hz
        agents: dict[str, subprocess.Popen] = {}
        agent_exit: dict[str, int | None] = {}
        try:
            fwd.start()
            # install the initial link-state table BEFORE agents spawn, so their
            # first connection attempts don't die as no_link
            positions0 = {nid: (v.x, v.y) for nid, v in self.vehicles.items()}
            fwd.update_links(build_table(positions0, self.jammers, 0.0,
                                         self.scenario.radio, self.pathloss))
            agents = self._spawn_agents() if self.scenario.agent.enabled else {}
            self.ready.set()
            t = 0.0
            wall0 = time.monotonic()
            # keep the channel alive until duration elapsed AND agents exited
            # (a dead agent mid-run is a logged event, not an abort)
            grace = self.scenario.duration_s + 10.0
            while t < self.scenario.duration_s or (
                    t < grace and any(p.poll() is None for p in agents.values())):
                poses = {nid: v.step(dt) for nid, v in self.vehicles.items()}
                positions = {nid: (p.x, p.y) for nid, p in poses.items()}
                fwd.update_links(build_table(positions, self.jammers, t,
                                             self.scenario.radio, self.pathloss))
                await asyncio.sleep(dt)
                t += dt
                max_tick_lag = max(max_tick_lag, (time.monotonic() - wall0) - t)
            for nid, p in agents.items():
                if p.poll() is None:
                    p.kill()
                agent_exit[nid] = p.wait(timeout=10)
            fwd.stop()
        finally:
            for p in agents.values():
                if p.poll() is None:
                    p.kill()
            self.topo.teardown()
        log.to_parquet(self.out_dir / "packets.parquet")
        (self.out_dir / "manifest.json").write_text(json.dumps({
            "scenario": self.scenario.model_dump(mode="json"),
            "seed": self.scenario.seed,
            "git_hash": _git_hash(),
            "max_tick_lag_s": max_tick_lag,
            "timing_ok": max_tick_lag < dt,
            "agent_exit_codes": agent_exit,
        }, indent=2))
