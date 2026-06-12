"""Load a run directory into render-ready structures for the dashboard.

A run is produced by ScenarioEngine: manifest.json (scenario + outcome),
packets.parquet (per-packet verdicts), positions.parquet (per-tick node poses),
agent_<id>.jsonl (pub/recv/mls/link events). Everything else the map needs
(jammer position/activation, satellite outage, foliage, extent, roles) is
reconstructed from the manifest, so this layer stays the single source of truth
for both the single-run view and A/B overlays.
"""
from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path

import pandas as pd

DROP_CAUSES = ["range", "foliage", "jam", "satloss", "queue", "no_link", "tx_error"]


@dataclass
class RunData:
    name: str
    path: Path
    manifest: dict
    positions: pd.DataFrame          # t, id, x, y, heading
    packets: pd.DataFrame            # t, src, dst, length, verdict, delay_s
    agent_events: dict[str, list]    # id -> [event dicts]
    config: dict = field(default_factory=dict)

    # --- scenario geometry (from manifest) ---
    @property
    def scenario(self) -> dict:
        return self.manifest["scenario"]

    @property
    def extent(self) -> tuple[float, float]:
        return tuple(self.scenario["environment"]["extent_m"])

    @property
    def foliage(self) -> list[dict]:
        return self.scenario["environment"].get("foliage", [])

    @property
    def duration_s(self) -> float:
        return float(self.scenario["duration_s"])

    @property
    def roles(self) -> dict[str, str]:
        return {n["id"]: n.get("role", "vehicle") for n in self.scenario["nodes"]}

    @property
    def jammers(self) -> list[dict]:
        return self.scenario.get("jammers", [])

    def jammer_active(self, jam: dict, t: float) -> bool:
        return jam["start_s"] <= t and (jam["stop_s"] is None or t < jam["stop_s"])

    @property
    def sat_outages(self) -> list[dict]:
        return self.scenario.get("satellite", {}).get("outages", [])

    def sat_up(self, t: float) -> bool:
        if not self.scenario.get("satellite", {}).get("enabled"):
            return False
        return not any(o["start_s"] <= t and (o["stop_s"] is None or t < o["stop_s"])
                       for o in self.sat_outages)


def _config_summary(scenario: dict) -> dict:
    """Compact 'backend + stress' descriptor shown as the run's identity."""
    a = scenario.get("agent", {})
    jams = scenario.get("jammers", [])
    sat = scenario.get("satellite", {})
    return {
        "transport": a.get("transport", "tcp"),
        "sync_mode": a.get("sync_mode", "delta"),
        "mls": a.get("mls", True),
        "hop_rate_hz": scenario["radio"]["hop"]["hop_rate_hz"],
        "jammers": ", ".join(
            f"{j['kind']} {j['tx_power_dbm']:.0f}dBm" for j in jams) or "none",
        "satellite": "yes" if sat.get("enabled") else "no",
        "nodes": len(scenario["nodes"]),
    }


def load_run(run_dir: str | Path) -> RunData:
    run_dir = Path(run_dir)
    manifest = json.loads((run_dir / "manifest.json").read_text())
    positions = (pd.read_parquet(run_dir / "positions.parquet")
                 if (run_dir / "positions.parquet").exists()
                 else pd.DataFrame(columns=["t", "id", "x", "y", "heading"]))
    packets = (pd.read_parquet(run_dir / "packets.parquet")
               if (run_dir / "packets.parquet").exists()
               else pd.DataFrame(columns=["t", "src", "dst", "length",
                                          "verdict", "delay_s"]))
    agent_events: dict[str, list] = {}
    for f in sorted(run_dir.glob("agent_*.jsonl")):
        nid = f.stem.removeprefix("agent_")
        agent_events[nid] = [json.loads(line) for line in
                             f.read_text().splitlines() if line.strip()]
    return RunData(
        name=run_dir.name, path=run_dir, manifest=manifest, positions=positions,
        packets=packets, agent_events=agent_events,
        config=_config_summary(manifest["scenario"]))


