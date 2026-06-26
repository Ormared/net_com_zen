"""Tests for bridge_run_metrics: synthetic run_dir with hand-computable values.

Topology: 3 nodes (d1, d2, d3), one zenoh router proc.
d3 never receives from d2 — exercises the partial-mesh path.

Hand-computed expected values are embedded alongside each assertion.
"""
from __future__ import annotations

import json

import pyarrow as pa
import pyarrow.parquet as pq
import pytest

from netcom_zen.harness.report import aggregate, bridge_run_metrics


# ---------------------------------------------------------------------------
# fixture
# ---------------------------------------------------------------------------

def _write_jsonl(path, events):
    """Compact JSON (no spaces) to match the format written by node.py."""
    path.write_text(
        "\n".join(json.dumps(e, separators=(",", ":")) for e in events) + "\n"
    )


@pytest.fixture
def bridge_run(tmp_path):
    """
    3-node bridge run: d1 (2 pubs), d2 (1 pub), d3 (1 pub).

    Recv events:
      d1 ← d2 @ 1_150_000 µs   latency = 50 ms
      d1 ← d3 @ 1_180_000 µs   latency = 80 ms
      d2 ← d1 @ 1_160_000 µs   latency = 60 ms
      d2 ← d3 @ 1_190_000 µs   latency = 90 ms
      d3 ← d1 @ 1_170_000 µs   latency = 70 ms
      d3 ← d2   ABSENT          ← partial mesh; completeness < 1.0

    All peer_ts_us = 1_100_000, all start_ts = 1_000_000.
    """
    # d1: 2 pubs, receives from d2 and d3
    _write_jsonl(tmp_path / "agent_d1.jsonl", [
        {"type": "start",       "id": "d1", "ts_us": 1_000_000},
        {"type": "pub",         "id": "d1", "seq": 1, "kind": "snap",
         "ts_us": 1_100_000, "bytes": 100},
        {"type": "pub",         "id": "d1", "seq": 2, "kind": "snap",
         "ts_us": 1_200_000, "bytes": 100},
        {"type": "recv",        "id": "d1", "from": "d2", "kind": "snap",
         "peer_seq": 1, "peer_ts_us": 1_100_000, "ts_us": 1_150_000, "bytes": 100},
        {"type": "recv",        "id": "d1", "from": "d3", "kind": "snap",
         "peer_seq": 1, "peer_ts_us": 1_100_000, "ts_us": 1_180_000, "bytes": 100},
        {"type": "final_state", "id": "d1", "ts_us": 2_000_000},
    ])

    # d2: 1 pub, receives from d1 and d3
    _write_jsonl(tmp_path / "agent_d2.jsonl", [
        {"type": "start",       "id": "d2", "ts_us": 1_000_000},
        {"type": "pub",         "id": "d2", "seq": 1, "kind": "snap",
         "ts_us": 1_100_000, "bytes": 100},
        {"type": "recv",        "id": "d2", "from": "d1", "kind": "snap",
         "peer_seq": 1, "peer_ts_us": 1_100_000, "ts_us": 1_160_000, "bytes": 100},
        {"type": "recv",        "id": "d2", "from": "d3", "kind": "snap",
         "peer_seq": 1, "peer_ts_us": 1_100_000, "ts_us": 1_190_000, "bytes": 100},
        {"type": "final_state", "id": "d2", "ts_us": 2_000_000},
    ])

    # d3: 1 pub, receives ONLY from d1 (missing d2 → partial mesh)
    _write_jsonl(tmp_path / "agent_d3.jsonl", [
        {"type": "start",       "id": "d3", "ts_us": 1_000_000},
        {"type": "pub",         "id": "d3", "seq": 1, "kind": "snap",
         "ts_us": 1_100_000, "bytes": 100},
        {"type": "recv",        "id": "d3", "from": "d1", "kind": "snap",
         "peer_seq": 1, "peer_ts_us": 1_100_000, "ts_us": 1_170_000, "bytes": 100},
        {"type": "final_state", "id": "d3", "ts_us": 2_000_000},
    ])

    # resources.parquet: node procs d1/d2/d3 + one zenoh router proc
    pq.write_table(pa.table({
        "t": pa.array(
            [1.0, 2.0,  1.0, 2.0,  1.0, 2.0,  1.0, 2.0],
            type=pa.float64(),
        ),
        "proc": pa.array(
            ["d1", "d1", "d2", "d2", "d3", "d3", "router:r1", "router:r1"]
        ),
        "cpu_pct": pa.array(
            [30.0, 40.0, 20.0, 60.0, 10.0, 20.0, 10.0, 20.0],
            type=pa.float64(),
        ),
        "rss_bytes": pa.array(
            [100_000_000, 120_000_000,
              80_000_000,  90_000_000,
              50_000_000,  70_000_000,
              50_000_000,  60_000_000],
            type=pa.int64(),
        ),
        "host_cpu_pct":    pa.array([5.0] * 8, type=pa.float64()),
        "host_mem_used":   pa.array([8_000_000_000] * 8, type=pa.int64()),
        "host_mem_avail":  pa.array([2_000_000_000] * 8, type=pa.int64()),
        "host_swap_used":  pa.array([0] * 8, type=pa.int64()),
    }), tmp_path / "resources.parquet")

    (tmp_path / "manifest.json").write_text(json.dumps({
        "substrate": "bridge",
        "rmw": "zenoh",
        "n_nodes": 3,
        "resource_samples": 2,
        "peak_host_mem_used_bytes": 8_000_000_000,
        "min_host_mem_avail_bytes": 2_000_000_000,
        "peak_host_swap_used_bytes": 0,
        "agent_exit_codes": {"d1": 0, "d2": 0, "d3": 0},
        "scenario": {},
        "seed": 42,
    }))

    return tmp_path


