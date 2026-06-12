# Roadmap

| Milestone | Scope | Exit criteria |
|---|---|---|
| **M1 — Channel core** | Mobility, propagation, FHSS/jammer models, channel engine forwarder, netns plumbing, validation suite | 2-node smoke test passes (zenoh pub/sub through the engine); validation report clean; deterministic replay test green |
| **M2 — Node agent** | Rust agent: zenoh + automerge telemetry sharing, MLS group at startup, metrics emission | 4 agents share state through the engine; AoI measured end-to-end |
| **M3 — MVP benchmark** | Jammer scenarios, sweep harness, paired-seed replay, plotting | Jamming resilience curves produced (PDR / latency / AoI vs jammer power × hop rate) |
| **M-gate — EMANE spike** | Timeboxed 1–2 days: reproduce one M3 scenario in EMANE + emane-jammer-simple | Decision doc comparing numbers + ergonomics; proceed hand-rolled or pivot, with evidence |
| **M4 — Satellite failover** | Satlink channel model, agent link manager, outage scenario | Failover-time metric measured: uplink dies → mesh carries state |
| **M5+** | Reactive jamming, routing comparisons, frequency-band sweeps, Sionna RT pathloss backend | per-feature |

Testing discipline throughout: unit tests per module; the model validation suite
(`validation/`) reproduces published propagation curves and closed-form FHSS/SINR
results, rendered as a report — it is the artifact that makes benchmark results
defensible. Integration smoke test runs in CI (requires CAP_NET_ADMIN; user-namespace
or privileged runner).
