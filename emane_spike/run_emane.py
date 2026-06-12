"""Run one net_com_zen scenario under EMANE and collect agent metrics.

Topology: one netns per node + one for the jammer, each with a veth to a shared
host bridge carrying OTA + event multicast. EMANE runs the RF Pipe NEM in each
node netns with a virtual TAP (emane0); pathloss events are fed from our own
CompositePathloss for the same geometry (precomputed propagation); the jammer is
toggled at jam start via emane-jammer-simple-control. The same Rust agent binary
from M2/M3 runs over each emane0 TAP.

Needs root. Usage:
    sudo .../python emane_spike/run_emane.py scenarios/resilience_4node.yaml \
         -o results/emane_cell [--no-agents] [--duration N] [--jammer-power dBm]
"""
from __future__ import annotations

import argparse
import json
import math
import os
import subprocess
import sys
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "sim"))
sys.path.insert(0, str(Path(__file__).resolve().parent))

import generate  # noqa: E402
from netcom_zen.config import load_scenario  # noqa: E402
from netcom_zen.ew import Jammer  # noqa: E402
from netcom_zen.propagation import CompositePathloss  # noqa: E402
from netcom_zen.terrain import FoliageRegion, Terrain  # noqa: E402

PREFIX = "nczE"
BRIDGE = "nczEbr0"
AGENT_BIN = ROOT / "agent/target/release/ncz-agent"
EMANE_PY = "/usr/bin/python3"  # system python has the emane bindings


def run(*args, **kw):
    return subprocess.run(args, check=True, capture_output=True, text=True, **kw)


def cleanup_all():
    """Idempotent: kill stray emane/jammer procs, delete nczE namespaces, the
    bridge, and any leaked host-side veths (netns deletion doesn't always reap
    them)."""
    subprocess.run(["pkill", "-f", "platform[0-9].xml"], capture_output=True)
    subprocess.run(["pkill", "-f", "emane_jammer_simple.service"], capture_output=True)
    out = subprocess.run(["ip", "-j", "netns", "list"], capture_output=True,
                         text=True).stdout
    for e in json.loads(out or "[]"):
        if e.get("name", "").startswith(f"{PREFIX}-"):
            subprocess.run(["ip", "netns", "del", e["name"]], capture_output=True)
    links = subprocess.run(["ip", "-j", "link", "show"], capture_output=True,
                           text=True).stdout
    for link in json.loads(links or "[]"):
        name = link.get("ifname", "")
        if name.startswith(f"{PREFIX}h"):
            subprocess.run(["ip", "link", "del", name], capture_output=True)
    subprocess.run(["ip", "link", "del", BRIDGE], capture_output=True)


class EmaneTopology:
    def __init__(self, node_ids, run_id="dbg"):
        self.run_id = run_id
        self.node_ids = list(node_ids)
        self.all = self.node_ids + ["jammer"]
        self.ns = {n: f"{PREFIX}-{run_id}-{i}" for i, n in enumerate(self.all)}

    def setup(self):
        self.teardown()
        run("ip", "link", "add", BRIDGE, "type", "bridge")
        run("ip", "link", "set", BRIDGE, "up")
        Path(f"/sys/devices/virtual/net/{BRIDGE}/bridge/multicast_snooping").write_text("0")
        for i, n in enumerate(self.all):
            ns = self.ns[n]
            host, inner = f"{PREFIX}h{i}", "ctrl0"
            run("ip", "netns", "add", ns)
            run("ip", "link", "add", host, "type", "veth", "peer", "name", inner)
            run("ip", "link", "set", host, "master", BRIDGE)
            run("ip", "link", "set", host, "up")
            run("ip", "link", "set", inner, "netns", ns)
            run("ip", "-n", ns, "addr", "add", f"{generate.CTRL_NET}.{i + 1}/24",
                "dev", inner)
            run("ip", "-n", ns, "link", "set", inner, "up")
            run("ip", "-n", ns, "link", "set", "lo", "up")
            run("ip", "-n", ns, "route", "add", "224.0.0.0/4", "dev", inner)

    def teardown(self):
        cleanup_all()


def build_pathloss(scenario):
    import numpy as np
    env = scenario.environment
    hm = np.load(env.heightmap) if env.heightmap else None
    terrain = Terrain(env.extent_m, hm,
                      [FoliageRegion(**f.model_dump()) for f in env.foliage])
    return CompositePathloss(terrain)


def static_positions(scenario):
    """First-waypoint positions; the spike runs a fixed-geometry cell."""
    return {n.id: tuple(n.waypoints[0]) for n in scenario.nodes}


