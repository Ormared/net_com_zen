# DDS Deployment Archetypes & Multi-Host Plan

**Status:** scoped 2026-07-03; **P1–P4 complete 2026-07-04** — results in
[results/dds-lan.md](results/dds-lan.md) (P3), [results/dds-star.md](results/dds-star.md)
(P4), and the final [recommendation table](results/dds-recommendations.md).
P5's netem/topology re-measures remain optional. Follows the QoS-plane study
([results/dds-rmw-qos-plane.md](results/dds-rmw-qos-plane.md)).
**Branch:** `dds-rmw-benchmark` (continues).
**Execution model:** architect (Claude, this doc + reviews) + implementer
(gpt-5.5 via `codex exec`) per repo `CLAUDE.md`. Each phase below is a
self-contained implementation spec.

## Why this plan

The QoS-plane study established that the walls are **not tunable**: QoS,
socket buffers, and the Discovery Server were all null on the single-host
bridge substrate. Three levers remain, and all three are here:

1. **Root-cause** the Fast DDS ~33-participant clique cap (last untested
   config lever: allocation resource limits).
2. **Real networks** — the bridge substrate's links are ideal (∞ bandwidth,
   ~0 ms, 0 loss, perfect multicast). Real deployments are not; and the
   mechanisms we measured as "useless" (Discovery Server, peer lists, Zenoh
   routers) are *designed* for multicast-hostile networks.
3. **Topology** — all-to-all N(N−1) reliable endpoints is the fundamental
   wall. Real systems use stars, trees, clusters, shared topics.

## Deployment archetypes (the new top-level axis)

| Archetype | Shape | Where it runs |
|---|---|---|
| **A. Swarm mesh** | N peers, all-to-all, degraded radio links | bridge substrate + netem (single host) |
| **B. Flat LAN** | many participants on several machines, one L2 network | main rig + home-mini over real Wi-Fi |
| **C. Centralized** | one main server + N smaller followers (star) | hub on home-mini, followers on main rig (and inverted) |

Topology variants (shared topic vs per-node topics, router star vs mesh,
clusters) apply *within* archetypes.

## Recon facts (measured 2026-07-03)

- **home-mini** (`orm_small_nix@192.168.1.9`, key auth works): NixOS 25.11,
  4 cores, 15 GiB RAM. Docker + podman + nix + rsync present; **no python3,
  no pixi on PATH; sudo requires a password** → run everything in Docker,
  nothing on the host needing root.
- **Wi-Fi only**: `wlp2s0` is the sole active NIC (both Ethernet ports DOWN).
  Measured RTT main-rig → home-mini: **81–104 ms, high jitter** (Wi-Fi
  power-save). Wi-Fi APs degrade multicast (basic-rate, no ACK) — expect
  native SPDP/scouting discovery across hosts to be impaired or dead. That is
  a *finding to measure*, not a blocker: archetypes B/C are exactly where
  unicast discovery (Discovery Server, Cyclone peer lists, Zenoh routers)
  earns its keep.
- home-mini is small: cap its cohort at **~16–24 workload nodes** (4 cores).
  It naturally plays the **hub** role in archetype C.
- Optional future axis: plug in one of home-mini's Ethernet ports for a
  wired-vs-Wi-Fi comparison. Not required for any phase below.

## Architecture decisions

### D1 — env parity via pixi-in-Docker
RoboStack Jazzy (main rig) ships different middleware builds than upstream
`ros:jazzy` apt images (e.g. Fast DDS 8.4.3 vs 2.14.x). Cross-version RTPS
interop would muddy every comparison. **Decision:** the remote container
builds the *same pixi `ros2` env* from this repo's `pixi.toml`/`pixi.lock`
(image: `ghcr.io/prefix-dev/pixi` base → `pixi install -e ros2`), so both
hosts run bit-identical middleware. Container runs with `--net=host` (DDS
needs the real NIC; Docker NAT breaks discovery and locators).

### D2 — the `lan` substrate
Third substrate next to `channel`/`bridge`: **no netns, no veth**. Nodes are
plain processes bound to the host's real NIC (interface pinned per RMW —
Cyclone `NetworkInterfaceAddress`, Fast DDS `interfaceWhiteList`, Zenoh
listen endpoints — so Docker bridges/wireguard don't leak in). A
`hosts:` section in the scenario assigns each node to `local` or a named
ssh host. Remote nodes are spawned via `ssh <host> docker exec …`; JSONL
results land in a shared per-run directory and are pulled back with rsync
when the run ends. No root needed on either side (no netns).

### D3 — clocks and metrics
The two hosts' clocks are not synchronized to sub-ms. **One-way latency is
not comparable cross-host** — do not report it. Primary metrics stay
clock-skew-immune: connected pairs / mesh ratio, per-pair message counts,
discovery-to-first-recv measured *per receiving node against its own start
time*. Optional later: an echo (request/reply) topic for true RTT.

### D4 — hub workload mode
`ros2_workload/node.py` gains a `--role hub|spoke` mode for archetype C:
spokes publish telemetry on a **shared topic** (`/swarm/telemetry`) and
subscribe only to `/swarm/command`; the hub subscribes to telemetry and
publishes commands. Endpoint count per spoke = O(1), hub = O(1) topics with
N matched writers. This doubles as the shared-topic-aggregation variant for
archetype A.

## Phases

