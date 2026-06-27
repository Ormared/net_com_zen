# Why each RMW collapses, and what we can tune to fix it

Companion to [`dds-rmw-scaling.md`](dds-rmw-scaling.md). That document established
*what* happens: all three RMWs are healthy through N = 24, hit a sharp knee at
N = 48, and form no usable mesh at N = 96 within a 30 s window — while the host
sits half-idle (29 GB free at N = 96), so the failure is in the **middleware
discovery / endpoint-matching path**, not the rig.

This document answers *why*, per architecture, and *what to turn* to push the
knee out. It is a design analysis grounded in the measured numbers and in how
each stack actually does discovery. Where it predicts the effect of a tweak we
have **not** yet run, it says so — those are the next experiments, listed at the
end.

---

## 0. What we ran with (the baseline being tuned)

Everything below is measured against this configuration. The whole point is that
it is the **stock, untuned** path for each RMW — that is the fair scaling
baseline, and it is also why there is so much headroom to recover.

| RMW | Discovery mechanism as run | Config we set | Left at default |
|---|---|---|---|
| **Fast DDS** (`rmw_fastrtps_cpp` 8.4.3) | Distributed SPDP + SEDP, native UDP multicast | `FASTDDS_BUILTIN_TRANSPORTS=UDPv4` (SHM off, no LARGE_DATA) | announcement period, discovery server (off), initial peers, history/QoS, socket buffers |
| **Cyclone** (`rmw_cyclonedds_cpp` 2.2.3) | Distributed SPDP + SEDP, native UDP multicast | `CYCLONEDDS_URI`: SHM off, `AllowMulticast=true`, interface autodetermine | **everything else** — single receive thread, default 1 MiB socket buffer, SPDP interval, no peers list |
| **Zenoh** (`rmw_zenoh_cpp` 0.2.9) | Router-brokered; one `rmw_zenohd` per netns | **full TCP router mesh**, one link per lower-index pair | scouting, gossip, batching, mode |

The kernel side is constant across all three: one netns + veth per drone, every
veth on a single Linux bridge, `mcast_snooping 0` (multicast floods to **all**
ports), STP off. SHM forced off everywhere so traffic is real UDP over the veth.

---

## 1. The shared root cause: O(N²) discovery on a flat segment

All three are doing, at bottom, the same dangerous thing: **every node tries to
establish state with every other node**, and nothing prunes that to the handful
of peers each node actually talks to. On a flat L2 segment with multicast
flooding, the work scales as N².

Concretely, for the two RTPS stacks (Fast DDS, Cyclone) at N = 96, stock simple
discovery means:

- **SPDP (participant discovery), multicast.** Each participant multicasts a
  PDP announcement on a timer (Fast DDS default 3 s). 96 announcers × flooded to
  96 bridge ports = every node's socket receives ~96 announcements per period,
  ~32 announcement-frames/s system-wide × 96 deliveries = **~3 000 frame
  deliveries/s of pure "I exist" traffic**, none of it pruned because IGMP
  snooping is off and they all share one multicast group.
- **SEDP (endpoint discovery), unicast reliable.** Once two participants see
  each other, they exchange their reader/writer lists over **reliable** builtin
  endpoints. That is a full N² mesh of stateful reliable endpoints — 96 × 95 ≈
  **9 000 endpoint-match conversations**, each with its own heartbeat/ACKNACK
  loop. This is the expensive part, and it is reliable, so any dropped fragment
  triggers retransmission.

The trap closes when the SPDP flood causes socket-buffer drops, those drops hit
SEDP fragments, reliable SEDP retransmits, the retransmissions add to the flood,
and discovery time goes super-linear. That is exactly the shape we measured:
discovery 0.67 s → 26.5 s → 30 s (Cyclone, N = 24 → 48 → 96) while CPU stays
~half-idle. The bottleneck is **latency of a feedback loop, not throughput** —
which is why a faster host wouldn't save it and why the fixes below are all about
*removing work*, not adding cores.

