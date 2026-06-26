from __future__ import annotations

import json
import secrets
import subprocess

PREFIX = "ncz"
ETHTOOL = "/usr/sbin/ethtool"


def _run(*args: str) -> str:
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


class NetnsTopology:
    """One netns + veth pair per node.

    Two dataplane modes (architecture.md / DDS benchmark plan):
    - bridge=False (channel substrate): host side carries no IP; the channel
      forwarder attaches AF_PACKET sockets to the host veths and relays.
    - bridge=True (bridge substrate): every host veth is enslaved to a single
      Linux kernel bridge, which does L2 forwarding. Multicast floods to all
      ports (snooping off) so each RMW's native discovery (SPDP/scouting)
      reaches every node. No userspace forwarder, no RF model."""

    def __init__(self, node_ids: list[str], bridge: bool = False):
        self.run_id = secrets.token_hex(2)
        self.nodes = list(node_ids)
        self.ns_names = {n: f"{PREFIX}-{self.run_id}-{i}" for i, n in enumerate(self.nodes)}
        self.host_ifaces = {n: f"{PREFIX}{self.run_id}h{i}" for i, n in enumerate(self.nodes)}
        self.addrs = {n: f"10.99.0.{i + 1}" for i, n in enumerate(self.nodes)}
        self.macs: dict[str, bytes] = {}
        # bridge mode: one shared L2 segment instead of an AF_PACKET relay
        self.bridge = bridge
        self.br_name = f"{PREFIX}{self.run_id}br" if bridge else None

    def setup(self) -> None:
        sweep_stale()
        if self.bridge:
            # STP/forward-delay off: a veth bridge has no loops, and the default
            # listening/learning blackout would corrupt discovery-time metrics.
            # mcast_snooping 0: flood multicast to every port so SPDP/scouting
            # reaches all nodes (plan Q2).
            _run("ip", "link", "add", self.br_name, "type", "bridge",
                 "stp_state", "0", "forward_delay", "0", "mcast_snooping", "0")
            _run("ip", "link", "set", self.br_name, "up")
        for i, n in enumerate(self.nodes):
            ns, host = self.ns_names[n], self.host_ifaces[n]
            inner = f"{PREFIX}{self.run_id}n{i}"
            _run("ip", "netns", "add", ns)
            _run("ip", "link", "add", host, "type", "veth", "peer", "name", inner)
            _run("ip", "link", "set", inner, "netns", ns)
            _run("ip", "-n", ns, "addr", "add", f"{self.addrs[n]}/24", "dev", inner)
            _run("ip", "-n", ns, "link", "set", inner, "up")
            _run("ip", "-n", ns, "link", "set", "lo", "up")
            if self.bridge:
                _run("ip", "link", "set", host, "master", self.br_name)
            _run("ip", "link", "set", host, "up")
            # veth leaves TCP/UDP checksums to "hardware" (CHECKSUM_PARTIAL), so
            # frames captured by the AF_PACKET forwarder would carry invalid
            # checksums and be dropped on re-injection; GSO/TSO/GRO would hand us
            # coalesced >MTU super-frames, distorting serialization delays. Still
            # needed on the bridge: the checksum must be valid end-to-end so the
            # receiving DDS UDP socket doesn't drop it.
            for args in ((["ip", "netns", "exec", ns, ETHTOOL, "-K", inner]),
                         ([ETHTOOL, "-K", host])):
                _run(*args, "tx", "off", "gso", "off", "tso", "off", "gro", "off")
            info = json.loads(_run("ip", "-n", ns, "-j", "link", "show", inner))
            self.macs[n] = bytes.fromhex(info[0]["address"].replace(":", ""))

    def teardown(self) -> None:
        for ns in self.ns_names.values():
            subprocess.run(["ip", "netns", "del", ns], capture_output=True)
        # host veths die with their netns peers; the bridge is host-side, delete it
        if self.br_name:
            subprocess.run(["ip", "link", "del", self.br_name], capture_output=True)


def sweep_stale() -> None:
    """Idempotent cleanup of leftovers from crashed runs (architecture.md).
    Deleting a netns destroys its interfaces; host-side veths die with their peers."""
    out = subprocess.run(["ip", "-j", "netns", "list"],
                         capture_output=True, text=True).stdout
    for entry in json.loads(out or "[]"):
        if entry.get("name", "").startswith(f"{PREFIX}-"):
            subprocess.run(["ip", "netns", "del", entry["name"]], capture_output=True)
    # bridges are host-side, so they outlive a netns sweep — drop ours by name
    links = subprocess.run(["ip", "-j", "link", "show", "type", "bridge"],
                           capture_output=True, text=True).stdout
    for link in json.loads(links or "[]"):
        name = link.get("ifname", "")
        if name.startswith(PREFIX) and name.endswith("br"):
            subprocess.run(["ip", "link", "del", name], capture_output=True)
