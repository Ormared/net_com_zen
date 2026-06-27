# The QoS plane: how Fast DDS, Cyclone & Zenoh actually scale to many participants

Third in the DDS/RMW series, and the one that gets to the bottom of it.
[`dds-rmw-scaling.md`](dds-rmw-scaling.md) found the *what* (a knee at N=48, no
mesh at N=96). [`dds-rmw-tuning.md`](dds-rmw-tuning.md) found that the obvious fix
(Fast DDS Discovery Server) does nothing, because the bottleneck is the
data-plane endpoint mesh, not discovery. This document drops every swarm/EW
constraint, treats each middleware as a system to characterize, and sweeps the
**whole DDS QoS contract + the transport buffer + startup timing** to answer one
question: *what actually lets each RMW reach a high participant count?*

It took a chain of experiments that kept overturning the tidy story. The honest
finding is that the three RMWs hit **three different walls**, and that **run-to-run
variance is large enough that a single run will lie to you** — so the method
matters as much as the result.

Metric: a **connected pair** is an ordered (receiver, sender) that exchanged ≥1
message during the run; **mesh** = connected pairs / N(N−1). All runs:
`substrate=bridge`, single host (24-core/62 GB), 255 B @ 5 Hz, driver
[`dds_qos_lab`](../../sim/netcom_zen/harness/dds_qos_lab.py). Host stayed ≥22 GB
free on every cell, so everything below is middleware behavior, not the rig.

## TL;DR — three different walls

| | Fast DDS | Cyclone DDS | Zenoh |
|---|---|---|---|
| N=96 mesh (mean) | **0.112** | 0.057 | 0.275 |
| run-to-run CV | **0 %** (deterministic) | 32 % | 35 % |
| nature of the wall | **hard ~33-participant clique** | thrashing retransmit storm | racy router-mesh startup |
| moved by QoS? | no (bit-for-bit invariant) | no (all knobs ≤ noise) | no (effects ≤ noise) |
| moved by buffer? | no | no (within noise) | no (within noise) |
| moved by **stagger**? | barely (+12 %) | **yes (2.5×)** | n/a |
| moved by **time**? | no (frozen at 300 s) | yes, crawls upward | no (plateaus by 30 s) |
| the real lever | reduce participant count / a resource-limit knob | **stagger joins** | accept variance, or fewer routers |

## The meta-finding: single runs lie (run-to-run variance)

The buffer and beststack phases produced impossible-looking contradictions — the
*same* Zenoh config gave mesh 0.146 in one phase and 0.365 in another. So before
trusting any QoS "effect" we replicated baseline ×5 per RMW
(`results/dds/qos/replicate/`):

| RMW | baseline mesh ×5 | mean | CV |
|---|---|---|---|
| **fastrtps** | 0.112, 0.112, 0.112, 0.112, 0.112 | 0.112 | **0 %** |
| cyclonedds | 0.034, 0.046, 0.049, 0.073, 0.082 | 0.057 | 32 % |
| zenoh | 0.163, 0.207, 0.241, 0.338, 0.428 | 0.275 | 35 % |

Fast DDS is **perfectly deterministic**. Cyclone and Zenoh swing ±~⅓. This single
table invalidates most of the per-knob "effects" the screening sweep appeared to
show for those two — including Zenoh's headline "manual-liveliness gives 2.1×":
replicated ×5, manual-liveliness averages **0.191**, *below* baseline's 0.275. It
was noise. **For Cyclone and Zenoh, any difference under ~2× from a single run is
not real.** Everything below is read against this noise floor.

## Fast DDS — a deterministic ~33-participant ceiling

The defining trait is **invariance**, and it is absolute:

- mesh = 0.112 (1022 connected pairs) at N=96, on **5/5** replicate runs, CV 0 %.
- the same 1022 pairs at **N=48** (mesh 0.453) — pair *count* is N-independent.
- the same 1022 pairs at a **300 s** window (mesh still 0.112 — frozen, not slow).
- the same 1022 pairs under **every** QoS knob (reliable/best-effort,
  KEEP_LAST(1)/KEEP_ALL, volatile/transient-local, deadline, lifespan,
  liveliness) and **every** buffer size (4/16/64 MB).

What is 1022 pairs? The per-node receive distribution at N=96 (300 s run) is
**bimodal**: **61 of 96 nodes hear from 0 peers**, while **35 nodes form a near
clique**, each hearing ~31 others (35 × ~29 ≈ 1020). So Fast DDS doesn't degrade
gracefully — it elects a **~33–35-participant fully-connected clique and leaves
the other ~63 completely deaf**, within ~2 s, then freezes. The clique size is
the same (~33) at N=48 and N=96, which is why pair count is constant.

