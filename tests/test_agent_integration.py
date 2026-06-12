import asyncio
import json
from pathlib import Path

import pytest

from netcom_zen.config import load_scenario
from netcom_zen.harness.aoi import aoi_summary
from netcom_zen.orchestrator import ScenarioEngine

AGENT_BIN = Path("agent/target/release/ncz-agent")


@pytest.mark.sudo
@pytest.mark.skipif(not AGENT_BIN.exists(),
                    reason="agent not built; run `pixi run build-agent`")
def test_four_agents_share_state_through_channel(tmp_path):
    scenario = load_scenario("scenarios/smoke_4node.yaml")
    engine = ScenarioEngine(scenario, out_dir=tmp_path)
    asyncio.run(engine.run())

    manifest = json.loads((tmp_path / "manifest.json").read_text())
    assert all(code == 0 for code in manifest["agent_exit_codes"].values())

    summary = aoi_summary(tmp_path)
    nodes = {"v1", "v2", "v3", "v4"}
    for obs in nodes:
        peers = set(summary[obs])
        assert peers == nodes - {obs}, f"{obs} missing peers: {nodes - {obs} - peers}"
        for peer, s in summary[obs].items():
            # 500 ms publish period over a clean close-range channel:
            # mean AoI must be sane (sub-2s), and updates actually flowed
            assert s["mean_s"] < 2.0, f"{obs}<-{peer} mean AoI {s['mean_s']:.2f}s"
            assert s["n_updates"] >= 5
