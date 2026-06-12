# Roadmap

| Milestone | Scope | Exit criteria |
|---|---|---|
| **M1 — Channel core** | Mobility, propagation, FHSS/jammer models, channel engine forwarder, netns plumbing, validation suite | 2-node smoke test passes (zenoh pub/sub through the engine); validation report clean; deterministic replay test green |
| **M2 — Node agent** | Rust agent: zenoh + automerge telemetry sharing, MLS group at startup, metrics emission | 4 agents share state through the engine; AoI measured end-to-end |
| **M3 — MVP benchmark** | Jammer scenarios, sweep harness, paired-seed replay, plotting | Jamming resilience curves produced (PDR / latency / AoI vs jammer power × hop rate) |
| **M-gate — EMANE spike** ✅ | Reproduced one M3 cell in EMANE + emane-jammer-simple | **DONE 2026-06-12:** [ADR-0005](adr/0005-emane-spike-decision.md) — proceed hand-rolled, retain EMANE for cross-validation; weak-link calibration + MLS hardening logged as follow-ups |
| **M4 — Satellite failover** ✅ | Satlink channel model, agent link manager, outage scenario | **DONE 2026-06-12:** clean fallback seamless (0.27 s swarm failover); under jamming the RF fallback's zenoh-over-TCP goodput collapses (sessions stay up via keepalives, no state crosses) → command isolated, link manager fires `link_down`. Reinforces the zenoh-over-UDP axis. |
| **Transport A/B** ✅ | zenoh-over-TCP vs UDP/best-effort under jamming | **DONE 2026-06-12:** UDP/best-effort wins app goodput + AoI at every power despite lower frame PDR (TCP head-of-line blocking). See [results/transport-tcp-vs-udp.md](results/transport-tcp-vs-udp.md) |
| **Sync A/B** ✅ | op-based delta vs state-based single-datagram snapshots | **DONE 2026-06-12:** state-based wins app goodput +21–29 % under jamming (no orphaning, no fragmentation); udp+state is +37 % over the tcp+delta baseline. See [results/sync-delta-vs-state.md](results/sync-delta-vs-state.md) |
| **FEC axis** ✅ | FEC + interleaving over FHSS dwells (`radio.fec_fraction`) | **DONE 2026-06-13:** flips the M3 slow-vs-fast-hop result — with FEC, fast hopping survives partial-band jamming (1000 hop/s frame PDR 0.37→0.92). See [results/fec-flips-hop-rate.md](results/fec-flips-hop-rate.md) |
| **M5+** | Reactive jamming, routing comparisons, frequency-band sweeps, Sionna RT pathloss backend | per-feature |

Testing discipline throughout: unit tests per module; the model validation suite
(`validation/`) reproduces published propagation curves and closed-form FHSS/SINR
results, rendered as a report — it is the artifact that makes benchmark results
defensible. Integration smoke test runs in CI (requires CAP_NET_ADMIN; user-namespace
or privileged runner).
