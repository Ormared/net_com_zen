"""M5.2 — routing-policy comparison over the per-tick link-state table.

Model-level, not a protocol (m5-plan.md): no MANET daemon runs and no packet
crosses a netns. Each policy consumes the same per-tick ``build_table()``
output the engine trusts everywhere else, and delivery is Monte-Carlo sampled
from ONE seeded uniform per (tick, directed link) — so every policy sees the
identical channel realization and differences are pure routing, the same
paired-seed discipline the sweep harness uses across cells.

Policies
--------
- ``direct``     one broadcast by the source; a destination hears it iff its
                 own link realization succeeded. The current engine behaviour.
- ``linkstate``  centralised shortest-viable-path upper bound for single-path
                 routing: per tick, Dijkstra on ``-log(delivery_prob)`` edge
                 weights; the update reaches a destination iff every hop of its
                 best path succeeded this tick. Cost counts one transmission
                 per traversed hop per destination (unicast forwarding — an RF
                 broadcast implementation could share hops between
                 destinations, so this is the cost upper bound).
- ``flood``      epidemic reliability upper bound: every node reached this
                 tick rebroadcasts once; a destination is reached iff ANY path
                 of successful links exists. Cost = number of transmitting
                 nodes.
- ``gossip``     probabilistic flood: non-source nodes rebroadcast with
                 probability ``forward_p`` while the update's hop count is
                 below ``ttl``. The realistic middle ground.

Within-tick multi-hop assumes forwarding latency (µs–ms) is negligible against
the 100 ms tick, mirroring how the engine treats per-packet delay. Every
policy draws a link's Bernoulli from the same uniform, so per-realization
``flood ⊇ linkstate`` and ``flood ⊇ direct`` hold by construction (a
successful path is a subgraph of the successful-link graph).

Delivery probabilities are evaluated at a fixed reference payload
(``REF_LENGTH_BYTES``, matching the linkstate log's reference length).

Run a sweep (no root needed — nothing crosses a netns):

    python -m netcom_zen.routing scenarios/sweep_routing.yaml -o results/routing
"""
from __future__ import annotations

import heapq
import math
import zlib
from dataclasses import dataclass, field

import numpy as np

from .channel.linkstate import LinkState, build_table
from .config import Scenario

POLICIES = ("direct", "linkstate", "flood", "gossip")

# Reference payload for delivery_prob — the agent's CRDT updates are a few
# hundred bytes (matches the linkstate log's reference length).
REF_LENGTH_BYTES = 200


def _link_uniform(seed: int, tick: int, src: str, dst: str) -> float:
    """One paired uniform per (tick, directed link): every policy judges this
    link with the same draw, so policy deltas are routing, not luck."""
    rng = np.random.default_rng([seed, tick,
                                 zlib.crc32(f"{src}>{dst}".encode())])
    return float(rng.random())


def _probs(table: dict[tuple[str, str], LinkState],
           length_bytes: int) -> dict[tuple[str, str], float]:
    return {pair: ls.delivery_prob(length_bytes) for pair, ls in table.items()}


def _realized(probs: dict[tuple[str, str], float], seed: int,
              tick: int) -> dict[tuple[str, str], bool]:
    return {(s, d): _link_uniform(seed, tick, s, d) < p
            for (s, d), p in probs.items()}


def best_paths(probs: dict[tuple[str, str], float],
               src: str) -> dict[str, list[str]]:
    """Dijkstra on -log(delivery_prob): the max-product-probability path from
    src to every reachable destination. Zero-probability edges are unusable."""
    adj: dict[str, list[tuple[str, float]]] = {}
    for (a, b), p in probs.items():
        if p > 0.0:
            adj.setdefault(a, []).append((b, -math.log(p)))
    dist: dict[str, float] = {src: 0.0}
    prev: dict[str, str] = {}
    heap: list[tuple[float, str]] = [(0.0, src)]
    while heap:
        d, u = heapq.heappop(heap)
        if d > dist.get(u, math.inf):
            continue
        for v, w in adj.get(u, ()):
            nd = d + w
            if nd < dist.get(v, math.inf):
                dist[v], prev[v] = nd, u
                heapq.heappush(heap, (nd, v))
    paths: dict[str, list[str]] = {}
    for dst in dist:
        if dst == src:
            continue
        node, hops = dst, [dst]
        while node != src:
            node = prev[node]
            hops.append(node)
        paths[dst] = hops[::-1]  # src ... dst
    return paths


@dataclass
class TickOutcome:
    """One source's update on one tick: which destinations it reached and how
    many transmissions the policy spent getting it there."""
    reached: set[str]
    transmissions: int