Levers tested against it, all negative or near-negative:
- **QoS / buffer:** zero effect (above).
- **Discovery Server** (`dds-rmw-tuning.md`): zero effect — so it is *not*
  participant (SPDP) discovery; the cap is downstream, in endpoint/data matching.
- **Time:** zero effect at 300 s — it is a hard cap, not a rate limit.
- **Startup stagger:** spawning the 96 participants 200 ms apart (incremental
  join over ~19 s) lifted it only to 0.126 (1148 pairs, +12 %) — a real but tiny
  nudge. So it is *not* primarily a simultaneous-startup race either.

The signature (a fixed-size mutually-discovered clique, immovable by discovery
brokering, time, or QoS) points at a **default resource/allocation limit on
matched remote participants or readers** in the Fast DDS participant — the
remaining untested lever is raising those allocation limits explicitly
(`ResourceLimitsQosPolicy` / participant allocation config). The practical
takeaway today: **Fast DDS on a flat segment tops out near ~33 mutually-connected
participants regardless of tuning; to go higher you must reduce the
participant/endpoint count (aggregate, partition) — not touch QoS.**

## Cyclone DDS — a self-poisoning retransmit storm that staggering defuses

Cyclone at N=96 is broadly broken: mesh 0.057 ± 32 %, with a pathological tail
(p50 ~700 ms, p99 ~2 s) — the reliable-retransmit storm diagnosed in
`dds-rmw-tuning.md`. Within the QoS plane, every knob is **≤ the 32 % noise band
or actively harmful**:
- best_effort and depth=1 cut latency (p50 → ~1–8 ms) but not mesh — they shed
  the backlog without connecting more pairs.
- **KEEP_ALL is catastrophic**: p99 18.6 s, unbounded queues, processes don't
  exit clean. Never use KEEP_ALL + reliable at scale on Cyclone.
- the buffer's apparent "+17 %" (screening) is **inside the noise** — the buffer
  sweep gave 0.040 / 0.040 / 0.046 / 0.040 across 0/4/16/64 MB, i.e. flat.

The one thing that **reliably** helps is **staggering joins**: 50 ms → 0.118,
200 ms → 0.145 — both clearly above the entire baseline band (max 0.082). And
Cyclone is the one RMW whose mesh **grows with time** (588 pairs @30 s → 1045
@120 s), i.e. it is slow-but-progressing rather than frozen. Both point the same
way: Cyclone's wall is the **burst** of simultaneous SPDP + reliable retransmits;
spread the participant joins out (or just give it much longer) and it crawls
further. **Lever: stagger joins + longer settling time; do not pile on QoS.**

## Zenoh — fastest mean, but dominated by startup variance

Zenoh has the **highest mean mesh (0.275)** — roughly 2.5× Fast DDS and ~5×
Cyclone — but also the **largest variance (CV 35 %, 0.163–0.428)**. Replication
shows the screening QoS "effects" (best_effort, manual-liveliness, KEEP_ALL all
"helping") do **not** survive: manual-liveliness ×5 averages *below* baseline.
The knobs also **don't stack** — the beststack combo (best_effort + KEEP_ALL +
manual-liveliness + buffer) gave 0.214, worse than baseline's mean. And unlike
Cyclone, Zenoh does **not** improve with time (≈flat 30 s → 120 s), so it
plateaus rather than crawls.

The variance traces to the **pre-wired full TCP router mesh** (one `rmw_zenohd`
per netns, N(N−1)/2 ≈ 4560 links at 96): how many sessions/declarations converge
depends on a startup race that lands anywhere in 0.16–0.43 mesh. So Zenoh's
limitation is **unpredictability, not a hard cap or a thrash** — and the lever
that `dds-rmw-tuning.md` flagged (collapse the full router mesh to a star/hub,
nodes as clients) attacks exactly the source of the variance. QoS tuning does
not. **Lever: fewer/hierarchical routers, not QoS.**

## The rate-law that wasn't (an honest detour)

