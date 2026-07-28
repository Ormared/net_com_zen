"""Aggregate sweep results into per-run metrics and resilience-curve plots.

    python -m netcom_zen.harness.report results/<sweep-dir>

Produces <sweep-dir>/summary.json and PNG curves (metric vs the x axis,
one line per series axis), aggregated mean over seeds.

Two substrates are supported:
  channel — AF_PACKET forwarder with RF model; produces packets.parquet.
  bridge  — kernel L2 bridge; produces agent_*.jsonl + resources.parquet.
"""
from __future__ import annotations

import argparse
import collections
import json
import statistics
from pathlib import Path

import pyarrow.parquet as pq

from .aoi import aoi_summary


# ---------------------------------------------------------------------------
# helpers
# ---------------------------------------------------------------------------

def _percentile(sorted_vals: list[float], p: float) -> float | None:
    """p-th percentile (0–1) via floor-index into a pre-sorted list."""
    if not sorted_vals:
        return None
    idx = int(p * (len(sorted_vals) - 1))
    return sorted_vals[idx]


def _is_bridge_run(run_dir: Path) -> bool:
    """True when the run used the bridge substrate.

    Prefer the explicit manifest field; fall back to artifact fingerprinting
    so old runs without a substrate key are handled gracefully.
    """
    manifest_path = run_dir / "manifest.json"
    if manifest_path.exists():
        m = json.loads(manifest_path.read_text())
        if m.get("substrate") == "bridge":
            return True
    # bridge runs have resources.parquet but no packets.parquet
    return ((run_dir / "resources.parquet").exists()
            and not (run_dir / "packets.parquet").exists())


# ---------------------------------------------------------------------------
# channel substrate (original)
# ---------------------------------------------------------------------------

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


# ---------------------------------------------------------------------------
# bridge substrate
# ---------------------------------------------------------------------------

