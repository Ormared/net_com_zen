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

import numpy as np
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
    # ground-truth per-tick link state (linkstate.parquet, opt-in): t, src,
    # dst, prx_dbm, noise_dbm, rho, jam_inchannel_dbm, delivery_prob,
    # foliage_db, terrain_db, medium. Empty for runs without the log.
    linkstate: pd.DataFrame = field(default_factory=pd.DataFrame)

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
    linkstate = (pd.read_parquet(run_dir / "linkstate.parquet")
                 if (run_dir / "linkstate.parquet").exists()
                 else pd.DataFrame())
    return RunData(
        name=run_dir.name, path=run_dir, manifest=manifest, positions=positions,
        packets=packets, agent_events=agent_events,
        config=_config_summary(manifest["scenario"]), linkstate=linkstate)


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


def link_state_truth(run: RunData, window_s: float = 1.0) -> pd.DataFrame:
    """Ground-truth link quality from linkstate.parquet, binned like
    link_quality(). pdr = mean model delivery_prob over the window — defined
    on EVERY link every tick, including idle ones (the whole point of the
    truth log). delivered is synthesized as pdr*attempts so downstream
    weighted aggregations (sum delivered / sum attempts) stay correct.
    Extra columns carry the jamming truth for footprint layers."""
    ls = run.linkstate
    if ls.empty:
        return pd.DataFrame(columns=["bin", "src", "dst", "pdr", "attempts",
                                     "delivered", "rho", "prx_dbm"])
    g = ls.copy()
    g["bin"] = (g["t"] // window_s) * window_s
    out = g.groupby(["bin", "src", "dst"]).agg(
        pdr=("delivery_prob", "mean"), attempts=("delivery_prob", "size"),
        rho=("rho", "mean"), prx_dbm=("prx_dbm", "mean")).reset_index()
    out["delivered"] = out["pdr"] * out["attempts"]
    return out


def link_quality_best(run: RunData, window_s: float = 1.0) -> pd.DataFrame:
    """Truth-based link quality when the run logged linkstate.parquet,
    packet-reconstructed link_quality() otherwise (old runs keep working)."""
    if not run.linkstate.empty:
        return link_state_truth(run, window_s=window_s)
    return link_quality(run, window_s=window_s)


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


def pdr_series(run: RunData, window_s: float = 1.0) -> pd.DataFrame:
    """Network-wide frame PDR per time-window: delivered/attempts over all links.

    Returns t, pdr, attempts (one row per non-empty bin)."""
    lq = link_quality(run, window_s=window_s)
    if lq.empty:
        return pd.DataFrame(columns=["t", "pdr", "attempts"])
    g = lq.groupby("bin").agg(delivered=("delivered", "sum"),
                              attempts=("attempts", "sum")).reset_index()
    g["pdr"] = g["delivered"] / g["attempts"]
    return g.rename(columns={"bin": "t"})[["t", "pdr", "attempts"]]


def update_delivery_series(run: RunData, window_s: float = 2.0) -> pd.DataFrame:
    """Application goodput over time: fraction of *published state updates* that
    actually land at a peer, binned by the receiver's run-time.

    This is the headline metric of the A/B write-ups (it, not raw frame PDR,
    is what UDP+state wins on). A published (src, seq) counts as delivered if any
    other node received it; we attribute the landing to the bin of the first
    receive. Denominator = published updates whose origin time falls in the bin.
    Returns t, delivery, published, delivered."""
    # published updates keyed by (src, seq) -> origin run-time
    pub_t: dict[tuple[str, int], float] = {}
    starts: dict[str, float] = {}
    for nid, events in run.agent_events.items():
        s = next((e["ts_us"] for e in events if e["type"] == "start"), None)
        if s is None:
            continue
        starts[nid] = s
        for e in events:
            if e["type"] == "pub":
                pub_t[(nid, e["seq"])] = (e["ts_us"] - s) / 1e6
    if not pub_t:
        return pd.DataFrame(columns=["t", "delivery", "published", "delivered"])
    # first landing per published update
    landed: set[tuple[str, int]] = set()
    for obs, events in run.agent_events.items():
        for e in events:
            if e["type"] == "recv" and e.get("peer_seq", -1) > 0:
                landed.add((e["from"], e["peer_seq"]))
    rows = []
    for (src, seq), t0 in pub_t.items():
        rows.append({"t": (t0 // window_s) * window_s,
                     "delivered": 1 if (src, seq) in landed else 0})
    df = pd.DataFrame(rows)
    g = df.groupby("t").agg(published=("delivered", "size"),
                            delivered=("delivered", "sum")).reset_index()
    g["delivery"] = g["delivered"] / g["published"]
    return g[["t", "delivery", "published", "delivered"]]


def aoi_mean(run: RunData, observer: str) -> float | None:
    """Time-averaged AoI (s) over all peers at one observer (area under the
    sawtooth / duration). None if no data."""
    df = aoi_series(run, observer)
    if df.empty:
        return None
    dur = run.duration_s or float(df["t"].max() or 1.0)
    means = []
    for _peer, g in df.groupby("peer"):
        g = g.sort_values("t")
        ts = g["t"].to_numpy()
        aoi = g["aoi_s"].to_numpy()
        # piecewise-linear sawtooth: between resets AoI grows at slope 1
        area = 0.0
        prev_t, prev_a = 0.0, aoi[0]
        for t, a in zip(ts, aoi):
            dt = t - prev_t
            if dt > 0:  # AoI grew from prev_a to prev_a+dt, then reset to a
                area += (prev_a + prev_a + dt) / 2 * dt
            prev_t, prev_a = t, a
        # tail to end of run
        dt = max(dur - prev_t, 0.0)
        area += (prev_a + prev_a + dt) / 2 * dt
        means.append(area / dur if dur else prev_a)
    return float(np.mean(means)) if means else None


def map_frames(run: RunData, stride: int = 2, window_s: float = 1.0) -> list[dict]:
    """Per sampled tick: everything the map needs to render one frame.

    Each frame: {t, nodes:[{id,x,y,heading}], links:[{a,b,pdr,attempts}],
    jammers:[{id,x,y,active,kind,power}], sat_up}. Links use the PDR window the
    tick falls in. Self-contained so a static frontend can scrub without joins."""
    pos = run.positions
    if pos.empty:
        return []
    lq = link_quality(run, window_s=window_s)
    roles = run.roles
    times = np.sort(pos["t"].unique())[::max(stride, 1)]
    frames = []
    for t in times:
        snap = pos[np.isclose(pos["t"], t)]
        xy = {r["id"]: (float(r["x"]), float(r["y"])) for _, r in snap.iterrows()}
        nodes = [{"id": r["id"], "x": float(r["x"]), "y": float(r["y"]),
                  "heading": float(r["heading"]),
                  "role": roles.get(r["id"], "vehicle")}
                 for _, r in snap.iterrows()]
        bin_t = (t // window_s) * window_s
        lb = lq[np.isclose(lq["bin"], bin_t)] if not lq.empty else lq
        links, seen = [], set()
        for _, r in lb.iterrows():
            a, b = r["src"], r["dst"]
            if (b, a) in seen or a not in xy or b not in xy:
                continue
            seen.add((a, b))
            links.append({"a": a, "b": b, "pdr": float(r["pdr"]),
                          "attempts": int(r["attempts"])})
        jammers = [{"id": j["id"], "x": float(j["position"][0]),
                    "y": float(j["position"][1]), "kind": j["kind"],
                    "power": float(j["tx_power_dbm"]),
                    "active": bool(run.jammer_active(j, float(t)))}
                   for j in run.jammers]
        frames.append({"t": round(float(t), 3), "nodes": nodes, "links": links,
                       "jammers": jammers, "sat_up": bool(run.sat_up(float(t)))})
    return frames


def export_run(run: RunData, *, stride: int = 2) -> dict:
    """Assemble a single JSON-serialisable bundle describing a run for the
    self-contained HTML frontend. Pulls together geometry, per-frame map state,
    and the metric series so the browser needs no further computation."""
    obs_default = max(run.agent_events, key=lambda k: len(run.agent_events[k]),
                      default=None) if run.agent_events else None
    aoi = {obs: aoi_series(run, obs).to_dict("records")
           for obs in run.agent_events}
    aoi_means = {obs: aoi_mean(run, obs) for obs in run.agent_events}
    pdr = pdr_series(run).to_dict("records")
    upd = update_delivery_series(run).to_dict("records")
    ev = events_timeline(run)
    drops = (run.packets[run.packets["verdict"] != "no_link"]["verdict"]
             .value_counts().to_dict() if not run.packets.empty else {})
    ext = run.extent
    return {
        "name": run.name,
        "config": run.config,
        "scenario_name": run.scenario.get("name"),
        "duration_s": run.duration_s,
        "extent": [float(ext[0]), float(ext[1])],
        "foliage": [{"x_min": float(f["x_min"]), "x_max": float(f["x_max"]),
                     "y_min": float(f["y_min"]), "y_max": float(f["y_max"])}
                    for f in run.foliage],
        "roles": run.roles,
        "frames": map_frames(run, stride=stride),
        "events": ev.to_dict("records") if not ev.empty else [],
        "pdr": pdr,
        "update_delivery": upd,
        "aoi": aoi,
        "aoi_mean": aoi_means,
        "aoi_default_observer": obs_default,
        "drop_attribution": {k: int(v) for k, v in drops.items()},
        "satellite_enabled": bool(
            run.scenario.get("satellite", {}).get("enabled")),
        "jammers": [{"id": j["id"], "kind": j["kind"],
                     "power": float(j["tx_power_dbm"]),
                     "start_s": j["start_s"], "stop_s": j["stop_s"],
                     "x": float(j["position"][0]), "y": float(j["position"][1])}
                    for j in run.jammers],
    }
