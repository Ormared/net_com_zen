"""Agent regression test on localhost (no root): full mesh state convergence."""
import subprocess
import time
from pathlib import Path

import pytest

from netcom_zen.harness.aoi import aoi_for_agent

AGENT_BIN = Path("agent/target/release/ncz-agent")
PORTS = {f"v{i}": 7461 + i for i in range(1, 5)}


@pytest.mark.skipif(not AGENT_BIN.exists(),
                    reason="agent not built; run `pixi run build-agent`")
def test_four_agents_localhost(tmp_path):
    procs = []
    for nid, port in PORTS.items():
        cmd = [str(AGENT_BIN), "--id", nid,
               "--listen", f"tcp/127.0.0.1:{port}",
               "--metrics", str(tmp_path / f"agent_{nid}.jsonl"),
               "--period-ms", "100", "--duration-s", "5",
               "--full-every", "10", "--wait-peers", "3"]
        for other, oport in PORTS.items():
            if other != nid:
                cmd += ["--connect", f"tcp/127.0.0.1:{oport}"]
        procs.append(subprocess.Popen(cmd))
    deadline = time.time() + 30
    for p in procs:
        p.wait(timeout=max(1.0, deadline - time.time()))
        assert p.returncode == 0
    for nid in PORTS:
        stats = aoi_for_agent(tmp_path / f"agent_{nid}.jsonl")
        assert set(stats) == set(PORTS) - {nid}
        for peer, s in stats.items():
            assert s["mean_s"] < 1.0, f"{nid}<-{peer} mean AoI {s['mean_s']:.2f}s"
