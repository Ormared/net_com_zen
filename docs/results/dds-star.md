# Star archetype: O(N) topology carries all three RMWs past the flat-LAN walls

**Date:** 2026-07-04 · **Branch:** `dds-rmw-benchmark`
**Archetype:** P4 centralized star ([dds-topology-plan.md](../dds-topology-plan.md)) —
hub on home-mini, N spokes on the main rig, same Wi-Fi as [dds-lan.md](dds-lan.md).
**Raw:** `results/dds/star/sweep/` (45 cells), `results/dds/star/probe_mapi/` (gitignored).

## Rig

Same two hosts and lan substrate as P3. Star workload (D4): spokes publish
`/swarm/telemetry` up, the hub fans `/swarm/command` back down — 2N directed
links instead of N(N−1). Forward = hub in the mini's container, all N spokes
native on the rig (every up/down link crosses the Wi-Fi AP); inverted check =
hub on the rig, spokes split half/half. 200 ms period, 255 B payload, 30 s,
3 reps. N spokes ∈ {16, 32, 64, 96} (lan node cap raised 96 → 128 to fit
96 spokes + hub).

Metrics are star-native: up = spoke→hub pairs and delivery, down = hub→spoke
fan-out pairs and delivery, discovery = hub's first-recv per spoke (coarse
cross-host proxy), plus the mini's CPU busy fraction over the run.

## Forward star (hub on the 4-core mini), 3 reps

| N spokes | Fast DDS | Cyclone | Zenoh |
|---|---|---|---|
| 16 | 16/16 both ways, deliv 0.96–0.98 | 16/16, deliv 0.96–0.97 | 16/16, deliv 0.97–0.98 |
| 32 | 32/32, deliv 0.94–0.97 | 32/32, deliv 0.89–0.95 | 32/32, deliv 0.97–0.98 |
| 64 | **64/64, deliv 0.94–0.95** | 64/64¹, deliv 0.75–0.83, disc ≤7.6 s | 64/64, deliv **0.98** |
| 96 | 96/96², deliv ~0.8 | 96/96¹, deliv 0.55–0.63, disc ~15 s | **96/96, deliv 0.95–0.97, disc ≤3.5 s** |

¹ after the participant-index fix below — the stock config crashed spokes 33+.
² r1 was a bad rep (87/96, deliv 0.42, disc 25 s); r2/r3 completed at 0.80–0.86.

- **Hub CPU is never the limit:** the mini's busy fraction peaks at 0.14
  (forward, N=96) / 0.22 (inverted, hub + router + half the spokes) — the
  4-core box loafs. R2 from the plan is retired: the hub role is one process
  and the archetype is bottlenecked by the air, not the hub.
- **Zenoh again degrades most gracefully:** delivery ≥0.95 with full pairs at
  every N, discovery under 3.5 s, run-to-run CV ~1 %. The inverted check is
  even flatter (disc 0.2 s — hub and router co-located with half the spokes).
- **Fast DDS gets its scale back:** the flat-LAN N=32 cross-host cliff
  ([dds-lan.md](dds-lan.md)) is gone — full 64-spoke star at 0.95 delivery,
  96 at ~0.8. O(N) unicast flows through one hub is a shape its discovery
  and data plane both survive; the mesh's N²/4 cross-host flows were the
  killer, not the AP as such.
- **Cyclone completes the star but pays its reliability tax on schedule:**
  delivery slides 0.96 → 0.55 and discovery stretches to 15 s as N grows —
  the same seconds-scale retransmit queueing as the flat LAN, now funneled
  through the hub's single Wi-Fi link.

## The Cyclone 32-spoke crash: participant-index exhaustion (new wall, found + fixed)

The first sweep produced a perfectly deterministic **exactly-32-of-64/96**
signature for Cyclone (×6 cells) — and it was not discovery. Spokes 33+
**died at startup**: `rmw_create_node: failed to create domain` (64 of 96
exit-1, only 33 agent files). Cyclone assigns every participant that shares
an IP address a *participant index* — its unicast port slot — and refuses to
create participants past `Discovery/MaxAutoParticipantIndex` (default ≈32).
The netns substrates never see this (each node has its own IP, so every
participant is index 0); packing 96 participants onto one host IP is exactly
what the lan substrate does.

Probe: raising `MaxAutoParticipantIndex` to 150 → 64/64 and 96/96 spokes
join (then the delivery/discovery numbers above apply). The engine now sets
it unconditionally in the Cyclone XML, sized above the lan node cap.

Rig-checklist addition (extends [dds-neighbor-table.md](dds-neighbor-table.md)):
*a deterministic cap at a round number can also live in the middleware's own
per-host tables, not just the kernel's.* Fast DDS and zenoh have no such
per-IP index; they were unaffected.

## Inverted check (hub on the rig, 32 spokes split half/half)

All three RMWs: full 32/32 both directions. Zenoh 0.95–0.96 delivery,
Cyclone 0.88–0.91, Fast DDS noisier (0.77–0.92, disc up to 13 s — its
cross-host discovery variance again). Placement of the hub doesn't change
the ranking; it mostly moves discovery time (co-located spokes find the hub
in milliseconds).

## Verdict on the P4 hypothesis

> O(N) endpoint topology escapes all three walls.

**Confirmed, with one new wall found on the way.** The star carries every
RMW to 96 spokes over real Wi-Fi — a scale the flat-LAN mesh reached for
zenoh only — and the hub hardware barely registers. The ranking still holds
(zenoh ≥ Fast DDS > Cyclone on lossy links), and Cyclone's index ceiling is
a deployment footgun to know about: **any many-processes-per-machine Cyclone
deployment needs `MaxAutoParticipantIndex` sized to the process count.**

Remaining from the plan: P5's netem/topology variants on the fixed bridge
(optional) and the final recommendation table across archetypes.