The screening data first looked like a clean law: connected-pair *count* is
constant per (stack, QoS) regardless of N (Fast DDS 1022 @48, 1021 @96), implying
mesh = rate·T/N(N−1) and predicting Fast DDS reaches a full 96-mesh at ~268 s.
The **window phase refuted it**: Fast DDS at 120 s and 300 s is still exactly
1022 pairs — frozen, not rate-limited. The "constant pair count" was real but the
mechanism was a **ceiling**, not a rate. Only Cyclone turned out to be genuinely
rate-limited (it crawls with time); Fast DDS and Zenoh both plateau. Recording
the dead end because it is the reason the window and replicate phases existed —
and the reason the final model is trustworthy.

## Buffer sweep — the "increase the buffer" hypothesis, settled

`{0, 4, 16, 64 MB}` socket + kernel buffer at N=96 (`results/dds/qos/buffer/`):
- **Fast DDS:** 1022 pairs at every size ≥ 4 MB — flat (the ceiling dominates).
- **Cyclone:** 0.040 / 0.040 / 0.046 / 0.040 — flat within noise. The buffer does
  **not** fix Cyclone's storm by itself (staggering does).
- **Zenoh:** 0.365 / 0.130 / 0.122 / 0.163 — pure variance, no monotonic trend.

So raising the buffer is necessary plumbing (Cyclone won't even *start* with a
large `SocketReceiveBufferSize` unless the kernel ceiling is raised first), but on
this all-to-all workload it is **not** the lever that reaches high participant
counts. The earlier tuning-doc prediction that bigger buffers would help Cyclone
holds only weakly and within noise; staggering is the stronger, reproducible win.

## Ceiling probe — pushing past N=96 confirms the cap  (`results/dds/qos/ceiling/`)

Baseline at N=128 and N=192 (45 s):

| RMW | N | mesh | connected pairs | 1022/N(N−1) |
|---|---|---|---|---|
| **fastrtps** | 128 | 0.063 | **1020** | 0.063 |
| **fastrtps** | 192 | 0.028 | **1022** | 0.028 |
| cyclonedds | 128 | 0.046 | 744 | — |
| cyclonedds | 192 | 0.019 | 693 | — |
| zenoh | 128 | 0.163 | 2652 | — |
| zenoh | 192 | 0.038 | 1396 | — |

**Fast DDS connects ~1021 pairs at N = 48, 96, 128 *and* 192** — the mesh tracks
1022/N(N−1) to three digits. The ~33-participant clique cap is **absolute**: it
is the same fixed set size whether the swarm is 48 or 192. (At N=192 a few Fast
DDS processes didn't exit clean — the host was down to ~20 GB free with 192
participants — but the pair count held.)

**Cyclone gets *worse* in absolute terms as N grows** (744 → 693 pairs): more
participants = a bigger simultaneous storm, so it's not a fixed cap but an
N-sensitive collapse. **Zenoh reaches furthest** (2652 pairs at N=128) but falls
back by N=192 (1396) and is the heaviest on the host — its per-netns router means
2N processes and an N(N−1)/2 router mesh (≈18 000 TCP links at 192), pushing p99
to ~20 s and leaving processes unclean. So Zenoh's *own* scaling cost is the
router fan-out, independent of the DDS pair's discovery problems.

## Practical guidance — reaching high participant counts

1. **Fast DDS:** QoS/buffer/time won't help — it caps near ~33 mutually-connected
   participants on a flat segment. Reduce participant/endpoint count (aggregation
   topic, partitions, fewer-larger nodes), or investigate raising the
   participant/reader allocation resource limits. Within its clique it is the most
   *predictable* stack (CV 0 %, tight latency).
2. **Cyclone:** **stagger participant joins** and give it settling time; keep QoS
   minimal (volatile, KEEP_LAST small, reliable is fine); never KEEP_ALL. Raise
   kernel/socket buffers as hygiene but don't expect it to be the fix.
3. **Zenoh:** best mean reach but plan for ±⅓ variance; the lever is **topology**
   (a star/hub or hierarchical routers instead of the full P2P router mesh), not
   QoS. Re-measure with replicates, never a single run.

## Method notes

- One ScenarioEngine bridge run per cell; full QoS contract + socket/kernel
  buffer + spawn stagger set per the `dds_qos_lab` driver. Pub and sub share one
  QoS profile so offered==requested always.
- The buffer lever raises host `net.core.{r,w}mem_max` (root writes `/proc/sys`)
  before any DDS proc starts — Cyclone treats its `SocketReceiveBufferSize` min as
  a hard floor and won't start otherwise — then restores them.
- Single host, single seed per cell, 255 B @ 5 Hz. The replicate phase is what
  makes the Cyclone/Zenoh numbers usable; treat any unreplicated single run for
  those two as indicative only.
