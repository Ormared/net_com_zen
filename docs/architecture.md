# Architecture

net_com_zen couples a network/RF emulation engine with minimal vehicle mobility to
benchmark secure, decentralized swarm communication (Zenoh / CRDT / MLS, FHSS, routing,
satellite fallback) under electronic warfare. Real protocol binaries run over an
emulated radio channel whose behavior is computed from vehicle positions, terrain,
and jammer activity.

## Two planes

| Plane | Carries | Mechanism |
|---|---|---|
| **Dataplane** (under test) | All Zenoh/CRDT/MLS traffic between vehicles | One Linux netns per vehicle, one TUN interface each; every frame passes through the channel engine |
| **Management plane** (invisible to the experiment) | Agent metrics, scenario commands | Unix socket per netns |

The separation guarantees discovery and measurement traffic can never bypass the
emulated radio.

## System diagram

```
                      ┌────────────── scenario engine (Python, one clock) ──────────────┐
                      │                                                                  │
 scenario.yaml ──►  mobility ──► positions ──► propagation ──► pathloss ──┐              │
                    (kinematics      │         (terrain DEM +             ▼              │
                     + waypoints)    │          foliage raster)    link-state table      │
                      │              │                             (SINR, PER, rate,     │
                    EW module ───────┴──► jammer emissions ──────►  delay per link)      │
                    (barrage/spot/                                        │              │
                     sweep/reactive)                                      ▼              │
                      │                                          channel engine          │
                      │                                          (per-packet verdict)    │
                      └──────────────────────────────────────────────┬───────────────────┘
                                                                     │
        ┌──────────┐     TUN      ┌────────────────┐      TUN     ┌──┴───────┐
        │ netns v1 │◄────────────►│ channel engine │◄────────────►│ netns v2 │ ... vN
        │ Rust agent│              │  forwarder     │              │ Rust agent│
        └────┬─────┘              └────────────────┘              └────┬─────┘
             └────────── mgmt socket ──► metrics collector ◄───────────┘
```

## Components

### Scenario engine (Python, `sim/`)

Single process owning the single clock; ticks at 10 Hz. There is deliberately no
physics co-simulator and therefore no clock-synchronization problem (see
[ADR-0002](adr/0002-no-gazebo-single-clock.md)). Loads a schema-validated scenario
YAML, supervises all child processes, and writes the run manifest.

### Mobility engine

Bicycle-kinematics waypoint following over a heightmap; emits pose/velocity per tick.
Behind a `MobilityProvider` interface so a full simulator (Gazebo/ROS 2) can replace
it later without touching anything else.

### Environment & propagation

A `PathlossProvider` interface — `(tx_pos, rx_pos, freq_hz) → dB` — deliberately
shaped like EMANE's external pathloss events ([ADR-0001](adr/0001-channel-emulation-hand-rolled-vs-emane.md)).
Composed from baseline + terrain + foliage models (see [models.md](models.md)).
Environment profiles (open-field / forest / mixed) are scenario parameters, making
penetration-vs-frequency a sweepable benchmark axis. Sionna RT or recorded real-world
traces can implement the same interface later ([ADR-0004](adr/0004-empirical-propagation-first.md)).

### EW module

Jammer entities with position, power, antenna, and behavior: barrage, spot, sweep,
reactive (sees transmissions through the engine, with configurable detection latency
and probability). Scriptable timeline, e.g. `t=120s: jammer_1 on, 30 dBm`.

### FHSS link model

Per-packet effective SINR from pathloss + jammer band overlap, then PER from SINR,
packet length, hop rate, and dwell time via a statistical hop-collision model
(see [models.md](models.md)). Hop rate is a primary sweep axis.

### Channel engine (dataplane forwarder)

Async Python (uvloop) userspace forwarder. Per frame: look up link state → roll a
**seeded** RNG against PER → drop with a cause label (`range | foliage | jam |
hop_collision | queue`) or deliver after serialization + queueing + propagation delay.
Every verdict is logged: per-packet ground truth with drop attribution.

The seeded RNG stream is the replay mechanism: same seed → same channel realization,
so different middleware configs can be compared pairwise with channel variance removed.

The satellite uplink is just another channel model in the same engine (fixed
delay/bandwidth + scripted outage), keeping drop attribution uniform.

### Node agent (Rust, `agent/`)

The software under test, one instance per netns, using the exact libraries targeted
for deployment ([ADR-0003](adr/0003-rust-agent-python-sim.md)):

- **zenoh** (peer mode) — transport and pub/sub over the TUN interface only
- **automerge** — CRDT document for shared swarm state (telemetry, coordinates, commands)
- **openmls** — group encryption of payloads; commit ordering via a deterministic
  committer rule (lowest live member ID, any member can take over) — no fixed
  coordinator, satisfying the no-single-point-of-failure constraint
- **link manager** — mesh vs satellite path selection (seam designed in M1–M3,
  implemented in M4)

Emits structured metrics (publish latency, per-peer state age-of-information, MLS
rekey events) on the management socket.

### Metrics & benchmark harness

Collector merges per-packet channel logs with agent events into parquet plus a run
manifest (full config, seeds, git hash). The sweep runner executes cartesian products
(hop rate × jammer power × seed) and renders resilience curves. See
[benchmarking.md](benchmarking.md).

## Data flow

**Per tick (10 Hz):** mobility → positions → propagation (+ jammer state) → atomically
swapped link-state table (SINR, PER, rate, delay per directed link).

**Per packet:** agent writes to TUN → forwarder reads frame → current link state →
verdict (drop with cause, or schedule delivery) → inject into destination TUN.

## Run integrity & error handling

- **Preflight:** `pixi run setup-check` verifies `CAP_NET_ADMIN`, kernel features,
  toolchains. Netns names are run-scoped; stale namespaces are swept on start
  (idempotent setup/teardown).
- **Supervision:** agent crash → logged, run continues (a dead vehicle is a valid
  scenario event) but the manifest marks it unplanned. Forwarder crash → run aborts,
  outputs flagged invalid.
- **Timing integrity:** the forwarder self-monitors queue latency; if processing delay
  becomes non-negligible vs. modeled link delays, the run is flagged rather than
  silently skewing latency results.
- **Config:** scenario YAML is pydantic-validated at load; fail fast, never mid-run.

## Repository layout

```
net_com_zen/
├── pixi.toml              # python env + tasks; rust toolchain via pixi
├── sim/                   # python package `netcom_zen`
│   ├── mobility/  propagation/  ew/  channel/  metrics/  harness/
│   └── orchestrator.py    # scenario engine: clock, supervision, lifecycle
├── agent/                 # rust workspace: node agent (zenoh, automerge, openmls)
├── scenarios/             # yaml scenario + sweep definitions
├── validation/            # model validation suite + report generation
├── docs/                  # this documentation
└── papers/                # reference literature
```
