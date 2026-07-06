
Candidates for the next item:
1. Per-tick link-state logging — small engine addition that makes the new UI's map show the real jammer
footprint/SINR (finishes the UI agent's request).
2. Reactive jamming — a smarter adversary that senses transmissions and jams the active channel; tests the
FHSS/FEC defenses against a harder threat.
3. MLS handshake hardening — the EMANE-spike robustness finding; small, improves every run.


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
| **R1 — ROS 2 env** ✅ | ROS 2 Jazzy via RoboStack as opt-in pixi environment ([ADR-0006](adr/0006-ros2-isaac-integration.md), [plan](ros2-integration-plan.md)) | **DONE 2026-06-13:** pub/sub round-trip green on Fast DDS + rmw_zenoh in `pixi run -e ros2 ros2-smoke`; default env unaffected |
| **R2 — RViz bridge** ✅ | Orchestrator publishes `/clock`, `/tf`, link/jammer markers; optional `--ros2-viz` | **DONE 2026-06-13:** live RViz view of `resilience_4node` on published sim time (links flip green→red at jammer start); root-free `viz-demo` task drives it without netns |
| **R3 — ROS 2 in the loop** ✅ | ROS 2 nodes per netns; agent vs raw rmw_zenoh as A/B axis | **DONE 2026-06-13:** agent (udp+state, MLS on) beats stock ROS 2 (rmw_zenoh, default QoS) on update delivery (+20 % @45 dBm) and AoI (−22 %) at every power; ROS 2's higher frame PDR is TCP backing off — retransmission converts loss into staleness. See [results/ros2-vs-agent.md](results/ros2-vs-agent.md) |
| **DDS / RMW scaling** ✅ | Fast DDS vs Cyclone vs Zenoh at swarm scale over a new kernel-`bridge` substrate, no EW vector ([plan](dds-benchmark-plan.md)) | **DONE 2026-06-27:** healthy through N=24, sharp knee at N=48, no usable mesh at N=96 — and host-pressure counters prove it's middleware discovery collapse, not rig saturation (29 GB free at N=96). Zenoh degrades most gracefully, Cyclone hardest. See [results/dds-rmw-scaling.md](results/dds-rmw-scaling.md) |
| **DDS / RMW QoS plane** ✅ | Full QoS contract × transport buffer × startup timing sweep to find what reaches high participant counts; per-RMW characterization ([tuning](results/dds-rmw-tuning.md)) | **DONE 2026-06-28:** three different walls — Fast DDS hits a deterministic ~33-participant clique cap (CV 0 %, invariant to QoS/buffer/time/N across 48→192); Cyclone self-poisons (only **staggered joins** help, 2.5×); Zenoh has the highest mean reach but ±35 % run-to-run variance (router-mesh races, QoS effects are noise). Buffer/Discovery-Server/QoS-stacking all null. See [results/dds-rmw-qos-plane.md](results/dds-rmw-qos-plane.md) |
| **DDS root cause + topology track** | Archetype plan ([dds-topology-plan.md](dds-topology-plan.md)): allocation probe, lan substrate, netem, shared/star/cluster topologies — and the rig root cause | **2026-07-03:** kernel `neigh.gc_thresh3=1024` (global across netns) was every prior "wall" — the ~33-clique cap, Cyclone's storm, the N=48 knee. Rig fixed in-engine; corrected baselines: all RMWs mesh 1.0 @48; Cyclone mesh 1.0 + 96 % delivery @96 (the real winner); Fast DDS data plane saturates @96; Zenoh full router mesh collapses (real wall — use star). See [results/dds-neighbor-table.md](results/dds-neighbor-table.md). Cross-host P3/P4 pending firewall openings |
| **R4 — Isaac Sim 6 mobility** | `IsaacMobilityProvider`, lockstep stepping, orchestrator stays clock master | existing scenario end-to-end on Isaac mobility; comparison vs bicycle kinematics |
| **Reactive jamming** ✅ | Follower jammer that senses the active transmission and retunes onto its channel ([backlog](ew-track-backlog.md)) | **DONE 2026-07-06:** modelled as a per-dwell lock probability `ρ = detection_prob·max(0,(T_dwell−τ)/T_dwell)` folded into the binomial-CDF survival model — no sub-tick stepping. Defence is a **cliff at `T_dwell = τ`**, not the gradual FEC climb: engine sweep shows hop 500/s crippled by a τ=1 ms follower (postjam PDR 0.23) but immune to τ=5 ms (1.00), hop 1000/s immune to both; FEC nearly irrelevant. See [results/reactive-jamming.md](results/reactive-jamming.md) |
| **M5+** | Routing comparisons, frequency-band sweeps, Sionna RT pathloss backend | per-feature |

Testing discipline throughout: unit tests per module; the model validation suite
(`validation/`) reproduces published propagation curves and closed-form FHSS/SINR
results, rendered as a report — it is the artifact that makes benchmark results
defensible. Integration smoke test runs in CI (requires CAP_NET_ADMIN; user-namespace
or privileged runner).

