"""DDS / RMW scaling benchmark — scenario generator (Phase B3).

PURPOSE
-------
Generate schema-valid ``substrate=bridge`` scenarios and matching per-N sweep
configs for the RMW × N scaling grid.  This module is the *source of truth* for
what gets benchmarked; the sweep runner (``harness/sweep.py``) and report
(``harness/report.py``) are driven by the files it emits.

FULL RMW × N GRID — HOW TO RUN
-------------------------------
N cannot be a sweep axis because ``apply_override`` only sets scalar values on
an existing dict — it cannot change the *length* of the nodes list.  The
solution: one base scenario YAML per N (with zenoh as the canonical base rmw),
plus one sweep YAML per N that varies only ``ros2.rmw``.  This gives 6
independent sweeps, each producing 3 cells (one per RMW), for a full 6×3=18
grid that the existing sweep.py + report.py handle without modification.

Regenerate the YAMLs any time workload params change::

    PYTHONNOUSERSITE=1 .pixi/envs/default/bin/python \\
        -m netcom_zen.harness.dds_scenarios

Then run the 6 per-N sweeps in Phase C (needs root for netns+bridge)::

    for N in 4 8 16 24 48 96; do
        sudo PYTHONNOUSERSITE=1 .pixi/envs/ros2/bin/python \\
            -m netcom_zen.harness.sweep \\
            scenarios/dds/sweep_n${N}.yaml \\
            -o results/dds/n${N}
    done

Point ``report.py`` at any per-N results dir::

    PYTHONNOUSERSITE=1 .pixi/envs/default/bin/python \\
        -m netcom_zen.harness.report results/dds/n4 --x ros2.rmw

The B2 extension of ``report.py`` adds discovery-time + CPU/mem curves; the
existing frame-PDR / latency path is unchanged for backward compatibility.
"""
from __future__ import annotations

import argparse
from pathlib import Path

import yaml

from ..config import Scenario

# Swarm sizes defined by the benchmark plan (docs/dds-benchmark-plan.md).
# 96 is the deliberate bridge-substrate stress point; 4 is the smoke baseline.
DDS_N_VALUES: list[int] = [4, 8, 16, 24, 48, 96]

# Three RMW implementations under test.
DDS_RMW_VALUES: list[str] = ["zenoh", "fastrtps", "cyclonedds"]

# Minimal radio block — substrate=bridge ignores all RF parameters at runtime,
# but the Scenario schema always requires one (shared with the EW track).
# Values mirror bridge_smoke_4node.yaml so the generated files are consistent
# with the existing manual smoke scenario.
_MINIMAL_RADIO: dict = {
    "freq_hz": 433.0e6,
    "bandwidth_hz": 250.0e3,
    "tx_power_dbm": 27.0,
    "data_rate_bps": 250.0e3,
    "hop": {"n_channels": 50, "hop_rate_hz": 100.0},
}


