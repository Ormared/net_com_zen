# EMANE Spike — Findings

Date: 2026-06-12. Decision gate from [roadmap.md](../docs/roadmap.md) M-gate and
[ADR-0001](../docs/adr/0001-channel-emulation-hand-rolled-vs-emane.md).

Goal: reproduce one M3 jamming cell in EMANE + emane-jammer-simple and compare
numbers and ergonomics against the hand-rolled channel engine before merging
`m1-channel-core`.

## What was built

`emane_spike/` runs the **same Rust agent binary** (M2/M3) over an EMANE RF Pipe
emulation, fed the **same `CompositePathloss`** our engine uses (EMANE's
`propagationmodel=precomputed` consumes our pathloss as PathlossEvents — the
EMANE-shaped interface ADR-0001 designed for). Topology: one netns per node +
jammer, a shared bridge carrying OTA + event multicast, a virtual TAP per NEM.

- `generate.py` — emits EMANE platform/NEM/MAC/PHY/transport XML from a scenario.
- `run_emane.py` — builds the netns+bridge topology, runs EMANE + agents, feeds
  pathloss continuously, toggles the jammer.
- `compare.py` — runs the matching hand-rolled cell and tabulates both.

## Ergonomics (the setup cost is itself a result)

Getting a single EMANE cell to pass an OTA ping took **six** non-obvious fixes,
each found only by reading daemon logs at debug level:

1. `id` attribute belongs on the platform's `<nem definition= id=>` reference,
   not the standalone NEM definition (XML validity error).
2. `transvirtual` `bitrate` is `uint64` — `0.0` is rejected, must be `0`.
3. TX power must be set per-NEM (`txpower`) or every link is 0 dBm.
4. Pathloss events must be fed **continuously** with a warmup; the first events
   take ~4 s to apply and all earlier OTA frames drop as "propagation model
   missing information".
5. `frequencyofinterest` must be set on the PHY or the receiver demodulates
   nothing ("frequency not in frequency of interest list") — the single most
   opaque failure; everything looked configured but no packet was received.
6. Single-host multi-NEM multicast needs `multicast_snooping=0` on the bridge
   and a `224.0.0.0/4` route per namespace.

By contrast the hand-rolled engine needed no per-run tuning — it is the same
code path the unit/validation suites already cover. EMANE also produces no
per-packet drop-attribution log and no seeded-replay capability; both are
first-class in the hand-rolled engine.

## Numbers (resilience_4node, barrage 40 dBm at t=20 s, 40 s, same agent)

| node | engine | MLS | handshake | recvs | peers | mean AoI |
|---|---|---|---|---|---|---|
| v1 | EMANE | y | 9720 ms | 134 | 2 | 0.60 s |
| v1 | hand-rolled | y | 362 ms | 172 | 3 | 3.16 s |
| v2 | EMANE | y | 1011 ms | 96 | 2 | 2.94 s |
| v2 | hand-rolled | y | 666 ms | 182 | 3 | 3.12 s |
| v3 | EMANE | y | 4203 ms | 138 | 2 | 1.70 s |
| v3 | hand-rolled | y | 663 ms | 161 | 3 | 3.20 s |
| v4 | EMANE | **N (failed)** | — | 0 | 0 | — |
| v4 | hand-rolled | y | 664 ms | 102 | 3 | 5.88 s |

## Findings

1. **Agreement where both connect.** For v1–v3 the mean AoI agrees in order of
   magnitude (EMANE 0.6–2.9 s, hand-rolled ~3.1 s) and pub/recv volumes match.
   This validates the hand-rolled engine's correctness for connected nodes — the
   core M3 result (barrage degrades AoI, hop rate ranking) is not an artifact of
   our simplified PHY.

2. **Weak-link divergence (the headline).** v4 sits behind the 100 m forest
   strip (118 dB pathloss). Our analytic FSK model gives ~22 dB SINR → near-zero
   PER, so v4 joins and shares state. EMANE's RF Pipe + spectrum monitor makes
   that same link **marginal**: v4's MLS handshake takes **11.4 s even with no
   jammer** (vs ~0.6 s for close nodes) and fails outright once the jammer adds
   stress. EMANE is more pessimistic at the foliage edge. This is a **calibration
   target**, not a refutation — our Weissberger + FSK stack likely under-models
   tail loss / fading that EMANE's spectrum monitor captures.

3. **Handshake fragility (engine-independent).** The MLS handshake's multi-round
   queryable exchange is not loss-tolerant; realistic OTA latency inflated it
   ~10× and pushed the weak node past timeout. This is an **agent hardening item
   for M2** regardless of which channel engine we keep (retry/backoff, longer or
   adaptive welcome timeout, gossiped welcome).

## Decision

See [ADR-0005](../docs/adr/0005-emane-spike-decision.md): **proceed with the
hand-rolled engine as primary; retain EMANE as a periodic cross-validation
harness** (the spike tooling stays in-tree and works). Track the weak-link
calibration and the MLS-handshake hardening as follow-ups.
