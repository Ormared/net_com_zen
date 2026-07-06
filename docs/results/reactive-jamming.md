# Fast hopping outruns a reactive (follower) jammer

The [FEC-flip result](fec-flips-hop-rate.md) showed that against **partial-band**
jamming, hop rate only helps once forward error correction is present: FEC spreads
a fixed jammed fraction across many dwells. A **reactive** (follower) jammer is a
harder, qualitatively different threat — it senses the active transmission and
retunes onto its channel, so it is always on the *right* channel when it locks.
The question this experiment answers: is hop rate still the lever, and is FEC even
needed?

## The model (`kind: reactive`)

A follower can only spoil a dwell if it retunes before the transmitter hops away.
Instead of tick-by-tick sub-dwell timing (the engine ticks far coarser than a 1 ms
dwell), the timing race collapses to a per-dwell **lock probability** that feeds
the same binomial-CDF survival model as partial-band jamming:

- `ρ_eff = detection_prob · f_overlap`,
  `f_overlap = max(0, (T_dwell − τ) / T_dwell)`, `T_dwell = 1 / hop_rate`.
- Once `T_dwell ≤ τ` (the hop period drops below the sense+tune latency),
  `f_overlap = 0`: the transmitter has moved on before the jammer locks, and the
  packet is untouched — **no FEC required**.
- The jammer is position-aware on both ends: `build_table` gates the lock on the
  source's SINR at the jammer clearing `sense_threshold_db` (it must *hear* the
  transmitter), and its in-channel power at the receiver goes through the same
  propagation stack (it must *reach* the destination).

See [models.md](../models.md); validated against a direct time-domain dwell
simulation in [validation/test_reactive_jammer.py](../../validation/test_reactive_jammer.py).

## Survival vs hop rate (follower τ = 2 ms, 255 B @ 250 kbps, detection 0.95)

| hop/s | dwells k | lock ρ | survive, no FEC | survive, FEC f=0.25 |
|---|---|---|---|---|
| 50 | 1 | 0.855 | 0.145 | 0.145 |
| 100 | 1 | 0.760 | 0.240 | 0.240 |
| 200 | 2 | 0.570 | 0.185 | 0.185 |
| 500 | 5 | **0.000** | **1.000** | **1.000** |
| 1000 | 9 | 0.000 | 1.000 | 1.000 |

The transition is a **cliff at `T_dwell = τ`**, not the gradual FEC-enabled climb
seen against partial-band jamming. Below the cliff (slow hopping) the follower
catches most dwells and FEC barely helps — with `k ≤ 2`, `floor(0.25·k) = 0`, so
there is nothing to correct. Above it, hop rate alone gives total immunity.

FEC still matters in the **transition band** where `T_dwell` is only slightly
above `τ` (the jammer catches part of each dwell): at τ = 1 ms, hop = 500/s
(`T_dwell = 2 ms`), `ρ = 0.475` and FEC lifts survival 0.04 → 0.22. But the
decisive move is crossing the cliff.

## Full-stack confirmation (reactive_4node, follower at swarm centre, udp+state)

Four vehicles share MLS-encrypted CRDT state; a follower jammer sits at the swarm
centre and activates at t = 15 s. Metrics are averaged over 2 seeds; PDR is the
frame delivery ratio while the jammer is active (`frame_pdr_postjam`). FEC rows
(`fec_fraction = 0.25`) are shown — the `fec = 0` rows are within noise of these,
so FEC is omitted for clarity.

| hop/s | τ (sense+tune) | T_dwell | frame PDR (jammer on) | update delivery | mean AoI |
|---|---|---|---|---|---|
| 100 | 1 ms | 10 ms | 0.141 | 0.25 | 9.5 s |
| 100 | 5 ms | 10 ms | 0.453 | 0.30 | 8.4 s |
| 500 | 1 ms | 2 ms | 0.233 | 0.25 | 9.5 s |
| 500 | 5 ms | 2 ms | **1.000** | **0.89** | **0.27 s** |
| 1000 | 1 ms | 1 ms | **1.000** | **0.90** | **0.27 s** |
| 1000 | 5 ms | 1 ms | **1.000** | **0.87** | **0.27 s** |

The end-to-end numbers land exactly on the cliff `T_dwell = τ`: hop = 500/s
(2 ms dwell) is still crippled by a fast τ = 1 ms follower (PDR 0.23) but
**completely immune** to a τ = 5 ms one (PDR 1.00, AoI 0.27 s — the un-jammed
baseline); hop = 1000/s (1 ms dwell) is immune to both. When the swarm outruns the
jammer, update delivery and AoI snap back to their un-jammed values — the follower
lands nothing. FEC barely moves these numbers (state snapshots are small, so a
packet spans few dwells and `floor(f·k) = 0`), confirming that against a follower
the lever is hop rate, not coding.

## Takeaway

Against a follower jammer the lever is **raw hop rate crossing `1/τ`**, not coding:
hop faster than the adversary can sense-and-retune and it never lands a dwell.
This is complementary to the FEC-flip result — together they say the resilient
FHSS configuration is *fast hopping*, for two independent reasons (outrunning the
follower, and — with FEC — surviving partial-band). The practical corollary is an
arms race on latency: the defender wins by pushing `T_dwell` below the jammer's
achievable sense+tune time `τ`.

## Caveat

`detection_prob` is a configured sensing-reliability ceiling, not a modelled
energy detector; `f_overlap` maps partial dwell coverage to erasure probability
linearly (a hard intra-dwell threshold / soft-decision PHY would refine the
transition-band numbers). The cliff itself — immunity once `T_dwell ≤ τ` — is a
timing identity and independent of those refinements.
