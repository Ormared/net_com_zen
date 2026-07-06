# DDS topology study — Part 1: single-host (bridge substrate)

> **⚠ CORRECTION (2026-07-03, hours after writing):** the "SPDP participant
> cap" conclusion below is wrong — the conserved 1022-pair sum across
> independent DDS domains (finding #1) was the clue that unmasked the real
> cause: the kernel neighbor table ([dds-neighbor-table.md](dds-neighbor-table.md)).
> Still valid: zenoh's star results (94/94 at N=48) and the star/shared
> harness itself. Fast DDS/Cyclone topology numbers must be re-measured on
> the fixed rig.
>
> **The re-measure (2026-07-06) is Part 1b at the bottom of this file** —
> read that, not the tables below. Every conclusion drawn from the old
> numbers is superseded there.

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

---

# Part 1b: the re-measure on the fixed rig (2026-07-06)

**Supersedes every table above.** Same harness, same 60 s window, but with
the kernel neighbor table sized to the swarm (in-engine since `862fa5e`).
Raw: `results/dds/topo2/` (32 cells), `results/dds/rstar/`,
`results/dds/heavy/` (gitignored). Metrics recomputed with one function over
new cells AND the mesh baselines (`results/dds/rebaseline/`), so every number
in this section is same-formula: **conn** = connected audience pairs / topology
audience (mesh N(N−1), star 2(N−1)); **deliv** = received / published-to-audience.
Reps: fastrtps ×2, cyclone/zenoh ×3 (ranges shown when they matter).

## Results (conn / deliv)

| topology | N | Fast DDS | Cyclone | Zenoh (full router mesh) |
|---|---|---|---|---|
| mesh (baseline) | 48 | 1.0 / 0.993 | 1.0 / 0.99 | 1.0 / 0.67 |
| mesh (baseline) | 96 | 0.99 / **0.32** | 1.0 / 0.96 | **0.0 / 0.0** ×3 |
| shared | 48 | 1.0 / 0.995 | 1.0 / 0.99 | 1.0 / 0.65–0.67 |
| shared | 96 | 1.0 / **0.05** | 1.0 / 0.84–0.89 | **0.0 / 0.0** ×3 |
| star | 48 | 1.0 / 0.99 | 1.0 / 0.99 | 1.0 / 0.62–0.73 |
| star | 96 | **1.0 / 0.94–0.96** | **1.0 / 0.95–0.97** | **0.0 / 0.0** ×3 |

### 1. Discovery is a solved problem on the fixed rig — topology moves the DATA plane

Connectivity is 1.0 in every DDS cell at every N and every topology (the old
"SPDP cap" and "discovery lottery" readings were pure neighbor-table
artifact). What topology moves is delivery under fan-in:

- **Star gives Fast DDS its N=96 back without touching a buffer**: stock
  socket buffers, deliv 0.94–0.96 and p99 ~50 ms at 96 (vs mesh's 0.32 /
  p50 0.7 s). The mesh problem was 95 reliable writers fanning into each
  node's default-size UDP socket; the star's hub-and-spoke endpoint graph
  caps fan-in at O(1) per spoke. Same fix as the 64 MB buffer
  (dds-neighbor-table.md), achieved architecturally.
- **Shared ONE topic is the anti-pattern for Fast DDS**: deliv 0.05 at N=96 —
  *worse than mesh*. A single topic still builds the N×N writer↔reader matrix
  but now every sample contends on one topic's history and heartbeat stream.
- **Cyclone barely cares** (0.84–0.97 everywhere at 96, conn 1.0 ×3): its
  internal pacing absorbs any of the three shapes. Star is its best shape
  too (0.95–0.97, p50 0.5 ms).

### 2. Zenoh's collapse at 96 is the ROUTER graph, not the endpoint graph

Workload topology is irrelevant for zenoh: shared and star at N=96 collapse
to zero exactly like mesh (all ×3, same "Closing transport" signature), while
every N=48 variant sits at its usual deliv ~0.7 regardless of shape. The
router full mesh (N(N−1)/2 TCP links + control state) is the wall.

### 3. The zenoh router-STAR graph, measured (new `zenoh_router_topology` knob)

The old "fix is star/tree routers" claim was inferred from N=48; now it's
measured (`results/dds/rstar/`: all-to-all workload, every router connects
only to d1's router — N−1 TCP links instead of N(N−1)/2):

| N | full router mesh | router star |
|---|---|---|
| 48 | conn 1.0, deliv 0.67, p99 1.7 s | conn 1.0, **deliv 0.98**, p99 0.4–0.8 s |
| 96 | **0.0 ×3 (collapse)** | conn 0.95–0.99, **deliv 0.08–0.13**, p50 8–26 s |

Two findings, one per N:

- **The router mesh was throttling zenoh everywhere, not just at the
  collapse point.** At N=48 the star graph raises delivery 0.67 → 0.98 and
  cuts p99 by 2–4× — zenoh's stable-but-mediocre delivery plateau was
  control-plane overhead all along, and its "real" data plane is competitive
  with the DDS impls once the graph is thin.
- **Star un-collapses N=96 but trades the wall for a funnel.** No more
  "Closing transport" (all 96 routers + agents exit clean, connectivity
  0.95–0.99), but one hub router now carries all 9 120 all-to-all flows:
  delivery ~0.1 with tens-of-seconds latency. The honest conclusion for
  all-to-all zenoh at 96 on one host is a router **tree** (unmeasured) —
  a single-hub star is not enough. Note the contrast with the cross-host
  archetypes, where the star carried zenoh to 96 *spokes* easily
  ([dds-star.md](dds-star.md)): there the workload was O(N) too; here the
  star router carries an O(N²) workload.

### 4. N=192: both zenoh graphs are dead — the funnel becomes a wall

Measured 2026-07-06 with a memory watchdog (`results/dds/heavy/`; the host
never dropped below 22 GB available — the old ~750-load incident does not
reproduce on the fixed engine):

| graph | conn | delivery |
|---|---|---|
| full router mesh, r1 | **0.000** | 0.0 |
| router star, r1/r2 | **0.003 / 0.001** | ~0.0 |

The full mesh collapses exactly as at 96. The more interesting result is the
star: at 96 it restored connectivity (0.95–0.99) and only throttled the data
plane; at 192 the single hub router cannot even establish the graph (98 and
24 of 36 672 pairs, p50 2–5 s for the trickle that exists). The hub-funnel
finding (#3) extrapolates into a hard wall — **past ~96 all-to-all
participants, a zenoh router tree is mandatory**, and a single-hub star is
not a fallback. Cyclone's N=192 mesh baseline (conn 0.47–0.62, deliv
0.2–0.3, [dds-neighbor-table.md](dds-neighbor-table.md)) remains the only
configuration measured moving data at that scale on this host.

## Method notes (Part 1b)

41 cells: {shared, star} × 3 RMWs × N ∈ {48, 96} (fastrtps ×2, others ×3),
zenoh router-star × N ∈ {48, 96} ×3, and zenoh N=192 (mesh ×1, router-star
×2, memory-watchdogged). 60 s window, 200 ms / 255 B
reliable telemetry, engine-managed neigh-table sizing, host ≥ 6 GB available
throughout, load ≤ 110 (zenoh 48-router cells). Discovery here = max over
nodes of (last first-recv from an audience peer − own start); a value of
~60 s means some pair only completed at window end.