def centralized_run_metrics(run_dir: Path) -> dict:
    """Metrics for the mixed centralized telemetry/command/RPC workload.

    Every latency/gap calculation uses timestamps produced by one process.
    This keeps LAN results valid even when host clocks are not synchronized.
    """
    manifest = json.loads((run_dir / "manifest.json").read_text())
    scenario = manifest["scenario"]
    central = scenario["ros2"]["centralized"]
    server_ids = set(central["server_ids"])
    client_order = [
        n["id"] for n in scenario["nodes"] if n["id"] not in server_ids]
    client_ids = set(client_order)
    if central["mode"] == "sharded":
        expected_clients_by_server = {
            sid: {
                nid for i, nid in enumerate(client_order)
                if central["server_ids"][i % len(server_ids)] == sid}
            for sid in server_ids}
    else:
        expected_clients_by_server = {
            sid: client_ids for sid in server_ids}
    events: dict[str, list[dict]] = {}
    for path in sorted(run_dir.glob("agent_*.jsonl")):
        nid = path.stem.removeprefix("agent_")
        events[nid] = [
            json.loads(line) for line in path.read_text().splitlines()
            if line.strip()
        ]

    telemetry_pubs = collections.Counter()
    telemetry_pub_times: dict[str, list[int]] = collections.defaultdict(list)
    telemetry_recv = collections.Counter()
    command_pubs = collections.Counter()
    command_recv = collections.Counter()
    command_duplicates = 0
    command_times: dict[str, list[int]] = collections.defaultdict(list)
    starts: dict[str, int] = {}
    rpc_counts = collections.Counter()
    rpc_rtt_ms: list[float] = []
    rpc_success_times: dict[str, list[int]] = collections.defaultdict(list)
    rpc_servers = collections.Counter()
    telemetry_first: dict[tuple[str, str], int] = {}

    for nid, rows in events.items():
        for ev in rows:
            kind = ev.get("type")
            if kind == "start":
                starts[nid] = ev["ts_us"]
            elif kind == "telemetry_pub":
                telemetry_pubs[nid] += 1
                telemetry_pub_times[nid].append(ev["ts_us"])
            elif kind == "telemetry_recv":
                telemetry_recv[(nid, ev["from"])] += 1
                pair = (nid, ev["from"])
                telemetry_first[pair] = min(
                    telemetry_first.get(pair, ev["ts_us"]), ev["ts_us"])
            elif kind == "command_pub":
                command_pubs[nid] += 1
            elif kind in {"command_recv", "command_duplicate"}:
                command_recv[(nid, ev["from"])] += 1
                command_times[nid].append(ev["ts_us"])
                command_duplicates += kind == "command_duplicate"
            elif kind and kind.startswith("rpc_"):
                rpc_counts[kind] += 1
                if kind == "rpc_success":
                    rpc_rtt_ms.append(
                        (ev["ts_us"] - ev["started_us"]) / 1e3)
                    rpc_success_times[nid].append(ev["ts_us"])
                    rpc_servers[str(ev.get("server", "")).split(":")[0]] += 1

    if central["mode"] == "sharded":
        expected_telemetry = sum(telemetry_pubs.values())
    elif (central["mode"] in {"active_active", "active_passive"}
          and central.get("failure_at_s") is not None):
        # Both servers receive before the injected primary failure; only the
        # surviving/warm standby can receive afterward.  Counting the dead
        # server for the rest of the run would label successful failover as
        # artificial packet loss.
        expected_telemetry = 0
        for nid, times in telemetry_pub_times.items():
            target = starts.get(nid, 0) + int(central["failure_at_s"] * 1e6)
            expected_telemetry += sum(2 if t < target else 1 for t in times)
    else:
        expected_telemetry = (
            sum(telemetry_pubs.values()) * len(server_ids))
    delivered_telemetry = sum(telemetry_recv.values())

    expected_commands = sum(
        count * len(client_ids) for sid, count in command_pubs.items()
        if sid in server_ids)
    delivered_commands = sum(command_recv.values())
    command_gaps_ms = sorted(
        (b - a) / 1e3
        for times in command_times.values()
        for a, b in zip(sorted(times), sorted(times)[1:])
    )

    attempts = sum(rpc_counts[k] for k in (
        "rpc_success", "rpc_error", "rpc_timeout", "rpc_unavailable"))
    duration_s = float(scenario["duration_s"])

    def outage_gap(times_by_node: dict[str, list[int]]) -> float | None:
        failure = central.get("failure_at_s")
        if failure is None:
            return None
        gaps = []
        for nid in client_ids:
            target = starts.get(nid, 0) + int(failure * 1e6)
            before = [t for t in times_by_node.get(nid, []) if t < target]
            after = [t for t in times_by_node.get(nid, []) if t >= target]
            if before and after:
                gaps.append((min(after) - max(before)) / 1e6)
        return max(gaps) if gaps else None

    client_ready = []
    for nid in client_ids:
        commands = sorted(command_times.get(nid, []))
        rpc = sorted(rpc_success_times.get(nid, []))
        if commands and rpc and nid in starts:
            client_ready.append(
                (max(commands[0], rpc[0]) - starts[nid]) / 1e6)
    telemetry_ready = []
    for sid in server_ids:
        firsts = [
            t for (server, client), t in telemetry_first.items()
            if server == sid and client in expected_clients_by_server[sid]]
        if (len(firsts) == len(expected_clients_by_server[sid])
                and sid in starts):
            telemetry_ready.append((max(firsts) - starts[sid]) / 1e6)
    ready_time_s = (
        max(client_ready + telemetry_ready)
        if len(client_ready) == len(client_ids) and telemetry_ready else None)

    telemetry_ratio = (
        delivered_telemetry / expected_telemetry
        if expected_telemetry else None)
    command_ratio = (
        delivered_commands / expected_commands
        if expected_commands else None)
    rpc_ratio = (
        rpc_counts["rpc_success"] / attempts if attempts else None)
    rpc_p99 = _percentile(sorted(rpc_rtt_ms), 0.99)
    command_gap_p99 = _percentile(command_gaps_ms, 0.99)
    expected_command_gap = central["command_period_ms"]

    resource_stats = {}
    resource_path = run_dir / "resources.parquet"
    if resource_path.exists():
        resource_rows = pq.read_table(resource_path).to_pylist()

        def role_resources(names: set[str], prefix: str) -> None:
            rows = [r for r in resource_rows if str(r["proc"]) in names]
            cpus = [r["cpu_pct"] for r in rows
                    if r["cpu_pct"] == r["cpu_pct"]]
            rss = [r["rss_bytes"] for r in rows]
            resource_stats[f"{prefix}_cpu_peak_pct"] = (
                max(cpus) if cpus else None)
            resource_stats[f"{prefix}_rss_peak_mb"] = (
                max(rss) / 1e6 if rss else None)

        role_resources(server_ids, "server")
        role_resources(client_ids, "client")
        infra_names = {
            str(r["proc"]) for r in resource_rows
            if str(r["proc"]).startswith("router:")}
        role_resources(infra_names, "infrastructure")

    result = {
        "telemetry_delivery_ratio": (
            telemetry_ratio),
        "telemetry_received": delivered_telemetry,
        "telemetry_received_per_s": delivered_telemetry / duration_s,
        "telemetry_server_distribution": {
            sid: sum(n for (receiver, _), n in telemetry_recv.items()
                     if receiver == sid)
            for sid in server_ids},
        "command_delivery_ratio": command_ratio,
        "command_received_per_s": delivered_commands / duration_s,
        "command_client_coverage": (
            sum(bool(command_times.get(n)) for n in client_ids)
            / len(client_ids) if client_ids else None),
        "command_duplicate_ratio": (
            command_duplicates / delivered_commands
            if delivered_commands else None),
        "command_gap_p50_ms": _percentile(command_gaps_ms, 0.50),
        "command_gap_p99_ms": _percentile(command_gaps_ms, 0.99),
        "command_gap_max_ms": max(command_gaps_ms, default=None),
        "rpc_success_ratio": rpc_ratio,
        "rpc_success": rpc_counts["rpc_success"],
        "rpc_success_per_s": rpc_counts["rpc_success"] / duration_s,
        "rpc_timeout": rpc_counts["rpc_timeout"],
        "rpc_unavailable": rpc_counts["rpc_unavailable"],
        "rpc_rtt_p50_ms": _percentile(sorted(rpc_rtt_ms), 0.50),
        "rpc_rtt_p99_ms": rpc_p99,
        "rpc_server_distribution": dict(rpc_servers),
        "command_failover_gap_s": outage_gap(command_times),
        "rpc_failover_gap_s": outage_gap(rpc_success_times),
        "n_clients": len(client_ids),
        "n_servers": len(server_ids),
        "server_mode": central["mode"],
        "profile_label": central.get("profile_label", "stock"),
        "discovery_mode": central.get("discovery_mode"),
        "telemetry_reliability": central.get("telemetry_reliability"),
        "socket_buffer_bytes": scenario["ros2"].get(
            "socket_buffer_bytes", 0),
        "rmw": manifest.get("rmw"),
        "ready_time_s": ready_time_s,
        # SLO views are overlays, never a replacement for the raw curves.
        "slo_relaxed": bool(
            telemetry_ratio is not None and telemetry_ratio >= 0.90
            and rpc_ratio is not None and rpc_ratio >= 0.90
            and rpc_p99 is not None and rpc_p99 <= 1000
            and command_gap_p99 is not None
            and command_gap_p99 <= 2 * expected_command_gap),
        "slo_tight_control": bool(
            telemetry_ratio is not None and telemetry_ratio >= 0.99
            and command_ratio is not None and command_ratio >= 0.99
            and rpc_ratio is not None and rpc_ratio >= 0.99
            and rpc_p99 is not None and rpc_p99 <= 200
            and command_gap_p99 is not None
            and command_gap_p99 <= 1.2 * expected_command_gap),
        **resource_stats,
        # Compatibility aliases let the generic curve plotter render central
        # cells while purpose-built reports use the names above.
        "delivery_ratio": telemetry_ratio,
        "latency_p50_ms": _percentile(sorted(rpc_rtt_ms), 0.50),
        "latency_p99_ms": _percentile(sorted(rpc_rtt_ms), 0.99),
        "discovery_time_s": None,
    }
    return result


