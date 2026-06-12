# Roadmap

| Milestone | Scope | Exit criteria |
|---|---|---|
| **M1 — Channel core** | Mobility, propagation, FHSS/jammer models, channel engine forwarder, netns plumbing, validation suite | 2-node smoke test passes (zenoh pub/sub through the engine); validation report clean; deterministic replay test green |
| **M2 — Node agent** | Rust agent: zenoh + automerge telemetry sharing, MLS group at startup, metrics emission | 4 agents share state through the engine; AoI measured end-to-end |
| **M3 — MVP benchmark** | Jammer scenarios, sweep harness, paired-seed replay, plotting | Jamming resilience curves produced (PDR / latency / AoI vs jammer power × hop rate) |
| **M-gate — EMANE spike** ✅ | Reproduced one M3 cell in EMANE + emane-jammer-simple | **DONE 2026-06-12:** [ADR-0005](adr/0005-emane-spike-decision.md) — proceed hand-rolled, retain EMANE for cross-validation; weak-link calibration + MLS hardening logged as follow-ups |
| **M4 — Satellite failover** | Satlink channel model, agent link manager, outage scenario | Failover-time metric measured: uplink dies → mesh carries state |
| **M5+** | Reactive jamming, routing comparisons, frequency-band sweeps, Sionna RT pathloss backend | per-feature |

Testing discipline throughout: unit tests per module; the model validation suite
(`validation/`) reproduces published propagation curves and closed-form FHSS/SINR
results, rendered as a report — it is the artifact that makes benchmark results
defensible. Integration smoke test runs in CI (requires CAP_NET_ADMIN; user-namespace
or privileged runner).
