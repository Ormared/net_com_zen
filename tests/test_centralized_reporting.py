import json

import yaml

from netcom_zen.harness import centralized_lab
from netcom_zen.harness import report
from netcom_zen.harness.centralized_lab import (
    _middleware_packages,
    combined_report,
    evaluate,
    finalize_provenance,
    write_analysis_artifacts,
)


def _row(seed: int, *, complete: bool = True) -> dict:
    return {
        "phase": "bridge-core",
        "sweep": "single_n32",
        "rmw": "zenoh",
        "profile_label": "tuned",
        "server_mode": "single",
        "n_clients": 32,
        "n_servers": 1,
        "seed": seed,
        "complete": complete,
        "cell_name": f"seed={seed}",
        "cell_status": "complete" if complete else "failed",
        "failure_reason": None if complete else "spawn failure",
        "telemetry_delivery_ratio": 0.99,
        "command_delivery_ratio": 0.99,
        "telemetry_received_per_s": 100.0 + seed,
        "rpc_success_ratio": 0.99,
        "rpc_success_per_s": 10.0,
        "rpc_rtt_p99_ms": 20.0,
        "ready_time_s": 2.0,
        "command_gap_p99_ms": 1010.0,
        "command_duplicate_ratio": 0.0,
        "server_cpu_peak_pct": 10.0,
        "server_rss_peak_mb": 100.0,
        "slo_relaxed": True,
        "slo_tight_control": seed == 0,
    }


def test_analysis_artifacts_cover_failures_variance_slos_and_plots(tmp_path):
    names = write_analysis_artifacts(
        [_row(0), _row(1), _row(2, complete=False)], tmp_path)

    expected = {
        "failure_accounting.json",
        "failure_accounting.csv",
        "replicate_variance.json",
        "replicate_variance.csv",
        "slo_capacity.json",
        "slo_capacity.csv",
        "sustainable_frontiers.json",
        "sustainable_frontiers.csv",
        "plot_delivery.png",
        "plot_throughput.png",
        "plot_rate_frontier.png",
        "plot_rpc.png",
        "plot_readiness.png",
        "plot_lan_rtt.png",
        "plot_commands.png",
        "plot_failover.png",
        "plot_resources.png",
        "plot_slo_capacity.png",
    }
    assert set(names) == expected
    assert all((tmp_path / name).exists() for name in expected)

    failures = json.loads(
        (tmp_path / "failure_accounting.json").read_text())
    assert failures == [{
        "phase": "bridge-core",
        "sweep": "single_n32",
        "cell_name": "seed=2",
        "cell_status": "failed",
        "rmw": "zenoh",
        "profile_label": "tuned",
        "n_clients": 32,
        "seed": 2,
        "failure_reason": "spawn failure",
    }]
    slo = json.loads((tmp_path / "slo_capacity.json").read_text())
    assert slo[0]["replicates_planned"] == 3
    assert slo[0]["replicates_complete"] == 2
    assert slo[0]["all_complete"] is False
    assert slo[0]["relaxed_pass_fraction"] == 2 / 3
    assert slo[0]["tight_pass_fraction"] == 1 / 3
    variance = json.loads(
        (tmp_path / "replicate_variance.json").read_text())
    throughput = next(
        row for row in variance
        if row["metric"] == "telemetry_received_per_s")
    assert throughput["replicates_with_value"] == 2
    assert throughput["minimum"] == 100.0
    assert throughput["maximum"] == 101.0


def test_combined_report_accounts_for_wholly_missing_sweep(
        tmp_path, monkeypatch):
    scenarios = tmp_path / "scenarios"
    results = tmp_path / "results"
    scenarios.mkdir()
    (scenarios / "missing_sweep.yaml").write_text(yaml.safe_dump({
        "base": "unused.yaml",
        "cells": [{"ros2.rmw": "zenoh"}],
        "seeds": [0, 1, 2],
    }))
    monkeypatch.setattr(
        centralized_lab, "PHASE_SWEEPS",
        {"lan": ["missing_sweep.yaml"]})

    summary = combined_report(results, scenarios)

    assert summary["planned"] == 3
    assert summary["complete"] == 0
    assert summary["failed"] == 3
    assert {row["cell_status"] for row in summary["rows"]} == {"missing"}
    assert {row["rmw"] for row in summary["rows"]} == {"zenoh"}
    assert all(
        row["failure_reason"]
        == "missing sweep manifest: lan/missing/sweep_manifest.json"
        for row in summary["rows"])


