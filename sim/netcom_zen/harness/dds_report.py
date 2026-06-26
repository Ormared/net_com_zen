"""Cross-N scaling report for the DDS/RMW benchmark.

Each `results/dds/n<N>` sweep dir holds one RMW comparison at a fixed swarm
size N (3 cells: zenoh / fastrtps / cyclonedds). The headline of the study is
the *scaling curve* — a metric vs N with one line per RMW — which needs the
per-N summaries merged. This module globs the n<N> dirs, reuses report.py's
bridge metrics + plotting (x=n_nodes, series=ros2.rmw), and emits a markdown
table for the write-up.

    python -m netcom_zen.harness.dds_report results/dds
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

from . import report

# headline metrics: (key, column label, lower-is-better)
METRICS = [
    ("discovery_time_s", "discovery_s", True),
    ("latency_p50_ms", "p50_ms", True),
    ("latency_p99_ms", "p99_ms", True),
    ("delivery_ratio", "delivery", False),
    ("mesh_completeness", "mesh", False),
    ("rss_peak_mb", "rss_mb", True),
    ("cpu_mean_pct", "cpu%", True),
]


def collect(root: Path) -> list[dict]:
    """All bridge-run rows across every n<N> sweep dir, tagged with n_nodes."""
    rows: list[dict] = []
    for d in sorted(root.glob("n*"),
                    key=lambda p: int(p.name[1:]) if p.name[1:].isdigit() else 0):
        if (d / "sweep_manifest.json").exists():
            rows += report.aggregate(d)
    return rows


def markdown_table(rows: list[dict]) -> str:
    rmws = sorted({r.get("ros2.rmw") for r in rows})
    ns = sorted({r["n_nodes"] for r in rows if r.get("n_nodes") is not None})
    hdr = "| N | RMW | " + " | ".join(lbl for _, lbl, _ in METRICS) + " |"
    sep = "|---" * (len(METRICS) + 2) + "|"
    lines = [hdr, sep]
    by = {(r["n_nodes"], r.get("ros2.rmw")): r for r in rows}
    for n in ns:
        for rmw in rmws:
            r = by.get((n, rmw))
            if not r:
                continue
            cells = []
            for key, _, _ in METRICS:
                v = r.get(key)
                cells.append("—" if v is None else
                             (f"{v:.3f}" if abs(v) < 100 else f"{v:.0f}"))
            lines.append(f"| {n} | {rmw} | " + " | ".join(cells) + " |")
    return "\n".join(lines)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("root", nargs="?", default="results/dds")
    args = ap.parse_args()
    root = Path(args.root)
    rows = collect(root)
    if not rows:
        print(f"no completed sweep dirs under {root}")
        return
    (root / "summary_all.json").write_text(json.dumps(rows, indent=2))
    # scaling curves: metric vs N, one line per RMW
    written = report.plot_curves(rows, root, x_axis="n_nodes",
                                 series_axis="ros2.rmw")
    table = markdown_table(rows)
    (root / "scaling_table.md").write_text(table + "\n")
    print(f"{len(rows)} cells aggregated -> {root/'summary_all.json'}")
    for p in written:
        print(f"wrote {p}")
    print(f"wrote {root/'scaling_table.md'}\n")
    print(table)


if __name__ == "__main__":
    main()