def bridge_run_metrics(run_dir: Path) -> dict:
    """Metrics for one bridge-substrate run directory.

    Reads agent_*.jsonl for timing/delivery, resources.parquet for CPU/mem,
    and manifest.json for host-pressure counters written by the orchestrator.
    """
    # --- parse per-agent JSONL ---
    start_ts: dict[str, int] = {}
    pub_counts: dict[str, int] = collections.Counter()
    # recv events keyed by the receiving node id
    recv_events: dict[str, list[dict]] = collections.defaultdict(list)

    for path in sorted(run_dir.glob("agent_*.jsonl")):
        nid = path.stem.removeprefix("agent_")
        for line in path.read_text().splitlines():
            if not line.strip():
                continue
            ev = json.loads(line)
            t = ev["type"]
            if t == "start":
                start_ts[nid] = ev["ts_us"]
            elif t == "pub":
                pub_counts[nid] += 1
            elif t == "recv":
                recv_events[nid].append(ev)

    node_ids = sorted(start_ts)
    n_reporting = len(node_ids)  # nodes that actually wrote a JSONL file

    # Load manifest early so n_nodes is available for the mesh denominator.
    # The same object is reused at the return site to avoid a double read.
    manifest = json.loads((run_dir / "manifest.json").read_text())
    n_manifest = manifest.get("n_nodes", n_reporting)
    # Use the manifest's n_nodes (not just the count of reporting files) as
    # the total_pairs denominator.  A node that never spawned contributes 0
    # heard-peers to connected_pairs (no one received from it; it received
    # from no one), so the numerator is already correct.  Before this fix,
    # missing nodes were silently excluded from the denominator, inflating
    # mesh_completeness toward 1.0 for runs where spawns failed (observed:
    # 6 missing JSONL files → mesh reported 1.0 when true value was ~0.87).
    total_pairs = n_manifest * (n_manifest - 1)  # ordered (receiver, sender) pairs

    # discovery_time_s — per node: max first-recv-ts over peers − node's start_ts.
    # Peers with no recv at all are excluded from the max (they never formed a link)
    # but DO count against mesh_completeness.
    connected_pairs = 0
    node_disc: list[float] = []
    for nid in node_ids:
        first_recv: dict[str, int] = {}
        for ev in recv_events[nid]:
            peer = ev["from"]
            if peer not in first_recv or ev["ts_us"] < first_recv[peer]:
                first_recv[peer] = ev["ts_us"]
        connected_pairs += len(first_recv)
        if first_recv and nid in start_ts:
            disc_us = max(first_recv.values()) - start_ts[nid]
            node_disc.append(disc_us / 1e6)

    discovery_time_s = max(node_disc) if node_disc else None
    mesh_completeness = connected_pairs / total_pairs if total_pairs > 0 else None

    # latency percentiles — one-way delay per recv event (recv_ts − pub_ts)
    latencies_ms = sorted(
        (ev["ts_us"] - ev["peer_ts_us"]) / 1e3
        for evs in recv_events.values()
        for ev in evs
    )
    latency_p50_ms = _percentile(latencies_ms, 0.50)
    latency_p99_ms = _percentile(latencies_ms, 0.99)

    # delivery_ratio — fraction of offered packets that arrived at each peer.
    # "offered" for pair (a, b) = total pubs by b (b broadcasts to all peers).
    recv_counts: collections.Counter = collections.Counter()
    for nid, evs in recv_events.items():
        for ev in evs:
            recv_counts[(nid, ev["from"])] += 1
    total_delivered = sum(recv_counts.values())
    total_offered = sum(
        pub_counts.get(sender, 0)
        for receiver in node_ids
        for sender in node_ids
        if sender != receiver
    )
    delivery_ratio = total_delivered / total_offered if total_offered else None

    # CPU / mem from resources.parquet, split by proc type:
    # workload procs = rows where proc does NOT start with "router:"
    # router procs  = rows where proc starts with "router:" (zenoh only)
    res = pq.read_table(run_dir / "resources.parquet").to_pylist()
    node_rows = [r for r in res if not str(r["proc"]).startswith("router:")]
    router_rows = [r for r in res if str(r["proc"]).startswith("router:")]

    def _proc_stats(rows: list[dict], prefix: str) -> dict:
        """mean-of-means cpu, max-of-peaks cpu, max-of-peaks rss, all by proc."""
        by_proc: dict[str, list] = collections.defaultdict(list)
        for r in rows:
            by_proc[str(r["proc"])].append(r)
        if not by_proc:
            return {f"{prefix}cpu_mean_pct": None,
                    f"{prefix}cpu_peak_pct": None,
                    f"{prefix}rss_peak_mb": None}
        # the sampler records NaN cpu for a PID that vanished mid-run (a proc
        # killed/exited during the collapse at scale); drop those so one dead
        # sample doesn't poison the whole mean (v == v is False only for NaN)
        cpu_by_proc = [[r["cpu_pct"] for r in s if r["cpu_pct"] == r["cpu_pct"]]
                       for s in by_proc.values()]
        cpu_means = [statistics.mean(c) for c in cpu_by_proc if c]
        cpu_peaks = [max(c) for c in cpu_by_proc if c]
        rss_peaks = [max(r["rss_bytes"] for r in s) for s in by_proc.values()]
        return {
            f"{prefix}cpu_mean_pct": statistics.mean(cpu_means) if cpu_means else None,
            f"{prefix}cpu_peak_pct": max(cpu_peaks) if cpu_peaks else None,
            f"{prefix}rss_peak_mb": max(rss_peaks) / 1e6,
        }

    # manifest already loaded above for the n_nodes denominator; reuse here.
    exit_codes = manifest.get("agent_exit_codes", {})

    return {
        "discovery_time_s": discovery_time_s,
        "mesh_completeness": mesh_completeness,
        "latency_p50_ms": latency_p50_ms,
        "latency_p99_ms": latency_p99_ms,
        "delivery_ratio": delivery_ratio,
        **_proc_stats(node_rows, ""),
        **_proc_stats(router_rows, "router_"),
        "n_nodes": manifest.get("n_nodes"),
        # nodes_reporting < n_nodes when some agents never started (the
        # denominator fix ensures mesh_completeness accounts for the gap).
        "nodes_reporting": n_reporting,
        "rmw": manifest.get("rmw"),
        # True only when every agent exited 0; None if no exit codes recorded
        "agents_exit_clean": (
            all(c == 0 for c in exit_codes.values()) if exit_codes else None
        ),
        "peak_host_mem_used_bytes": manifest.get("peak_host_mem_used_bytes"),
        "min_host_mem_avail_bytes": manifest.get("min_host_mem_avail_bytes"),
        "peak_host_swap_used_bytes": manifest.get("peak_host_swap_used_bytes"),
    }


