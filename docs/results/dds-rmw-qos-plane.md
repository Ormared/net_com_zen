# The QoS plane: how Fast DDS, Cyclone & Zenoh actually scale to many participants

Third in the DDS/RMW series, and the one that gets to the bottom of it.
[`dds-rmw-scaling.md`](dds-rmw-scaling.md) found the *what* (a knee at N=48, no
mesh at N=96). [`dds-rmw-tuning.md`](dds-rmw-tuning.md) found that the obvious fix
(Fast DDS Discovery Server) does nothing, because the bottleneck is the
data-plane endpoint mesh, not discovery. This document drops every swarm/EW
constraint, treats each middleware as a system to characterize, and sweeps the
**whole DDS QoS contract + the transport buffer** to answer one question: *what
actually lets each RMW reach a high participant count?*

The answer turned out to be a single law that the earlier "collapse" framing hid.

## The headline: mesh formation is rate-limited pair establishment

Define a **connected pair** as an ordered (receiver, sender) that exchanged at
least one message during the run, and **mesh** = connected pairs / N(N−1). The
screening sweep makes one fact impossible to miss:

> **Each (middleware, QoS) establishes connected pairs at a characteristic
> RATE. The number of pairs wired in a fixed window is constant — independent
> of N. So `mesh = rate · T / N(N−1)`, and the "knee" is just where N(N−1)
> outgrows `rate · T`.**

The proof is Fast DDS, whose pair *count* is invariant to everything:

| | N=48 (30 s) | N=96 (30 s) | every QoS knob @ N=96 |
|---|---|---|---|
| mesh | 0.453 | 0.112 | **0.112** (all 9 cells) |
| connected pairs | **1022** | **1021** | **1017–1022** |
| rate (pairs/s) | 34.1 | 34.0 | **33.9–34.1** |

Fast DDS wires ~1020 pairs in 30 s whether there are 48 nodes or 96, and whether
QoS is reliable or best-effort, KEEP_LAST(1) or KEEP_ALL, volatile or
transient-local, default or 16 MB buffered. **34 pairs/second, full stop.** The
N=48 "knee" was never a knee — 1020 pairs is a full mesh at N≈33 (33·32=1056) but
only 11 % of N=96's 9120 pairs. Same rate, bigger denominator.

This reframes the entire question. "How do I reach high participant count?" is
really two levers: **raise the establishment rate**, or **give it more time**
(full-mesh time = N(N−1)/rate). The three RMWs differ in their base rate, how
tunable that rate is, and whether they stay stable while doing it.

## Screening: one QoS knob at a time, N=96, 30 s (`results/dds/qos/screen/`)

From a stock baseline (reliable, KEEP_LAST 10, volatile, no
deadline/lifespan, automatic liveliness, default buffer) each cell flips exactly
one policy. Rate (pairs/s) is the column that matters; mesh is rate·30/9120.

| rmw | knob flipped | mesh | rate (/s) | delivery | p50 (ms) | p99 (ms) | notes |
|---|---|---|---|---|---|---|---|
| **fastrtps** | baseline | 0.112 | 34.0 | 0.111 | 5.5 | 23.8 | |
| fastrtps | best_effort | 0.112 | 33.9 | 0.109 | 5.8 | 31.1 | rate unmoved |
| fastrtps | depth=1 | 0.112 | 34.0 | 0.111 | 5.4 | 23.2 | fast disco, same rate |
| fastrtps | keep_all | 0.112 | 34.1 | 0.111 | 5.2 | 21.0 | |
| fastrtps | transient_local | 0.112 | 34.1 | 0.111 | 5.6 | 23.0 | |
| fastrtps | lifespan 500 ms | 0.112 | 34.1 | 0.111 | 5.6 | 23.1 | |
| fastrtps | deadline 1 s | 0.112 | 34.1 | 0.111 | 6.0 | 25.0 | |
| fastrtps | manual liveliness | 0.112 | 34.0 | 0.111 | 7.1 | 29.1 | |
| fastrtps | **buffer 16 MB** | 0.112 | 34.1 | 0.110 | 6.3 | 81.4 | **inert** |
| **cyclonedds** | baseline | 0.064 | 19.6 | 0.015 | 662 | 1998 | thrashing tail |
| cyclonedds | best_effort | 0.032 | 9.9 | 0.001 | 1.0 | 7.6 | low latency, *worse* mesh |
| cyclonedds | depth=1 | 0.052 | 15.9 | 0.003 | 8.4 | 208 | |
| cyclonedds | keep_all | 0.027 | 8.3 | 0.014 | 3436 | 18582 | **unbounded queue, exits≠0** |
| cyclonedds | transient_local | 0.046 | 14.1 | 0.012 | 625 | 6305 | |
| cyclonedds | lifespan 500 ms | 0.050 | 15.2 | 0.003 | 204 | 3002 | |
| cyclonedds | deadline 1 s | 0.056 | 16.9 | 0.013 | 592 | 1998 | |
| cyclonedds | manual liveliness | 0.020 | 6.0 | 0.003 | 737 | 3919 | worst |
| cyclonedds | **buffer 16 MB** | 0.075 | 22.9 | 0.016 | 612 | 1995 | **only knob that helps (+17 %)** |
| **zenoh** | baseline | 0.146 | 44.4 | 0.136 | 11.6 | 1366 | |
| zenoh | best_effort | 0.247 | 75.2 | 0.211 | 21.0 | 2146 | +69 % rate |
| zenoh | depth=1 | 0.261 | 79.3 | 0.177 | 16.9 | 446 | |
| zenoh | keep_all | 0.269 | 81.7 | 0.237 | 30.6 | 9312 | |
| zenoh | transient_local | 0.189 | 57.4 | 0.151 | 22.9 | 3110 | |
| zenoh | lifespan 500 ms | 0.189 | 57.4 | 0.168 | 15.1 | 1862 | |
| zenoh | deadline 1 s | 0.152 | 46.1 | 0.132 | 11.5 | 1870 | ~no effect |
| zenoh | manual liveliness | **0.314** | **95.4** | 0.227 | 26.9 | 3445 | **best single knob (2.1×)** |
| zenoh | buffer 16 MB | 0.180 | 54.7 | 0.161 | 14.6 | 1728 | kernel-buffer raise helps even TCP |

