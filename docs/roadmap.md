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
| **R1 — ROS 2 env** ✅ | ROS 2 Jazzy via RoboStack as opt-in pixi environment ([ADR-0006](adr/0006-ros2-isaac-integration.md), [plan](ros2-integration-plan.md)) | **DONE 2026-06-13:** pub/sub round-trip green on Fast DDS + rmw_zenoh in `pixi run -e ros2 ros2-smoke`; default env unaffected |
| **R2 — RViz bridge** ✅ | Orchestrator publishes `/clock`, `/tf`, link/jammer markers; optional `--ros2-viz` | **DONE 2026-06-13:** live RViz view of `resilience_4node` on published sim time (links flip green→red at jammer start); root-free `viz-demo` task drives it without netns |
| **R3 — ROS 2 in the loop** ✅ | ROS 2 nodes per netns; agent vs raw rmw_zenoh as A/B axis | **DONE 2026-06-13:** agent (udp+state, MLS on) beats stock ROS 2 (rmw_zenoh, default QoS) on update delivery (+20 % @45 dBm) and AoI (−22 %) at every power; ROS 2's higher frame PDR is TCP backing off — retransmission converts loss into staleness. See [results/ros2-vs-agent.md](results/ros2-vs-agent.md) |
| **R4 — Isaac Sim 6 mobility** | `IsaacMobilityProvider`, lockstep stepping, orchestrator stays clock master | existing scenario end-to-end on Isaac mobility; comparison vs bicycle kinematics |
| **M5+** | Reactive jamming, routing comparisons, frequency-band sweeps, FEC/interleaving axis, Sionna RT pathloss backend | per-feature |

Testing discipline throughout: unit tests per module; the model validation suite
(`validation/`) reproduces published propagation curves and closed-form FHSS/SINR
results, rendered as a report — it is the artifact that makes benchmark results
defensible. Integration smoke test runs in CI (requires CAP_NET_ADMIN; user-namespace
or privileged runner).
