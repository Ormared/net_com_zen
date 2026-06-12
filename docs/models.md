# RF & EW Models

All models live behind interfaces in `sim/propagation/` and `sim/ew/` and are
validated against analytical results and published curves (see `validation/`).
Every model parameter is scenario-configurable; defaults below.

## Propagation stack

Per-link pathloss is a sum of components, each pluggable:

| Component | Model | Applies when |
|---|---|---|
| Baseline | Free-space (Friis) / two-ray ground reflection | always (two-ray beyond crossover distance) |
| Terrain | Knife-edge diffraction from DEM profile (ITM/Longley-Rice candidate later) | terrain occludes the link path |
| Foliage | Weissberger MED (foliage depth ≤ 400 m); ITU-R P.833 as cross-check | land-cover raster marks vegetation along the path |

Foliage depth is integrated along each link's ground path through the land-cover
raster, recomputed as vehicles move. Vegetation loss is strongly frequency-dependent,
which makes carrier band a benchmark dimension (penetration vs. antenna size vs.
jamming resistance — the project's core SWaP trade-off).

**Radio defaults:** UHF 433 MHz and ISM 2.4 GHz profiles; TX power ≤ 1 W
(SWaP-bounded); omni antennas, 0 dBi; noise figure 7 dB.

## Link budget → SINR

For directed link (i → j) at frequency f:

```
P_rx   = P_tx + G_tx + G_rx − PL(i, j, f)
N      = thermal noise (kTB) + noise figure
I_jam  = Σ over jammers: jammer power received at j within the receiver bandwidth,
         per hop channel (see EW models)
SINR   = P_rx − 10·log10(N + I_jam)
```

PER from SINR uses a modulation-dependent waterfall curve (configurable; default:
logistic approximation of FSK BER → PER for the configured packet length). The data
rate and serialization delay come from the configured waveform profile.

## FHSS hop-collision model

Hopping is modeled statistically per packet rather than by simulating individual
dwells (a packet may span one dwell at slow hop rates or many at fast ones — the
model covers both):

- N hop channels, jammer covers a subset J (spot/sweep) or all with reduced spectral
  density (barrage). Overlap fraction `ρ = |J ∩ hopset| / N`.
- A packet of duration `T_pkt` spans `k = ceil(T_pkt / T_dwell)` dwells.
- A dwell is "hit" if the jammer's in-channel power drives SINR below the waterfall
  threshold for that dwell.
- `ρ_eff` is the per-dwell hit probability given current geometry and jammer power.

**FEC + interleaving** (`radio.fec_fraction`, 0 = none). With an interleaved
erasure code that recovers up to a fraction `f` of erased dwells, a packet
survives jamming as long as no more than `e = floor(f·k)` of its `k` dwells are
jammed. With interleaving the erasures are independent, so survival is a
binomial CDF:

- `P(survive jamming) = P(B ≤ e)`, `B ~ Binomial(k, ρ_eff)`, `e = floor(f·k)`
- `PER_total = 1 − (1 − PER_thermal) · P(survive jamming)`
- `f = 0` ⇒ `e = 0` ⇒ survival `(1 − ρ_eff)^k` (every dwell must be clean — the
  original no-FEC model).

This is the axis that **flips the slow-vs-fast-hop trade-off**: without FEC a
single jammed dwell kills the packet, so slow hopping (fewer dwells) wins;
with FEC, once `k` is large enough that `floor(f·k) ≥ 1`, fast hopping spreads a
partial-band jammer's hits across many dwells and the code corrects them, so
fast hopping wins whenever the jammed fraction stays below `f`. See
[results/fec-flips-hop-rate.md](results/fec-flips-hop-rate.md).

Friendly same-dwell collisions between own nodes are folded into the same model
(uniform random hop sequences, ≤ 5 nodes ⇒ rare); contention-MAC effects are out of
scope and documented as a known fidelity limit.

Validated against closed-form binomial expectations and direct dwell-erasure
Monte-Carlo (with and without FEC).

## Jammer models

| Type | Behavior | Key parameters |
|---|---|---|
| Barrage | Wideband noise across the whole band; raises noise floor everywhere | power, bandwidth |
| Spot | Concentrates power on k of N hop channels | power, channel set |
| Sweep | Spot jammer sweeping the band | power, sweep rate, step |
| Reactive | Observes transmissions via the channel engine; jams the active channel after a detection delay | power, detection latency, detection probability |

Jammers are position-aware: their received power at each vehicle goes through the
same propagation stack as friendly signals (a jammer behind a forest is attenuated
too).

## Satellite link model

Modeled as a channel in the same engine: fixed propagation delay (default LEO
~40 ms RTT), bandwidth cap, optional loss rate, and a scripted outage timeline
(the M4 failover trigger). No orbital mechanics — outage is the phenomenon under
test, not visibility geometry.

**M4 implementation:** one node has `role: command`. While the satellite is up,
every vehicle⇄command link uses the satellite overlay (`medium="sat"`, jammer-
immune, low latency), overriding the RF mesh. During an outage window those
links fall back to the normal RF model — exposing the fallback path to range,
foliage, and jamming. Command is an RF mesh node, so swarm state reaches it over
vehicle⇄command RF links once the uplink drops. The vehicle agent's **link
manager** tracks command-update freshness; if command goes stale (uplink
presumed lost) it logs `link_down` and pushes a full snapshot to accelerate
mesh resync, logging `link_up` on recovery.

**Failover-time metric** (`harness/failover.py`): per vehicle, the wall time
from outage start until command applies that vehicle's first post-outage state
update; the swarm value is the max over vehicles.

## Known fidelity limits

- No sub-dwell signal simulation (no I/Q); PHY effects enter via the waterfall curve.
- No contention MAC; see FHSS section.
- Mobility is kinematic, not dynamic (no slip/suspension) — positions only feed
  link budgets, where this is irrelevant.
- Real-time execution: agent-side timing includes host OS scheduling noise;
  mitigated by paired-seed replay and multi-seed sweeps.