def route_tick(policy: str, src: str, nodes: list[str],
               probs: dict[tuple[str, str], float],
               realized: dict[tuple[str, str], bool],
               rng: np.random.Generator, forward_p: float = 0.6,
               ttl: int = 4) -> TickOutcome:
    """Propagate one update from src under a policy, on this tick's realized
    link successes. rng drives only policy-internal choices (gossip forwarding
    coins), never the channel — the channel is `realized`, shared by all."""
    others = [n for n in nodes if n != src]
    if policy == "direct":
        return TickOutcome({d for d in others if realized[(src, d)]}, 1)
    if policy == "linkstate":
        # Dijkstra's prev[] makes the union of best paths a tree rooted at
        # src: hops shared between destinations are transmitted once, and
        # _tree_transmissions charges only edges whose upstream chain
        # succeeded (a relay that never got the update can't forward it)
        reached: set[str] = set()
        edges: set[tuple[str, str]] = set()
        for dst, path in best_paths(probs, src).items():
            path_edges = list(zip(path, path[1:]))
            edges.update(path_edges)
            if all(realized[e] for e in path_edges):
                reached.add(dst)
        return TickOutcome(reached, _tree_transmissions(edges, realized, src))
    if policy in ("flood", "gossip"):
        reached_set = {src}
        transmitters = [src]
        tx = 0
        depth = 0
        while transmitters and (policy == "flood" or depth < ttl):
            tx += len(transmitters)
            frontier: list[str] = []
            for u in transmitters:
                for v in nodes:
                    if v in reached_set or v == u:
                        continue
                    if realized.get((u, v), False):
                        reached_set.add(v)
                        frontier.append(v)
            if policy == "gossip":
                frontier = [v for v in frontier if rng.random() < forward_p]
            transmitters = frontier
            depth += 1
        return TickOutcome(reached_set - {src}, tx)
    raise ValueError(f"unknown policy {policy!r}")


def _tree_transmissions(edges: set[tuple[str, str]],
                        realized: dict[tuple[str, str], bool],
                        src: str) -> int:
    """Transmissions actually sent on a forwarding tree: an edge is sent iff
    every edge on its chain back to src succeeded (a relay that never received
    the update cannot forward it)."""
    sent = 0
    have = {src}
    # edges form a tree rooted at src; propagate in BFS order
    remaining = set(edges)
    progress = True
    while progress:
        progress = False
        for (a, b) in list(remaining):
            if a in have:
                remaining.discard((a, b))
                sent += 1
                if realized[(a, b)]:
                    have.add(b)
                progress = True
    return sent


@dataclass
class PolicyMetrics:
    updates_offered: int = 0          # (update, destination) pairs
    updates_delivered: int = 0
    transmissions: int = 0
    # AoI bookkeeping: per (observer, source) tick of last delivery
    _last: dict[tuple[str, str], int] = field(default_factory=dict)
    _aoi_sum: float = 0.0
    _aoi_samples: int = 0

    def sample_aoi(self, tick: int, dt: float, pairs) -> None:
        for obs, src in pairs:
            last = self._last.get((obs, src))
            if last is not None:
                self._aoi_sum += (tick - last) * dt
                self._aoi_samples += 1

    def summary(self) -> dict:
        return {
            "update_delivery": (self.updates_delivered / self.updates_offered
                                if self.updates_offered else None),
            "mean_aoi_s": (self._aoi_sum / self._aoi_samples
                           if self._aoi_samples else None),
            "transmissions": self.transmissions,
            "tx_per_delivered": (self.transmissions / self.updates_delivered
                                 if self.updates_delivered else None),
        }


def evaluate(scenario: Scenario, policies: tuple[str, ...] = POLICIES,
             period_ticks: int = 5, forward_p: float = 0.6, ttl: int = 4,
             length_bytes: int = REF_LENGTH_BYTES) -> dict[str, dict]:
    """Run every policy over the scenario's mobility/jammer timeline on paired
    channel realizations. Returns {policy: metrics summary}."""
    from .orchestrator import build_world  # deferred: orchestrator pulls netns

    world = build_world(scenario)
    nodes = [n.id for n in scenario.nodes]
    dt = 1.0 / scenario.tick_hz
    n_ticks = int(scenario.duration_s * scenario.tick_hz)
    metrics = {p: PolicyMetrics() for p in policies}
    # gossip coins: one independent stream per policy, seeded — deterministic
    # but uncorrelated with the channel draws
    coin = {p: np.random.default_rng([scenario.seed, zlib.crc32(p.encode())])
            for p in policies}
    pairs = [(o, s) for o in nodes for s in nodes if o != s]
    try:
        for tick in range(n_ticks):
            poses = world.mobility.step(dt)
            positions = {nid: (p.x, p.y) for nid, p in poses.items()}
            t = tick * dt
            table = build_table(positions, world.jammers, t, scenario.radio,
                                world.pathloss,
                                command_id=scenario.command_id,
                                satellite=scenario.satellite)
            probs = _probs(table, length_bytes)
            realized = _realized(probs, scenario.seed, tick)
            publish = tick % period_ticks == 0
            for p in policies:
                m = metrics[p]
                if publish:
                    for src in nodes:
                        out = route_tick(p, src, nodes, probs, realized,
                                         coin[p], forward_p, ttl)
                        m.updates_offered += len(nodes) - 1
                        m.updates_delivered += len(out.reached)
                        m.transmissions += out.transmissions
                        for dst in out.reached:
                            m._last[(dst, src)] = tick
                m.sample_aoi(tick, dt, pairs)
    finally:
        world.mobility.close()
    return {p: metrics[p].summary() for p in policies}