Host stayed ≥22 GB free on every cell (the bridge + 96–192 procs is never the
limit), so every number above is middleware behavior.

## Per-RMW characterization

### Fast DDS — a metronome you can't speed up (in this plane)
The defining trait is **invariance**. Rate is 34 pairs/s to three digits across
the entire QoS+buffer+discovery-server plane and across N. Nothing in this study
moves it. QoS knobs change only the *discovery-latency distribution* and the tail
(e.g. best_effort pushes the slowest-pair time out to the full 30 s; depth=1
tightens it to 1.2 s) — but the same ~1020 pairs get wired either way. The
limiter is the **SEDP/data endpoint-matching throughput** itself, a fixed
~34 reliable reader↔writer completions per second, and it is downstream of
participant discovery (the Discovery Server, which removes SPDP entirely, also
left it at 0.112 — see `dds-rmw-tuning.md`). **Lever: time, not tuning.** The
window phase below tests the direct prediction — full 96-mesh at ~268 s.

### Cyclone DDS — slower and self-poisoning
Cyclone is rate-limited too (~20 pairs/s baseline) but, unlike Fast DDS, its rate
is *fragile*: almost every QoS knob makes it worse, because Cyclone is already in
a reliable-retransmit storm (baseline p50 662 ms, p99 ~2 s — pathological) and
anything that adds reliability/liveliness/queue pressure feeds the storm.
KEEP_ALL is catastrophic (p99 18.6 s, unbounded queues, processes don't exit
clean). best_effort and depth=1 cut latency but starve the mesh further. **The
one knob that helps is the buffer** (16 MB → +17 % rate), exactly because the
storm starts with socket-overflow drops — consistent with the tuning-doc
prediction. Cyclone is the stack most in need of buffer + unicast-peers, and the
least forgiving of casual QoS choices.

### Zenoh — fastest base rate, and the only QoS-tunable one
Zenoh starts at 44 pairs/s (already 1.3× Fast DDS, 2.2× Cyclone) and its rate
*responds* to loosening the contract: manual liveliness 95/s, KEEP_ALL 82/s,
depth=1 79/s, best_effort 75/s. These are the knobs that reduce per-pair
bookkeeping in its router-brokered session layer. transient_local and lifespan
help a little; deadline does nothing. Even the "buffer" cell helps (+23 %) —
because the kernel `rmem_max`/`wmem_max` raise benefits Zenoh's TCP session
sockets too, not just the DDS UDP path. **Zenoh is where stacking the winning
knobs should pay off** (tested in the beststack phase).

## Window phase — testing the rate law  (`results/dds/qos/window/`)

_Pending — runs fastrtps at 120 s & 300 s, cyclone/zenoh at 120 s, N=96. The
prediction: fastrtps mesh ≈ 34·120/9120 = 0.45 at 120 s and ≈ 1.0 (full) near
268 s. If it holds, "Fast DDS fails at 96" becomes "Fast DDS needs a 9× window."_

## Buffer sweep — the "increase the buffer" hypothesis  (`results/dds/qos/buffer/`)

_Pending — {0, 4, 16, 64 MB} at N=96. Screening already shows the buffer is inert
for Fast DDS, helps Cyclone (+17 % at 16 MB) and helps Zenoh via the kernel
ceiling raise. The sweep maps the response curve and finds the knee per RMW._

## Beststack & ceiling — best config pushed past 96  (`results/dds/qos/{beststack,ceiling}/`)

_Pending — each RMW's winning knobs combined, then pushed to N=128/192 to find
each one's participant wall._

## Method notes

- One ScenarioEngine bridge run per cell; full QoS contract + socket/kernel
  buffer set per the [`dds_qos_lab`](../../sim/netcom_zen/harness/dds_qos_lab.py)
  driver. Pub and sub share one QoS profile so offered==requested always.
- The buffer lever raises host `net.core.{r,w}mem_max` (root writes `/proc/sys`)
  before any DDS proc starts — Cyclone treats its `SocketReceiveBufferSize` min
  as a hard floor and won't start otherwise — then restores them.
- Single host (24-core/62 GB), single seed, 255 B @ 5 Hz. This is the
  scaling-rate characterization, not an exhaustive payload/rate study.
