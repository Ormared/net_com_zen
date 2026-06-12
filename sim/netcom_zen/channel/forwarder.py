from __future__ import annotations

import asyncio
import socket
import time

from ..metrics import PacketLog, PacketRecord
from .linkstate import LinkState, link_rng

ETH_P_ALL = 0x0003
PACKET_OUTGOING = 4  # sll_pkttype: frames we injected; must not re-process


class ChannelForwarder:
    """Userspace dataplane: every inter-node frame gets a seeded, logged verdict."""

    MAX_QUEUE_S = 0.5

    def __init__(self, topo, seed: int, log: PacketLog, clock=time.monotonic):
        self.topo = topo
        self.seed = seed
        self.log = log
        self.clock = clock
        self.t0 = clock()
        self._table: dict[tuple[str, str], LinkState] = {}
        self._rngs = {}
        self._busy_until: dict[tuple[str, str], float] = {}
        self._socks: dict[str, socket.socket] = {}

    def start(self) -> None:
        loop = asyncio.get_running_loop()
        for node, iface in self.topo.host_ifaces.items():
            s = socket.socket(socket.AF_PACKET, socket.SOCK_RAW, socket.htons(ETH_P_ALL))
            s.bind((iface, 0))
            s.setblocking(False)
            self._socks[node] = s
            loop.add_reader(s.fileno(), self._on_readable, node, s)

    def stop(self) -> None:
        loop = asyncio.get_running_loop()
        for s in self._socks.values():
            loop.remove_reader(s.fileno())
            s.close()
        self._socks.clear()

    def update_links(self, table: dict[tuple[str, str], LinkState]) -> None:
        self._table = table  # atomic swap (GIL)

    def _rng(self, link: tuple[str, str]):
        if link not in self._rngs:
            self._rngs[link] = link_rng(self.seed, *link)
        return self._rngs[link]

    def _targets(self, src: str, frame: bytes) -> list[str]:
        dst_mac = frame[:6]
        if dst_mac[0] & 1:  # broadcast/multicast
            return [n for n in self.topo.nodes if n != src]
        return [n for n, m in self.topo.macs.items() if m == dst_mac and n != src]

    def _on_readable(self, src: str, s: socket.socket) -> None:
        while True:
            try:
                frame, addr = s.recvfrom(65535)
            except BlockingIOError:
                return
            except OSError:
                return  # socket closed during shutdown
            if addr[2] == PACKET_OUTGOING:
                continue  # our own injected frame echoing back
            t = self.clock() - self.t0
            for dst in self._targets(src, frame):
                self._process(t, src, dst, frame)

    def _process(self, t: float, src: str, dst: str, frame: bytes) -> None:
        link = (src, dst)
        st = self._table.get(link)
        if st is None:
            self.log.add(PacketRecord(t, src, dst, len(frame), "no_link", 0.0))
            return
        ser = len(frame) * 8 / st.data_rate_bps
        start = max(t, self._busy_until.get(link, 0.0))
        if start - t > self.MAX_QUEUE_S:
            self.log.add(PacketRecord(t, src, dst, len(frame), "queue", 0.0))
            return
        self._busy_until[link] = start + ser  # airtime consumed even by losses
        rng = self._rng(link)
        verdict = st.verdict(len(frame), rng.random(), rng.random())
        if verdict != "deliver":
            self.log.add(PacketRecord(t, src, dst, len(frame), verdict, 0.0))
            return
        delay = (start - t) + ser + st.prop_delay_s
        asyncio.get_running_loop().call_later(delay, self._deliver, t, src, dst,
                                              frame, delay)

    def _deliver(self, t: float, src: str, dst: str, frame: bytes,
                 delay: float) -> None:
        try:
            self._socks[dst].send(frame)
            self.log.add(PacketRecord(t, src, dst, len(frame), "delivered", delay))
        except (OSError, KeyError):
            self.log.add(PacketRecord(t, src, dst, len(frame), "tx_error", 0.0))
