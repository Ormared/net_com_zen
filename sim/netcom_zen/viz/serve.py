"""Build the standalone dashboard and serve it on localhost.

    pixi run dashboard                 # builds from NCZ_RESULTS, serves :8501

This is a thin convenience wrapper: it calls the static exporter (the dashboard
itself is a single self-contained HTML — see export.py) and serves it so the
``pixi run dashboard`` workflow is unchanged. The generated file is fully
portable; you can also just open it from disk.
"""
from __future__ import annotations

import argparse
import http.server
import os
import socketserver
import tempfile
import webbrowser
from pathlib import Path

from . import export


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--port", type=int,
                    default=int(os.environ.get("NCZ_PORT", "8501")))
    ap.add_argument("--results", default=os.environ.get("NCZ_RESULTS", "results"))
    ap.add_argument("--stride", type=int, default=2)
    ap.add_argument("--no-browser", action="store_true")
    args = ap.parse_args(argv)

    run_dirs = export.discover_playable(args.results)
    if not run_dirs:
        print(f"no playable runs under {Path(args.results).resolve()} "
              "(need positions.parquet). Run a scenario first.")
        return 1
    html = export.build_html(run_dirs, stride=args.stride)

    tmp = Path(tempfile.mkdtemp(prefix="ncz_dash_"))
    (tmp / "index.html").write_text(html)
    os.chdir(tmp)

    class Handler(http.server.SimpleHTTPRequestHandler):
        def log_message(self, *a):  # quiet
            pass

    url = f"http://localhost:{args.port}/"
    print(f"net_com_zen dashboard · {len(run_dirs)} run(s) · {url}")
    print("  (single self-contained HTML; Ctrl-C to stop)")
    if not args.no_browser:
        try:
            webbrowser.open(url)
        except Exception:
            pass
    with socketserver.TCPServer(("", args.port), Handler) as httpd:
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nbye")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
