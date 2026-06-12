import asyncio
from pathlib import Path

import pytest

from netcom_zen.config import load_scenario
from netcom_zen.harness.failover import failover_times
from netcom_zen.orchestrator import ScenarioEngine

AGENT_BIN = Path("agent/target/release/ncz-agent")


@pytest.mark.sudo
@pytest.mark.skipif(not AGENT_BIN.exists(),
                    reason="agent not built; run `pixi run build-agent`")
def test_clean_failover_seamless(tmp_path):
    # short clean-mesh failover: command must re-observe the swarm over RF
    scenario = load_scenario("scenarios/failover_4node.yaml").model_copy(
        update={"duration_s": 30.0})
    scenario.satellite.outages[0].start_s = 15.0
    asyncio.run(ScenarioEngine(scenario, tmp_path).run())

    r = failover_times(tmp_path, "cmd")
    assert r["vehicles_recovered"] == 3, r
    # clean RF fallback should recover the whole swarm within a few seconds
    assert r["swarm_failover_s"] < 5.0, r
