# ADR-0003: Rust node agent, Python simulation side

**Status:** accepted
**Date:** 2026-06-12

## Context

The node agent (software under test) needs Zenoh, a CRDT, and MLS. Zenoh is
Rust-native with good Python bindings; automerge is a Rust core with thinner Python
bindings; OpenMLS is Rust-only — there is no credible Python MLS implementation.
The deployment target (Intel N100 vehicles) would run the Rust libraries natively.

## Decision

- **Agent in Rust:** zenoh, automerge, openmls used natively — the exact libraries
  that would deploy, so benchmark numbers are deployment-representative and there is
  no binding-maturity risk on the critical path.
- **Simulation/harness in Python:** mobility, propagation, EW, channel engine,
  metrics, sweeps — iteration speed matters more than throughput at telemetry rates
  (channel forwarder on uvloop; revisit in Rust only if timing-integrity monitoring
  flags it).
- Both toolchains managed by pixi.

## Consequences

- (+) MLS is real (OpenMLS), not stubbed or wrapped.
- (+) Agent binary is a candidate seed for the eventual vehicle software.
- (−) Two toolchains; Rust compile times in the inner loop.
