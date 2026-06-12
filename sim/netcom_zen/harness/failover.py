"""Failover-time metric (M4): satellite outage -> swarm state reaches command
over the RF mesh.

For each vehicle v, failover time at command = (wall time command applies the
first v-state update whose payload timestamp is at/after the outage start) minus
the outage start. The max over vehicles is the time for the *whole* swarm to be
re-observed by command on the fallback path.

Reads command's agent_<command>.jsonl recv events (peer_ts_us = origin time of
the applied state) and the outage start from the run manifest.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path


def outage_start_s(run_dir: Path) -> float | None:
    manifest = json.loads((run_dir / "manifest.json").read_text())
    outages = manifest["scenario"].get("satellite", {}).get("outages", [])
    return min((o["start_s"] for o in outages), default=None)


def failover_times(run_dir: Path, command_id: str) -> dict:
    """Per-vehicle failover seconds + the swarm-wide max."""
    run_dir = Path(run_dir)
    t0 = outage_start_s(run_dir)
    if t0 is None:
        return {"error": "no satellite outage in scenario"}
    # command's clock zero: its start event ts; convert peer_ts/recv to run-seconds
    cmd_log = run_dir / f"agent_{command_id}.jsonl"
    events = [json.loads(l) for l in cmd_log.read_text().splitlines() if l.strip()]
    start_us = next((e["ts_us"] for e in events if e["type"] == "start"), None)
    if start_us is None:
        return {"error": "command has no start event"}

    # first post-outage update applied per vehicle (peer origin time >= outage)
    outage_us = start_us + t0 * 1e6
    first_recover: dict[str, float] = {}
    for e in events:
        if e["type"] != "recv":
            continue
        if e.get("peer_ts_us", -1) >= outage_us:
            v = e["from"]
            recover_s = (e["ts_us"] - outage_us) / 1e6
            if v not in first_recover or recover_s < first_recover[v]:
                first_recover[v] = recover_s
    per_vehicle = {v: round(s, 3) for v, s in sorted(first_recover.items())}
    return {
        "outage_start_s": t0,
        "per_vehicle_failover_s": per_vehicle,
        "swarm_failover_s": round(max(per_vehicle.values()), 3) if per_vehicle else None,
        "vehicles_recovered": len(per_vehicle),
    }


def main() -> None:
    run_dir = Path(sys.argv[1])
    command_id = sys.argv[2] if len(sys.argv) > 2 else "cmd"
    result = failover_times(run_dir, command_id)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
