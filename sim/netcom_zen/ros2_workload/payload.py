"""Telemetry payload: JSON header padded to a target wire size, kept
parseable so the receiver can recover (id, seq, ts_us) for AoI bookkeeping."""
from __future__ import annotations

import json


def pack(node_id: str, seq: int, ts_us: int, target_bytes: int) -> str:
    head = {"id": node_id, "seq": seq, "ts_us": ts_us, "pad": ""}
    base = len(json.dumps(head, separators=(",", ":")))
    head["pad"] = "x" * max(target_bytes - base, 0)
    return json.dumps(head, separators=(",", ":"))


def unpack(data: str) -> tuple[str, int, int]:
    head = json.loads(data)
    return head["id"], head["seq"], head["ts_us"]
