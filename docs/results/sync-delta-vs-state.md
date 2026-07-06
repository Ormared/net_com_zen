# CRDT sync A/B: op-based delta vs state-based snapshots under jamming

Follow-up to the [transport A/B](transport-tcp-vs-udp.md). Even over UDP/best-
effort, the op-based sync leaves two loss weaknesses: a lost automerge delta
**orphans** all later deltas at a receiver until the next full snapshot, and the
full snapshots (~7 KB encrypted) **fragment** across UDP datagrams, so one lost
fragment loses the whole snapshot.

## The two strategies (`agent.sync_mode`)

- **delta** (op-based, default): automerge `save_incremental` each tick + a full
  `save()` every N ticks. Causal — needs the prior deltas to apply.
- **state** (state-based): each tick ships the node's full compact view of all
  peers as a `{id:[seq,ts]}` map (**255 bytes encrypted — one datagram**),
  merged by per-node max-seq. Every message is self-contained and idempotent: a
  lost one is simply superseded by the next. automerge remains the local store;
  the wire/merge strategy is the variable under test.

## Result (resilience_4node, barrage, UDP/best-effort, jammer active)

| jammer dBm | sync | update delivery | mean AoI |
|---|---|---|---|
| 55 | delta | 0.590 | 2.82 s |
| 55 | **state** | **0.713** | **2.49 s** |
| 65 | delta | 0.458 | 3.84 s |
| 65 | **state** | **0.590** | **3.35 s** |

State-based delivers **+21 % (55 dBm) to +29 % (65 dBm)** more application state
and lower AoI; the advantage **grows with loss** — self-contained single-datagram
snapshots neither orphan nor fragment.

## Cumulative win

Stacking both axes against the original baseline at 65 dBm:

| config | update delivery |
|---|---|
| tcp + delta (baseline) | 0.430 |
| udp + delta | 0.458 |
| **udp + state** | **0.590** (+37 %) |

**udp/best-effort + state-based sync** is the resilient configuration for low-
bandwidth critical state under EW — consistent with the project's payload
priority (telemetry/coordinates over guaranteed in-order delivery).

## Caveat

State-based ships O(N) peer entries per message; fine at the project's ≤5 nodes
(255 B) but grows with swarm size. For larger swarms, ship only changed entries
or a digest — out of scope here.
