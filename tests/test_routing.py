import numpy as np
import pytest

from netcom_zen.config import Scenario
from netcom_zen.routing import (POLICIES, best_paths, evaluate, route_tick,
                                _tree_transmissions)

NODES = ["a", "b", "c", "d"]


def full_probs(default=1.0, **overrides):
    """Directed complete graph; overrides keyed 'a>b'."""
    probs = {(s, d): default for s in NODES for d in NODES if s != d}
    for k, v in overrides.items():
        s, d = k.split(">")
        probs[(s, d)] = v
    return probs


def all_up(probs, down=()):
    return {pair: pair not in down for pair in probs}


def rng():
    return np.random.default_rng(7)


def test_best_paths_prefers_reliable_relay():
    # a->c direct is poor (0.1); a->b->c is 0.9*0.9=0.81 — relay wins
    probs = full_probs(default=0.9, **{"a>c": 0.1})
    paths = best_paths(probs, "a")
    assert paths["c"] == ["a", "b", "c"] or paths["c"] == ["a", "d", "c"]
    assert paths["b"] == ["a", "b"]


def test_best_paths_skips_dead_edges():
    probs = {("a", "b"): 0.0, ("a", "c"): 1.0, ("c", "b"): 1.0}
    assert best_paths(probs, "a")["b"] == ["a", "c", "b"]


def test_direct_reaches_only_realized_links():
    probs = full_probs()
    realized = all_up(probs, down=[("a", "c")])
    out = route_tick("direct", "a", NODES, probs, realized, rng())
    assert out.reached == {"b", "d"} and out.transmissions == 1


def test_flood_routes_around_dead_direct_link():
    probs = full_probs()
    # a can't reach c or d directly, but a->b works and b's links work
    realized = all_up(probs, down=[("a", "c"), ("a", "d")])
    out = route_tick("flood", "a", NODES, probs, realized, rng())
    assert out.reached == {"b", "c", "d"}
    assert out.transmissions == 4  # every reached node (incl. src) broadcasts


def test_flood_contains_every_other_policy_per_realization():
    # random probs + realizations: flood's reach must be a superset
    gen = np.random.default_rng(42)
    for _ in range(25):
        probs = {(s, d): float(gen.random())
                 for s in NODES for d in NODES if s != d}
        realized = {pair: bool(gen.random() < p) for pair, p in probs.items()}
        flood = route_tick("flood", "a", NODES, probs, realized, rng()).reached
        for policy in ("direct", "linkstate", "gossip"):
            out = route_tick(policy, "a", NODES, probs, realized, rng())
            assert out.reached <= flood, (policy, out.reached, flood)


def test_linkstate_charges_only_deliverable_tree_edges():
    # a->b fails: the b->c hop is never transmitted (relay never got it)
    edges = {("a", "b"), ("b", "c")}
    realized = {("a", "b"): False, ("b", "c"): True}
    assert _tree_transmissions(edges, realized, "a") == 1


def test_gossip_ttl_limits_depth():
    # chain a->b->c->d with only chain links up; ttl=1: one round after src
    probs = {("a", "b"): 1.0, ("b", "c"): 1.0, ("c", "d"): 1.0}
    realized = all_up(probs)
    out = route_tick("gossip", "a", ["a", "b", "c", "d"], probs, realized,
                     rng(), forward_p=1.0, ttl=1)
    assert out.reached == {"b"}
    out = route_tick("gossip", "a", ["a", "b", "c", "d"], probs, realized,
                     rng(), forward_p=1.0, ttl=3)
    assert out.reached == {"b", "c", "d"}


CHAIN_SCENARIO = {
    "name": "routing-chain", "duration_s": 6.0, "tick_hz": 10.0, "seed": 3,
    "radio": {"freq_hz": 2.4e9, "bandwidth_hz": 250e3, "tx_power_dbm": 10.0,
              "data_rate_bps": 250e3,
              "hop": {"n_channels": 50, "hop_rate_hz": 100.0}},
    # 3 static nodes in a line; tx power low enough that the far pair's
    # direct link is marginal while neighbours are solid
    "nodes": [{"id": "v1", "waypoints": [[100, 500]]},
              {"id": "v2", "waypoints": [[800, 500]]},
              {"id": "v3", "waypoints": [[1500, 500]]}],
    "environment": {"extent_m": [2000, 1000]},
}


def test_evaluate_deterministic_and_ordered():
    scenario = Scenario.model_validate(CHAIN_SCENARIO)
    r1 = evaluate(scenario)
    r2 = evaluate(Scenario.model_validate(CHAIN_SCENARIO))
    assert r1 == r2  # same seed -> identical channel + policy draws
    assert set(r1) == set(POLICIES)
    # multi-hop must beat direct on this geometry (far pair needs the relay)
    assert r1["flood"]["update_delivery"] > r1["direct"]["update_delivery"]
    assert (r1["linkstate"]["update_delivery"]
            >= r1["direct"]["update_delivery"])
    # reliability has a bandwidth price
    assert (r1["flood"]["tx_per_delivered"]
            > r1["direct"]["tx_per_delivered"] * 0.99)
    # AoI: flood's extra deliveries must not make staleness worse
    assert r1["flood"]["mean_aoi_s"] <= r1["direct"]["mean_aoi_s"] + 1e-9


def test_realization_is_seed_and_tick_dependent():
    # (this geometry's link probs are saturated 0/1, so seed sensitivity is
    # asserted where it lives: the per-(tick, link) paired uniform)
    from netcom_zen.routing import _link_uniform, _realized
    u = {s: _link_uniform(s, tick=5, src="a", dst="b") for s in range(20)}
    assert len(set(u.values())) == 20  # distinct across seeds
    t = {k: _link_uniform(1, tick=k, src="a", dst="b") for k in range(20)}
    assert len(set(t.values())) == 20  # distinct across ticks
    # a marginal link flips with the seed at fixed tick
    probs = {("a", "b"): 0.5}
    outcomes = {_realized(probs, seed, 0)[("a", "b")] for seed in range(30)}
    assert outcomes == {True, False}
