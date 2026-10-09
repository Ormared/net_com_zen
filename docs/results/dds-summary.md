# ROS 2 middleware (RMW) comparison: Fast DDS vs Cyclone DDS vs Zenoh

A self-contained report of the whole evaluation: what was tested, what the
environment limits turned out to be, and which middleware to choose for which
deployment. **Section 6 is the condensed version.**

---

## 1. What was tested

### 1.1 Common to both tracks

**Middleware.** The same three ROS 2 RMW implementations run identical
application code; nothing changes between runs except `RMW_IMPLEMENTATION` and
its configuration.

| RMW | Version | Architecture |
|---|---|---|
| Fast DDS (`rmw_fastrtps_cpp`) | 2.14.5 / RMW 8.4.3 | peer-to-peer DDS; multicast discovery by default, optional Discovery Server for unicast |
| Cyclone DDS (`rmw_cyclonedds_cpp`) | 0.10.5 / RMW 2.2.3 | peer-to-peer DDS; multicast discovery by default, optional static peer lists |
| Zenoh (`rmw_zenoh_cpp` 0.2.9, `libzenohc` 1.7.2) | — | brokered; every node attaches to a zenoh router, and the graph between routers is configured explicitly |

**Network substrates.** Where the processes run and what carries the packets:

| Substrate | Shape | Link quality |
|---|---|---|
| Bridge | one Linux network namespace per node, all attached to one veth bridge on a single host | ideal — no loss, microsecond latency |
| Bridge + netem | the same, with a per-interface queueing discipline calibrated to the delay, jitter and loss of the real Wi-Fi link | emulated Wi-Fi |
| LAN | two physical machines — the main rig runs its nodes natively, the second machine runs its share in a container — on one L2 network | real Wi-Fi. Baseline round-trip between the machines was 80–100 ms with high jitter during the all-to-all runs; during the later centralized runs the same link measured 2.5 / 4.0 / 37 / 92 ms (min / median / 95th pct / max) |

No cabled multi-machine configuration was tested. Everything described below as
an "ideal network" is the single-host bridge.

**Machines.** Main rig: Intel Core Ultra 9 275HX, 24 cores, 62.2 GiB RAM,
Linux 7.0.0. Second machine ("home-mini"): NixOS, 4 cores, 15 GiB RAM, Wi-Fi
only. The second machine is the small one — it hosts the server or hub, and its
own share of workload processes is capped at roughly 16–24.

**Scale and repetition.** Node counts from 4 to 192, each in a default
("stock") and a tuned configuration. Measurement points are repeated at least
three times with different seeds — five for high-availability tests, two for a
few Fast DDS topology cells whose spread was negligible. A capacity figure is
reported only when *every* repetition passes; one good run never establishes a
result.

**Two different pass criteria. Node counts are not comparable between the two
tracks.**

- *Centralized* — telemetry delivery ≥ 0.90, RPC success ≥ 0.90, RPC 99th
  percentile latency ≤ 1 s, and command-gap 99th percentile ≤ twice the command
  period. A stricter variant (delivery and RPC success ≥ 0.99, RPC 99th
  percentile ≤ 200 ms, command gap ≤ 1.2× the period) was also evaluated and
  **no node count passed it for any middleware on any substrate.**
- *All-to-all* — at least 99 % of node pairs connected, delivery ratio ≥ 0.90,
  and 99th percentile latency well under 1 s (the publish rate is 5 Hz).

### 1.2 Centralized track (primary)

Clients exchange traffic with one or two application servers and never with
each other. This is treated as a design in its own right, not as a fallback
from a peer swarm: no jammer, no RF model, no peer mesh.

Three traffic classes are measured separately, because they do not agree about
which middleware is best:

1. **Telemetry** — clients publish upstream to the server. Reported as delivery
   ratio and delivered messages per second.
2. **Commands** — the server publishes downstream to clients. Reported as the
   99th percentile gap between consecutive commands, against the nominal
   command period, with duplicate commands suppressed and counted.
3. **Remote procedure calls** — clients call a ROS 2 service and record
   end-to-end round-trip time. Reported as success rate and 99th percentile
   latency. This class separates the three middlewares most sharply.

Also measured: **readiness** (time from launch until the deployment carries
traffic), **failover gap** (time to resume after the primary server process is
killed 45 s into the run), and CPU and memory use.