def test_middleware_provenance_reads_rmw_versions():
    packages = _middleware_packages()
    by_name = {item["name"]: item for item in packages}
    for name in (
            "ros-jazzy-rmw-fastrtps-cpp",
            "ros-jazzy-rmw-cyclonedds-cpp",
            "ros-jazzy-rmw-zenoh-cpp"):
        assert by_name[name]["version"]
        assert by_name[name]["build"]


def test_finalize_provenance_preserves_run_start_snapshot(
        tmp_path, monkeypatch):
    original = {
        "recorded_at": "start",
        "git_commit": "benchmark-commit",
        "swap_counters": {"pswpin": 1, "pswpout": 2},
        "middleware_packages": "SyntaxError",
    }
    (tmp_path / "provenance.json").write_text(json.dumps(original))
    monkeypatch.setattr(
        centralized_lab, "_middleware_packages",
        lambda: [{"name": "rmw", "version": "1"}])

    finalize_provenance(tmp_path)

    updated = json.loads((tmp_path / "provenance.json").read_text())
    assert updated["recorded_at"] == "start"
    assert updated["git_commit"] == "benchmark-commit"
    assert updated["swap_counters"] == {"pswpin": 1, "pswpout": 2}
    assert updated["middleware_packages"] == [{
        "name": "rmw", "version": "1"}]
    assert updated["report_generated_at"]
    assert set(updated["swap_counters_at_report"]) == {
        "pswpin", "pswpout"}


def test_aggregate_records_exact_remote_launcher_failure(
        tmp_path, monkeypatch):
    run = tmp_path / "seed=0"
    run.mkdir()
    (tmp_path / "sweep_manifest.json").write_text(json.dumps({
        "sweep": {},
        "cells": [{
            "name": "seed=0", "overrides": {}, "seed": 0,
            "status": "complete",
        }],
    }))
    (run / "manifest.json").write_text(json.dumps({
        "topology": "centralized",
        "agent_exit_codes": {"s1": 0},
        "remote_launcher_exit": {"mini": 255},
    }))
    monkeypatch.setattr(
        report, "centralized_run_metrics", lambda _run: {})

    row = report.aggregate(tmp_path)[0]

    assert row["complete"] is False
    assert row["cell_status"] == "failed"
    assert row["failure_reason"] == (
        "non-zero remote launcher exits: mini=255")


def test_evaluate_resume_preserves_original_provenance(
        tmp_path, monkeypatch):
    scenarios = tmp_path / "scenarios"
    results = tmp_path / "results"
    scenarios.mkdir()
    results.mkdir()
    original = '{"git_commit":"original"}'
    (results / "provenance.json").write_text(original)
    calls = []
    monkeypatch.setattr(
        centralized_lab, "preflight",
        lambda *args, **kwargs: calls.append(("preflight", kwargs)))
    monkeypatch.setattr(
        centralized_lab, "write_provenance",
        lambda *args: calls.append(("write_provenance", {})))
    monkeypatch.setattr(
        centralized_lab, "generate",
        lambda *args: calls.append(("generate", {})))

    async def fake_run_phase(*args):
        calls.append(("run_phase", {}))

    monkeypatch.setattr(centralized_lab, "run_phase", fake_run_phase)
    monkeypatch.setattr(
        centralized_lab, "combined_report",
        lambda *args: calls.append(("combined_report", {})))

    evaluate("lan", scenarios, results)

    assert (results / "provenance.json").read_text() == original
    assert ("preflight", {"resumed": True}) in calls
    assert not any(name == "write_provenance" for name, _ in calls)
