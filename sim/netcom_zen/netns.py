from __future__ import annotations

import json
import secrets
import subprocess

PREFIX = "ncz"


def _run(*args: str) -> str:
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


class NetnsTopology:
    """One netns + veth pair per node. Host side carries no IP; the channel
    forwarder attaches AF_PACKET sockets there (architecture.md dataplane)."""

    def __init__(self, node_ids: list[str]):
        self.run_id = secrets.token_hex(2)
        self.nodes = list(node_ids)
        self.ns_names = {n: f"{PREFIX}-{self.run_id}-{i}" for i, n in enumerate(self.nodes)}
        self.host_ifaces = {n: f"{PREFIX}{self.run_id}h{i}" for i, n in enumerate(self.nodes)}
        self.addrs = {n: f"10.99.0.{i + 1}" for i, n in enumerate(self.nodes)}
        self.macs: dict[str, bytes] = {}

    def setup(self) -> None:
        sweep_stale()
        for i, n in enumerate(self.nodes):
            ns, host = self.ns_names[n], self.host_ifaces[n]
            inner = f"{PREFIX}{self.run_id}n{i}"
            _run("ip", "netns", "add", ns)
            _run("ip", "link", "add", host, "type", "veth", "peer", "name", inner)
            _run("ip", "link", "set", inner, "netns", ns)
            _run("ip", "-n", ns, "addr", "add", f"{self.addrs[n]}/24", "dev", inner)
            _run("ip", "-n", ns, "link", "set", inner, "up")
            _run("ip", "-n", ns, "link", "set", "lo", "up")
            _run("ip", "link", "set", host, "up")
            info = json.loads(_run("ip", "-n", ns, "-j", "link", "show", inner))
            self.macs[n] = bytes.fromhex(info[0]["address"].replace(":", ""))

    def teardown(self) -> None:
        for ns in self.ns_names.values():
            subprocess.run(["ip", "netns", "del", ns], capture_output=True)


def sweep_stale() -> None:
    """Idempotent cleanup of leftovers from crashed runs (architecture.md).
    Deleting a netns destroys its interfaces; host-side veths die with their peers."""
    out = subprocess.run(["ip", "-j", "netns", "list"],
                         capture_output=True, text=True).stdout
    for entry in json.loads(out or "[]"):
        if entry.get("name", "").startswith(f"{PREFIX}-"):
            subprocess.run(["ip", "netns", "del", entry["name"]], capture_output=True)
