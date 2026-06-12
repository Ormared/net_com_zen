import asyncio
import subprocess

import pytest

from netcom_zen.channel.forwarder import ChannelForwarder
from netcom_zen.channel.linkstate import LinkState
from netcom_zen.metrics import PacketLog
from netcom_zen.netns import NetnsTopology


def good_link(src, dst):
    return LinkState(src=src, dst=dst, prx_dbm=-60, noise_dbm=-113, rho=0.0,
                     jam_inchannel_dbm=None, data_rate_bps=1e6, hop_rate_hz=100,
                     prop_delay_s=1e-6, foliage_db=0.0, terrain_db=0.0)


@pytest.mark.sudo
def test_udp_through_channel():
    topo = NetnsTopology(["v1", "v2"])
    topo.setup()
    try:
        log = PacketLog()

        async def go():
            fwd = ChannelForwarder(topo, seed=1, log=log)
            fwd.update_links({("v1", "v2"): good_link("v1", "v2"),
                              ("v2", "v1"): good_link("v2", "v1")})
            fwd.start()
            recv = subprocess.Popen(
                ["ip", "netns", "exec", topo.ns_names["v2"], "python", "-c",
                 "import socket;s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM);"
                 "s.bind(('10.99.0.2',9000));s.settimeout(5);print(len(s.recv(100)))"],
                stdout=subprocess.PIPE, text=True)
            await asyncio.sleep(1.0)
            for _ in range(5):  # first sends may race ARP resolution
                subprocess.run(
                    ["ip", "netns", "exec", topo.ns_names["v1"], "python", "-c",
                     "import socket;s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM);"
                     "s.sendto(b'x'*64,('10.99.0.2',9000))"], check=True)
                await asyncio.sleep(0.2)
            out, _ = recv.communicate(timeout=10)
            assert out.strip() == "64"
            fwd.stop()

        asyncio.run(go())
        assert any(r.verdict == "delivered" for r in log.records)
    finally:
        topo.teardown()
