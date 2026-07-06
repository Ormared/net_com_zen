"""DDS QoS-plane characterization lab (the "understand the whole plane" study).

This is the driver for the systematic exploration of how each RMW (Fast DDS,
Cyclone, Zenoh) behaves across the full DDS QoS + transport-buffer space, with
the single goal of reaching a HIGH number of participants on the kernel-bridge
substrate. Swarm/EW realism is deliberately out of scope here — every node is a
bare telemetry pub/sub and we move QoS knobs to find what scales.

DESIGN
------
Running a full factorial of every QoS policy × every level × 3 RMW × several N
is combinatorially hopeless. The study is staged instead:

* ``screen``  — one-factor-at-a-time (OFAT) from a fixed baseline at the stress
                point N=96. Identifies which knobs actually move mesh/delivery.
* ``buffer``  — sweep the socket/kernel buffer size at N=96 (the explicit
                "increase the buffer" hypothesis).
* ``factorial``— full factorial over the handful of knobs ``screen`` flags as
                active, at N=48 and N=96.
* ``ceiling`` — best config per RMW pushed past 96 to find each one's wall.

Each cell is one ScenarioEngine run on substrate=bridge; results land under
``results/dds/qos/<phase>/<cell>/`` and are aggregated with
``netcom_zen.harness.report.bridge_run_metrics``. Needs root (netns + bridge +
the /proc/sys buffer raise): ::

    sudo .pixi/envs/ros2/bin/python -m netcom_zen.harness.dds_qos_lab screen
    sudo .pixi/envs/ros2/bin/python -m netcom_zen.harness.dds_qos_lab buffer
    # ... factorial / ceiling (see CELLS_* below)

Then aggregate any phase dir to a table::

    .pixi/envs/default/bin/python -m netcom_zen.harness.dds_qos_lab report \\
        results/dds/qos/screen
"""
from __future__ import annotations

import argparse
import asyncio
import itertools
import json
from pathlib import Path

from ..config import Scenario
from .dds_scenarios import _MINIMAL_RADIO

# Stock-ROS2 baseline: the exact config the original scaling study used, so a
# baseline cell here reproduces those numbers and every OFAT delta is one knob.
BASELINE_QOS: dict = {
    "reliability": "reliable",
    "durability": "volatile",
    "history": "keep_last",
    "depth": 10,
    "deadline_ms": 0.0,
    "lifespan_ms": 0.0,
    "liveliness": "automatic",
    "liveliness_lease_ms": 0.0,
    "socket_buffer_bytes": 0,
}

RMWS = ("fastrtps", "cyclonedds", "zenoh")
_MB = 1 << 20


def make_scenario(n: int, rmw: str, *, duration_s: float = 30.0,
                  **qos_over) -> dict:
    """Schema-valid bridge scenario for *n* nodes, *rmw*, baseline QoS + overrides.

    Mirrors ``dds_scenarios.bridge_scenario`` but threads the full QoS block so
    the lab can move any policy. Validates immediately (raises on a bad combo)."""
    ros2 = {"rmw": rmw, "period_ms": 200, "payload_bytes": 255,
            **BASELINE_QOS, **qos_over}
    d = {
        "name": f"qos-n{n}-{rmw}",
        "duration_s": duration_s,
        "seed": 0,
        "substrate": "bridge",
        "radio": _MINIMAL_RADIO,
        "nodes": [{"id": f"d{i}", "waypoints": [[0, 0]]}
                  for i in range(1, n + 1)],
        "workload": "ros2",
        "ros2": ros2,
    }
    Scenario.model_validate(d)
    return d


# ── Phase: screen — OFAT at N=96 ───────────────────────────────────────────
# (label, {override}). The baseline cell is the all-defaults control. Each other
# cell flips exactly one policy away from BASELINE_QOS.
SCREEN_CELLS: list[tuple[str, dict]] = [
    ("baseline", {}),
    ("best_effort", {"reliability": "best_effort"}),
    ("depth1", {"depth": 1}),
    ("keep_all", {"history": "keep_all"}),
    ("transient_local", {"durability": "transient_local"}),
    ("lifespan500ms", {"lifespan_ms": 500.0}),
    ("deadline1s", {"deadline_ms": 1000.0}),
    ("manual_liveliness", {"liveliness": "manual_by_topic",
                           "liveliness_lease_ms": 2000.0}),
    ("buf16m", {"socket_buffer_bytes": 16 * _MB}),
]

