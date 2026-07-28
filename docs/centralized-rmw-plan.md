# Centralized ROS 2 RMW benchmark

**Status:** resumable evaluation infrastructure implemented 2026-07-28; full
matrix awaiting runs.
**Results:** [results/dds-centralized.md](results/dds-centralized.md)

## Question

This track treats centralized operation as the intended architecture.  It is
not a swarm-resilience experiment and has no jammer, RF model, peer mesh, CRDT,
or decentralized-key requirement.

The application contract has three independent traffic classes:

- clients publish telemetry to one or two application servers;
- application servers publish commands to clients;
- clients call a ROS 2 service and record end-to-end request RTT.

Fast DDS, Cyclone DDS, and Zenoh RMW run the identical application workload.
Discovery Servers and Zenoh routers are middleware infrastructure, not
application servers, and their resources are reported separately.

## Server modes

| Mode | Semantics |
|---|---|
| `single` | One application server and one centralized middleware rendezvous path |
| `active_active` | Two serving nodes; clients receive from/call both, then the primary and its colocated infrastructure are terminated |
| `active_passive` | Warm standby receives telemetry but creates command/service endpoints only when the primary is terminated |
| `sharded` | Clients are assigned round-robin to one of two server-specific namespaces |

HA is deliberately at-least-once.  Clients measure and suppress duplicate
logical commands; the lab does not claim consensus, durable state replication,
or exactly-once execution.

## Middleware shapes

- **Fast DDS:** native multicast or one Discovery Server per application
  server.  Redundant endpoints are supplied through `ROS_DISCOVERY_SERVER`.
- **Cyclone DDS:** native multicast or multicast-disabled static peers pointing
  at the application-server addresses.
- **Zenoh:** one router per application server.  Clients connect directly to
  those router endpoints; the decentralized per-client router mesh is not used.

On `substrate=lan`, the same shapes are created across the main rig and
home-mini.  Cross-host one-way delay is never reported because their clocks are
not synchronized sufficiently; command gaps and RPC RTT use one client clock.

## Generate and run

Generate the capacity, rate, payload, HA, and sharding YAML:

```bash
.pixi/envs/default/bin/python -m netcom_zen.harness.centralized_lab generate
```

Run the complete serial workflow (or substitute `tune`, `bridge-core`,
`bridge-ha`, `lan`, or `report` for a resumable phase):

```bash
sudo .pixi/envs/ros2/bin/python -m netcom_zen.harness.centralized_lab all
```

The tuning phase writes `results/centralized/locked_profiles.json` and
regenerates every later sweep using the selected RMW-specific settings. The
report phase writes combined JSON and CSV:

```bash
.pixi/envs/default/bin/python -m netcom_zen.harness.centralized_lab report
```

Every phase is serial and resumable. A new evaluation requires 10 GiB disk
and 12 GiB available memory. Resumed phases and new cells require 8 GiB
available memory, while a running centralized cell aborts after 10 sustained
seconds below 6 GiB. This preserves a 2 GiB continuation margin without
discarding safe progress because host availability fluctuated slightly below
the conservative fresh-start threshold. A phase stops below 3 GiB free disk.
Planned, malformed, aborted, and missing cells remain explicit rows in the
combined summary.

The generated matrix contains:

- N=32/96 calibration sweeps over native/centralized discovery,
  reliable/best-effort telemetry, and 0/16/64 MiB DDS socket buffers; these
  select a documented Pareto profile before the main cells are interpreted;
- single-server N = 4, 16, 32, 64, 96, 128, three reps;
- at N=64, paired telemetry/RPC rate sweeps and 255 B/4 KiB/64 KiB payloads;
- active-active, active-passive, and sharded N = 32/96;
- five reps for failure cells and three reps elsewhere.

Single-server capacity cells retain both a labelled stock profile (native
discovery, reliable telemetry) and the centralized tuned profile
(centralized rendezvous, best-effort telemetry).  RMW-specific buffer choices
come from the calibration sweep and must be recorded before final runs.

Real-LAN cells use the existing LAN host configuration/deployment helper.  The
server placement must be made explicit in the generated base scenario: primary
on home-mini, clients on the rig, then the specified inverted and one-server-
per-host redundancy checks.

## Interpretation

The report exposes:

- telemetry delivery, receive throughput, and per-server distribution;
- command delivery, client coverage, duplicates, inter-arrival gaps, and
  post-failure maximum outage;
- RPC success/timeouts, throughput, p50/p99 RTT, server distribution, and
  recovery gap;
- application-server, client, and middleware-infrastructure CPU/RSS;
- raw curves, sustainable delivered throughput, and two SLO overlays.

The relaxed overlay requires telemetry/RPC success at least 90%, RPC p99 at
most 1 s, and no command gap above two periods.  The tight overlay requires
telemetry, command, and RPC success at least 99%, RPC p99 at most 200 ms, and
no command gap above 1.2 periods.  These overlays do not replace the raw
metrics and are not combined into a single middleware score.
