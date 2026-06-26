# DDS / RMW Scaling Benchmark — Plan

**Status:** scoped 2026-06-27, not started. Fresh-context start point.
**Branch:** `dds-rmw-benchmark` (off `main`).
**Track owner doc.** Deferred/parallel items live in [ew-track-backlog.md](ew-track-backlog.md).

## Goal

Benchmark and stress-test three ROS 2 DDS/RMW implementations at swarm scale.
This is a **middleware performance** study, **not** a jamming/resilience study —
there is **no EW vector** here (no jammer, no RF degradation).

- **RMW axis (3-way):** `rmw_fastrtps_cpp` (Fast DDS), `rmw_cyclonedds_cpp`
  (Cyclone DDS), `rmw_zenoh_cpp` (Zenoh — already wired). Both new RMWs are
  installable in the `ros2` pixi env (robostack-jazzy: `ros-jazzy-rmw-fastrtps-cpp`,
  `ros-jazzy-rmw-cyclonedds-cpp`).
- **Scale sweep:** N = 4 → 8 → 16 → 24 -> 48 -> 96 drones. The point is the scaling *curve*
  — where each RMW degrades.

## Non-goals (explicit)

- No jammers, no pathloss, no FHSS, no FEC. The channel engine
  (`channel/forwarder.py`, `ew.py`, `propagation.py`, `terrain.py`,
  `channel/linkstate.py`) is **not on this path**.
- No Isaac Sim is optional, if we are inconfident in the simulation, we can try isaac sim and record rosbag from it.
- Not testing RMW behaviour under loss — that's the EW track (backlog).

## What we measure

Per swarm size, per RMW:
- **Discovery time** — wall time from node start until the full N-node mesh is
  formed (every node has received ≥1 message from every peer). The headline
  scaling differentiator. *Mostly derivable from existing logs* (see below).
- **Latency** p50 / p99 per delivered message (`recv_us − peer_ts_us` from the
  workload JSONL).
- **Throughput / delivery ratio** — delivered vs published, and as published
  rate is cranked.
- **CPU + memory per node** — sampled from process handles during the run (new).
- **Discovery / background traffic** — optional, via iface counters per netns.

## Architecture: the `bridge` substrate

The existing dataplane is netns + veth + a single-thread asyncio AF_PACKET
forwarder (`channel/forwarder.py`) that applies per-packet pathloss/drops. That
forwarder is O(N²) broadcast fan-out in one Python thread — it cannot model 20+
nodes faithfully, and **for a pure-transport benchmark we don't want per-packet
RF modelling at all**.

**Decision:** add a second substrate instead of rewriting the forwarder.

- `substrate: channel` (existing) — AF_PACKET forwarder + RF model. **Retained
  for the EW track.**
- `substrate: bridge` (new) — netns per drone + veth into a **Linux bridge**;
  the **kernel** does L2 forwarding. Scales for free; it is also the fair,
  standard baseline for DDS middleware benchmarks (a clean switched segment).
  A `tc netem` hook is stubbed (no-op) for optional controlled loss/latency later
  — not used in this study.