Zenoh avoids the multicast storm (it is brokered) but trades it for an O(N²)
**router mesh** — see §4.

---

## 2. Fast DDS — distributed simple discovery, the textbook N² case

**Measured:** healthy to 24; N = 48 mesh 0.45 / delivery 0.45 / disc 3.24 s;
N = 96 mesh 0.11 / delivery 0.11 / disc 4.28 s but the **tightest tail of the
DDS pair** (p99 24.6 ms). The tight tail is the tell: the *packets that get
through* are fine; Fast DDS is not pathologically retransmitting like Cyclone,
it simply **never finishes matching all the endpoints** in time. Its discovery
time barely rises (3.2 → 4.3 s) while its mesh craters — i.e. it gives up / plateaus
rather than thrashes.

### Why it dies where it does
Stock Fast DDS runs **distributed** SPDP+SEDP. Nothing brokers discovery, so the
N² builtin-endpoint mesh is unavoidable, and the 30 s window isn't enough to
complete ~9 000 reliable endpoint matches once the SPDP flood starts causing
drops. It degrades more gracefully than Cyclone because Fast DDS uses a
thread-pool reception model and is less sensitive to a single overflowing socket
(see §3 for the contrast).

### The knobs, what each one actually does, and expected effect

1. **Discovery Server (the big lever).** `ROS_DISCOVERY_SERVER=<ip:port>` (or XML
   `<discoveryServer>`). Replaces distributed discovery with a **client–server
   broker**: every node connects only to one (or a few) server participants; the
   server redistributes discovery data. This turns the O(N²) participant mesh
   into **O(N)** — each client has one discovery link instead of 95. This is the
   single highest-leverage change for Fast DDS at scale and is the officially
   recommended large-fleet path. *Expected:* pushes the knee well past 96; the
   SPDP multicast storm disappears entirely (discovery becomes unicast to the
   server). Cost: the server is a new SPOF (mitigate with redundant/backup
   servers) and a topology decision (which netns hosts it).

2. **Initial peers + disable multicast.** `<initialPeersList>` with an explicit
   unicast locator list, plus turning off multicast announcements. Converts SPDP
   from "flood the segment" to "unicast to known peers." Removes the multicast
   storm but **keeps O(N²)** (you still list/contact everyone) — useful when you
   can't run a server but multicast flooding is the dominant pain. *Expected:*
   helps moderately at 48, less at 96 because the endpoint mesh is still N².

3. **Announcement period** (`leaseDuration_announcementperiod`, default 3 s).
   Lengthening it thins the SPDP flood (fewer frames/s) at the cost of slower
   *initial* discovery and slower failure detection. *Expected:* a second-order
   lever — buys headroom on the flood but doesn't change the N² endpoint cost.
   Worth combining with #2.

4. **Static EDP / static discovery** (XML `<staticEDP>` with an endpoint XML).
   Pre-declares every reader/writer so **SEDP never runs** — endpoints are known
   a priori. Eliminates the entire endpoint-matching conversation (the expensive
   reliable N² part). *Expected:* large win on discovery *time* specifically;
   brittle (must regenerate the static file whenever topics change), so it suits
   a fixed swarm topology like ours where every drone has the same one topic.

5. **Transport: `LARGE_DATA` mode** (`FASTDDS_BUILTIN_TRANSPORTS=LARGE_DATA`).
   Uses UDP for discovery but TCP for user data and shared-memory where local.
   Not a discovery fix per se, but relevant if payload grows; for our 255 B
   payload it's irrelevant — noting it so it isn't confused with a scaling lever.

6. **QoS / history depth.** Smaller KEEP_LAST depth and disabling unneeded
   builtin endpoints trims per-endpoint state. Second-order vs. the topology
   levers above.

**If you change one thing in Fast DDS: run a Discovery Server.** Everything else
is incremental; the server is the structural fix.

---

## 3. Cyclone DDS — same protocol, collapses hardest. Why?