# ── Phase: buffer — buffer-size sweep at N=96 (DDS only; zenoh is TCP) ──────
BUFFER_SIZES = [0, 4 * _MB, 16 * _MB, 64 * _MB]

# ── Phase: factorial — filled in from screen findings (reliability × buffer ×
# history are the a-priori actives); 2×2×2 per RMW per N. Adjust after screen.
FACTORIAL_AXES: dict[str, list] = {
    "reliability": ["reliable", "best_effort"],
    "socket_buffer_bytes": [0, 16 * _MB],
    "history": ["keep_last", "keep_all"],
}

# ── Phase: window — the rate-law confirmation ───────────────────────────────
# Screening showed connected-pair COUNT is fixed per (stack,QoS) regardless of N
# (Fast DDS: 1022 pairs @N=48, 1021 @N=96) -> establishment is rate-limited and
# the 30s window is the binding constraint. Prediction: mesh = rate*T/N(N-1), so
# longer T should grow mesh ~linearly until full. Fast DDS @34/s needs ~268s for
# a full 96-mesh. Per-RMW windows (300s only where it's worth the wall-clock).
WINDOW_PLAN: dict[str, list[float]] = {
    "fastrtps": [120.0, 300.0],   # the decisive test: 0.45 @120s, ~full @300s?
    "cyclonedds": [120.0],        # rate-limited, or does it plateau (thrash)?
    "zenoh": [120.0],
}

# ── Phase: beststack — each RMW's winning knobs combined (max rate) ──────────
# From screening: zenoh's rate-lifters stack-test (best_effort+keep_all+manual
# liveliness+buffer); cyclone only the buffer helped; fast DDS nothing did.
BESTSTACK: dict[str, dict] = {
    "zenoh": {"reliability": "best_effort", "history": "keep_all",
              "liveliness": "manual_by_topic", "liveliness_lease_ms": 2000.0,
              "socket_buffer_bytes": 16 * _MB},
    "cyclonedds": {"socket_buffer_bytes": 64 * _MB},
    "fastrtps": {"socket_buffer_bytes": 16 * _MB},  # control: still inert?
}

# ── Phase: ceiling — best config per RMW pushed past 96 ─────────────────────
CEILING_N = [128, 192]

# ── Phase: alloc — Fast DDS allocation-limit probe (docs/dds-topology-plan.md
# P1). The ~33-participant clique cap is invariant to QoS/buffer/time/N; the
# last config hypothesis is that discovery preallocation limits bind. fastrtps
# only; Fast DDS is CV~0% so 2 reps suffice.
ALLOC_SIZES = [0, 64, 128, 256]   # 0 = stock control (env route, no XML)
ALLOC_REPS = 2


async def _run_cell(n: int, rmw: str, label: str, overrides: dict,
                    out_root: Path, duration_s: float) -> dict:
    from ..orchestrator import ScenarioEngine
    from . import report
    cell_dir = out_root / f"{rmw}__{label}"
    if (cell_dir / "manifest.json").exists():
        print(f"  {rmw}/{label} — exists, skipping")
    else:
        print(f"  {rmw}/{label} (n={n}, {duration_s:.0f}s) {overrides}")
        scn = Scenario.model_validate(
            make_scenario(n, rmw, duration_s=duration_s, **overrides))
        await ScenarioEngine(scn, cell_dir).run()
    m = report.bridge_run_metrics(cell_dir)
    mani = json.loads((cell_dir / "manifest.json").read_text())
    mesh = m.get("mesh_completeness")
    # The unifying metric of this study: connected (receiver,sender) pairs and
    # the rate at which they were established over the window. mesh = rate*T /
    # N(N-1), so the *rate* is the N- and window-independent characteristic of a
    # stack+QoS, while mesh-at-30s is just a slice of it.
    pairs = mesh * n * (n - 1) if isinstance(mesh, (int, float)) else None
    rate = pairs / duration_s if pairs is not None else None
    return {"rmw": rmw, "label": label, "n": n, "duration_s": duration_s,
            "overrides": overrides,
            "mesh": mesh, "pairs": pairs, "rate_pairs_s": rate,
            "delivery": m.get("delivery_ratio"),
            "discovery_s": m.get("discovery_time_s"),
            "p50_ms": m.get("latency_p50_ms"),
            "p99_ms": m.get("latency_p99_ms"),
            "rss_mb": m.get("rss_peak_mb"),
            "min_host_mem_avail_gb": round(
                mani.get("min_host_mem_avail_bytes", 0) / 1e9, 1),
            "exits_ok": set(mani.get("agent_exit_codes", {}).values()) <= {0}}