Server modes:

| Mode | Semantics |
|---|---|
| Single | one application server, reached through a single discovery endpoint |
| Sharded | clients assigned round-robin to one of two server namespaces |
| Active-active | two servers both live; the primary and its middleware infrastructure are then killed |
| Active-passive | a warm standby receives telemetry but creates command and service endpoints only once the primary dies |

Middleware configuration in this track: Fast DDS uses one Discovery Server per
application server, with redundant endpoints supplied through
`ROS_DISCOVERY_SERVER`; Cyclone DDS disables multicast and uses static peer
lists pointing at the server addresses; zenoh runs one router per application
server with clients attached directly to it.

Matrix size: 573 planned measurement points, 561 completed cleanly, 12 recorded
failures — 11 of them genuine process collapses at 96 and 128 nodes, one a
corrupted output file. No failure is excluded from the denominator.

### 1.3 All-to-all track (secondary)

Every node publishes to and subscribes from every other node, which means
N×(N−1) reliable endpoint pairs. This is the peer swarm case: no privileged
node and nothing to lose when one node dies, at the cost of quadratic growth in
connections.

Measured: fraction of node pairs that ever connected, delivery ratio, median
and 99th percentile latency, time to complete discovery, peak memory and CPU.

Variations inside the track:

| Variation | What it changes |
|---|---|
| One topic per node vs. one shared topic | whether all N×N writers contend on a single topic's history and heartbeat stream |
| Full mesh vs. star | a hub-and-spoke endpoint graph, which is O(N) connections instead of O(N²) |
| Hub on the small machine vs. on the large one | whether hub CPU or the radio link is the constraint |
| Zenoh router mesh vs. router star | zenoh's *infrastructure* graph, configured independently of the application's endpoint graph |
| Multicast vs. unicast discovery | Wi-Fi access points degrade multicast, so Discovery Server and static peer lists are tested as replacements |

### 1.4 Environment problems that shaped the method

These cost significant time and invalidated earlier conclusions, so they are
reported as results in their own right.

- **The kernel's ARP neighbour table was the real capacity limit, not the
  middleware.** `net.ipv4.neigh.default.gc_thresh3` defaults to 1024 entries
  and is shared across all network namespaces on a host, so every single-host
  run silently ran out of neighbour entries as node count grew. This produced
  three plausible, reproducible and entirely false middleware findings: an
  apparent capacity ceiling at 48 nodes, an apparent hard limit near 33
  participants for Fast DDS, and an apparent Cyclone DDS collapse under its own
  retransmissions. All three disappeared when the table was sized to 2·N×(N−1),
  which the test harness now does automatically. Three earlier write-ups in
  this repository (`dds-rmw-scaling.md`, `dds-rmw-qos-plane.md`,
  `dds-rmw-tuning.md`) predate the fix; their reasoning is still useful, their
  numbers are void.
- **A single run is not evidence.** The median coefficient of variation across
  repeated measurements is 0.0135, but the 95th percentile is 0.475. Delivery
  and throughput are stable; latency tails and failover times are not — zenoh's
  service-call tail latency on Wi-Fi at 96 nodes varied most, at a coefficient
  of variation of 1.22.
- **Emulating a link is not emulating the network.** A netem profile calibrated
  against the real Wi-Fi link's delay, jitter and loss reproduced the latency
  budget but ranked the three middlewares in the wrong order. It models neither
  loss that rises with offered load nor a medium shared between all
  participants.
- **Clocks across machines are not synchronized**, so no one-way cross-machine
  latency is reported anywhere. Round-trip times, command gaps and readiness
  are all derived from timestamps taken within a single process; ping is used
  only as a link-quality indicator.
- **Wi-Fi conditions were not controlled.** Ping at run start had a 95th
  percentile roughly nine times its median.
- **Resource figures on the LAN cover local processes only** — the remote
  server and router were not sampled, so cross-middleware resource comparisons
  on the LAN are incomplete.

---

## 2. Centralized results

Largest node count that passed the centralized criteria in *every* repetition,
and the behaviour behind it. "None" means no tested node count passed every
repetition; it does not mean no traffic was delivered. **All figures are medians
across repetitions.**

