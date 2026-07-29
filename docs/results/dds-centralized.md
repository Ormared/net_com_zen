# Centralized ROS 2 RMW comparison

**Status:** full controlled and real-LAN matrix executed on 2026-07-28/29.

This evaluation compares Fast DDS (`rmw_fastrtps_cpp`), Cyclone DDS
(`rmw_cyclonedds_cpp`), and Zenoh (`rmw_zenoh_cpp`) with the mixed centralized
telemetry, command, and RPC workload described in
[centralized-rmw-plan.md](../centralized-rmw-plan.md). Results are reported as
separate delivery, throughput, latency, readiness, failover, and resource
curves; there is no composite winner score.

## Scope and completeness

The generated plan contains 573 serial cells:

| Stage | Planned cells | Coverage |
|---|---:|---|
| Tuning | 156 | N=32/96 candidates, three seeds |
| Controlled capacity/frontiers | 162 | N=4–128, rate and payload, stock/tuned |
| Controlled HA/sharding | 78 | N=32/96, five HA or three sharding seeds |
| Real LAN | 177 | capacity, inversion, HA, sharding, and rate |

The completeness-aware report currently records 561 clean cells and 12
explicit failures. Eleven are measured N=96/N=128 process-collapse failures.
One interrupted LAN cell produced sparse NUL-filled JSONL artifacts and is
retained as a malformed infrastructure failure after its single permitted
retry. No missing, aborted, or malformed cell is omitted from the 573-cell
denominator.

Raw data and generated artifacts remain under `results/centralized/` (ignored
by Git):

- `summary_all.json` and `summary_all.csv`: all cells and metrics;
- `failure_accounting.{json,csv}`: exact failure reasons;
- `slo_capacity.{json,csv}` and `sustainable_frontiers.{json,csv}`;
- `replicate_variance.{json,csv}`;
- plots for delivery, throughput, rate, RPC, readiness, LAN RTT, command
  behavior, failover, CPU/RSS, and SLO capacity;
- `locked_profiles.json`, including the complete 26-row tuning table.

## Method

All cells ran serially. Capacity and rate cells use three seeds; active-active
and active-passive use five; sharding uses three. Fresh evaluation startup
required 12 GiB available memory and 10 GiB disk, resumed phases required
8 GiB available memory, a cell aborted only after ten sustained seconds below
6 GiB available memory, and phases stopped below 3 GiB disk. Infrastructure
and spawn failures were eligible for one retry; performance failures were not.

The two SLO overlays are:

- **Relaxed:** telemetry delivery at least 0.90, RPC success at least 0.90,
  RPC p99 at most 1000 ms, and command-gap p99 at most twice the command
  period.
- **Tight control:** telemetry and command delivery at least 0.99, RPC success
  at least 0.99, RPC p99 at most 200 ms, and command-gap p99 at most 1.2 times
  the command period.

A capacity or rate is called sustainable only when every planned replicate is
complete and every replicate passes that SLO. This deliberately prevents one
good seed or a missing failed seed from defining capacity.

### Provenance

The benchmark source was committed locally as
`d66b5d6922d1815f3b3a68a96b26d24cfb9db7ae`; the Pixi lock SHA-256 is
`ebe7c3e81acafe53b0278d1c7816ec5ac30193c96d86cafe6cf87626264c5738`.
The rig used Linux `7.0.0-28-generic`, an Intel Core Ultra 9 275HX (24 cores,
one hardware thread per core), and 62.2 GiB RAM. Relevant middleware versions
were Fast DDS 2.14.5 / RMW Fast DDS 8.4.3, Cyclone DDS 0.10.5 / RMW Cyclone
DDS 2.2.3, and RMW Zenoh 0.2.9 with `libzenohc` 1.7.2.

The provenance snapshot was overwritten by early versions of the resume
workflow, so its recorded start and swap counters represent the last resume,
not the beginning of the entire multi-day evaluation. Between that snapshot
and final reporting, `pswpin` rose by 765 pages and `pswpout` did not change.
The
resume path now preserves the original provenance file, but full-run swap
activity cannot be reconstructed retrospectively.

## Locked tuning profiles

Selection required complete centralized candidates, then maximized tight SLO
passes, relaxed passes, delivered telemetry, and RPC throughput, with worst
RPC p99 and server/infrastructure CPU as tie-breakers.

| RMW | Discovery | Telemetry QoS | Socket buffer |
|---|---|---|---:|
| Fast DDS | centralized discovery server | reliable | default (0) |
| Cyclone DDS | centralized static peers | best effort | default (0) |
| Zenoh | centralized router | best effort | default (0) |

The chosen profile is a deterministic operating point, not evidence that every
individual metric improves over stock. The complete Pareto table is preserved
in `locked_profiles.json`.