def write_pathloss_feeder(scenario, info, out: Path) -> Path:
    """Emit a standalone script (run with system python in the jammer netns)
    that publishes node<->node and jammer->node pathloss events once."""
    pl = build_pathloss(scenario)
    pos = static_positions(scenario)
    ids = info["node_ids"]
    nem = {ids[i]: info["nem_ids"][i] for i in range(len(ids))}
    freq = info["freq_hz"]

    # node<->node directed pathloss (dB), and jammer->node
    pairs = {}
    for a in ids:
        for b in ids:
            if a != b:
                pairs[(nem[a], nem[b])] = round(
                    pl.loss(pos[a], pos[b], freq).total_db, 2)
    jammer_nem = info["jammer_nem"]
    jpos = tuple(scenario.jammers[0].position) if scenario.jammers else (0, 0)
    jam_pl = {nem[b]: round(pl.loss(jpos, pos[b], freq).total_db, 2) for b in ids}

    feeder = out / "feed_pathloss.py"
    feeder.write_text(
        "import sys, time\n"
        "from emane.events import EventService, PathlossEvent\n"
        f"node_nems = {info['nem_ids']}\n"
        f"pairs = {{ {', '.join(f'({a},{b}):{v}' for (a,b),v in pairs.items())} }}\n"
        f"jammer_nem = {jammer_nem}\n"
        f"jam_pl = {jam_pl}\n"
        "rounds = int(sys.argv[1]) if len(sys.argv) > 1 else 3\n"
        "svc = EventService(('224.1.2.8', 45703, 'ctrl0'))\n"
        "time.sleep(1)\n"
        # republish periodically for the whole run: EMANE applies the latest
        # pathloss state, and republishing covers NEMs that start/miss events
        "for _ in range(rounds):\n"
        "    for rx in node_nems:\n"
        "        ev = PathlossEvent()\n"
        "        for tx in node_nems:\n"
        "            if tx != rx:\n"
        "                ev.append(tx, forward=pairs[(tx,rx)], reverse=pairs[(rx,tx)])\n"
        "        ev.append(jammer_nem, forward=jam_pl[rx], reverse=jam_pl[rx])\n"
        "        svc.publish(rx, ev)\n"
        "    time.sleep(1.0)\n"
        "print('pathloss feed done')\n")
    return feeder