| Measurement | Fast DDS | Cyclone DDS | Zenoh |
|---|---|---|---|
| Ideal network — max nodes | **96** | **96** | 64 |
| Ideal network — offered rate sustained at 64 nodes (telemetry / RPC per s) | 1,280 / 320 | 320 / 64 | **3,200 / 640** |
| Ideal network — RPC 99th pct at 64 nodes | **3.7–4.2 ms** | **3.6–3.9 ms** | 46–85 ms |
| Ideal network — RPC 99th pct at 96 nodes, single server | 19.4 ms | **9.4 ms** | 1,094 ms |
| Wi-Fi LAN — max nodes | none | none | **64** — the only middleware passing every repetition |
| Wi-Fi LAN — telemetry delivery at 64 nodes | 0.951 default; **0.000 tuned** with the server on the remote machine | 0.78 default / 0.85 tuned — below the 0.90 bar | **0.970** |
| Wi-Fi LAN — RPC 99th pct at 64 nodes | 193 ms | 237 ms default / 57 ms tuned | **21 ms** |
| Two-server sharding at 96 nodes, repetitions passed | 3/3 ideal, 1/3 LAN | **3/3 ideal, 3/3 LAN** | 0/3 ideal, 2/3 LAN before completeness penalties |
| Failover gap at 96 nodes, active-active on Wi-Fi | **~1.0 s** | **~1.0 s** | 15.5 s |

At 128 nodes every tuned middleware completed the run and none met the criteria,
so the single-server limit is at or below 96 nodes on the ideal network. On
Wi-Fi the single-server limit is 64 nodes for zenoh and below any tested count
for the two DDS implementations.

Tuning is not uniformly an improvement. Fast DDS tuning cut median server CPU
from 49.6 % to 35.8 % but slightly reduced delivery. Cyclone DDS tuning was
neutral on steady-state delivery and made the deployment much slower to become
ready (2.08 s to 8.27 s). Zenoh tuning reduced RPC tail latency from 85 ms to
46 ms and server CPU from 66.9 % to 43.7 %.

---

## 3. All-to-all results

Maximum usable node count under the all-to-all criteria, with the connectivity
and delivery behind it.

| Substrate and topology | Fast DDS | Cyclone DDS | Zenoh |
|---|---|---|---|
| Ideal network, full mesh | 48 at default settings; **96** with 64 MB socket buffers (delivery 0.85 over a 60 s window; 0.66 over 30 s) | **96 at default settings** — all pairs connected, delivery 0.96, low variance; traffic still flows at 192 | 48 with a router star (delivery 0.98); a full router mesh holds it to 0.67 and produces no traffic at all at 96 |
| Ideal network, star | **96** at default settings — all pairs connected, delivery 0.94–0.96, 99th pct ~50 ms | **96** — delivery 0.95–0.97, median latency 0.5 ms | 48 at delivery 0.62–0.73; no traffic at 96 (the router graph, not the topology) |
| Ideal network, one shared topic | avoid: delivery 0.05 at 96 nodes, worse than full mesh | tolerates it — delivery 0.84–0.89 at 96 vs 0.96 on a mesh | no traffic at 96, same as every other shape |
| Wi-Fi LAN, full mesh | 24 (delivery 0.89–0.92, 99th pct 0.7–0.8 s); cross-machine pairs stop connecting at 32 | 8–16, limited by latency — pairs stay connected to 48 but the 99th percentile is already 0.5–0.9 s at 8 nodes and 1.5–7 s above 16 | **32** (delivery 0.94–0.95, 99th pct ~25 ms). At 48 all pairs still connect at delivery 0.92, but the 99th percentile reaches 1.8 s, above the criterion |
| Wi-Fi LAN, star | 64 spokes (delivery 0.95 both directions); 96 spokes at ~0.8 | 32 spokes (delivery ≥ 0.89); completes 96 at delivery 0.55 | **96 or more spokes** — delivery 0.95–0.97, discovery under 3.5 s; its limit was never reached |

At 192 nodes on the ideal network, Cyclone DDS still moves traffic while
neither zenoh router graph forms a usable one.

---

## 4. Recommended choice