def list_runs(results_root: str | Path) -> list[dict]:
    """Discover runs (dirs with a manifest) under results_root with a summary."""
    root = Path(results_root)
    out = []
    for mf in sorted(root.glob("**/manifest.json")):
        try:
            scenario = json.loads(mf.read_text())["scenario"]
        except (json.JSONDecodeError, KeyError):
            continue
        has_pos = (mf.parent / "positions.parquet").exists()
        out.append({"name": str(mf.parent.relative_to(root)),
                    "path": str(mf.parent), "scenario_name": scenario.get("name"),
                    "has_positions": has_pos, **_config_summary(scenario)})
    # runs with a position track (full map playback) first
    return sorted(out, key=lambda r: (not r["has_positions"], r["name"]))


def link_quality(run: RunData, window_s: float = 1.0) -> pd.DataFrame:
    """Per directed link per time-window: delivered/attempts + dominant medium.

    Drives the live link colouring on the map. Excludes no_link bookkeeping."""
    pk = run.packets
    if pk.empty:
        return pd.DataFrame(columns=["bin", "src", "dst", "pdr", "attempts"])
    pk = pk[pk["verdict"] != "no_link"].copy()
    pk["bin"] = (pk["t"] // window_s) * window_s
    pk["ok"] = (pk["verdict"] == "delivered").astype(int)
    g = pk.groupby(["bin", "src", "dst"]).agg(
        attempts=("ok", "size"), delivered=("ok", "sum")).reset_index()
    g["pdr"] = g["delivered"] / g["attempts"]
    return g


def aoi_series(run: RunData, observer: str) -> pd.DataFrame:
    """Sawtooth AoI(t) per peer as seen by one observer, in run-seconds.

    AoI = now - origin-timestamp of the freshest applied peer update; sampled at
    each applied update and reset there. Returns long-format t, peer, aoi_s."""
    events = run.agent_events.get(observer, [])
    start = next((e["ts_us"] for e in events if e["type"] == "start"), None)
    if start is None:
        return pd.DataFrame(columns=["t", "peer", "aoi_s"])
    rows = []
    best_origin: dict[str, float] = {}
    for e in events:
        if e["type"] != "recv" or e.get("peer_ts_us", -1) <= 0:
            continue
        peer = e["from"]
        origin = e["peer_ts_us"]
        if origin <= best_origin.get(peer, -1):
            continue  # stale re-delivery
        best_origin[peer] = origin
        t_run = (e["ts_us"] - start) / 1e6
        aoi = (e["ts_us"] - origin) / 1e6
        rows.append({"t": round(t_run, 3), "peer": peer, "aoi_s": round(aoi, 3)})
    return pd.DataFrame(rows)


def events_timeline(run: RunData) -> pd.DataFrame:
    """Notable events in run-seconds for the event ticker / map annotations:
    jammer on/off, satellite outage start, per-node mls_ready / link_down / up."""
    rows = []
    for j in run.jammers:
        rows.append({"t": j["start_s"], "kind": "jammer_on",
                     "label": f"{j['id']} {j['kind']} on @ {j['tx_power_dbm']}dBm"})
        if j["stop_s"] is not None:
            rows.append({"t": j["stop_s"], "kind": "jammer_off",
                         "label": f"{j['id']} off"})
    for o in run.sat_outages:
        rows.append({"t": o["start_s"], "kind": "sat_outage",
                     "label": "satellite uplink lost"})
    for nid, events in run.agent_events.items():
        start = next((e["ts_us"] for e in events if e["type"] == "start"), None)
        if start is None:
            continue
        for e in events:
            if e["type"] in ("mls_ready", "link_down", "link_up"):
                rows.append({"t": round((e["ts_us"] - start) / 1e6, 2),
                             "kind": e["type"], "label": f"{nid}: {e['type']}"})
    df = pd.DataFrame(rows)
    return df.sort_values("t").reset_index(drop=True) if not df.empty else df