def bridge_scenario(
    n: int,
    rmw: str,
    *,
    duration_s: float = 30.0,
    period_ms: int = 200,
    payload_bytes: int = 255,
    reliability: str = "reliable",
) -> dict:
    """Return a schema-valid bridge Scenario dict for *n* drones and the given RMW.

    Parameters
    ----------
    n:              Number of drone nodes (d1..dN).  Must be ≤96 (bridge cap).
    rmw:            RMW under test: ``"zenoh"``, ``"fastrtps"``, or
                    ``"cyclonedds"``.
    duration_s:     Scenario wall-clock duration in seconds.
    period_ms:      Telemetry publish interval in ms (sweepable).
    payload_bytes:  Payload size in bytes (sweepable; default ~one state snapshot).
    reliability:    QoS reliability: ``"reliable"`` or ``"best_effort"``.

    Returns
    -------
    dict that passes ``Scenario.model_validate``.  Raises ``ValidationError``
    if the arguments are out of range (e.g. n > 96, bad rmw string).

    Notes
    -----
    * ``substrate=bridge`` → no RF model, no jammer, no mobility in the engine.
    * The ``radio`` block is schema-required but completely ignored at runtime on
      the bridge substrate; values are set to the same defaults as the smoke
      scenario so they are consistent if inspected.
    * Each node gets a single static waypoint ``[0, 0]`` — schema requires ≥1.
    * No jammers: this is a pure-transport benchmark, not an EW study.
    """
    d: dict = {
        "name": f"bridge-n{n}-{rmw}",
        "duration_s": duration_s,
        "seed": 0,
        "substrate": "bridge",
        # radio is ignored by the bridge engine but must be present for schema validity
        "radio": _MINIMAL_RADIO,
        # All nodes at the origin — position is irrelevant on the bridge substrate.
        # waypoints needs ≥1 entry per schema; [[0, 0]] is the canonical no-op.
        "nodes": [{"id": f"d{i}", "waypoints": [[0, 0]]} for i in range(1, n + 1)],
        "workload": "ros2",
        "ros2": {
            "rmw": rmw,
            "period_ms": period_ms,
            "payload_bytes": payload_bytes,
            "reliability": reliability,
        },
    }
    # Validate immediately: raises pydantic.ValidationError on bad args so the
    # caller (or test) sees a clear error rather than a silent bad dict.
    Scenario.model_validate(d)
    return d


def _sweep_dict(n: int) -> dict:
    """Sweep config that varies only ros2.rmw over the bridge_n{n} base scenario.

    Why not a single sweep with N as an axis?  ``harness/sweep.py``'s
    ``apply_override`` sets scalar values on an existing dict; it cannot change
    the *length* of the nodes list.  Baking N into the base scenario and sweeping
    only the RMW is the clean solution: 6 per-N sweeps × 3 RMWs = 18 cells,
    all driven by the existing sweep.py + report.py without any code changes.

    Seeds: a single seed (0) is sufficient here because the bridge substrate has
    no stochastic RF model — every cell with the same RMW+N is deterministic.
    """
    return {
        "base": f"bridge_n{n}.yaml",
        "axes": {"ros2.rmw": DDS_RMW_VALUES},
        "seeds": [0],
    }


def generate_all(out_dir: Path) -> list[Path]:
    """Write bridge_n{N}.yaml and sweep_n{N}.yaml for every N in DDS_N_VALUES.

    Both files land in *out_dir*.  The sweep yaml references the base via a
    relative path (just the filename), matching how ``run_sweep`` resolves it:
    ``(sweep_path.parent / sweep["base"])``.

    Returns
    -------
    Sorted list of paths written.
    """
    out_dir.mkdir(parents=True, exist_ok=True)
    written: list[Path] = []

    for n in DDS_N_VALUES:
        # Base scenario: zenoh as the canonical rmw.  All three RMWs are equally
        # valid; zenoh is already wired in the engine so it smoke-runs first
        # without needing the B1 per-RMW spawn strategies.
        base_dict = bridge_scenario(n, "zenoh")
        base_path = out_dir / f"bridge_n{n}.yaml"
        base_path.write_text(
            "# AUTO-GENERATED by netcom_zen.harness.dds_scenarios — do not edit\n"
            "# Regenerate: python -m netcom_zen.harness.dds_scenarios\n"
            + yaml.dump(base_dict, default_flow_style=False, sort_keys=False)
        )
        written.append(base_path)

        sweep_path = out_dir / f"sweep_n{n}.yaml"
        sweep_path.write_text(
            "# AUTO-GENERATED by netcom_zen.harness.dds_scenarios — do not edit\n"
            "# Regenerate: python -m netcom_zen.harness.dds_scenarios\n"
            + yaml.dump(_sweep_dict(n), default_flow_style=False, sort_keys=False)
        )
        written.append(sweep_path)

    return sorted(written)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument(
        "--out",
        default=None,
        help="output directory (default: scenarios/dds/ relative to cwd)",
    )
    args = ap.parse_args()
    out = Path(args.out) if args.out else Path("scenarios/dds")
    written = generate_all(out)
    for p in written:
        print(p)


if __name__ == "__main__":
    main()
