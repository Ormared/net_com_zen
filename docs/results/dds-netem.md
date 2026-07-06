# Emulated Wi-Fi vs the real thing: per-link netem gets the ranking wrong

**Date:** 2026-07-06 · **Branch:** `dds-rmw-benchmark`
**Closes:** P5's sim-to-real loop ([dds-topology-plan.md](../dds-topology-plan.md)) —
the bridge substrate's netem hook, calibrated against the flat-LAN rig that
produced [dds-lan.md](dds-lan.md).
**Raw:** `results/dds/netem_wifi/` (27 cells), `results/dds/cyctune/` (9 cells), gitignored.

## The profile

Per-veth egress `tc netem`, calibrated against the P3 link (main rig ↔
home-mini through the home AP). Measured on the day: ping avg RTT 112 ms,
mdev 93 ms, idle loss 0 %. netem delay is one-way per egress (pair RTT =
2·delay), so:

| knob | value | provenance |
|---|---|---|
| delay | 55 ms | measured (½ · avg RTT) |
| jitter | 45 ms | measured (≈ ½ · mdev) |
| loss | 1 % | **assumed** — idle ICMP loss is 0 %; stands in for 802.11 contention loss |
| rate | 100 Mbit | **assumed** — nominal Wi-Fi share per station |

Mesh workload, 30 s window, 3 reps — deliberately mirrors the P3 cells so the
two tables are directly comparable.

## Results: emulated wifi (bridge + netem) vs real Wi-Fi (P3 lan)

conn / delivery; P3 columns from [dds-lan.md](dds-lan.md).

| N | Fast DDS emu | Fast DDS real | Cyclone emu | Cyclone real | Zenoh emu | Zenoh real |
|---|---|---|---|---|---|---|
| 16 | 1.0 / 0.99 | 1.0 / 0.97 | 1.0 / 0.99 | 1.0 / 0.88–0.93 | 1.0 / 0.99 | 1.0 / 0.97 |
| 32 | 1.0 / 0.97 | **0.39–0.58** / — | 1.0 / 0.99 | 1.0 / 0.67–0.69 | 1.0 / 0.98 | 1.0 / 0.94 |
| 48 | 1.0 / 0.91–0.94 | **0.25** / — | 1.0 / 0.95–0.98 | 0.94 / 0.56–0.59 | **0.5–1.0 / 0.34–0.47** | 1.0 / 0.92 |

Latency tells the same story: emulated Cyclone p99 stays at 0.2–0.4 s where
the real link taxed it 1.5–7 s; emulated Fast DDS holds p99 ~1.1–1.5 s at
every N where the real link killed its cross-host pairs outright; emulated
zenoh's p99 blows up to 5–15 s at N=48 where the real link held it at 1.8 s.

## The finding: the emulation inverts the ranking

The real Wi-Fi ranking (P3) was **zenoh > Cyclone ≫ Fast DDS**. The emulated
ranking is **Cyclone ≥ Fast DDS > zenoh** — close to the ideal-fabric bridge
ranking, and nearly the opposite of the deployment truth. Per-link netem
failed to reproduce a single one of P3's three failure modes:

1. **Fast DDS's cross-host cliff at N=32 doesn't appear.** The real cliff was
   N²/4 unicast flows contending for airtime on a shared medium — loss that
   *escalates with offered load*. netem's independent 1 % per-packet loss is
   trivially absorbed by the reliable protocol at any N; nothing in a
   per-link qdisc couples one flow's load to another flow's loss.
2. **Cyclone's seconds-scale retransmit tax doesn't appear.** Same cause:
   its real p99 came from bursty, load-correlated loss plus AP queueing.
   Against uniform random loss its retransmit machinery is cheap (p99
   0.2–0.4 s, delivery 0.95–0.99 — the best of the three, as on the ideal
   bridge).
3. **Zenoh inverts for a rig reason, not a protocol reason.** On the bridge,
   every node runs its OWN router in a full router mesh (N(N−1)/2 TCP links
   — already zenoh's known wall, [dds-topology.md](dds-topology.md) Part 1b);
   adding 110 ms RTT + loss to every one of 1128 router links strains its
   control plane (one rep failed mesh formation entirely, conn 0.5). The
   real deployment ran ONE router per host with a single TCP session over
   the air — a topology the single-host substrate cannot express. The
   emulated zenoh cell tests a deployment shape nobody would ship on Wi-Fi.

**Practical conclusion for the tutorial/roadmap:** a per-link netem profile is
fine for *latency budgets* (the p50s land exactly where the calibration says:
~RTT/2 + queueing) but useless for *ranking middlewares on a shared medium* —
the failure modes that decide the ranking live in load-coupled loss and in
deployment topology, neither of which a per-veth qdisc models. Benchmark on
the real transport (P3/P4 did); use netem only to stress latency/jitter
tolerance of application logic. This validates the multi-host track: had we
trusted the emulation, every P3/P4 conclusion would have been wrong.

## Cyclone retransmit knobs on the lossy profile (`results/dds/cyctune/`)

The one QoS region the ideal bridge could never exercise: Cyclone's
retransmit/pacing behaviour under loss (new engine knob
`ros2.cyclonedds_internal` → CycloneDDS `<Internal>` elements). N=32 on the
wifi profile, 3 reps each, vs the stock wifi32 baseline above:

| variant | conn | delivery | p50 / p99 ms |
|---|---|---|---|
| stock (reliable) | 1.0 | 0.985–0.989 | 85 / 220–340 |
| `NackDelay: 100 ms` | 1.0 | **0.57**, 0.988, 0.988 | 85–89 / 306–835 |
| `RetransmitMerging: always` | 1.0 | 0.989–0.990 | 84 / 250–330 |
| best_effort QoS | 1.0 | 0.980–0.981 | **60 / 100** |

Verdict: **stock wins among the reliable configs.** Retransmit merging is a
wash; delaying NACKs only adds variance (the 0.57 rep spent 25 s in
discovery-recovery — late readers wait a full NackDelay round per repair).
The only knob with a real effect is dropping reliability altogether:
best_effort pays the raw 1 % netem loss (delivery 0.98) and in exchange p99
collapses to the ~100 ms the link itself costs — the right contract for
5 Hz telemetry where a stale sample is worthless anyway. (Read with the
caveat above: uniform netem loss is the easy case for a reliable protocol;
treat effects here as directional only.)