**Measured (the worst of the three):** N = 48 mesh 0.20 / delivery 0.059 /
**disc 26.5 s** / p99 1979 ms; N = 96 mesh 0.054 / delivery 0.011 / **p50 664 ms**
/ p99 2975 ms. Cyclone is RTPS too — same SPDP+SEDP as Fast DDS — yet it falls
apart at 48 where Fast DDS is merely wounded. The *pathological tail* (p99 ≈ 2–3 s,
p50 blown to 664 ms) is the opposite signature from Fast DDS: this is a stack
**thrashing on retransmits**, not one quietly plateauing.

### Why Cyclone specifically thrashes
The difference is the reception/buffering model, not the protocol:

- **Default single receive path + a small socket buffer.** Stock Cyclone reads
  the network on a lean receive path and requests a modest socket receive buffer
  (`Internal/MinimumSocketReceiveBufferSize`, ~1 MiB by default, itself capped by
  the kernel's `net.core.rmem_max`). Under the SPDP multicast flood (every one of
  the 96 announcements hitting every port) that single socket **overflows and
  drops** discovery datagrams.
- **Reliable SEDP turns drops into a storm.** Dropped SEDP fragments are
  retransmitted reliably. The retransmissions compound the flood, which causes
  more drops — the feedback loop from §1, but Cyclone enters it earlier because
  its default buffering has less slack than Fast DDS's pooled reception. That is
  why discovery jumps to 26 s at the *same* N where Fast DDS is at 3 s.
- We left **everything** at default (our `CYCLONEDDS_URI` only turns SHM off and
  sets the interface). So Cyclone is the most under-tuned of the three and, not
  coincidentally, the worst — which also means it has the **most recoverable
  headroom**.

### The knobs, what each does, expected effect

1. **Bigger socket + kernel buffers (cheapest first move).**
   `<Internal><SocketReceiveBufferSize min="..."/></Internal>` plus raising
   `net.core.rmem_max` / `wmem_max` and `net.core.netdev_max_backlog` on the
   host. Directly attacks the overflow that starts the retransmit storm.
   *Expected:* the highest value-for-effort Cyclone change — should pull the 26 s
   discovery at N = 48 down sharply on its own, because it breaks the drop →
   retransmit → more-drop loop at the source. (Note: the kernel sysctls are a
   host-level change the bridge substrate would need to set; worth wiring into
   the netns setup.)

2. **Multiple receive threads.** `<Internal><MultipleReceiveThreads enabled="true"/>`.
   Parallelises packet reception so one hot socket isn't the whole story.
   *Expected:* complements #1; helps most once buffers are already enlarged.

3. **Unicast discovery via a peers list.** `<Discovery><Peers>` with explicit
   `<Peer address="..."/>` entries and `<AllowMulticast>false</AllowMulticast>`.
   Cyclone has **no discovery-server/broker** equivalent to Fast DDS, so this is
   the way to kill the multicast flood: every participant unicasts SPDP to a
   known list instead of flooding the group. *Expected:* removes the storm
   (large win at 48/96), but keeps O(N²) contact and requires generating a
   96-entry peers list per run — straightforward from our scenario, since we
   already own every netns's veth address.

4. **SPDP interval.** `<Discovery><SPDPInterval>`. Same trade as Fast DDS's
   announcement period — thin the flood vs. slower initial discovery. Second-order;
   pair with #3.

5. **`MaxAutoParticipantIndex` / participant index.** Governs how many
   participant slots Cyclone probes on the multicast locator. Mostly relevant
   when many participants share a host/port space; here each is in its own netns
   so it's not our bottleneck — flagged so it isn't mistaken for one.

6. **Fragmentation / message size** (`General/MaxMessageSize`,
   `Internal/FragmentSize`). Tuning fragment size to stay under the path MTU
   reduces fragment loss amplification. Minor for 255 B user data; matters more
   for the (larger) SEDP bursts.

