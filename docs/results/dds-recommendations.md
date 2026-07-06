# DDS recommendation table: RMW × archetype → max usable N

**Date:** 2026-07-04, revised 2026-07-06 with the P5 fixed-rig re-measures.
**Branch:** `dds-rmw-benchmark`
**Closes:** the [dds-topology-plan.md](../dds-topology-plan.md) definition of done.
**Sources:** [dds-neighbor-table.md](dds-neighbor-table.md) (corrected bridge
baselines + QoS plane), [dds-lan.md](dds-lan.md) (P3), [dds-star.md](dds-star.md)
(P4), [dds-topology.md](dds-topology.md) Part 1b (fixed-rig topology modes +
zenoh router graphs), [dds-netem.md](dds-netem.md) (emulated-Wi-Fi sim-to-real
check + Cyclone lossy-link knobs).

**"Usable" criterion:** full (or ≥0.99) pair connectivity AND delivery ≥0.90
within the run window AND p99 latency compatible with the 5 Hz publish
contract (≪ 1 s). Replicated ×3; single-run numbers are never trusted
(Cyclone/zenoh CV alone can exceed 30 % on a bad rig).

## The table

| Archetype (transport) | Fast DDS | Cyclone | Zenoh |
|---|---|---|---|
| **A. Swarm mesh** — single-host bridge, ideal links | 48 stock · **96 with 64 MB socket buffers** (deliv 0.85 @60 s) *or* **96 as a star** (deliv 0.94–0.96 stock) | **96 stock** (mesh 1.0, deliv 0.96, deterministic); data still flows at 192 | **48 with router-star** (deliv 0.98 — full router mesh throttles it to 0.67); full mesh collapses at 96; router-star un-collapses 96 but one hub router funnels to deliv ~0.1 → needs a router tree |
| **B. Flat LAN mesh** — 2 hosts, real Wi-Fi | **24** (deliv 0.89–0.92); cross-host pairs collapse at 32 | **16–24 by latency**: mesh holds to 32/48 but p99 is 1.5–7 s from N=16 up | **48+** (mesh 1.0, deliv 0.92, p99 ms-scale to N=32) — never hit its wall |
| **C. Star** — hub on small box, spokes over Wi-Fi | **64 spokes** (0.95 both ways) · 96 at ~0.8 | **32 spokes** (deliv ≥0.89); completes 96 but deliv 0.55, disc 15 s | **96+ spokes** (deliv 0.95–0.97, disc ≤3.5 s) — never hit its wall |

## How to read it

1. **The transport picks the winner, not the middleware.** Cyclone owns the
   ideal fabric (A); zenoh owns every real Wi-Fi row (B, C). No single-host
   result predicted the multi-host ranking — benchmark on the transport you
   deploy on.
2. **Topology buys more than tuning.** Going mesh→star on the same Wi-Fi
   link raises max usable N by ~3–4× for every RMW (B vs C rows), and on the
   ideal fabric a star gives Fast DDS its N=96 back with stock buffers —
   the same delivery the 64 MB socket buffer buys, achieved architecturally.
   One anti-pattern: aggregating everything on ONE shared topic makes
   Fast DDS *worse* than mesh at 96 (deliv 0.05 — the N×N writer matrix
   contends on a single topic's history).
3. **Zenoh's numbers are router-graph-conditional.** Its per-host router is
   why it wins rows B/C (one TCP stream over the air); its full router mesh
   throttles it everywhere on row A (0.67 at 48) and collapses at 96. A
   router star raises 48 to deliv 0.98, and at 96 it trades the collapse for
   a single-hub data funnel (deliv ~0.1) — for all-to-all workloads at that
   scale the router graph must be a tree, not one hub.
4. **Cyclone on lossy links is a latency decision.** Its reliable protocol
   keeps the mesh alive but queues seconds of retransmits (p99 0.5 s at
   N=8!). If the contract is delivery-eventually, row B reads "48"; if p99
   matters, it reads "16". None of its `<Internal>` retransmit knobs beat
   stock under emulated loss ([dds-netem.md](dds-netem.md)); the knob that
   works is best_effort QoS (p99 340→100 ms for −1 % delivery).
5. **Don't trust per-link emulation to pick a middleware.** A netem profile
   calibrated on the real link's delay/jitter/loss reproduces latency
   budgets but inverts the middleware ranking vs the real shared-medium
   Wi-Fi — load-coupled loss and deployment topology are what decide the
   ranking, and a per-veth qdisc models neither ([dds-netem.md](dds-netem.md)).

## Known walls, root causes, and the knob that moves them

| Wall | Who | Root cause | Lever |
|---|---|---|---|
| "~33-participant clique cap", "knee at N=48" (single host, any RMW) | rig, not middleware | kernel `neigh.default.gc_thresh3=1024` spans netns | size to 2·N(N−1) (in-engine since `862fa5e`) |
| Delivery ~0.1 at N=96, bridge | Fast DDS | default UDP socket buffers drop the 95-writer fan-in | `socket_buffer_bytes` ≥ 64 MB (+ kernel rmem) |
| Exactly 32 processes per machine, others crash at startup | Cyclone | `Discovery/MaxAutoParticipantIndex` — per-IP participant index exhaustion | set ≥ process count (in-engine since `01ef62e`) |
| Zero delivery, "Closing transport", N≈96 | zenoh | full router mesh overloads its own control plane | router star un-collapses it (`zenoh_router_topology`), but one hub funnels O(N²) traffic (deliv ~0.1) — use a tree for all-to-all at 96 |
| Cross-host pairs die at N=32, local fine (Wi-Fi mesh) | Fast DDS | N²/4 unicast flows over a contended AP | topology (star) — buffers don't fix air loss |
| p99 in seconds on Wi-Fi at any N | Cyclone | reliable retransmit machinery vs. lossy link | best_effort QoS or accept the latency; untested: retransmit pacing knobs |

## Residual gaps

All P5 items are now measured except one:

- **Zenoh at N=192** (full mesh confirmation + router-star/tree): heavy —
  the previous attempt loaded the rig to ~750 with RAM at 0.8 GB, so it
  stays behind an explicit user go-ahead. Low information value for the
  full mesh (collapse at 96 ×3 makes 192 a foregone conclusion); the
  router-tree question is the only genuinely open cell.
