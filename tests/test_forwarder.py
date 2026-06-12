import asyncio

from netcom_zen.channel.forwarder import ChannelForwarder
from netcom_zen.metrics import PacketLog
from tests.test_linkstate import make_state


class FakeTopo:
    nodes = ["a", "b"]
    host_ifaces = {}
    macs = {"a": b"\xaa" * 6, "b": b"\xbb" * 6}


def mk_fwd(table):
    log = PacketLog()
    fwd = ChannelForwarder(FakeTopo(), seed=42, log=log)
    fwd.update_links(table)
    return fwd, log


def run_process(fwd, t, src, dst, frame):
    async def go():
        fwd._process(t, src, dst, frame)
        await asyncio.sleep(0.05)
    asyncio.run(go())


def test_drop_is_logged_with_cause():
    fwd, log = mk_fwd({("a", "b"): make_state(prx_dbm=-150)})
    run_process(fwd, 0.0, "a", "b", b"\xbb" * 6 + b"\xaa" * 6 + b"x" * 52)
    assert [r.verdict for r in log.records] == ["range"]


def test_unknown_link_logged():
    fwd, log = mk_fwd({})
    run_process(fwd, 0.0, "a", "b", b"\xbb" * 6 + b"\xaa" * 6 + b"x" * 52)
    assert [r.verdict for r in log.records] == ["no_link"]


def test_queue_overflow_drops():
    st = make_state(data_rate_bps=1000.0)  # 64B frame = 0.512 s serialization
    fwd, log = mk_fwd({("a", "b"): st})
    frame = b"\xbb" * 6 + b"\xaa" * 6 + b"x" * 52

    async def go():
        for _ in range(3):  # later frames queue past MAX_QUEUE_S = 0.5
            fwd._process(0.0, "a", "b", frame)
    asyncio.run(go())
    assert "queue" in [r.verdict for r in log.records]


def test_mac_targeting():
    fwd, _ = mk_fwd({})
    assert fwd._targets("a", b"\xff" * 6 + b"\xaa" * 6) == ["b"]  # broadcast
    assert fwd._targets("a", b"\xbb" * 6 + b"\xaa" * 6) == ["b"]  # unicast
    assert fwd._targets("a", b"\xcc" * 6 + b"\xaa" * 6) == []     # unknown mac