### Why the bridge is the right call here
- Kernel forwarding has no GIL/asyncio ceiling → 24 nodes at full rate is trivial.
- A real kernel network stack is *more* faithful than a userspace Python relay.
- **Multicast works on a bridge** → each RMW uses its **native discovery**
  (Fast DDS SPDP, Cyclone, Zenoh scouting). That's exactly the behaviour we want
  to benchmark. (Contrast: the `channel` path forces multicast *off* and
  hand-wires peers because the forwarder can't carry multicast.)

### Transport isolation (the one rule)
Force traffic over the veth so it is real, measurable, and per-node isolated —
**no same-host shared-memory shortcut**. Multicast discovery stays **ON**.

| RMW | Daemon? | SHM disable | Discovery on bridge |
|---|---|---|---|
| `rmw_fastrtps_cpp` | none | `FASTDDS_BUILTIN_TRANSPORTS=UDPv4` (or profiles XML w/ no SHM descriptor) | native SPDP multicast |
| `rmw_cyclonedds_cpp` | none | `CYCLONEDDS_URI` XML: Iceoryx/SHM off, multicast on bridge iface | native multicast |
| `rmw_zenoh_cpp` | `rmw_zenohd` router (architecture is router-based) | dial over veth, not loopback/SHM | router topology — see open question Q1 |

## Config schema changes (`sim/netcom_zen/config.py`)

1. **New field on `Scenario`:** `substrate: Literal["channel", "bridge"] = "channel"`.
2. **New field on `Ros2WorkloadConfig`:**
   `rmw: Literal["zenoh", "fastrtps", "cyclonedds"] = "zenoh"` (maps to the
   `rmw_*_cpp` impl name + spawn strategy).
3. **Relax the node cap.** `Scenario.nodes` is currently
   `Field(min_length=2, max_length=8)` (line 137) — raise to ≥24 (or drop the
   cap for `substrate=bridge`; keep a sane cap like 64).
4. Workload params already exist on `Ros2WorkloadConfig` (`period_ms`,
   `payload_bytes`, `reliability`, `port`) — sweepable as-is.

## Engine changes (`sim/netcom_zen/orchestrator.py`)

`ScenarioEngine.run()` currently always builds mobility→pathloss→link-table→
forwarder. Branch on substrate:

- `substrate == "bridge"`: **skip** `build_world` RF parts, `build_table`,
  `ChannelForwarder`, jammers, and (optionally) per-tick mobility/positions.
  Set up `NetnsTopology` + bridge, spawn workload, then run a lightweight loop
  for `duration_s` that **samples CPU/mem per PID** and lets the run finish.
  No `packets.parquet` (no forwarder); metrics come from workload JSONL + the
  resource sampler.
- `NetnsTopology` (`netns.py`) needs a **bridge mode**: create one bridge, attach
  every host-side veth to it (`ip link set <host> master <br>`), bring the bridge
  up, enable multicast (`ip link set <br> type bridge mcast_snooping 0` or ensure
  flooding). Keep the existing offload-disable (`tx/gso/tso/gro off`) — still
  needed so DDS UDP checksums are valid across veth.
- **`_spawn_ros2` becomes per-RMW.** Today it hardcodes `rmw_zenoh_cpp`, the
  `rmw_zenohd` router-per-netns, and `ZENOH_CONFIG_OVERRIDE` (lines 105–158).
  Split into a strategy keyed by `cfg.rmw`:
  - `zenoh`: existing router + override path (decide topology, Q1).
  - `fastrtps`: **no router**; env `RMW_IMPLEMENTATION=rmw_fastrtps_cpp`,
    `FASTDDS_BUILTIN_TRANSPORTS=UDPv4`; spawn the workload node directly in each
    netns.
  - `cyclonedds`: **no router**; env `RMW_IMPLEMENTATION=rmw_cyclonedds_cpp`,
    `CYCLONEDDS_URI=file://<generated.xml>` (SHM off, multicast on bridge iface).
  - Keep the `base_env` minimal-env pattern (sudo strips activation; children
    need `AMENT_PREFIX_PATH`, `ROS_LOG_DIR`, `RMW_IMPLEMENTATION`,
    `PYTHONNOUSERSITE`, `PATH` — verified in R3).

## Workload (HYBRID)

- **Backbone — synthetic parametric.** Reuse `ros2_workload/node.py` as-is: it
  already publishes `/swarm/<id>/telemetry` on a timer and logs `pub`/`recv`
  events with `ts_us` (latency + first-recv-per-peer derivable). Sweep
  `period_ms` / `payload_bytes` / N freely.
- **Realism anchor — one rosbag.** Record **one** representative telemetry stream
  once (`ros2 bag record`), then replay (`ros2 bag play`) through each RMW as a
  cross-check that the synthetic pattern is representative. Not the sweep
  backbone — a single confirmatory run.

## Metrics (`sim/netcom_zen/harness/`)

- **Discovery time:** extend `harness/report.py`. Each node logs a `start` event
  (`ts_us`) and `recv` events tagged with `from`. Discovery time for a node =
  `max over peers (first recv ts_us from peer) − start ts_us`; swarm discovery =
  max over nodes. **No workload change needed** — derive from existing JSONL.
- **Latency percentiles / delivery ratio:** extend the existing report (it already
  greps `"type":"pub"` / `recv`).
- **CPU/mem sampler:** new; sample `psutil` per workload PID (and router PID for
  zenoh) on an interval during `run()`, write `resources.parquet`.
- **Sweep driver:** `harness/sweep.py` exists — add an RMW × N grid; emit a
  results table + scaling-curve plots to `docs/results/dds-rmw-scaling.md`.

## Execution shape (answers "what's parallel")

```
Phase A  (SOLO)     bridge substrate + config schema + engine branch
                    └─ freezes the substrate API + config shape
Phase B  (≤3 PARALLEL, disjoint once A's schema is frozen)
   ├─ B1  per-RMW spawn strategies + SHM-off env/XML (orchestrator)
   ├─ B2  metrics: discovery-time + percentiles in report.py + resource sampler
   └─ B3  parametric N=4/8/16/24 scenarios + rosbag record path
Phase C  (SERIAL)   run RMW × N sweep + rosbag cross-check + write-up
                    └─ benchmark RUNS are always serial on one host (timing)
```

**1 → 3 → 1.** Sequential where there's a true dependency (foundation; and the
runs); parallel only in the middle build-out.

## Open questions / risks

- **Q1 — Zenoh topology fairness.** Fast DDS/Cyclone are routerless (multicast
  SPDP); Zenoh is router-based. Options: (a) one `rmw_zenohd` per netns with
  multicast scouting ON (closest to peer-native), (b) keep the explicit router
  mesh. Pick one and **document the asymmetry** in the results — it's a real
  architectural difference, not a bug.
- **Q2 — `mcast_snooping`.** Ensure the bridge floods multicast to all ports
  (snooping off, or an mrouter) so SPDP discovery reaches every node.
- **Q3 — Node cap.** Confirm `10.99.0.{i+1}` /24 addressing (netns.py) is fine to
  24 (it is) and raise the pydantic `max_length`.
- **Q4 — Fairness knobs.** Same QoS (`reliability`), same `period_ms`/
  `payload_bytes`, same depth across RMWs; record exact versions in the manifest.
- **Q5 — Sudo.** netns + bridge setup needs root; the path-globbed NOPASSWD entry
  already covers `.pixi/envs/ros2/bin/python`. Verify with
  `sudo -n .pixi/envs/ros2/bin/python -c 'pass'`.

## Definition of done

Scaling curves (discovery time, p50/p99 latency, delivery ratio, CPU/mem) for
Fast DDS vs Cyclone vs Zenoh across N=4/8/16/24, plus a one-shot rosbag
cross-check, written up in `docs/results/dds-rmw-scaling.md` and linked from
`docs/roadmap.md`.
