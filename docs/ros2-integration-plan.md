# ROS 2 Jazzy + Isaac Sim 6 integration plan

Decisions in [ADR-0006](adr/0006-ros2-isaac-integration.md): orchestrator stays clock
master; RoboStack-via-pixi; Isaac replaces mobility only; agent-mediated vs raw
rmw_zenoh kept as an A/B axis. Phases are ordered so each lands value alone.

## R1 — Environment bootstrap

ROS 2 Jazzy as an opt-in pixi environment; default env untouched.

- `pixi.toml`: `[feature.ros2]` with channel `robostack-jazzy`, deps
  `ros-jazzy-ros-base`, `ros-jazzy-rmw-zenoh-cpp`, `ros-jazzy-rviz2`; environment
  `ros2` = default + ros2 features. Python resolves to 3.12 (RoboStack pin) — within
  the existing `>=3.11,<3.13` constraint, but verify `eclipse-zenoh` and the editable
  install resolve under 3.12.
- No colcon/ament packages yet: phase R2 publishes via `rclpy` with standard message
  types only. Custom messages (and the `setuptools<=58.2` colcon pin) deferred until
  actually needed.
- Tasks: `pixi run -e ros2 ros2 ...`; smoke-test task (talker/listener).

**Exit:** `ros2 topic echo` round-trip works in the pixi env on both Fast DDS and
rmw_zenoh; default-env tests still green.

## R2 — Observability bridge (RViz)

`sim/netcom_zen/ros2_bridge/` — optional module, imported only when the run is
started with `--ros2-viz` (orchestrator runs inside the `ros2` env then).

- Publishes from the scenario clock: `/clock` (sim time), `/tf` (vehicle poses over
  the heightmap), `MarkerArray` for links (color = PER/SINR, from the link-state
  table), jammer entities and their active emissions, drop-cause event markers.
- RViz config checked into `scenarios/` or `viz/`; `pixi run -e ros2 rviz` task.
- Complements (does not replace) the viz-dashboard branch; both read the same
  link-state/metrics surfaces.

**Exit:** live RViz view of a jamming scenario — vehicles moving, links degrading,
jammer markers — driven entirely by published sim time.

## R3 — ROS 2 nodes in the loop (comms A/B)

ROS 2 processes inside per-vehicle netns'es, telemetry workload crossing the
emulated channel both ways of the A/B:

- **Wiring A (agent-mediated):** the Rust agent's own telemetry workload (CRDT +
  MLS, udp+state champion config) at matched period/payload. *Scoping decision:*
  the original idea of ROS 2 nodes feeding the agent through a local API adds a
  loopback hop that doesn't cross the channel and can't move the comparison; the
  local API is deferred until a real autonomy workload exists (R4+/Isaac era).
- **Wiring B (raw rmw_zenoh):** `RMW_IMPLEMENTATION=rmw_zenoh_cpp` nodes per netns
  (`ros2_workload` package); one zenoh router per netns, explicit TCP mesh over
  the TUN addresses (multicast scouting off — nothing bypasses the channel),
  sessions reach their router over loopback. Spiked successfully on localhost
  before integration.
- Harness: the workload node emits the agent's JSONL metrics schema
  (`pub`/`recv`/`final_state` into `agent_<id>.jsonl`), so AoI/report/sweep
  consume ROS 2 runs unchanged; `workload: agent | ros2` is a sweep axis.
- Benchmark: `scenarios/sweep_ros2_vs_agent.yaml` — workload × jammer power,
  paired seeds.

**Exit:** resilience curves (PDR / latency / AoI vs jammer power) for both wirings
from one sweep definition; drop attribution still per-packet.

## R4 — Isaac Sim 6 as MobilityProvider

- Separate pixi environment `isaac` (pypi `isaacsim[all,extscache,ros2]==6.0.0.1`,
  CUDA torch first, `extra-index-url pypi.nvidia.com`); tens of GB, never in default.
- `IsaacMobilityProvider`: Isaac runs headless as a separate process; per scenario
  tick the orchestrator steps it (ROS 2 Simulation Control `/step_simulation` with
  N physics frames per 100 ms tick) and reads poses (`/tf` or entity-state service)
  → `Pose`. Fallback if service round-trip dominates the tick: embed
  `SimulationApp` in a dedicated stepper process with a leaner IPC.
- Scene: flat ground + simple vehicle assets first (poses are all the channel needs);
  terrain mesh from the existing DEM later so trajectories respect the same heights
  the propagation model uses.
- Determinism note recorded per run manifest: trajectories Isaac-sourced, not
  seed-exact; channel RNG still seeded/replayable.

**Exit:** an existing scenario runs end-to-end with Isaac mobility — same metrics
pipeline, comparison run vs bicycle-kinematics mobility showing equivalent link-state
dynamics at matched trajectories.

*Implementation note (as built):* the "leaner IPC" fallback was chosen up front
instead of ROS 2 Simulation Control services. Reasons: the orchestrator runs as
root in any env, and a service client would force rclpy + `simulation_interfaces`
into it (coupling it to the ros2/isaac envs); Sim Control stepping means an
action/service round-trip per 100 ms tick; and a socket protocol is trivially
fakeable for CI. Shape: `netcom_zen.isaac_stepper` (isaac env, user-owned, one
process reused across sweep runs) embeds headless `SimulationApp` and serves
newline-JSON over a unix socket; `IsaacMobilityProvider` (stdlib-only) drives it
in lockstep — `mobility.physics_hz / tick_hz` PhysX frames per tick. Vehicles
are dynamic cuboids velocity-controlled by the same unicycle law as
`WaypointVehicle`, with the controller fed PhysX ground-truth positions, so
trajectories track waypoint kinematics closely but are Isaac-integrated
(manifest: `trajectories_seed_exact: false`). `--backend kinematic` serves the
same wire protocol without Isaac for tests/dry-runs. Step latency lands in the
existing timing monitor (`max_tick_lag_s`, `timing_ok`); measured on the RTX
5090 host (4 vehicles, 6 frames/step): median 5.8 ms, p95 6.4 ms per 100 ms
tick — the risk-list concern is settled. Two timing fixes were needed to make
that monitor trustworthy: (a) the tick loop now sleeps to the absolute tick
deadline instead of a flat `dt`, so per-tick work no longer accumulates as
drift (a 30 s run previously reported `late_fraction ~0.79` purely from
scheduling slack); (b) warp JIT-compiles on Isaac's first physics step (~3.5 s),
so `IsaacBackend` warms the kernels during scene build (before t=0) and restores
spawn state, keeping the stall out of the live loop. Cold boot is still 11-40 s,
which is why the stepper is a long-lived user-owned process the orchestrator
connects to, not a per-run child. The isaac pixi env is
standalone (no default feature): isaacsim's exact pypi pins conflict with
conda-solved defaults; the stepper needs only stdlib + `netcom_zen` + isaacsim.

## Risks / open questions

- **rmw_zenoh router topology in netns'es (R3-B):** how routers discover/peer over
  the emulated channel without leaking via the host. Mitigation: early spike, mgmt
  plane stays unix-socket only.
- **Agent local API (R3-A):** smallest useful surface — likely a localhost zenoh
  listener with a fixed key-space mapping. Needs its own mini-design.
- **Isaac step latency (R4):** service-driven lockstep at 10 Hz with rendering off
  should hold; measure before building on it. Timing-integrity monitor already
  flags overruns.
- **Python 3.12 ripple:** if anything in the default env can't do 3.12, the ros2
  env forks the version pin (pixi handles per-env pins; solve-groups would not).
