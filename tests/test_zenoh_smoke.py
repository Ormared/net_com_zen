import asyncio
import subprocess
import sys

import pytest

from netcom_zen.config import load_scenario
from netcom_zen.orchestrator import ScenarioEngine


@pytest.mark.sudo
def test_zenoh_pubsub_through_channel(tmp_path):
    scenario = load_scenario("scenarios/smoke_2node.yaml")
    scenario = scenario.model_copy(update={"duration_s": 12.0})
    engine = ScenarioEngine(scenario, out_dir=tmp_path)

    async def go():
        run_task = asyncio.create_task(engine.run())
        await asyncio.wait_for(engine.ready.wait(), timeout=10)
        ns1, ns2 = engine.topo.ns_names["v1"], engine.topo.ns_names["v2"]
        sub = subprocess.Popen(["ip", "netns", "exec", ns1, sys.executable,
                                "tests/helpers/zsub.py"],
                               stdout=subprocess.PIPE, text=True)
        await asyncio.sleep(1.5)
        subprocess.run(["ip", "netns", "exec", ns2, sys.executable,
                        "tests/helpers/zpub.py", "10.99.0.1"], check=True, timeout=30)
        out, _ = sub.communicate(timeout=30)
        assert int(out.strip()) > 0  # zenoh messages crossed the emulated channel
        await run_task

    asyncio.run(go())
