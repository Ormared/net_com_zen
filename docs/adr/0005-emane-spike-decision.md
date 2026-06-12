# ADR-0005: EMANE spike outcome — keep hand-rolled engine, retain EMANE for cross-validation

**Status:** accepted
**Date:** 2026-06-12
**Supersedes the re-evaluation gate in** [ADR-0001](0001-channel-emulation-hand-rolled-vs-emane.md)

## Context

ADR-0001 committed to the hand-rolled channel engine with EMANE-shaped
interfaces and a re-evaluation gate after M3: reproduce one jamming cell in
EMANE + emane-jammer-simple, compare, then proceed or pivot. The spike is done
(see [emane_spike/FINDINGS.md](../../emane_spike/FINDINGS.md)). EMANE 1.5.3 was
installed, a 4-node RF Pipe emulation was driven by our own `CompositePathloss`
(via EMANE PathlossEvents), and the same Rust agent binary ran over EMANE TAPs
against the matching hand-rolled cell.

## Evidence

- **Correctness:** where both engines connect (v1–v3), mean AoI agrees in order
  of magnitude (EMANE 0.6–2.9 s vs hand-rolled ~3.1 s) and pub/recv volumes
  match. The hand-rolled engine is not producing artifactual M3 results.
- **Ergonomics:** a single EMANE cell took six non-obvious config fixes
  (NEM-id placement, `bitrate`/`txpower` typing, continuous pathloss feed +
  warmup, `frequencyofinterest`, bridge multicast), each found only via
  debug-level daemon logs. The hand-rolled engine needs no per-run tuning and is
  covered by the existing unit/validation suites.
- **Capability gap:** EMANE provides no per-packet drop attribution and no
  seeded channel replay — the two properties that make our benchmark
  comparisons variance-controlled and causal (benchmarking.md).
- **EMANE's unique value showed up too:** it is more pessimistic on the weak
  foliage-edge link (v4, 118 dB), where its spectrum monitor makes the MLS
  handshake marginal while our analytic FSK model treats the link as clean.

## Decision

1. **Primary engine: hand-rolled.** Merge `m1-channel-core` after this ADR.
2. **Retain EMANE as a cross-validation harness.** `emane_spike/` stays in-tree
   and working; run it periodically to validate hand-rolled results at specific
   operating points, especially weak/foliage-edge links.
3. **Open follow-ups (tracked, not blocking the merge):**
   - *Propagation calibration:* investigate the weak-link divergence; our
     Weissberger + FSK stack likely under-models tail loss/fading that EMANE
     captures. Candidate axis for the Sionna RT / recorded-trace backend
     ([ADR-0004](0004-empirical-propagation-first.md)).
   - *MLS handshake hardening:* the queryable welcome exchange is not
     loss-tolerant (inflated ~10× under EMANE latency, failed on the weak node).
     Add retry/backoff, adaptive welcome timeout, or gossiped welcome — an agent
     fix independent of the channel engine.

## Consequences

- (+) Keep the fast, reproducible, fully-attributed hand-rolled engine as the
  benchmark workhorse, now externally validated against a government-pedigree
  emulator.
- (+) EMANE remains available for credibility-sensitive cross-checks without
  committing the benchmark harness to its tooling.
- (−) We carry a second, heavier emulation path; mitigated by keeping it to a
  cross-validation role, not the inner benchmark loop.
- (−) A real calibration gap (weak-link loss) is now documented and must be
  addressed before weak-link results are quoted as quantitative.
