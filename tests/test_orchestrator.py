import asyncio
import json
import subprocess

import pyarrow.parquet as pq
import pytest

from netcom_zen.config import load_scenario
from netcom_zen.orchestrator import ScenarioEngine


@pytest.mark.sudo
def test_smoke_run_produces_artifacts(tmp_path):
    scenario = load_scenario("scenarios/smoke_2node.yaml")
    engine = ScenarioEngine(scenario, out_dir=tmp_path)

    async def go():
        run_task = asyncio.create_task(engine.run())
        await asyncio.wait_for(engine.ready.wait(), timeout=10)
        ns1 = engine.topo.ns_names["v1"]
        # generate traffic: ping v2 from v1 through the channel
        subprocess.run(["ip", "netns", "exec", ns1, "ping", "-c", "3", "-W", "2",
                        "10.99.0.2"], check=True, capture_output=True)
        await run_task

    asyncio.run(go())
    tbl = pq.read_table(tmp_path / "packets.parquet")
    assert tbl.num_rows > 0
    assert "delivered" in set(tbl.column("verdict").to_pylist())
    manifest = json.loads((tmp_path / "manifest.json").read_text())
    assert manifest["scenario"]["name"] == "smoke-2node"
    assert manifest["seed"] == 42 and "git_hash" in manifest
    assert manifest["timing_ok"] is True  # tick loop kept up with wall clock
