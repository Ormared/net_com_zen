# FEC + interleaving flips the slow-vs-fast-hop result

M3 found that against partial-band jamming **without** coding, slow FHSS hopping
beats fast hopping: a packet survives only if *every* one of its dwells is clean,
so spreading a packet across more dwells (fast hopping) only adds more chances to
be hit. That is correct physics for an uncoded waveform — and it is exactly what
forward error correction + interleaving is designed to defeat in real EW radios.

## The model (`radio.fec_fraction`)

With an interleaved erasure code that recovers a fraction `f` of erased dwells, a
`k`-dwell packet survives jamming iff at most `e = floor(f·k)` dwells are jammed:
`P(survive) = P(B ≤ e)`, `B ~ Binomial(k, ρ_eff)`. `f = 0` is the uncoded model.
See [models.md](../models.md); validated against dwell-level Monte-Carlo.

## Survival vs hop rate (spot jammer, 16 % of dwells jammed, 64 B packet)

| hop/s | dwells k | survive, no FEC | survive, FEC f=0.4 |
|---|---|---|---|
| 10 | 1 | 0.840 | 0.840 |
| 500 | 2 | 0.706 | 0.706 |
| 1000 | 3 | 0.593 | **0.931** |
| 2000 | 5 | 0.418 | **0.968** |

FEC needs `floor(f·k) ≥ 1` (here `k ≥ 3`) before it can correct a dwell; past
that the coded curve **rises** with hop rate while the uncoded curve falls.

## Full-stack confirmation (resilience_4node, spot 60 dBm, udp+state, jammer active)

| hop/s | FEC | frame PDR | update delivery | mean AoI |
|---|---|---|---|---|
| 100 | 0.0 | 0.736 | 0.948 | 0.32 s |
| 1000 | 0.0 | **0.372** | 0.709 | 0.77 s |
| 100 | 0.4 | 0.754 | 0.955 | 0.31 s |
| 1000 | 0.4 | **0.924** | 0.996 | 0.26 s |

**Uncoded:** fast hopping (1000) is far worse than slow (100) — 0.37 vs 0.74
frame PDR. **With FEC f=0.4:** fast hopping becomes the best configuration —
0.92 PDR (2.5× the uncoded value) and the lowest AoI, beating slow hopping.

## Takeaway

The optimal FHSS hop rate is **coding-dependent**: report hop-rate results
together with the FEC rate. For a coded waveform under partial-band jamming, fast
hopping + interleaving is the resilient choice — combine with the established
udp/best-effort + state-based sync for the full resilient stack.

## Caveat

The FEC model assumes ideal interleaving (independent dwell erasures) and a
hard erasure-fraction threshold; it does not model coding gain on the thermal
SINR or finite-blocklength effects. Good enough to capture the hop-rate/coding
interaction; a soft-decision PHY model would refine the absolute numbers.
