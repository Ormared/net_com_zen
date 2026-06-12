"""Age-of-information from agent JSONL metrics (benchmarking.md).

AoI for (observer, peer) at time t = t - timestamp of the freshest peer state
the observer has applied. Between updates it grows linearly; each applied
update drops it to the transport+staleness gap. We integrate the sawtooth
over the run to get the time-averaged AoI.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path


def _events(path: Path):
    for line in path.read_text().splitlines():
        if line.strip():
            yield json.loads(line)


def aoi_for_agent(path: Path) -> dict[str, dict[str, float]]:
    """Per-peer AoI stats for one observer's metrics file."""
    # (recv_wall_ts, freshest_peer_state_ts), both seconds, per peer
    updates: dict[str, list[tuple[float, float]]] = {}
    t_end = None
    for ev in _events(path):
        if ev["type"] == "recv" and ev.get("peer_ts_us", -1) > 0:
            updates.setdefault(ev["from"], []).append(
                (ev["ts_us"] / 1e6, ev["peer_ts_us"] / 1e6))
        elif ev["type"] == "final_state":
            t_end = ev["ts_us"] / 1e6
    out: dict[str, dict[str, float]] = {}
    for peer, evs in updates.items():
        evs.sort()
        # keep only updates that increase freshness (CRDT may re-deliver)
        fresh: list[tuple[float, float]] = []
        best = -1.0
        for t, pts in evs:
            if pts > best:
                best = pts
                fresh.append((t, pts))
        if not fresh:
            continue
        end = t_end if t_end is not None else fresh[-1][0]
        integral = 0.0
        peak = 0.0
        for (t0, p0), (t1, _) in zip(fresh, fresh[1:] + [(end, 0.0)]):
            if t1 <= t0:
                continue
            a0 = t0 - p0
            a1 = t1 - p0
            integral += (t1 - t0) * (a0 + a1) / 2
            peak = max(peak, a1)
        span = max(end - fresh[0][0], 1e-9)
        out[peer] = {"mean_s": integral / span, "max_s": peak,
                     "n_updates": float(len(fresh))}
    return out


def aoi_summary(run_dir: str | Path) -> dict[str, dict[str, dict[str, float]]]:
    """observer -> peer -> stats, for every agent_*.jsonl in a run directory."""
    run_dir = Path(run_dir)
    return {p.stem.removeprefix("agent_"): aoi_for_agent(p)
            for p in sorted(run_dir.glob("agent_*.jsonl"))}


def main() -> None:
    summary = aoi_summary(sys.argv[1])
    for obs, peers in summary.items():
        for peer, s in sorted(peers.items()):
            print(f"{obs} <- {peer}: mean AoI {s['mean_s']*1e3:8.1f} ms   "
                  f"max {s['max_s']*1e3:8.1f} ms   updates {int(s['n_updates'])}")


if __name__ == "__main__":
    main()
