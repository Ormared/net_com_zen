# DDS topology study — Part 1: single-host (bridge substrate)

**Date:** 2026-07-03 · **Branch:** `dds-rmw-benchmark` · **Plan:** [dds-topology-plan.md](../dds-topology-plan.md) (P4/P5)
**Raw:** `results/dds/topo/*` (gitignored). 32 cells: {shared, star} × {fastrtps, cyclonedds, zenoh} × N ∈ {48, 96}, 60 s, reps 2 (fastrtps, CV≈0) / 3 (cyclone, zenoh).
Part 2 (cross-host archetypes over real Wi-Fi) pending the firewall openings.

## Question

The QoS-plane study ([dds-rmw-qos-plane.md](dds-rmw-qos-plane.md)) killed every
*configuration* lever; the last hypothesis standing was **topology**: collapse
the O(N²) endpoint matrix to O(N) — one shared aggregation topic, or a
hub-and-spoke star — and the walls should move. Workload modes are D4
(`ros2.topology: mesh|shared|star`, `hub_id`).

Metrics: `mesh` keeps the all-to-all denominator N(N−1) for comparability;
star's full success is 2(N−1) directed pairs, reported below as **star-conn =
pairs / 2(N−1)**.

## Results

Mesh baselines for reference: N=48 → fastrtps 0.453 / cyclone 0.20 / zenoh 0.92;
N=96 → 0.112 / 0.057 / 0.275.

| topology | N | Fast DDS | Cyclone | Zenoh |
|---|---|---|---|---|
| shared | 48 | 0.453 (**1022 pairs**) | 0.424–0.426 (≈957) | 0.53–0.62 |
| shared | 96 | 0.112 (**1022 pairs**) | 0.087–0.098 (≈850) | 0.29–0.42 |
| star (star-conn) | 48 | 0.64–0.68 | 0.47–0.83 | **1.00, 1.00**, 0.83 |
| star (star-conn) | 96 | 0.00–0.35 | 0.05–0.06 | 0.55–**0.96**–0.63 |

### 1. The Fast DDS 1022-pair cap survives topic aggregation — the wall is SPDP, not SEDP

Shared-topic runs reproduce **exactly 1022 connected pairs at both N=48 and
N=96** — the same number as every mesh run at N=48/96/128/192, now under a
completely different topic structure (N publishers + N subscribers on ONE
topic instead of N(N−1) topic-pairs). That is the seventh independent
invariance (QoS, buffer, time, Discovery Server, stagger, allocation, and now
endpoint topology). Conclusion, now definitive: **the cap operates at
participant-level mutual discovery (SPDP)** — ~33 participants fully discover
each other and the rest stay deaf — *before* endpoint counts matter at all.
No topic-level design can move it.

### 2. Star makes DDS *worse* — it concentrates the discovery lottery on the hub

A star still has N participants doing all-to-all SPDP; it only changes which
pairs *matter*. The hub is one participant subject to the same ~33-clique cap:
- fastrtps star48: 60–64 of 94 pairs — the hub mutually discovers ~32 of 47
  spokes, i.e. the clique cap measured from the hub's chair.
- fastrtps star96 r1: **0 pairs** — the hub landed outside the clique
  entirely; r2: 66. The single point of failure isn't load, it's discovery
  membership.
- Cyclone star96: ~11 of 190 pairs (storm unchanged, plus the lottery).

### 3. Zenoh is the exception: topology IS its lever

Zenoh has no peer-to-peer SPDP — connectivity rides the router mesh — and it
responds to topology exactly as the QoS study predicted:
- **star48: 94/94 pairs (100 % full star) in 2 of 3 reps** — the first fully
  connected N=48 result of the entire track.
- star96: up to 183/190 (96 %), variance still ±⅓ (the router-mesh startup
  races; un-tamed).
- shared96: 0.29–0.42 vs mesh 0.275 — up to +50 %, establishment rate up to
  63 pairs/s (3.7× Fast DDS's frozen 17/s).

### 4. Cyclone on a shared topic: variance collapses, storm persists

shared96 mesh 0.087–0.098 (vs 0.057 mesh baseline, +65 %) and CV drops from
32 % to ~6 % — a real, reproducible improvement (fewer SEDP exchanges to
storm through). But delivery is 2–3 % with p50 ~600 ms: one shared RELIABLE
topic still creates an N×N writer↔reader matrix, so every reader tracks 95
writers' heartbeats and the retransmit storm continues at the data plane.
(Note: 60 s window flatters Cyclone vs the 30 s baselines — it crawls upward
with time — so +65 % is an upper bound.)

## Implications

1. **For Fast DDS / Cyclone, participant count per DDS domain is the only
   currency.** Topic topology cannot fix SPDP. The remaining single-host lever
   is **domain partitioning**: K clusters on separate `ROS_DOMAIN_ID`s (~24
   participants each, under every knee) bridged by dual-homed gateway nodes.
2. **For Zenoh, engineer the router topology, not QoS.** Full star at N=48 is
   already deliverable today; the router-mesh startup race is the remaining
   variance source at 96.
3. The star archetype (one main server + followers, the user's archetype C)
   is **viable on Zenoh, fragile on DDS multicast discovery** — cross-host
   Part 2 will re-test this where SPDP multicast doesn't even cross the AP and
   unicast discovery mechanisms take over.

## Method notes

Star probes at N=4 validated the harness (exactly 2(N−1)=6 pairs, self-drop on
shared verified). All 32 cells exited clean; host never saturated (≥10 GB
free). Windows: 60 s here vs 30 s in the baselines — irrelevant for Fast DDS
(frozen ≤3 s), flattering for Cyclone, neutral for Zenoh (plateaus).
