"""Aggregate sweep results into per-run metrics and resilience-curve plots.

    python -m netcom_zen.harness.report results/<sweep-dir>

Produces <sweep-dir>/summary.json and PNG curves (metric vs the x axis,
one line per series axis), aggregated mean over seeds.
"""
from __future__ import annotations

import argparse
import collections
import json
import statistics
from pathlib import Path

import pyarrow.parquet as pq

from .aoi import aoi_summary


def run_metrics(run_dir: Path) -> dict:
    """Channel + application metrics for one run directory."""
    t = pq.read_table(run_dir / "packets.parquet").to_pylist()
    verdicts = collections.Counter(r["verdict"] for r in t)
    relevant = sum(n for v, n in verdicts.items() if v != "no_link")
    delivered = verdicts.get("delivered", 0)
    delays = sorted(r["delay_s"] for r in t if r["verdict"] == "delivered")

    # pre/post-jam split: whole-run averages dilute the jamming effect
    manifest = json.loads((run_dir / "manifest.json").read_text())
    jam_start = min((j["start_s"] for j in manifest["scenario"]["jammers"]),
                    default=None)
    pdr_phase = {}
    if jam_start is not None:
        for phase, sel in (("prejam", lambda r: r["t"] < jam_start),
                           ("postjam", lambda r: r["t"] >= jam_start)):
            ph = [r for r in t if sel(r) and r["verdict"] != "no_link"]
            dlv = sum(1 for r in ph if r["verdict"] == "delivered")
            pdr_phase[f"frame_pdr_{phase}"] = dlv / len(ph) if ph else None

    aoi = aoi_summary(run_dir)
    aoi_means = [s["mean_s"] for peers in aoi.values() for s in peers.values()]
    pair_count = sum(len(peers) for peers in aoi.values())

    # app-level delivery: fresh updates applied vs published, per observer pair
    pubs = {}
    for f in run_dir.glob("agent_*.jsonl"):
        nid = f.stem.removeprefix("agent_")
        pubs[nid] = sum(1 for line in f.read_text().splitlines()
                        if '"type":"pub"' in line)
    updates = sum(s["n_updates"] for peers in aoi.values() for s in peers.values())
    offered = sum(pubs.get(p, 0) for obs, peers in aoi.items() for p in peers)

    return {
        "frame_pdr": delivered / relevant if relevant else None,
        **pdr_phase,
        "drop_counts": {v: n for v, n in verdicts.items() if v != "delivered"},
        "median_delay_ms": delays[len(delays) // 2] * 1e3 if delays else None,
        "mean_aoi_s": statistics.mean(aoi_means) if aoi_means else None,
        "max_aoi_s": max((s["max_s"] for peers in aoi.values()
                          for s in peers.values()), default=None),
        "pairs_connected": pair_count,
        "update_delivery": updates / offered if offered else None,
    }


def aggregate(sweep_dir: Path) -> list[dict]:
    manifest = json.loads((sweep_dir / "sweep_manifest.json").read_text())
    rows = []
    for cell in manifest["cells"]:
        run_dir = sweep_dir / cell["name"]
        if not (run_dir / "packets.parquet").exists():
            continue
        rows.append({**cell["overrides"], "seed": cell["seed"],
                     **run_metrics(run_dir)})
    return rows


def plot_curves(rows: list[dict], sweep_dir: Path, x_axis: str,
                series_axis: str | None) -> list[Path]:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    written = []
    for metric, label in [("frame_pdr", "Frame PDR"),
                          ("frame_pdr_postjam", "Frame PDR (jammer active)"),
                          ("update_delivery", "State-update delivery ratio"),
                          ("mean_aoi_s", "Mean age of information (s)"),
                          ("median_delay_ms", "Median delivery latency (ms)")]:
        fig, ax = plt.subplots(figsize=(7, 4.5))
        series_vals = sorted({r.get(series_axis) for r in rows}) if series_axis else [None]
        for sv in series_vals:
            pts = collections.defaultdict(list)
            for r in rows:
                if (series_axis is None or r.get(series_axis) == sv) \
                        and r.get(metric) is not None:
                    pts[r[x_axis]].append(r[metric])
            if not pts:
                continue
            xs = sorted(pts)
            means = [statistics.mean(pts[x]) for x in xs]
            err = [statistics.stdev(pts[x]) if len(pts[x]) > 1 else 0 for x in xs]
            lbl = f"{series_axis.split('.')[-1]}={sv}" if series_axis else None
            ax.errorbar(xs, means, yerr=err, marker="o", capsize=3, label=lbl)
        ax.set_xlabel(x_axis.split(".")[-1])
        ax.set_ylabel(label)
        ax.grid(True, alpha=0.3)
        if series_axis:
            ax.legend()
        fig.tight_layout()
        out = sweep_dir / f"curve_{metric}.png"
        fig.savefig(out, dpi=130)
        plt.close(fig)
        written.append(out)
    return written


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("sweep_dir")
    ap.add_argument("--x", default=None, help="x axis (default: first sweep axis)")
    ap.add_argument("--series", default=None,
                    help="series axis (default: second sweep axis, if any)")
    args = ap.parse_args()
    sweep_dir = Path(args.sweep_dir)
    manifest = json.loads((sweep_dir / "sweep_manifest.json").read_text())
    # default plot axes: the sweep axes that actually vary
    axes = [a for a, vals in manifest["sweep"].get("axes", {}).items()
            if len(set(map(str, vals))) > 1]
    x_axis = args.x or (axes[0] if axes else "seed")
    series_axis = args.series or (axes[1] if len(axes) > 1 else None)

    rows = aggregate(sweep_dir)
    (sweep_dir / "summary.json").write_text(json.dumps(rows, indent=2))
    print(f"{len(rows)} runs aggregated -> {sweep_dir/'summary.json'}")
    for p in plot_curves(rows, sweep_dir, x_axis, series_axis):
        print(f"wrote {p}")


if __name__ == "__main__":
    main()