async def run_phase(phase: str, out_root: Path, rmws: tuple[str, ...]) -> None:
    out_root.mkdir(parents=True, exist_ok=True)
    rows: list[dict] = []
    if phase == "screen":
        for rmw in rmws:
            for label, ov in SCREEN_CELLS:
                rows.append(await _run_cell(96, rmw, label, ov, out_root, 30.0))
    elif phase == "buffer":
        for rmw in rmws:
            for b in BUFFER_SIZES:
                rows.append(await _run_cell(
                    96, rmw, f"buf{b // _MB}m", {"socket_buffer_bytes": b},
                    out_root, 30.0))
    elif phase == "window":
        for rmw in rmws:
            for dur in WINDOW_PLAN.get(rmw, [120.0]):
                rows.append(await _run_cell(
                    96, rmw, f"dur{int(dur)}s", {}, out_root, dur))
    elif phase == "replicate":
        # The buffer/beststack phases exposed large run-to-run variance (zenoh
        # baseline swung 0.146<->0.365 on identical config). Replicate baseline
        # x5 per RMW to get a variance band, plus zenoh's manual-liveliness knob
        # x5 to test whether that "2.1x" effect survives the noise floor.
        reps = 5
        for rmw in rmws:
            for r in range(reps):
                rows.append(await _run_cell(
                    96, rmw, f"baseline_r{r}", {}, out_root, 30.0))
        for r in range(reps):
            rows.append(await _run_cell(
                96, "zenoh", f"manualliv_r{r}",
                {"liveliness": "manual_by_topic", "liveliness_lease_ms": 2000.0},
                out_root, 30.0))
    elif phase == "stagger":
        # test whether staggering participant joins breaks the Fast DDS ~33-node
        # discovery ceiling (simultaneous-startup SPDP collapse hypothesis).
        # duration must outlast the stagger so all nodes overlap for a real
        # observation window: dur = stagger_total + 60s.
        for rmw in ("fastrtps", "cyclonedds"):
            for st in (50.0, 200.0):
                dur = round(st / 1e3 * 96 + 60.0, 0)
                rows.append(await _run_cell(
                    96, rmw, f"stagger{int(st)}ms", {"spawn_stagger_ms": st},
                    out_root, dur))
    elif phase == "beststack":
        for rmw in rmws:
            # give the stack room (120s) so a higher rate shows as higher mesh
            rows.append(await _run_cell(
                96, rmw, "beststack", BESTSTACK.get(rmw, {}), out_root, 120.0))
    elif phase == "ceiling":
        # push baseline past 96 to find each RMW's wall. For Fast DDS the test is
        # whether connected-pair COUNT stays ~1022 (absolute ~33-clique cap) as N
        # grows -> mesh would fall as 1022/N(N-1). 45s window.
        for rmw in rmws:
            for n in CEILING_N:
                rows.append(await _run_cell(
                    n, rmw, f"n{n}", {}, out_root, 45.0))
    elif phase == "factorial":
        keys = list(FACTORIAL_AXES)
        for rmw in rmws:
            for n in (48, 96):
                for combo in itertools.product(*FACTORIAL_AXES.values()):
                    ov = dict(zip(keys, combo))
                    label = f"n{n}__" + "_".join(
                        f"{k.split('_')[0]}={v}" for k, v in ov.items())
                    rows.append(await _run_cell(n, rmw, label, ov, out_root, 30.0))
    elif phase == "alloc":
        # allocation-limit probe: fastrtps only; a=0 is the stock control that
        # takes the env-var route (no XML), a>0 preallocates discovery resources.
        # 60s window (2× baseline) gives the rate law room to show a difference.
        for a in ALLOC_SIZES:
            ov = {} if a == 0 else {"fastdds_allocation_participants": a}
            for r in range(1, ALLOC_REPS + 1):
                rows.append(await _run_cell(
                    96, "fastrtps", f"alloc{a}_r{r}", ov, out_root, 60.0))
    else:
        raise SystemExit(f"unknown phase {phase!r}")
    _write_summary(out_root, rows)