# ---------------------------------------------------------------------------
# aggregation + plotting (substrate-aware)
# ---------------------------------------------------------------------------

def aggregate(sweep_dir: Path) -> list[dict]:
    manifest = json.loads((sweep_dir / "sweep_manifest.json").read_text())
    rows = []
    for cell in manifest["cells"]:
        run_dir = sweep_dir / cell["name"]
        base = {**cell["overrides"], "seed": cell["seed"]}
        status = cell.get("status", "unknown")
        try:
            run_manifest = json.loads(
                (run_dir / "manifest.json").read_text())
            if run_manifest.get("topology") == "centralized":
                metrics = centralized_run_metrics(run_dir)
            elif _is_bridge_run(run_dir):
                metrics = bridge_run_metrics(run_dir)
            elif (run_dir / "packets.parquet").exists():
                metrics = run_metrics(run_dir)
            else:
                raise ValueError("no recognized result artifacts")
            exit_codes = run_manifest.get("agent_exit_codes", {})
            faulted = {
                event.get("server_id")
                for event in run_manifest.get("fault_events", [])
                if event.get("type") == "primary_failure"}
            complete = (
                bool(exit_codes)
                and not run_manifest.get("resource_abort_reason")
                and all(code == 0 or nid in faulted
                        for nid, code in exit_codes.items()))
            rows.append({
                **base, **metrics, "cell_name": cell["name"],
                "cell_status": "complete" if complete else "failed",
                "complete": complete,
                "failure_reason": (
                    None if complete else
                    "missing or non-zero agent exit codes"),
            })
        except Exception as exc:
            rows.append({
                **base, "cell_name": cell["name"], "cell_status": status,
                "complete": False,
                "failure_reason": f"{type(exc).__name__}: {exc}",
            })
    return rows


def plot_curves(rows: list[dict], sweep_dir: Path, x_axis: str,
                series_axis: str | None) -> list[Path]:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    # Pick the metric list from the substrate of the data:
    # bridge rows carry discovery_time_s; channel rows carry frame_pdr.
    if rows and "discovery_time_s" in rows[0]:
        metrics = [
            ("discovery_time_s",  "Discovery time (s)"),
            ("latency_p50_ms",    "Latency p50 (ms)"),
            ("latency_p99_ms",    "Latency p99 (ms)"),
            ("delivery_ratio",    "Delivery ratio"),
            ("rss_peak_mb",       "RSS peak per node (MB)"),
        ]
    else:
        metrics = [
            ("frame_pdr",         "Frame PDR"),
            ("frame_pdr_postjam", "Frame PDR (jammer active)"),
            ("update_delivery",   "State-update delivery ratio"),
            ("mean_aoi_s",        "Mean age of information (s)"),
            ("median_delay_ms",   "Median delivery latency (ms)"),
        ]

    written = []
    for metric, label in metrics:
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