## Capacity and rate frontiers

Maximum all-replicate sustainable frontiers are:

| Substrate | RMW | Relaxed max N | Relaxed telemetry / RPC offered rate at N=64 | Tight max N / rate |
|---|---|---:|---:|---|
| Bridge | Fast DDS | 96 | 1,280 / 320 s⁻¹ | none |
| Bridge | Cyclone DDS | 96 | 320 / 64 s⁻¹ | none |
| Bridge | Zenoh | 64 | 3,200 / 640 s⁻¹ | none |
| Wi-Fi LAN | Fast DDS | none | none | none |
| Wi-Fi LAN | Cyclone DDS | none | none | none |
| Wi-Fi LAN | Zenoh | 64 | 1,280 / 320 s⁻¹ | none |

“None” means no tested point passed in every replicate; it does not mean zero
traffic was delivered. The tight overlay was intentionally severe: individual
cells passed, but no capacity point passed across all three seeds.

At controlled N=128, all tuned RMWs completed but none met the relaxed SLO.
The Fast DDS stock N=128 seed 1 cell also lost two clients to SIGKILL. Thus the
controlled single-server knee is at or below N=96 for the relaxed overlay.

### Stock versus tuned at N=64

Median controlled results show why no single tuning label should be treated as
universally better:

| RMW | Profile | Telemetry delivery | RPC success | RPC p99 | Readiness |
|---|---|---:|---:|---:|---:|
| Fast DDS | stock | 0.985 | 0.986 | 4.24 ms | 3.04 s |
| Fast DDS | tuned | 0.971 | 0.979 | 3.67 ms | 4.00 s |
| Cyclone DDS | stock | 0.976 | 0.989 | 3.62 ms | 2.08 s |
| Cyclone DDS | tuned | 0.979 | 0.988 | 3.93 ms | 8.27 s |
| Zenoh | stock | 0.976 | 0.987 | 85.15 ms | 2.31 s |
| Zenoh | tuned | 0.980 | 0.995 | 46.18 ms | 1.96 s |

Fast DDS tuning reduced median server CPU from 49.6% to 35.8% but slightly
reduced delivery. Cyclone tuning was nearly neutral on steady-state delivery
and much slower to become ready. Zenoh tuning materially reduced RPC tail
latency and server CPU (66.9% to 43.7%) while improving delivery slightly.

## Bridge versus real Wi-Fi LAN

The LAN used the rig at `192.168.1.6` (`wlp130s0f0`) and `home-mini` at
`192.168.1.9` (`wlp2s0`). Normal placement put the primary/single server on
`home-mini` and clients on the rig. Inverted N=64 put the server on the rig and
24 clients on `home-mini`. Across complete LAN cells, run-start ping RTT had a
2.48 ms minimum, 3.98 ms median, 37.21 ms p95, and 91.70 ms maximum.

At normal-placement N=64:

| RMW | Profile | Telemetry delivery | RPC success | RPC p99 |
|---|---|---:|---:|---:|
| Fast DDS | stock | 0.951 | 0.973 | 193.1 ms |
| Fast DDS | tuned | 0.000 | 0.000 | unavailable |
| Cyclone DDS | stock | 0.783 | 0.840 | 236.9 ms |
| Cyclone DDS | tuned | 0.852 | 0.865 | 57.4 ms |
| Zenoh | stock | 0.970 | 0.988 | 21.0 ms |
| Zenoh | tuned | 0.965 | 0.981 | 26.3 ms |

Wi-Fi therefore changed the ordering. Zenoh retained bridge-like delivery and
the only all-seed relaxed capacity through N=64. Cyclone tuning improved its
LAN delivery and RPC tail substantially over stock but stayed below the
relaxed delivery threshold. Fast DDS centralized discovery was placement
sensitive: tuned normal placement delivered no application traffic at N=64,
whereas the inverted tuned placement reached 0.942 telemetry delivery and
0.958 RPC success. Conversely, Fast DDS stock fell from 0.951 telemetry
delivery in normal placement to 0.050 inverted. Zenoh was nearly
placement-invariant (about 0.97 telemetry delivery in both directions).

Cross-host clocks were not synchronized. No cross-host one-way latency is
reported; RPC RTT, readiness intervals, gaps, and duplicates use timestamps
produced within one process. Ping RTT is a link-quality indicator only.

## Redundancy and sharding

At controlled N=96, single-server relaxed passes were 3/3 for Fast DDS and
Cyclone DDS and 1/3 for Zenoh. Two-server sharding passed 3/3 for Fast DDS and
Cyclone DDS and 0/3 for Zenoh. Median single versus sharded outcomes were:

| RMW | Mode | Telemetry / RPC delivered s⁻¹ | Telemetry / RPC success | RPC p99 |
|---|---|---:|---:|---:|
| Fast DDS | single | 458.2 / 91.9 | 0.954 / 0.972 | 19.4 ms |
| Fast DDS | sharded | 454.7 / 91.2 | 0.947 / 0.965 | 34.3 ms |
| Cyclone DDS | single | 457.4 / 91.5 | 0.953 / 0.968 | 9.4 ms |
| Cyclone DDS | sharded | 466.7 / 92.9 | 0.972 / 0.982 | 3.1 ms |
| Zenoh | single | 454.3 / 90.8 | 0.946 / 0.961 | 1,094 ms |
| Zenoh | sharded | 455.8 / 91.0 | 0.950 / 0.963 | 1,142 ms |

Cyclone benefited from sharding at N=96; Fast DDS did not, and Zenoh’s service
queue remained the bottleneck. Because two-server modes were only tested at
N=32/96, this experiment does not establish a two-server N=128 capacity.

Active-active generally recovered faster than active-passive. At controlled
N=96, median command/RPC failover gaps were about 1.0/1.0 s for Fast DDS and
Cyclone active-active, versus 3.4/3.1 s and 2.1/2.0 s respectively for
active-passive. Zenoh active-active was 3.0/4.0 s and active-passive 3.1/4.0 s,
with RPC p99 above one second in both modes.

On Wi-Fi at N=96, active-active median RPC failover gaps were about 1.0 s for
Fast DDS and Cyclone but 15.5 s for Zenoh. Active-passive was about 1.1 s for
Fast DDS, 1.0 s for Cyclone, and 15.0 s for Zenoh. Cyclone sharding was the
only LAN N=96 two-server mode with a 3/3 relaxed pass result; Fast DDS sharding
passed 1/3 and Zenoh 2/3 before completeness penalties.

## Failure modes

- **Process collapse at scale:** Fast DDS stock lost two clients in one
  controlled N=128 replicate and 6–9 clients in all three normal-placement LAN
  N=96 stock replicates.
- **Zenoh service saturation:** multiple N=96 LAN modes emitted
  `Query queue depth of 10 reached` and dropped RPC queries. Several cells
  ended with SIGKILLed clients or launcher/server exits; surviving runs showed
  RPC p99 from hundreds of milliseconds to more than one second.
- **Discovery/placement sensitivity:** Fast DDS centralized tuned N=64
  communicated when the server was local but delivered no application traffic
  with the server on `home-mini`; the stock profile showed the inverse
  sensitivity.
- **Wi-Fi variability:** run-start RTT p95 was roughly nine times the median,
  and RPC p99 CV exceeded 0.8 in several LAN groups.
- **Interrupted artifact corruption:** one Cyclone active-passive N=96
  replicate contains sparse NUL-filled JSONL files after the remote container
  stopped cleanly during an earlier pass. Its one permitted infrastructure
  retry also produced malformed artifacts, which are preserved in place and
  remain an explicit failed cell.

## Variance

Across outcome metrics with at least two values, median replicate CV was
0.0135 and p95 was 0.475. Tail latency dominated the high-variance groups:
Zenoh LAN sharded N=96 RPC p99 had CV 1.22; Zenoh LAN active-passive N=96 had
CV 1.08; and several payload/rate or placement groups exceeded 0.8. Median
delivery and throughput are consequently much more stable than the tail and
failover measurements. CPU CVs are not directly comparable on LAN because
only local processes were sampled.

## Strengths, weaknesses, and limitations

- **Fast DDS:** strong controlled relaxed capacity and low controlled RPC
  latency; centralized discovery reduced CPU at N=64. Weaknesses were
  placement sensitivity, complete tuned LAN communication loss in normal N=64
  placement, and client SIGKILLs at the largest stock scales.
- **Cyclone DDS:** strongest controlled sharded N=96 result and the best
  two-server LAN N=96 relaxed result. Weaknesses were slow readiness in the
  tuned controlled profile and sub-0.90 single-server LAN delivery.
- **Zenoh:** most stable N=64 behavior across LAN placement and the only
  single-server RMW with an all-seed relaxed LAN capacity/rate frontier.
  Weaknesses were the depth-10 service query queue, very large N=96 RPC tails,
  and long LAN failover gaps.

The study uses one rig, one `home-mini`, one Wi-Fi environment, and one
middleware-version set. LAN CPU/RSS covers local processes only; remote
`home-mini` resource sampling was not implemented, so cross-RMW LAN resource
claims exclude the remote server/router. Wi-Fi conditions were not controlled,
and run-start ping cannot describe within-cell jitter. HA faults kill the
primary server/infrastructure process at 45 s; they are not whole-machine,
access-point, or network-partition failures. Tested points bound the reported
frontiers but do not locate thresholds between them.