| Deployment | Choose | Why not the others |
|---|---|---|
| Centralized, single-host or ideal network | **Fast DDS** or **Cyclone DDS** | both reach 96 clients; Fast DDS carries the higher rate of the two (1,280 telemetry/s vs 320), Cyclone DDS the lower tail latency at 96. Zenoh sustains 2.5× Fast DDS's rate but stops at 64 clients and its service-call tail latency is an order of magnitude worse |
| Centralized, Wi-Fi, one server | **Zenoh** | the only middleware to pass every repetition; Cyclone DDS is below the delivery bar, Fast DDS depends on where the server sits |
| Centralized, Wi-Fi, 96 nodes with redundancy | **Cyclone DDS, sharded** | the only two-server LAN configuration passing all three repetitions; zenoh's RPC tail exceeds 1 s and its failover takes 15 s |
| All-to-all, single-host or ideal network | **Cyclone DDS** at default settings | Fast DDS needs 64 MB socket buffers or a star topology to match; zenoh's router graph holds it back at 48 and fails at 96 |
| All-to-all, Wi-Fi, full mesh | **Zenoh** (32 nodes) | Fast DDS stops connecting at 32 nodes; Cyclone DDS keeps the mesh but with tail latency in seconds from 8 nodes up |
| All-to-all, Wi-Fi, star | **Zenoh** (96+ spokes) | Fast DDS reaches 64, Cyclone DDS 32 |

---

## 5. Capacity limits, causes and fixes

| Track | Observed limit | Component | Cause | Fix |
|---|---|---|---|---|
| both | Apparent ceilings at 48 nodes and near 33 participants, and apparent Cyclone DDS collapse under retransmission | the test host, not the middleware | the kernel ARP neighbour table (`gc_thresh3`, 1024 entries) is shared across network namespaces | size it to 2·N×(N−1); now automatic |
| centralized | Tuned Fast DDS delivers no traffic at all on Wi-Fi at 64 nodes | Fast DDS | Discovery Server behaviour depends on server placement: the tuned profile works with the server local and fails with it remote (0.000), while the default profile fails the other way round (0.951 with the server remote, 0.050 with it local) | fix the placement, measure that placement, and do not assume the two directions behave alike |
| centralized | RPC calls dropped, "Query queue depth of 10 reached" logged | Zenoh | the service query queue is 10 requests deep; raising it was not attempted | shard across servers, or keep RPC traffic off zenoh at this scale |
| centralized | Client processes killed at high node counts | Fast DDS | process collapse: 2 clients at 128 nodes on the ideal network, 6–9 clients in all three default-configuration LAN runs at 96 nodes | stay at or below 96 nodes, and tune |
| centralized | Recovery from server loss takes 15 s on Wi-Fi where the DDS implementations take ~1 s | Zenoh | router-mediated re-establishment after the primary router dies with its server | use active-active with two routers, or a DDS implementation where recovery time matters |
| all-to-all | Delivery drops to ~0.1 at 96 nodes | Fast DDS | default UDP socket buffers cannot absorb the fan-in from 95 writers | raise `socket_buffer_bytes` to 64 MB and the kernel receive buffer with it, or use a star topology instead |
| all-to-all | Exactly 32 processes start per machine; the rest fail at startup | Cyclone DDS | `Discovery/MaxAutoParticipantIndex` exhausts the per-address participant index | set it to at least the process count; now automatic |
| all-to-all | No traffic at all, with "Closing transport" logged, around 96 nodes | Zenoh | a full mesh between per-node routers overloads zenoh's own control plane | a router star restores 48 nodes at delivery 0.98; at 96 the single hub router must carry all N×N traffic and delivery falls to ~0.1; at 192 no graph forms. A router tree would be required and is not implemented |
| all-to-all | Cross-machine pairs stop connecting at 32 nodes while local pairs are fine | Fast DDS | N²/4 unicast flows across one contended access point | change topology; larger buffers do not recover packets lost over the air |
| all-to-all | 99th percentile latency in seconds on Wi-Fi at any node count | Cyclone DDS | its reliable delivery machinery retransmits into a lossy link and queues behind itself | switch that traffic to best-effort QoS. On *emulated* Wi-Fi at 32 nodes this moved the tail from 340 ms to 100 ms for 1 % less delivery — directional evidence only, since uniform emulated loss is the easy case. None of its internal retransmission settings beat the defaults |

---

## 6. Summary

