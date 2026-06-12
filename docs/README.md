# Documentation

- [architecture.md](architecture.md) — system architecture, components, data flow, repo layout
- [models.md](models.md) — RF propagation, FHSS, jammer, and satellite link models; fidelity limits
- [benchmarking.md](benchmarking.md) — metrics, variance control, MVP scenario, scenario format
- [roadmap.md](roadmap.md) — milestones M1–M5 and the EMANE decision gate
- [adr/](adr/) — architecture decision records
  - [0001](adr/0001-channel-emulation-hand-rolled-vs-emane.md) — hand-rolled channel engine vs EMANE
  - [0002](adr/0002-no-gazebo-single-clock.md) — no physics co-simulator; single clock
  - [0003](adr/0003-rust-agent-python-sim.md) — Rust agent, Python sim
  - [0004](adr/0004-empirical-propagation-first.md) — empirical propagation first, Sionna RT later
  - [0005](adr/0005-emane-spike-decision.md) — EMANE spike outcome: keep hand-rolled, retain EMANE for cross-validation

See also [emane_spike/FINDINGS.md](../emane_spike/FINDINGS.md) for the full spike report.
