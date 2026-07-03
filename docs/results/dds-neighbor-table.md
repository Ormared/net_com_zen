# The neighbor table: one kernel sysctl was every "middleware wall"

**Date:** 2026-07-03 · **Branch:** `dds-rmw-benchmark`
**Corrects:** [dds-rmw-scaling.md](dds-rmw-scaling.md), [dds-rmw-tuning.md](dds-rmw-tuning.md),
[dds-rmw-qos-plane.md](dds-rmw-qos-plane.md), [dds-topology.md](dds-topology.md) (banners added).
**Raw:** `results/dds/neigh_probe*`, `results/dds/rebaseline/*` (gitignored).

## The finding

`net.ipv4.neigh.default.gc_thresh3` — the Linux ARP neighbor-table garbage-
collection ceiling, default **1024** — is enforced against an entry count
that spans **all network namespaces**. The bridge substrate gives every node
its own netns and IP; N nodes exchanging unicast DDS traffic need ~N(N−1)
neighbor entries host-wide. That crosses 1024 exactly between N=24
(552 — healthy in every study) and N=48 (2256 — "the knee"). Beyond it, the
table refuses/evicts entries and unicast between unresolved peers silently
dies. Multicast SPDP needs no ARP — participants could *hear* announcements
but never complete unicast endpoint exchange or data delivery.

Every prior "wall" was this one artifact wearing three costumes:

| costume | reality |
|---|---|
| Fast DDS "~33-participant clique cap", pairs pinned at 1020–1022 across QoS/buffer/DS/time/stagger/allocation/topology/domains | 1022 ≈ the 1024-entry table; the "clique" is spawn-order table fill; determinism is deterministic fill |
| Cyclone "self-poisoning retransmit storm" | reliable retransmits into ARP-unresolvable peers |
| the universal "knee at N=48, collapse at 96" scaling curve | N(N−1) crossing gc_thresh3 |

### How it was caught

The cluster-partition sweep (K separate `ROS_DOMAIN_ID`s) produced the tell:
**pair totals conserved at ~1022 across K=1/2/4 — summed over *independent*
DDS domains — draining in spawn order** (K=4: 508/280/132/102 per domain).
A conserved sum across isolated protocol domains means a shared host
resource, and 1022 sits two below a power of two. `gc_thresh3=1024`.
Raising thresholds to 16384 for one run settled it in ten minutes.

