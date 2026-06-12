# ADR-0001: Hand-rolled channel engine with EMANE-shaped interfaces

**Status:** accepted (with re-evaluation gate after M3)
**Date:** 2026-06-12

## Context

We need real Zenoh/CRDT/MLS binaries running over an emulated RF channel with
jamming. EMANE (NRL) provides battle-tested SINR machinery (per-frequency spectrum
service), an existing jammer (emane-jammer-simple: fixed/sweep/hop-set), and
hardware-in-the-loop/scale paths. A hand-rolled engine (netns + userspace forwarder)
provides full transparency, a modern pixi-native stack, seeded channel replay, and
per-packet drop attribution.

Decisive observation: the project's distinctive models — FHSS hop statistics,
foliage/terrain propagation, EW scenarios, satellite failover, benchmark harness —
are custom code under **either** option (EMANE has no sub-packet FHSS and no foliage
models; it takes external pathloss events). EMANE's unique contributions are generic
SINR bookkeeping (small at N ≤ 5) and HIL/scale (out of scope per README).

## Decision

Build the hand-rolled engine, but keep interfaces EMANE-shaped
(`PathlossProvider: (tx, rx, freq) → dB`; location/pathloss as events) so EMANE can
replace the channel engine later. After M3, run a timeboxed EMANE spike reproducing
one scenario and decide with evidence (see roadmap M-gate).

## Consequences

- (+) Seeded paired-channel replay → variance-controlled middleware comparisons,
  impossible with EMANE's real-time pipeline.
- (+) Per-packet drop attribution (cause labels) → causal benchmark narratives.
- (+) Single modern stack (Python/Rust, pixi, CI-friendly); reactive jammer is a
  parameter, not plumbing.
- (−) We own correctness: a model validation suite (analytical link budgets,
  published curves) is mandatory, ~20–30 % of channel-engine effort.
- (−) No "we used EMANE" citability; mitigated by validation artifacts.
- (−) No HIL/multi-machine path now; hedged by the interface shape.