def _plot(rows: list[dict], out_dir, x_axis: str,
          facet_axis: str | None = None) -> None:
    """One figure per (metric, facet value); lines = policies. flood and
    linkstate often coincide exactly, so linkstate is drawn dashed on top."""
    import collections
    import statistics

    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    styles = {"direct": dict(marker="o"), "flood": dict(marker="s"),
              "gossip": dict(marker="^"),
              "linkstate": dict(marker="x", linestyle="--")}
    facets = (sorted({r[facet_axis] for r in rows}) if facet_axis else [None])
    for metric, label in (("update_delivery", "End-to-end update delivery"),
                          ("mean_aoi_s", "Mean age of information (s)"),
                          ("tx_per_delivered", "Transmissions per delivery")):
        for fv in facets:
            sel = [r for r in rows if fv is None or r[facet_axis] == fv]
            fig, ax = plt.subplots(figsize=(7, 4.5))
            # linkstate last: it often coincides with flood exactly and the
            # dashed overlay must land on top to stay visible
            for policy in ("direct", "flood", "gossip", "linkstate"):
                pts = collections.defaultdict(list)
                for r in sel:
                    if r["policy"] == policy and r.get(metric) is not None:
                        pts[r[x_axis]].append(r[metric])
                if not pts:
                    continue
                xs = sorted(pts)
                means = [statistics.mean(pts[x]) for x in xs]
                err = [statistics.stdev(pts[x]) if len(pts[x]) > 1 else 0
                       for x in xs]
                ax.errorbar(xs, means, yerr=err, capsize=3, label=policy,
                            **styles[policy])
            ax.set_xlabel(x_axis)
            ax.set_ylabel(label)
            if fv is not None:
                ax.set_title(f"{facet_axis} = {fv}")
            ax.grid(True, alpha=0.3)
            ax.legend()
            fig.tight_layout()
            suffix = f"_{facet_axis.split('.')[-1]}={fv}" if fv is not None else ""
            fig.savefig(out_dir / f"routing_{metric}{suffix}.png", dpi=130)
            plt.close(fig)


def main() -> None:
    """Sweep-style runner: cartesian axes x seeds over a base scenario, one
    evaluate() per cell, every policy on paired realizations. Root-free."""
    import argparse
    import json
    from pathlib import Path

    import copy

    import yaml

    from .harness.sweep import apply_override, expand

    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("sweep")
    ap.add_argument("-o", "--out", required=True)
    ap.add_argument("--x", default=None,
                    help="plot x axis (default: first sweep axis)")
    args = ap.parse_args()
    sweep_path = Path(args.sweep)
    sweep = yaml.safe_load(sweep_path.read_text())
    base = yaml.safe_load((sweep_path.parent / sweep["base"]).read_text())
    routing_kw = sweep.get("routing", {})
    out_dir = Path(args.out)
    out_dir.mkdir(parents=True, exist_ok=True)

    cells = expand(sweep)
    rows: list[dict] = []
    for i, (overrides, seed) in enumerate(cells, 1):
        cfg = copy.deepcopy(base)
        for k, v in overrides.items():
            apply_override(cfg, k, v)
        cfg["seed"] = seed
        scenario = Scenario.model_validate(cfg)
        print(f"[{i}/{len(cells)}] {overrides} seed={seed}")
        for policy, summary in evaluate(scenario, **routing_kw).items():
            rows.append({**overrides, "seed": seed, "policy": policy,
                         **summary})
    (out_dir / "summary.json").write_text(json.dumps(rows, indent=2))
    axes = [a for a, vals in sweep.get("axes", {}).items()
            if len(set(map(str, vals))) > 1]
    x_axis = args.x or (axes[0] if axes else "seed")
    facet = next((a for a in axes if a != x_axis), None)
    _plot(rows, out_dir, x_axis, facet)
    print(f"{len(rows)} rows -> {out_dir/'summary.json'}")


if __name__ == "__main__":
    main()
