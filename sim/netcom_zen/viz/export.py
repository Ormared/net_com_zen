"""Export run(s) into a single self-contained HTML dashboard.

The dashboard is a portable static file: it embeds the run bundles (produced by
``data.export_run``) as JSON plus the inlined CSS/JS, so it opens in any browser
with no server and can be archived inside a run dir as an artifact.

    pixi run viz-export                       # all runs with positions -> out
    python -m netcom_zen.viz.export --out dashboard.html run_a run_b ...

``data.py`` stays the single source of truth; this module only assembles its
output into the frontend template.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path

from . import data as D

WEB = Path(__file__).resolve().parent / "web"


def _json_default(o):
    # numpy / pandas scalars sneak through; coerce to plain python
    try:
        import numpy as np
        if isinstance(o, (np.integer,)):
            return int(o)
        if isinstance(o, (np.floating,)):
            return float(o)
        if isinstance(o, (np.bool_,)):
            return bool(o)
    except Exception:  # pragma: no cover
        pass
    raise TypeError(f"not serialisable: {type(o)}")


def build_html(run_dirs: list[str | Path], *, stride: int = 2,
               title: str | None = None) -> str:
    """Assemble the standalone HTML for the given run dirs (first = default)."""
    runs: dict[str, dict] = {}
    order: list[str] = []
    for rd in run_dirs:
        run = D.load_run(rd)
        bundle = D.export_run(run, stride=stride)
        runs[bundle["name"]] = bundle
        order.append(bundle["name"])
    payload = {"runs": runs, "order": order}
    data_json = json.dumps(payload, default=_json_default, separators=(",", ":"))

    template = (WEB / "template.html").read_text()
    css = (WEB / "app.css").read_text()
    js = (WEB / "app.js").read_text()
    html = template.replace("/*__CSS__*/", css)
    html = html.replace("/*__DATA__*/", data_json)
    html = html.replace("/*__JS__*/", js)
    if title:
        html = html.replace("<title>net_com_zen — swarm comms under EW</title>",
                            f"<title>{title}</title>")
    return html


def discover_playable(results_root: str | Path) -> list[str]:
    """Run paths that have a position track (full map playback), default order."""
    return [r["path"] for r in D.list_runs(results_root) if r["has_positions"]]


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("runs", nargs="*",
                    help="run dirs to embed (default: all playable under "
                         "NCZ_RESULTS / results)")
    ap.add_argument("--out", default="dashboard.html",
                    help="output HTML path (default: dashboard.html)")
    ap.add_argument("--stride", type=int, default=2,
                    help="sample every Nth position tick (default 2)")
    ap.add_argument("--results", default=os.environ.get("NCZ_RESULTS", "results"),
                    help="results root for auto-discovery")
    args = ap.parse_args(argv)

    run_dirs = args.runs or discover_playable(args.results)
    if not run_dirs:
        print(f"no playable runs found under {Path(args.results).resolve()} "
              "(need positions.parquet)")
        return 1
    html = build_html(run_dirs, stride=args.stride)
    out = Path(args.out)
    out.write_text(html)
    kb = len(html.encode()) / 1024
    print(f"wrote {out.resolve()}  ({kb:.0f} KB, {len(run_dirs)} run(s))")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