def write_jammer_cfg(out: Path) -> Path:
    cfg = out / "jammer-service.xml"
    cfg.write_text(
        '<emane-jammer-simple-service endpoint="0.0.0.0:45715">\n'
        '  <ota-channel group="224.1.2.8" port="45702" device="ctrl0"/>\n'
        '  <ota-message destination="65535" registration-id="65535" sub-id="1"/>\n'
        '</emane-jammer-simple-service>\n')
    return cfg


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("scenario", nargs="?")
    ap.add_argument("-o", "--out")
    ap.add_argument("--no-agents", action="store_true")
    ap.add_argument("--duration", type=float, default=None)
    ap.add_argument("--jam-start", type=float, default=None)
    ap.add_argument("--jammer-power", type=float, default=None)
    ap.add_argument("--cleanup", action="store_true",
                    help="tear down any leftover topology and exit")
    args = ap.parse_args()
    if os.geteuid() != 0:
        sys.exit("must run as root")
    if args.cleanup:
        cleanup_all()
        print("cleaned up")
        return
    if not args.scenario or not args.out:
        sys.exit("scenario and -o/--out are required")

    scenario = load_scenario(args.scenario)
    duration = args.duration or scenario.duration_s
    jam_start = args.jam_start if args.jam_start is not None else (
        scenario.jammers[0].start_s if scenario.jammers else None)
    jam_power = args.jammer_power if args.jammer_power is not None else (
        scenario.jammers[0].tx_power_dbm if scenario.jammers else 0.0)
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    cfg_dir = out / "emane_cfg"
    info = generate.generate(scenario, cfg_dir)
    feeder = write_pathloss_feeder(scenario, info, cfg_dir)
    jammer_cfg = write_jammer_cfg(cfg_dir)

    topo = EmaneTopology(info["node_ids"])
    procs = []

    def nspopen(nid, *cmd, **kw):
        return subprocess.Popen(["ip", "netns", "exec", topo.ns[nid], *cmd], **kw)

    try:
        topo.setup()
        print(f"topology up: {len(topo.all)} namespaces on {BRIDGE}")
        for i, nid in enumerate(info["node_ids"]):
            log = open(out / f"emane_{nid}.log", "w")
            procs.append(nspopen(
                nid, "emane", str((cfg_dir / f"platform{i}.xml").resolve()),
                "--realtime", "-l", "3",
                "-f", str((out / f"emane_{nid}_daemon.log").resolve()),
                stdout=log, stderr=subprocess.STDOUT))
        # jammer service in the jammer netns
        jlog = open(out / "jammer_service.log", "w")
        procs.append(nspopen(
            "jammer", EMANE_PY, "-m", "emane_jammer_simple.service",
            "emane_jammer_simple.service.plugin.Plugin",
            "--config-file", str(jammer_cfg.resolve()),
            stdout=jlog, stderr=subprocess.STDOUT))
        time.sleep(3)
        for nid in info["node_ids"]:
            r = subprocess.run(["ip", "netns", "exec", topo.ns[nid], "ip", "-br",
                                "addr", "show", "emane0"], capture_output=True, text=True)
            if "emane0" not in r.stdout:
                raise RuntimeError(f"{nid} emane0 missing: {r.stdout}{r.stderr}")
        print("EMANE NEMs up; feeding pathloss events (continuous)")
        # persistent background pathloss feed for the whole run + warmup
        feed_rounds = int(duration) + 20
        procs.append(subprocess.Popen(
            ["ip", "netns", "exec", topo.ns["jammer"], EMANE_PY,
             str(feeder.resolve()), str(feed_rounds)],
            stdout=open(out / "feed.log", "w"), stderr=subprocess.STDOUT))
        # warmup: let pathloss state propagate before any traffic (the 'missing
        # propagation information' drops happen until the first events apply)
        time.sleep(8)

        # connectivity probe: ping node2 from node1 over EMANE OTA
        ping = subprocess.run(
            ["ip", "netns", "exec", topo.ns[info["node_ids"][0]], "ping", "-c", "3",
             "-W", "2", info["tap_ips"][1]], capture_output=True, text=True)
        ok = " 0% packet loss" in ping.stdout
        print(f"OTA ping {info['node_ids'][0]}->{info['node_ids'][1]}: "
              f"{'OK' if ok else 'loss'}  "
              f"{[l for l in ping.stdout.splitlines() if 'packet loss' in l]}")
        (out / "ping.txt").write_text(ping.stdout + ping.stderr)

        if not args.no_agents:
            run_agents(topo, info, scenario, duration, jam_start, jam_power, out)
    finally:
        for p in procs:
            if p.poll() is None:
                p.terminate()
        time.sleep(1)
        for p in procs:
            if p.poll() is None:
                p.kill()
        topo.teardown()


def run_agents(topo, info, scenario, duration, jam_start, jam_power, out):
    members = ",".join(info["node_ids"])
    agents = []
    for i, nid in enumerate(info["node_ids"]):
        others = [f"tcp/{info['tap_ips'][j]}:{scenario.agent.port}"
                  for j in range(len(info["node_ids"])) if j != i]
        cmd = ["ip", "netns", "exec", topo.ns[nid], str(AGENT_BIN),
               "--id", nid, "--listen", f"tcp/{info['tap_ips'][i]}:{scenario.agent.port}",
               "--metrics", str((out / f"agent_{nid}.jsonl").resolve()),
               "--period-ms", str(scenario.agent.period_ms),
               "--duration-s", str(duration),
               "--full-every", str(scenario.agent.full_every),
               "--wait-peers", str(len(info["node_ids"]) - 1),
               "--members", members]
        if not scenario.agent.mls:
            cmd.append("--plaintext")
        for o in others:
            cmd += ["--connect", o]
        agents.append(subprocess.Popen(cmd))
    print(f"agents launched (duration {duration}s, jam at {jam_start}s @ {jam_power} dBm)")

    if jam_start is not None:
        def fire():
            time.sleep(jam_start)
            # jam every hop channel: barrage over the whole band centered on freq
            jctl = subprocess.run(
                ["ip", "netns", "exec", topo.ns["jammer"], "emane-jammer-simple-control",
                 "127.0.0.1:45715", "on", str(info["jammer_nem"]),
                 str(int(info["freq_hz"])), "-p", str(jam_power),
                 "-b", str(int(info["bandwidth_hz"]))],
                capture_output=True, text=True)
            print(f"  jammer ON: {jctl.stdout.strip() or jctl.stderr.strip()[:150]}")
        threading.Thread(target=fire, daemon=True).start()

    for p in agents:
        p.wait(timeout=duration + 30)
    print("agents done ->", out)


if __name__ == "__main__":
    main()