The resource sampler had watched CPU/RAM/swap all along ("host NOT
saturated, 29 GB free") — nobody watched `/proc/net/stat/arp_cache`.

## The corrected scaling picture (rig fixed, 60 s, ideal links)

Fix is now built into the engine: every bridge run raises gc_thresh to
2·N(N−1) before netns creation and restores it after (manifest records the
effective value).

| N | Fast DDS | Cyclone | Zenoh (full router mesh) |
|---|---|---|---|
| 48 | mesh **1.000**, deliv 0.993, p50 4.9 ms | mesh **1.000**, deliv 0.99, p50 3.6 ms | mesh **1.000**, deliv 0.67, p99 1.7 s |
| 96 | mesh 0.994–0.998, **deliv 0.32**, p50 ~650 ms | mesh **1.000** ×3, **deliv 0.96**, p50 35 ms | **0.000** ×3 (collapse) |
| 192 | mesh ~0 (8–66 pairs) | mesh 0.47–0.62, deliv 0.2–0.3 | not run (host overload kill) |

- **The old ranking inverts.** "Zenoh degrades most gracefully, Cyclone
  hardest" becomes: **Cyclone is by far the strongest all-to-all scaler**
  (perfect 96-mesh with 96 % delivery, run-to-run deterministic — its
  "±32 % variance" was also the artifact). Fast DDS completes discovery at
  96 but its reliable data plane saturates (delivery 32 %). Zenoh's full
  router mesh — once no longer sheltered by ARP starvation pruning it —
  **overloads its own control plane and collapses to zero**
  (`Unable to push non droppable network message … Closing transport!`,
  reproducible ×3). Zenoh's wall is real and architectural, and its fix is
  known from [dds-topology.md](dds-topology.md): star/tree router topology
  (94/94 perfect star at N=48).
- **N=192 now stresses the actual host** (load avg >750, free RAM to 0.8 GB;
  the sweep's zenoh cells were killed): with the network artifact gone, the
  next confound on a single machine is CPU/memory contention — which is
  exactly why the multi-host `lan` substrate (P2/P3) is the right next rig.
- What survives from the old studies: the methodology (replicate everything;
  Cyclone/Zenoh single runs lie), the Discovery-Server analysis (a metadata
  broker can't fix a data-plane limit), zenoh's star-topology result, and
  the netem/cluster/topology harnesses.

## The QoS plane, re-measured (results/dds/qos2)

With the rig fixed, the full OFAT screen + buffer sweep + replicated
combinations were re-run (Fast DDS / Cyclone at N=96, Zenoh at N=48 where its
delivery margin leaves room for effects; 33 + 15 cells). The corrected story:

### Fast DDS: the socket buffer is THE lever — the original instinct was right

| config (N=96) | mesh @30 s | delivery |
|---|---|---|
| baseline ×3 | 0.29–0.32 | 0.09 |
| buf 4 MB | 0.70 | 0.37 |
| buf 16 MB | 0.83 | 0.51 |
| buf 64 MB ×3 | **0.95–0.96** | **0.66–0.67** |
| buf 64 MB, 60 s | **1.000** | **0.854** |

A clean, replicated, monotone dose-response — and with 64 MB + 60 s Fast DDS
reaches **full mesh at N=96 with 85 % delivery**. The original study measured
this exact sweep as *null*: the neighbor table capped everything at 1022
pairs, so no socket-level improvement could ever show. Mechanism: ~95
reliable writers fan into each participant's single UDP socket; the default
receive buffer drops the bursts, and the NACK/retransmit churn consumes the
window. Every *other* QoS knob (reliability, durability, history, depth,
deadline, lifespan, liveliness) moves Fast DDS by ≤±15 % — noise next to the
buffer. Even best_effort adds almost nothing *on top of* the buffer
(0.95–0.97 mesh): once the socket stops dropping, the reliable protocol was
never the bottleneck.

### Cyclone: insensitive to the entire QoS plane — robust by default

Mesh 1.000 in **every** cell; delivery 0.90–0.95 across all nine QoS knobs,
all buffer sizes, and the transient_local+buf64 stack (0.94). Nothing helps
much because nothing is broken; depth=1 costs ~3 % delivery, everything else
is noise. Cyclone's internal pacing does at N=96 what Fast DDS needs 64 MB of
socket buffer to approximate.

### Zenoh: QoS effects are marginal; the lever stays topology

At N=48 (mesh 1.0 everywhere): baseline delivery 0.34–0.38, keep_all
0.36–0.44, best_effort 0.34–0.41 — ~+10 % means with overlapping spreads.
Consistent with the old study's one durable zenoh finding: QoS is not where
zenoh scales; router topology is (star = 94/94 at N=48; full mesh = collapse
at 96).

### Practical guidance (supersedes the old doc's)

1. **Fast DDS:** set `socket_buffer_bytes` ≥ 64 MB (and kernel rmem to
   match) — it is the difference between 10 % and 85 % delivery at N=96.
   Leave the rest of the QoS contract at stock.
2. **Cyclone:** use it. Stock config full-meshes N=96 at 96 % delivery and
   still moves data at N=192 (mesh ~0.5) — the strongest all-to-all scaler.
3. **Zenoh:** don't tune QoS; fix the router topology (star/tree), and
   replicate every measurement (delivery CV ~10 % even at N=48).

## Implications

1. **The user's realism requirement was the compass.** On real distributed
   hardware each machine's neighbor table needs only N−1 entries — this
   artifact *cannot exist* there. The single-host rig didn't just approximate
   reality imperfectly; it manufactured all three headline conclusions.
2. **Rig checklist for any container/netns network benchmark at N≥32:**
   `neigh.default.gc_thresh*` (global across netns!), `rmem/wmem`, conntrack,
   bridge FDB, and watch `/proc/net/stat/arp_cache` during runs. A metric
   frozen at a suspiciously round number (~2^k) is a kernel table, not a
   protocol.
3. **Cross-host work (P3/P4) restarts from these corrected baselines**, and
   re-measuring topology variants on the fixed rig (shared/star/clusters)
   is cheap now that the harness exists.
