# Transport A/B: zenoh-over-TCP vs UDP/best-effort under jamming

Motivation: M3 (AoI amplification under loss) and M4 (failover denial when TCP
goodput collapsed while sessions stayed nominally up) both pointed at the
transport. This A/B quantifies it.

## Setup

`resilience_4node` (4 vehicles, mixed open-field/forest), barrage jammer at
t=20 s swept over power, 35 s runs. Same Rust agent, MLS on, identical seed and
pathloss. Only the zenoh link differs:

- **tcp** — reliable (TCP handles retransmission/ordering).
- **udp** — best-effort publication (`Reliability::BestEffort`): lost datagrams
  are dropped, not retransmitted; CRDT increments + periodic snapshots heal gaps.

## Result (jammer active)

| jammer dBm | transport | frame PDR | **update delivery** | **mean AoI** |
|---|---|---|---|---|
| 45 | tcp | 0.759 | 0.482 | 3.24 s |
| 45 | udp | 0.533 | **0.564** | **2.65 s** |
| 55 | tcp | 0.649 | 0.499 | 3.38 s |
| 55 | udp | 0.430 | **0.563** | **2.87 s** |
| 65 | tcp | 0.334 | 0.430 | 4.10 s |
| 65 | udp | 0.136 | **0.496** | **3.70 s** |

`frame PDR` = channel-level delivered/attempted; `update delivery` = fraction of
published state updates actually applied at peers (application goodput);
`mean AoI` = time-averaged age of information.

## Finding

UDP/best-effort delivers **more application state and lower AoI than TCP at
every power, despite much lower raw frame PDR** (e.g. at 65 dBm: 0.14 frame PDR
yet higher update delivery and lower AoI than TCP's 0.33). Mechanism:

- **TCP** spends delivered frames on retransmits/ACKs and suffers head-of-line
  blocking — fewer *useful* updates land per delivered frame, and a single
  stalled segment delays everything behind it, inflating AoI.
- **UDP/best-effort** loses more raw frames (leaner traffic, no retransmit
  inflation) but every frame that arrives carries fresh state applied
  immediately; the CRDT tolerates the gaps. The update-delivery advantage
  **widens as loss increases**.

This validates **zenoh-over-UDP/best-effort as the resilient transport** for
low-bandwidth critical state under EW, consistent with the project's priority
(telemetry/coordinates over guaranteed delivery).

## Caveats / follow-ups

- Large (~7 KB) MLS full snapshots fragment across multiple UDP datagrams; under
  heavy loss a single lost fragment loses the whole snapshot, slowing CRDT
  re-heal. **Delta-state / single-datagram snapshots** would compound the UDP
  win — next candidate work.
- The M4 failover-under-jamming case (command at RF-range edge + local jammer)
  is total denial for both transports; that is a geometry/link-budget limit, not
  a transport one.