Three ROS 2 middleware implementations — Fast DDS 2.14.5, Cyclone DDS 0.10.5
and `rmw_zenoh_cpp` 0.2.9 — were compared under two application architectures:
a **centralized** one (clients exchange telemetry, commands and service calls
with one or two servers) and an **all-to-all** one (every node talks to every
other). Each ran on an ideal single-host network, on emulated Wi-Fi, and across
two machines over real Wi-Fi, from 4 to 192 nodes, in default and tuned
configurations. Every point was repeated at least three times and a capacity
counted only when every repetition passed. The centralized track alone covers
573 measurement points, of which 561 completed and 12 failed for recorded
reasons. A node count counts as passing when telemetry delivery and RPC success
are ≥ 0.90 and RPC tail latency is ≤ 1 s (centralized), or when ≥ 99 % of pairs
connect at delivery ≥ 0.90 and sub-second tail latency (all-to-all). **A
stricter centralized target — 0.99 delivery and success, 200 ms tail latency —
was met by no middleware at any node count on any network.** Client counts in
the centralized track and peer counts in the all-to-all track are not
comparable.

**There is no overall winner; the choice follows from the network and the
traffic type.** Centralized, on an ideal network: Fast DDS and Cyclone DDS both
reach 96 clients, with service-call tail latency of 19 ms and 9 ms respectively
at that scale (both under 5 ms at 64 clients). Fast DDS carries four times
Cyclone's message rate; zenoh sustains 2.5× Fast DDS's rate but only to 64
clients, with a tail latency an order of magnitude worse. Over real Wi-Fi the
ranking inverts: zenoh was the only middleware to reach 64 clients in every
repetition, holding 0.97 telemetry delivery and 21 ms service-call tail
latency, while Cyclone DDS stayed below the 0.90 delivery bar and Fast DDS
proved sensitive to server placement — its tuned configuration delivered
nothing at all with the server on the remote machine, and its default
configuration failed in the opposite direction. No single server passed above
96 clients on the ideal network, or above 64 on Wi-Fi.

**Zenoh's Wi-Fi advantage is in publish/subscribe traffic only.** Its
weaknesses are service calls and recovery: a service query queue fixed at 10
requests, tail latency above one second at 96 clients, and 15 s to recover from
a server failure where both DDS implementations recover in about one second.
For a telemetry-dominated deployment over radio, choose zenoh; for one that
depends on service calls or on redundancy, choose Cyclone DDS with clients
sharded across two servers — the only two-server configuration that passed
every repetition on Wi-Fi.

**For all-to-all deployments, topology is a stronger lever than tuning.**
Cyclone DDS is the most robust at default settings on an ideal network: 96
peers fully connected at 0.96 delivery, and still moving traffic at 192. On
Wi-Fi, moving from a full mesh to hub-and-spoke raised the usable node count
three- to fourfold for every middleware — more than any configuration change
achieved. On the ideal network a star recovered Fast DDS's 96 peers at default
settings, matching what raising its socket buffers to 64 MB buys. The same
lever cuts the other way: routing all traffic through one shared topic made
Fast DDS worse than a plain mesh (0.05 delivery at 96), and zenoh's results
depend entirely on the graph between its routers — a full mesh between them
fails at 96 peers, a single hub router then has to carry all N×N traffic, and
above roughly 96 peers no configuration we have works.

**The most costly finding concerned the test environment rather than the
middleware.** Three separate, reproducible and internally consistent capacity
ceilings turned out to be one kernel setting: the ARP neighbour table holds
1024 entries by default and is shared across all network namespaces on a host,
so single-host runs silently ran out of entries as node count grew. Sizing it
to 2·N×(N−1) removed all three. Two further limits found later — Cyclone DDS's
per-address participant index (`MaxAutoParticipantIndex`, which caps a machine
at 32 processes) and Fast DDS's default socket buffers (which need 64 MB under
high fan-in) — were ceilings of the same kind: properties of the environment
that read as properties of the software. The transferable rules are to rule out
the host before attributing a limit to middleware, to repeat every measurement
before believing it, and to benchmark on the network that will actually be
deployed on: an emulated link calibrated to the real one reproduced the latency
budget and still ranked the three middlewares in the wrong order.

---

*Detailed per-phase data: `docs/results/dds-centralized.md`,
`dds-recommendations.md`, `dds-neighbor-table.md`, `dds-lan.md`,
`dds-star.md`, `dds-topology.md`, `dds-netem.md`. Raw output under
`results/centralized/` and `results/dds/`.*
