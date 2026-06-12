# Benchmarking Methodology

## Metrics

| Metric | Definition | Source |
|---|---|---|
| Packet delivery ratio (PDR) | delivered / sent, per link and end-to-end | channel engine logs |
| Latency | publish → remote delivery, per message class | agent metrics |
| State staleness (AoI) | age of information: now − timestamp of last applied CRDT update, per peer | agent metrics |
| Drop attribution | loss decomposed by cause: range / foliage / jam / hop_collision / queue | channel engine logs |
| MLS health | rekey success rate, time-to-rekey under loss | agent metrics |
| Failover time (M4) | per vehicle: outage start → command applies that vehicle's first post-outage state update; swarm = max over vehicles | `harness/failover.py` from command's recv log + manifest |

Per-packet channel verdicts and agent events are merged on a shared monotonic
timebase into parquet; every run writes a manifest (full config, seeds, git hash,
package versions) for reproducibility.

## Variance control

- **Seeded channel realizations:** all channel randomness derives from one seed;
  identical seed ⇒ byte-identical verdict stream. Middleware variants are compared
  **pairwise on the same channel realization**, removing channel variance from the
  comparison — essential for small effect sizes at N < 5 nodes.
- **Multi-seed sweeps:** each configuration runs across a seed set; curves are
  reported with spread, never single runs.
- Agent-side OS scheduling noise remains; it is small relative to modeled link
  effects and averaged by the seed sweep.

## MVP scenario (M3): jamming resilience curve

4 vehicles patrol waypoints on a mixed open-field/forest map, sharing telemetry
state via Zenoh + automerge with an MLS group established at start. A jammer
activates mid-run.

Sweep matrix:

| Axis | Values (default) |
|---|---|
| FHSS hop rate | 4 values (e.g. 10 / 100 / 500 / 1000 hop/s) |
| Jammer power | ramp, e.g. 10–40 dBm |
| Jammer type | barrage, spot |
| Seeds | ≥ 5 per cell |

Output: PDR, latency, and AoI vs. jammer power, one curve per hop rate, with drop
attribution breakdowns.

## Scenario definition

Scenarios are YAML files in `scenarios/` validated against a pydantic schema:
map + environment profile, vehicles (waypoints, radio profile), jammers (type,
position, timeline), satellite link, sweep matrix, seeds, duration. A scenario file
plus the git hash fully determines a run.
