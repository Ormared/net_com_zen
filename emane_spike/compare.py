"""Run the matching hand-rolled cell and tabulate it against the EMANE cell.

Both engines run the same scenario (resilience_4node), same jammer (barrage
40 dBm at t=20 s), same duration, same Rust agent binary, and the EMANE side is
fed the same CompositePathloss. The only difference is the channel engine:
hand-rolled FSK/SINR vs EMANE RF Pipe + spectrum monitor.

    sudo .../python emane_spike/compare.py --emane results/emane_jam40 \
         --duration 40 -o results/handrolled_jam40
"""
from __future__ import annotations

import argparse
import asyncio
import glob
import json
import statistics
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "sim"))

from netcom_zen.config import load_scenario  # noqa: E402
from netcom_zen.harness.aoi import aoi_for_agent  # noqa: E402
from netcom_zen.orchestrator import ScenarioEngine  # noqa: E402


def cell_metrics(run_dir: Path) -> dict:
    """Per-engine summary from agent_*.jsonl (works for both engines)."""
    nodes = {}
    for f in sorted(glob.glob(str(run_dir / "agent_*.jsonl"))):
        nid = Path(f).stem.removeprefix("agent_")
        events = [json.loads(l) for l in open(f) if l.strip()]
        mls = next((e for e in events if e["type"] == "mls_ready"), None)
        pubs = sum(1 for e in events if e["type"] == "pub")
        recvs = sum(1 for e in events if e["type"] == "recv")
        aoi = aoi_for_agent(Path(f))
        aoi_means = [s["mean_s"] for s in aoi.values()]
        nodes[nid] = {
            "mls": bool(mls),
            "mls_ms": mls["handshake_ms"] if mls else None,
            "pubs": pubs, "recvs": recvs,
            "peers_seen": len(aoi),
            "mean_aoi_s": statistics.mean(aoi_means) if aoi_means else None,
        }
    return nodes


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--emane", required=True, help="EMANE result dir")
    ap.add_argument("--duration", type=float, default=40)
    ap.add_argument("-o", "--out", required=True, help="hand-rolled result dir")
    args = ap.parse_args()

    scenario = load_scenario("scenarios/resilience_4node.yaml").model_copy(
        update={"duration_s": args.duration})
    out = Path(args.out)
    print(f"running hand-rolled cell ({args.duration}s)...")
    asyncio.run(ScenarioEngine(scenario, out).run())

    hand = cell_metrics(out)
    emane = cell_metrics(Path(args.emane))

    print("\n=== EMANE vs hand-rolled (same scenario, jammer, agent, pathloss) ===")
    hdr = f"{'node':>5} | {'engine':>11} | {'MLS':>4} {'hs ms':>6} | " \
          f"{'pubs':>5} {'recvs':>6} {'peers':>5} {'mean AoI s':>10}"
    print(hdr)
    print("-" * len(hdr))
    for nid in sorted(set(hand) | set(emane)):
        for label, d in (("EMANE", emane.get(nid)), ("hand-rolled", hand.get(nid))):
            if d is None:
                print(f"{nid:>5} | {label:>11} | {'--':>4}")
                continue
            aoi = f"{d['mean_aoi_s']:.2f}" if d['mean_aoi_s'] is not None else "-"
            print(f"{nid:>5} | {label:>11} | {'y' if d['mls'] else 'N':>4} "
                  f"{d['mls_ms'] or '-':>6} | {d['pubs']:>5} {d['recvs']:>6} "
                  f"{d['peers_seen']:>5} {aoi:>10}")

    (Path(args.out) / "comparison.json").write_text(json.dumps(
        {"emane": emane, "handrolled": hand}, indent=2))
    print(f"\nwrote {args.out}/comparison.json")


if __name__ == "__main__":
    main()
