# ADR-0006: ROS 2 Jazzy integration; Isaac Sim 6 behind the MobilityProvider seam

**Status:** accepted
**Date:** 2026-06-12

## Context

ADR-0002 rejected physics co-simulation to keep one clock, but deliberately left two
seams: `MobilityProvider` (`step(dt) -> Pose`) and `PathlossProvider`. The next stage
of the project is autonomy-in-the-loop: ROS 2 nodes as the application layer on each
vehicle, and eventually a full world simulator. Verified facts driving this decision
(2026-06):

- Host is Ubuntu 24.04 (Jazzy's tier-1 platform) with an RTX 5090 Laptop / 24 GB
  VRAM / driver 580.159 — clears Isaac Sim 6 minimums (RTX 4080 / 16 GB / 580.95).
- Isaac Sim 6.0 went GA 2026-06-04: pip-installable (Python 3.12), ships internal
  ROS 2 Jazzy libraries, and exposes lockstep stepping to an external process via
  ROS 2 Simulation Control services (`/step_simulation`, `/simulate_steps`).
- ROS 2 Jazzy supports `rmw_zenoh_cpp`; RoboStack publishes Jazzy (incl. rmw_zenoh)
  on the `robostack-jazzy` conda channel, installable through pixi (Python pinned
  to 3.12).

rmw_zenoh matters specifically here: ROS 2 topic traffic over zenoh is directly
comparable to the agent's zenoh sessions, extending the transport/sync A/B series
to "stock ROS 2 middleware vs purpose-built agent" under jamming.

## Decision

1. **The orchestrator stays the single clock master.** ROS 2 sim time (`/clock`) is
   *published by us* from the scenario clock; ROS 2 nodes run `use_sim_time`. Isaac
   Sim is driven in lockstep — the orchestrator steps it per tick (Simulation Control
   services, or an embedded `SimulationApp` loop if service latency proves hostile)
   and reads poses back. ADR-0002's no-second-clock principle survives; what degrades
   is trajectory bit-reproducibility (PhysX is not guaranteed deterministic), while
   the seeded channel RNG and replay mechanism remain intact.
2. **ROS 2 via RoboStack in pixi**, not apt: `robostack-jazzy` channel as a pixi
   feature/environment. Reproducible, CI-friendly, no machine-global state. Cost:
   the ROS-enabled environment pins Python 3.12.
3. **Isaac Sim replaces mobility only** (a `MobilityProvider` implementation).
   Propagation, EW, channel engine, netns dataplane, metrics are untouched — Isaac
   provides poses (later: sensors for autonomy), never comms.
4. **Two comms wirings, kept as an A/B axis** (like transport/sync A/Bs):
   - **agent-mediated** — ROS 2 nodes talk only to the local Rust agent in their
     netns; the agent (zenoh + automerge + MLS) remains the sole channel crosser.
   - **raw rmw_zenoh** — ROS 2 nodes peer across the emulated channel directly via
     `rmw_zenoh_cpp` (no MLS/CRDT layer).
   Same workload both ways; "what does the purpose-built agent buy over stock
   ROS 2 middleware under EW" becomes a headline benchmark.

## Consequences

- (+) Autonomy-in-the-loop without surrendering the clock; replay semantics
  explicitly re-scoped rather than silently broken.
- (+) The A/B wiring turns integration plumbing into publishable comparison data.
- (−) A second Python pin (3.12) and a heavyweight optional environment (Isaac is
  tens of GB); kept out of the default env so the existing workflow is unaffected.
- (−) Trajectories from Isaac are not seed-exact across runs; paired-seed channel
  comparisons must either record/replay trajectories or accept trajectory variance.
- (−) rmw_zenoh's bundled zenoh version may diverge from the agent's `zenoh` crate;
  they never interoperate on the wire (separate sessions), so this is a version-
  hygiene concern only.