# ---------------------------------------------------------------------------
# tests
# ---------------------------------------------------------------------------

def test_discovery_time(bridge_run):
    m = bridge_run_metrics(bridge_run)
    # d1: max(1_150_000, 1_180_000) − 1_000_000 = 180_000 µs = 0.18 s
    # d2: max(1_160_000, 1_190_000) − 1_000_000 = 190_000 µs = 0.19 s
    # d3: max(1_170_000)            − 1_000_000 = 170_000 µs = 0.17 s
    # swarm = max(0.18, 0.19, 0.17) = 0.19 s
    assert m["discovery_time_s"] == pytest.approx(0.19, abs=1e-9)


def test_mesh_completeness_partial(bridge_run):
    m = bridge_run_metrics(bridge_run)
    # 5 of 6 ordered pairs received ≥1 message; d3←d2 never arrived
    assert m["mesh_completeness"] == pytest.approx(5 / 6, abs=1e-9)
    # must be strictly less than 1.0, not an exception
    assert m["mesh_completeness"] < 1.0


def test_latency_percentiles(bridge_run):
    m = bridge_run_metrics(bridge_run)
    # all recv latencies ms, sorted: [50, 60, 70, 80, 90]
    # p50: floor(0.50 * 4) = idx 2 → 70 ms
    # p99: floor(0.99 * 4) = idx 3 → 80 ms
    assert m["latency_p50_ms"] == pytest.approx(70.0, abs=1e-9)
    assert m["latency_p99_ms"] == pytest.approx(80.0, abs=1e-9)


def test_delivery_ratio(bridge_run):
    m = bridge_run_metrics(bridge_run)
    # delivered = 5 recv events total
    # offered = pub_counts[sender] summed over all (receiver, sender) pairs:
    #   (d1,d2)=1  (d1,d3)=1  (d2,d1)=2  (d2,d3)=1  (d3,d1)=2  (d3,d2)=1 → 8
    assert m["delivery_ratio"] == pytest.approx(5 / 8, abs=1e-9)


def test_cpu_mem_workload_procs(bridge_run):
    m = bridge_run_metrics(bridge_run)
    # d1 cpu=[30,40]→mean 35; d2 cpu=[20,60]→mean 40; d3 cpu=[10,20]→mean 15
    # mean-of-means = (35+40+15)/3 = 30.0
    assert m["cpu_mean_pct"] == pytest.approx(30.0, abs=1e-9)
    # max-of-peaks = max(40, 60, 20) = 60.0
    assert m["cpu_peak_pct"] == pytest.approx(60.0, abs=1e-9)
    # d1 rss peak 120MB; d2 90MB; d3 70MB → max 120.0
    assert m["rss_peak_mb"] == pytest.approx(120.0, abs=1e-6)


def test_cpu_mem_router_proc(bridge_run):
    m = bridge_run_metrics(bridge_run)
    # router:r1 cpu=[10,20]→mean 15.0; rss=[50M,60M]→peak 60MB
    assert m["router_cpu_mean_pct"] == pytest.approx(15.0, abs=1e-9)
    assert m["router_rss_peak_mb"] == pytest.approx(60.0, abs=1e-6)


def test_manifest_passthrough(bridge_run):
    m = bridge_run_metrics(bridge_run)
    assert m["n_nodes"] == 3
    assert m["rmw"] == "zenoh"
    assert m["agents_exit_clean"] is True
    assert m["peak_host_mem_used_bytes"] == 8_000_000_000
    assert m["min_host_mem_avail_bytes"] == 2_000_000_000
    assert m["peak_host_swap_used_bytes"] == 0


def test_aggregate_routes_bridge_cell(tmp_path, bridge_run):
    """aggregate() must call bridge_run_metrics for substrate==bridge cells."""
    (tmp_path / "sweep_manifest.json").write_text(json.dumps({
        "sweep": {"axes": {"n_nodes": [3]}},
        "cells": [{"name": bridge_run.name, "overrides": {"n_nodes": 3}, "seed": 42}],
    }))
    # symlink the run dir under the sweep dir so aggregate can find it
    (tmp_path / bridge_run.name).symlink_to(bridge_run)

    rows = aggregate(tmp_path)
    assert len(rows) == 1
    row = rows[0]
    assert "discovery_time_s" in row
    assert row["discovery_time_s"] == pytest.approx(0.19, abs=1e-9)
    assert row["delivery_ratio"] == pytest.approx(5 / 8, abs=1e-9)
