# Flat LAN over real Wi-Fi: the ranking is not the bridge ranking

**Date:** 2026-07-04 · **Branch:** `dds-rmw-benchmark`
**Archetype:** P3 flat LAN ([dds-topology-plan.md](../dds-topology-plan.md)) — one workstation
+ one 4-core NixOS mini, same home Wi-Fi network, nodes split half/half.
**Raw:** `results/dds/lan/sweep/` (gitignored), `summary.json` = 45 cells.

## Rig

- **Hosts:** main rig (192.168.1.6, Wi-Fi) runs half the nodes natively; home-mini
  (192.168.1.9, Wi-Fi only, 80–100 ms baseline RTT, 4 cores) runs the other half
  inside the pixi-parity `dds-lab` container (`--network host`). `lan` substrate:
  real NIC, no netns, interface pinning on both hosts, native SPDP multicast
  discovery (crosses the AP — verified in P2).
- **Workload:** all-to-all mesh, 200 ms period, 255 B payload, 30 s, 3 reps per cell.
  N ∈ {8, 16, 24, 32, 48} × {fastrtps, cyclonedds, zenoh}.
- **Zenoh topology:** one router per host, host-level router mesh (2 routers) — NOT
  the per-node full router mesh that collapses on the bridge.
- **Integrity:** remote nodes launched via a single per-host ssh launcher script
  (per-node ssh hit sshd MaxStartups and silently dropped nodes at N≥24);
  mesh denominator taken from the manifest's `n_nodes`, so a node that never
  spawns counts as zero (the old report averaged only over reporting nodes and
  showed 1.0 where the truth was 0.87). All cells below are post-fix.
- **Caveat:** cross-host one-way latency is not measurable (clocks unsynced);
  p99 below is same-host latency plus the protocol's queueing delay, which is
  where the signal actually is.

## Results (mesh = connected pairs / N(N−1), 3 reps)

| N | Fast DDS | Cyclone | Zenoh |
|---|---|---|---|
| 8 | mesh 1.0, deliv 0.97–0.99, p99 6–76 ms | mesh 1.0, deliv 0.86–0.93, **p99 0.5–0.9 s** | mesh 1.0, deliv 0.98, p99 7–8 ms |
| 16 | mesh 1.0, deliv 0.97, p99 ~11 ms | mesh 1.0, deliv 0.88–0.93, p99 1.2–1.5 s | mesh 1.0, deliv 0.97, p99 ~11 ms |
| 24 | mesh 1.0, deliv 0.89–0.92, p99 0.7–0.8 s | mesh 1.0, deliv 0.90, p99 1.5–2.5 s | mesh 1.0, deliv 0.96, p99 ~19 ms |
| 32 | **mesh 0.39–0.58** (cross-host 4–99 of 512) | mesh 1.0, deliv 0.67–0.69, **p99 ~4 s** | mesh 1.0, deliv 0.94–0.95, p99 ~25 ms |
| 48 | **mesh 0.25–0.27** (cross-host ~0 of 1152) | mesh 0.94–0.95, deliv 0.56–0.59, **p99 5.6–7.3 s** | **mesh 1.0, deliv 0.92, p99 1.8 s** |

The local/cross pair split is the diagnostic lens: every failure mode shows up
as cross-host pairs dying while same-host pairs stay healthy.

### Zenoh dominates the real network — at every N

Full mesh in all 15 cells, delivery 0.92–0.99, p99 in the tens of milliseconds
through N=32. At N=48 delivery holds at 0.92 and p99 grows to ~1.8 s (two
routers funneling 1152 cross-host flows over one Wi-Fi hop start to queue).
The brokered architecture that collapses on the single-host bridge (96-router
full mesh) is exactly right here: 24 nodes per host collapse onto ONE
router-to-router TCP session over the lossy link, so the air sees one ordered
stream instead of 576 independent UDP flows.

### Fast DDS: a cross-host cliff between N=24 and N=32

At N≤24, full mesh and the best delivery of the three. At N=32 the cross-host
half of the mesh collapses (512 expected cross pairs → 99/32/4 across reps;
local pairs stay ≥80% complete) and at N=48 cross-host connectivity is
essentially zero — while both same-host cliques remain perfect. Per-node UDP
unicast discovery + reliable data flows over a contended AP fall apart
together. (The N=48 r2 "delivery 0.82" is a denominator illusion: delivery is
computed over the pairs that exist, and only the healthy local cliques
survive.) The bridge lever — bigger socket buffers — is untested here; the
loss is on the air, not in the socket, so it is unlikely to be the same fix.

### Cyclone: holds the mesh, pays in seconds

Full mesh through N=32 and 0.94 at N=48 — discovery is again the strongest of
the two DDSes. But its reliable retransmit machinery, tuned for a fast fabric,
turns Wi-Fi loss into seconds of queueing: p99 climbs monotonically from 0.5 s
(N=8!) to ~7 s at N=48, and delivery within the window drops to 0.56–0.69.
On the ideal bridge Cyclone was the undisputed winner; on a real lossy link it
is the slowest by two orders of magnitude. Same protocol, opposite verdict —
the transport medium, not the middleware, picks the winner.

## The inversion, side by side

| rig | winner | loser | why |
|---|---|---|---|
| single-host bridge, ideal links (N=96) | Cyclone (mesh 1.0, deliv 0.96) | zenoh full router mesh (0.000) | retransmits are free, router fan-in is the bottleneck |
| flat LAN, real Wi-Fi (N=48) | zenoh (mesh 1.0, deliv 0.92) | Fast DDS (mesh 0.25) | one TCP stream per host-pair beats 1152 UDP flows; retransmits queue for seconds |

Neither single-host result predicted the LAN ranking. This is the
[neighbor-table](dds-neighbor-table.md) lesson generalized: benchmark on the
transport you will deploy on.

## Harness gotchas found (fixed / logged)

1. **sshd MaxStartups** silently dropped ~¼ of remote nodes at N≥24 with
   per-node ssh connections → per-host launcher script through one connection
   (`0aa6dca`). Four N=24 cells and all N≥32 cells were re-run post-fix.
2. **Mesh denominator** must come from the manifest, not from the set of
   reporting nodes — a never-spawned node otherwise inflates mesh to 1.0
   (`0aa6dca`).
3. **Zenoh router port race:** back-to-back runs can find 7447 still bound on
   the remote ("Address already in use" → that host's nodes go silent, cross
   pairs = 0). One cell hit it; re-run clean. TODO: the lan engine should wait
   for port release (or bind-retry) before starting the router.

## What this changes

1. **For any deployment with a real radio hop, test zenoh first.** Its router
   indirection — a liability on an ideal fabric — is the right shape for
   lossy shared media.
2. **Cyclone's bridge dominance does not transfer.** Reliable-QoS DDS over
   Wi-Fi buys mesh completeness at a latency cost (seconds) that breaks any
   real-time contract well before delivery drops.
3. **Fast DDS needs its cross-host story fixed before it is usable at N≥32
   on Wi-Fi** — candidate levers are discovery tuning and unicast-only
   transport, not socket buffers (untested hypotheses).
4. **Next (P4):** the star archetype — hub on the mini, spokes on the rig —
   plus wired-vs-Wi-Fi separation if a cable becomes available; netem on the
   bridge can approximate loss/delay but not AP contention.
