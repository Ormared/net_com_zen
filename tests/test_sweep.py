import pytest
import yaml

from netcom_zen.config import Scenario
from netcom_zen.harness.sweep import apply_override, cell_name, expand


def test_apply_override_nested_and_list():
    cfg = {"radio": {"hop": {"hop_rate_hz": 1.0}},
           "jammers": [{"tx_power_dbm": 10.0}]}
    apply_override(cfg, "radio.hop.hop_rate_hz", 500.0)
    apply_override(cfg, "jammers.0.tx_power_dbm", 42.0)
    assert cfg["radio"]["hop"]["hop_rate_hz"] == 500.0
    assert cfg["jammers"][0]["tx_power_dbm"] == 42.0


def test_expand_cartesian():
    cells = expand({"axes": {"a": [1, 2], "b": [10]}, "seeds": [7, 8]})
    assert len(cells) == 4
    assert ({"a": 1, "b": 10}, 7) in cells
    names = {cell_name(o, s) for o, s in cells}
    assert "a=1__b=10__seed=7" in names


def test_base_scenarios_validate():
    for path in ("scenarios/resilience_4node.yaml", "scenarios/smoke_4node.yaml"):
        Scenario.model_validate(yaml.safe_load(open(path)))


def test_sweep_cells_validate():
    sweep = yaml.safe_load(open("scenarios/sweep_resilience_smoke.yaml"))
    base = yaml.safe_load(open("scenarios/resilience_4node.yaml"))
    import copy
    for overrides, seed in expand(sweep):
        cfg = copy.deepcopy(base)
        for k, v in overrides.items():
            apply_override(cfg, k, v)
        cfg["seed"] = seed
        Scenario.model_validate(cfg)


def test_aggregate_on_run_artifacts(tmp_path):
    # synthetic single-run sweep dir exercising report.run_metrics end to end
    import json
    import pyarrow as pa
    import pyarrow.parquet as pq
    from netcom_zen.harness.report import aggregate
    run = tmp_path / "a=1__seed=7"
    run.mkdir()
    pq.write_table(pa.table({
        "t": [0.1, 0.2, 0.3], "src": ["v1"] * 3, "dst": ["v2"] * 3,
        "length": [64] * 3, "verdict": ["delivered", "delivered", "jam"],
        "delay_s": [0.005, 0.006, 0.0]}), run / "packets.parquet")
    (run / "agent_v1.jsonl").write_text(
        '{"type":"pub","id":"v1","seq":1,"ts_us":1000000}\n')
    (run / "agent_v2.jsonl").write_text(
        '{"type":"recv","from":"v1","peer_ts_us":900000,"ts_us":1000000}\n'
        '{"type":"final_state","ts_us":2000000}\n')
    (tmp_path / "sweep_manifest.json").write_text(json.dumps({
        "sweep": {"axes": {"a": [1]}},
        "cells": [{"name": "a=1__seed=7", "overrides": {"a": 1}, "seed": 7}]}))
    rows = aggregate(tmp_path)
    assert len(rows) == 1
    assert rows[0]["frame_pdr"] == pytest.approx(2 / 3)
    assert rows[0]["drop_counts"] == {"jam": 1}
    assert rows[0]["update_delivery"] == pytest.approx(1.0)