**If you change one thing in Cyclone: enlarge the receive buffers (socket +
kernel).** Its collapse is a buffer-overflow-triggered retransmit storm, and
that is the cheapest thing to fix. Unicast peers is the structural second step
since Cyclone has no broker to fall back on.

---

## 4. Zenoh — brokered, degrades most gracefully, still O(N²) as we wired it

**Measured (best of the three at the knee):** N = 48 mesh **0.918** / delivery
0.873 / disc 3.21 s — roughly *double* the mesh of the DDS pair. But by N = 96
it too falls to mesh 0.130 / delivery 0.120. So Zenoh buys you one doubling of
scale, then hits the same wall.

### Why it holds at 48 and why it still falls at 96
Zenoh is **not** RTPS. There is no SPDP/SEDP multicast storm. Each node runs a
local `rmw_zenohd` router; routers gossip topic **declarations**
(publishers/subscribers/queryables) to each other; data flows router-to-router.
Because we **pre-wired** the router connections (explicit TCP, one link per
lower-index pair), there is no "find each other" flood at all — that is precisely
why Zenoh degrades gracefully through 48 and why its "discovery time" is really
session/topic-matching time, not peer discovery (the asymmetry caveat in the
scaling doc).

But the way we wired it is a **full router mesh**: N(N-1)/2 = **4 560 TCP
sessions** at N = 96, each with keepalives, and every one of the 96 subscriber
declarations has to propagate across all 96 routers (~9 000 declaration entries
gossiped through the mesh). So we removed the *multicast* N² and replaced it with
a *router-connection* N². At 48 that mesh (≈1 100 links) still fits in the
window; at 96 the gossip + keepalive load on 4 560 sessions doesn't converge in
30 s. The collapse is structural to **the topology we chose**, not to Zenoh.

### The knobs, what each does, expected effect

1. **Topology: star/brokered instead of full mesh (the big lever).** Run **one**
   shared `rmw_zenohd` (or a small fixed set) reachable by all nodes over the
   bridge, with each node's session in **`client`** mode connecting to it. That
   makes node→router connections **O(N)** and collapses the router-mesh gossip to
   a single hub. This is Zenoh playing to its actual strength — it is *designed*
   to be brokered, and our full P2P mesh fought that. *Expected:* the largest
   Zenoh win by far; should carry N = 96 comfortably since the per-node link
   count drops from 95 to 1. Trade: the hub router is a SPOF/bottleneck (mitigate
   with 2–3 hub routers in a small mesh = hierarchical/gateway topology).

2. **`mode`: client vs peer.** Nodes as **`client`** (dial the hub, no peer
   scouting) rather than `peer` (try to mesh with everyone). Directly enforces
   the star above and stops nodes from forming incidental peer links. *Expected:*
   necessary companion to #1.

3. **Scouting / gossip config.** With an explicit hub, turn **off multicast
   scouting** and lean on gossip from the hub only. Removes any residual
   discovery chatter. *Expected:* small once #1/#2 are in place; mostly hygiene.

4. **Transport batching / queue** (`transport/link/tx/batch_size`, queue sizing).
   Larger batches amortise per-message overhead across the (now fewer) links.
   *Expected:* second-order; matters at high message rate, not at our 5 Hz.

5. **Hierarchical gateways.** For very large N, tier the routers
   (leaf routers → gateway routers) so no single router holds all sessions. The
   generalisation of #1 beyond what one hub can carry. *Expected:* the path past
   a few hundred nodes; overkill at 96 if a single hub suffices.

**If you change one thing in Zenoh: collapse the full router mesh to a star (one
hub router, nodes as `client`).** Our O(N²) wiring, not Zenoh, is what caps it at
48.

---

## 5. Cross-cutting levers (apply to all three)

These sit underneath the per-RMW knobs and move every curve:

