# DDS recommendation table: RMW × archetype → max usable N

**Date:** 2026-07-04 · **Branch:** `dds-rmw-benchmark`
**Closes:** the [dds-topology-plan.md](../dds-topology-plan.md) definition of done.
**Sources:** [dds-neighbor-table.md](dds-neighbor-table.md) (corrected bridge
baselines + QoS plane), [dds-lan.md](dds-lan.md) (P3), [dds-star.md](dds-star.md)
(P4), [dds-topology.md](dds-topology.md) (single-host topology modes).

**"Usable" criterion:** full (or ≥0.99) pair connectivity AND delivery ≥0.90
within the run window AND p99 latency compatible with the 5 Hz publish
contract (≪ 1 s). Replicated ×3; single-run numbers are never trusted
(Cyclone/zenoh CV alone can exceed 30 % on a bad rig).

## The table

| Archetype (transport) | Fast DDS | Cyclone | Zenoh |
|---|---|---|---|
| **A. Swarm mesh** — single-host bridge, ideal links | 48 stock · **96 with 64 MB socket buffers** (mesh 1.0, deliv 0.85 @60 s) | **96 stock** (mesh 1.0, deliv 0.96, deterministic); data still flows at 192 | 48 full router mesh (deliv 0.67) · **router star: 48 at 94/94**; full mesh collapses at 96 (control-plane overload) |
| **B. Flat LAN mesh** — 2 hosts, real Wi-Fi | **24** (deliv 0.89–0.92); cross-host pairs collapse at 32 | **16–24 by latency**: mesh holds to 32/48 but p99 is 1.5–7 s from N=16 up | **48+** (mesh 1.0, deliv 0.92, p99 ms-scale to N=32) — never hit its wall |
| **C. Star** — hub on small box, spokes over Wi-Fi | **64 spokes** (0.95 both ways) · 96 at ~0.8 | **32 spokes** (deliv ≥0.89); completes 96 but deliv 0.55, disc 15 s | **96+ spokes** (deliv 0.95–0.97, disc ≤3.5 s) — never hit its wall |

## How to read it

1. **The transport picks the winner, not the middleware.** Cyclone owns the
   ideal fabric (A); zenoh owns every real Wi-Fi row (B, C). No single-host
   result predicted the multi-host ranking — benchmark on the transport you
   deploy on.
2. **Topology buys more than tuning.** Going mesh→star on the same Wi-Fi
   link raises max usable N by ~3–4× for every RMW (B vs C rows). The only
   tuning knob that ever mattered comparably is Fast DDS's socket buffer on
   the ideal fabric (row A: 48→96).
3. **Zenoh's numbers are topology-conditional.** Its per-host router is why
   it wins rows B/C (one TCP stream over the air) and its full router mesh
   is why it collapses in row A at 96. Deploy it star/tree; never full-mesh
   the routers.
4. **Cyclone on lossy links is a latency decision.** Its reliable protocol
   keeps the mesh alive but queues seconds of retransmits (p99 0.5 s at
   N=8!). If the contract is delivery-eventually, row B reads "48"; if p99
   matters, it reads "16".

## Known walls, root causes, and the knob that moves them

| Wall | Who | Root cause | Lever |
|---|---|---|---|
| "~33-participant clique cap", "knee at N=48" (single host, any RMW) | rig, not middleware | kernel `neigh.default.gc_thresh3=1024` spans netns | size to 2·N(N−1) (in-engine since `862fa5e`) |
| Delivery ~0.1 at N=96, bridge | Fast DDS | default UDP socket buffers drop the 95-writer fan-in | `socket_buffer_bytes` ≥ 64 MB (+ kernel rmem) |
| Exactly 32 processes per machine, others crash at startup | Cyclone | `Discovery/MaxAutoParticipantIndex` — per-IP participant index exhaustion | set ≥ process count (in-engine since `01ef62e`) |
| Zero delivery, "Closing transport", N≈96 | zenoh | full router mesh overloads its own control plane | star/tree router topology |
| Cross-host pairs die at N=32, local fine (Wi-Fi mesh) | Fast DDS | N²/4 unicast flows over a contended AP | topology (star) — buffers don't fix air loss |
| p99 in seconds on Wi-Fi at any N | Cyclone | reliable retransmit machinery vs. lossy link | best_effort QoS or accept the latency; untested: retransmit pacing knobs |

## Residual gaps (all optional)

- Zenoh bridge re-baseline at N=192 and router-star at 96 on the fixed rig
  (heavy; needs a go-ahead — N=192 loads the rig to ~700 load avg).
- P5 netem "wifi-like" profile on the bridge, calibrated from P3's measured
  link, to close the sim-to-real loop on one host.
- Cyclone retransmit/pacing tuning on lossy links (the one QoS-plane region
  never explored, since the bridge's ideal links hid it).