def _write_summary(out_root: Path, rows: list[dict]) -> None:
    (out_root / "summary.json").write_text(json.dumps(rows, indent=2, default=str))
    hdr = ("| rmw | label | n | T(s) | mesh | pairs | rate(/s) | delivery | "
           "disc(s) | p50 | p99 | rss(MB) | free(GB) | ok |")
    sep = "|" + "---|" * 14
    lines = [hdr, sep]
    for r in rows:
        def f(x, p=3):
            return f"{x:.{p}f}" if isinstance(x, (int, float)) else str(x)
        lines.append(
            f"| {r['rmw']} | {r['label']} | {r['n']} | "
            f"{f(r.get('duration_s'),0)} | {f(r['mesh'])} | {f(r.get('pairs'),0)} | "
            f"{f(r.get('rate_pairs_s'),1)} | {f(r['delivery'])} | "
            f"{f(r['discovery_s'],2)} | {f(r['p50_ms'],2)} | {f(r['p99_ms'],2)} | "
            f"{f(r['rss_mb'],0)} | {r['min_host_mem_avail_gb']} | "
            f"{'Y' if r['exits_ok'] else 'N'} |")
    (out_root / "table.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


def _report_existing(out_root: Path, rmws: tuple[str, ...]) -> None:
    """Re-aggregate whatever cells already exist under out_root (no runs)."""
    from . import report
    rows = []
    for cell_dir in sorted(out_root.glob("*__*")):
        if not (cell_dir / "manifest.json").exists():
            continue
        rmw, label = cell_dir.name.split("__", 1)
        m = report.bridge_run_metrics(cell_dir)
        mani = json.loads((cell_dir / "manifest.json").read_text())
        n = mani.get("n_nodes")
        dur = mani.get("scenario", {}).get("duration_s")
        mesh = m.get("mesh_completeness")
        pairs = mesh * n * (n - 1) if isinstance(mesh, (int, float)) and n else None
        rate = pairs / dur if pairs is not None and dur else None
        rows.append({
            "rmw": rmw, "label": label, "n": n, "duration_s": dur,
            "overrides": {}, "mesh": mesh, "pairs": pairs, "rate_pairs_s": rate,
            "delivery": m.get("delivery_ratio"),
            "discovery_s": m.get("discovery_time_s"),
            "p50_ms": m.get("latency_p50_ms"), "p99_ms": m.get("latency_p99_ms"),
            "rss_mb": m.get("rss_peak_mb"),
            "min_host_mem_avail_gb": round(
                mani.get("min_host_mem_avail_bytes", 0) / 1e9, 1),
            "exits_ok": set(mani.get("agent_exit_codes", {}).values()) <= {0}})
    _write_summary(out_root, rows)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("phase", choices=["screen", "buffer", "window", "stagger",
                                      "replicate", "beststack", "ceiling",
                                      "factorial", "alloc", "report"])
    ap.add_argument("path", nargs="?", help="for 'report': the phase dir to aggregate")
    ap.add_argument("--out", default="results/dds/qos",
                    help="root for phase output dirs")
    ap.add_argument("--rmws", default=",".join(RMWS),
                    help="comma-separated subset of rmws to run")
    args = ap.parse_args()
    rmws = tuple(args.rmws.split(","))
    if args.phase == "report":
        _report_existing(Path(args.path), rmws)
        return
    out_root = Path(args.out) / args.phase
    asyncio.run(run_phase(args.phase, out_root, rmws))


if __name__ == "__main__":
    main()