Each phase = one codex implementation task + architect review + a run I
(or a runner agent) execute. Phases are sequential (they touch the same
files: `config.py`, `orchestrator.py`, harness).

### P1 — Fast DDS allocation-limit sweep (root-cause the clique cap)
*Existing rig, no new substrate. Carry-over from the QoS study.*

- Extend `_fastdds_profiles_xml()` with participant allocation knobs:
  `<rtps><allocation>` (`total_participants`/`total_readers`/`total_writers`
  initial+maximum), `<remote_locators>` (`max_unicast_locators`,
  `max_multicast_locators`), and the transport descriptor's
  `maxInitialPeersRange`. New `Ros2WorkloadConfig` field
  `fastdds_allocation_participants: int = 0` (0 = stock XML unchanged);
  when set, size all three allocation totals and `maxInitialPeersRange`
  to that value.
- New `dds_qos_lab` phase `alloc`: fastrtps only, N=96, 120 s, cells =
  {stock, alloc=64, alloc=128, alloc=256}. Fast DDS is CV 0 % → 1 run/cell
  suffices, but do 2 for safety.
- Unit tests for the XML generator (pattern: `tests/test_qos_buffers.py`).
- **Decides:** cap moves → Fast DDS re-enters the race, tuning doc corrected;
  cap stays → last config hypothesis falsified, cap is architectural.

### P2 — `lan` substrate + remote runner (infrastructure)
- `docker/dds-lab/Dockerfile` per D1; a `deploy` helper (rsync repo →
  home-mini, build image, start a long-lived container).
- Config: `substrate: "lan"`; `Scenario.hosts: dict[str, SshHostConfig]`
  (`ssh_target`, `container`, `workdir`, `iface`); `NodeConfig.host:
  str = "local"`. Validators: `lan` requires hosts for any non-local node;
  netem/bridge options rejected on `lan`.
- Orchestrator: `_run_lan()` — no netns; local spawn direct, remote spawn
  via `ssh docker exec`; stagger honored globally (one clock: the
  orchestrator issues spawns); result collection via rsync; manifest
  records per-node host placement + RTT probe (ping) at run start.
- Smoke: N=4 (2 local + 2 remote), each RMW, mesh=1.0 expected. **This
  smoke is itself the first experiment:** does native multicast discovery
  even cross the Wi-Fi AP? Record the answer per RMW.

### P3 — Archetype B: flat LAN sweep
- N = 8/16/24/32/48 split across hosts (home-mini ≤ 24), all-to-all.
- Per RMW, two discovery modes where applicable: native multicast vs
  unicast-assisted (Fast DDS Discovery Server on home-mini; Cyclone
  `Peers/Peer` list; Zenoh: routers on each host, connect by IP). **The
  Discovery Server re-test in its intended environment.**
- Replicates ×3 minimum (real Wi-Fi will be noisy); report mesh, pairs,
  establishment rate, per-host breakdown (local-local vs cross-host pairs
  — the interesting split).

### P4 — Archetype C: centralized star
- Hub on home-mini (D4 workload mode), spokes on main rig: N spokes =
  16/32/64/96. Also inverted (hub local, spokes partly remote) as a check.
- Measures: spoke→hub delivery ratio, hub→spoke fan-out delivery, hub CPU
  (4-core box saturation point), discovery time vs N.
- Hypothesis to test: O(N) endpoint topology escapes all three walls —
  Fast DDS clique cap (only N+1 participants but O(N) *endpoints on hub*),
  Cyclone storm (no all-to-all SEDP), Zenoh variance (no router mesh).

### P5 — Archetype A revisited: netem + topology variants (single host)
- Un-stub the bridge substrate's netem hook: per-veth delay/jitter/loss/rate
  from config. Calibrate a "wifi-like" profile from P3's measured link.
- Variants at N=48/96: shared-topic aggregation (D4 spoke mode, no hub),
  Zenoh router star (1 router) vs mesh, K-cluster partitioning
  (ROS_DOMAIN_ID per cluster, heads bridging).
- **Decides:** which topology, under realistic links, reaches the highest
  usable scale per RMW → the final recommendation table.

## Open questions / risks

- **R1 — Wi-Fi multicast may be fully dead cross-host.** Then P3's "native"
  cells score ~0 cross-host pairs; that IS the result, and the
  unicast-assisted cells become the story. P2's smoke answers this early.
- **R2 — home-mini is 4 cores.** Cohort caps enforced in config validation
  (per-host max nodes). Hub role is 1 process — fine.
- **R3 — Wi-Fi run-to-run variance** will exceed even Zenoh's ±35 %.
  Replicates ×3 minimum, report CV alongside means, never single runs.
- **R4 — long-lived container drift.** The deploy helper re-rsyncs the repo
  and reinstalls the env only when `pixi.lock` changes; manifest records
  image + git SHA on both ends.
- **R5 — password-sudo on home-mini.** Nothing in this plan needs root
  there (no netns on `lan`; Docker group covers container ops).

## Definition of done

Per-archetype results docs (`results/dds-lan.md`, `results/dds-star.md`,
`results/dds-topology.md`) + a final recommendation table (RMW × archetype ×
topology → max usable N, with variance), linked from the roadmap. The
Fast DDS clique-cap root cause resolved (either direction) in
[results/dds-rmw-qos-plane.md](results/dds-rmw-qos-plane.md).