- **The 30 s run window (measurement, not middleware).** Caveat 1 of the scaling
  doc: part of the N ≥ 48 collapse is discovery not *completing* in time, not
  being impossible — Cyclone's 26 s discovery at 48 proves some pairs *do*
  connect, slowly. **Re-run 48 and 96 with a 120–300 s window** to separate
  "slow" from "broken." This is the first experiment to run because it tells you,
  per RMW, whether you're tuning a *latency* problem (it converges, just late) or
  a *capacity* problem (it never converges) — and those want different fixes.

- **Kernel socket buffers** (`net.core.rmem_max`, `wmem_max`,
  `netdev_max_backlog`). Caps how much slack any RMW's receive buffer can claim.
  Especially decisive for Cyclone (§3.1) but raises the drop threshold for all.
  Currently left at host defaults; the bridge setup is the natural place to bump
  them per run.

- **IGMP snooping on the bridge.** We deliberately set `mcast_snooping 0` (flood
  to all ports) so each RMW gets pure native multicast. Turning snooping *on*
  would prune multicast to subscribed ports — but since all DDS participants join
  the **same** SPDP group, snooping wouldn't actually thin the SPDP flood much.
  The real flood reduction is **unicast discovery** (Fast DDS server / Cyclone
  peers / Zenoh hub), not L2 snooping. Flagged so it isn't mistaken for a fix.

- **QoS / payload / rate.** Single seed, 255 B, 5 Hz, reliable. Reliable QoS is
  part of why drops hurt (retransmits); a best-effort variant would isolate how
  much of the collapse is the reliability machinery vs. raw discovery. Worth one
  best-effort cell at N = 48 as a diagnostic.

---

## 6. Next experiments (predicted effect → how to verify)

Ordered by value-for-effort. Each is a small delta on the existing sweep harness;
the host-pressure counters already in place will confirm the host still isn't the
bottleneck so any improvement is attributable to the tweak.

| # | Change | RMW(s) | Predicted effect | Verify by |
|---|---|---|---|---|
| 1 | **Longer window** (120–300 s) at N = 48, 96 | all | Separates "slow" from "broken"; expect Cyclone/Fast DDS mesh to climb materially given time | mesh & delivery vs. the 30 s baseline at same N |
| 2 | **Cyclone: raise socket + kernel recv buffers** | Cyclone | 26 s discovery at 48 drops sharply; tail latency (p99 2 s) shrinks — breaks the retransmit storm | discovery_time_s, p99 at N = 48 |
| 3 | **Zenoh: star topology** (one hub router, nodes `client`) | Zenoh | Largest single win; N = 96 should recover toward full mesh (links per node 95 → 1) | mesh/delivery at N = 96; router connection count |
| 4 | **Fast DDS: Discovery Server** | Fast DDS | Knee pushed past 96; SPDP multicast storm gone | discovery_time_s flat vs. N; mesh at 96 |
| 5 | **Cyclone: unicast peers list** (multicast off) | Cyclone | Removes the SPDP flood; complements #2 | per-port multicast frame rate; discovery at 48/96 |
| 6 | **Best-effort QoS** diagnostic cell | all | Isolates how much collapse is reliability retransmit vs. discovery | delivery/latency delta vs. reliable at N = 48 |

The harness change for each is small: a new env/XML/topology branch in
`_spawn_*` plus a per-N scenario, reusing `dds_report.py` to re-aggregate. The
clean way to present the payoff is a **before/after at N = 48 and 96** for each
RMW on the same axes as `dds-rmw-scaling.md`, so the knee visibly moves.

## 7. One-line summary per architecture

- **Fast DDS** plateaus (doesn't thrash); its N² is the *endpoint* mesh →
  **broker it with a Discovery Server.**
- **Cyclone** thrashes on a buffer-overflow retransmit storm; it has no broker →
  **enlarge receive buffers, then go unicast-peers.**
- **Zenoh** was capped by *our* full router mesh, not by Zenoh → **make it a star
  (one hub, clients).**

All three share the same disease — every node discovering every node on a flat
segment — and the cure is always the same shape: **stop doing O(N²); broker,
unicast, or hub the discovery so each node only does O(1) discovery work.**
