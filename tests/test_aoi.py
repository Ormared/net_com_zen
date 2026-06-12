import json

import pytest

from netcom_zen.harness.aoi import aoi_for_agent


def write_events(path, events):
    path.write_text("\n".join(json.dumps(e) for e in events) + "\n")


def test_aoi_sawtooth(tmp_path):
    # peer state ts 0.9 received at 1.0; ts 1.9 received at 2.0; run ends at 3.0
    # AoI: [1,2]: 0.1 -> 1.1 (mean 0.6); [2,3]: 0.1 -> 1.1 (mean 0.6)
    f = tmp_path / "agent_v1.jsonl"
    write_events(f, [
        {"type": "recv", "from": "v2", "peer_ts_us": 900_000, "ts_us": 1_000_000},
        {"type": "recv", "from": "v2", "peer_ts_us": 1_900_000, "ts_us": 2_000_000},
        {"type": "final_state", "ts_us": 3_000_000},
    ])
    stats = aoi_for_agent(f)["v2"]
    assert stats["mean_s"] == pytest.approx(0.6, abs=1e-6)
    assert stats["max_s"] == pytest.approx(1.1, abs=1e-6)
    assert stats["n_updates"] == 2


def test_aoi_ignores_stale_redelivery(tmp_path):
    f = tmp_path / "agent_v1.jsonl"
    write_events(f, [
        {"type": "recv", "from": "v2", "peer_ts_us": 900_000, "ts_us": 1_000_000},
        # re-delivery of older state must not count as an update
        {"type": "recv", "from": "v2", "peer_ts_us": 800_000, "ts_us": 1_500_000},
        {"type": "final_state", "ts_us": 2_000_000},
    ])
    assert aoi_for_agent(f)["v2"]["n_updates"] == 1
