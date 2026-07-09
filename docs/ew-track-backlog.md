# EW Track — Backlog (deferred & optional-parallel)

Items split out of the [DDS benchmark plan](dds-benchmark-plan.md) on 2026-06-27.
None are on the DDS critical path. They belong to the **EW / jamming-resilience
track** (the `substrate: channel` dataplane: `channel/forwarder.py`, `ew.py`,
`propagation.py`, `terrain.py`).

## Optional — parallel-safe (separate worktrees, zero file overlap with DDS work)

Spin these up only to keep the EW track warm while the DDS benchmark runs. Each
touches a disjoint file set, so they merge cleanly alongside `dds-rmw-benchmark`.

### MLS handshake hardening ✅
- **Why:** EMANE spike found the OpenMLS queryable-based handshake is not
  loss-tolerant (~10× AoI inflation under EMANE loss; full fail on a weak
  foliage-edge link).
- **Files:** `agent/src/` (Rust) only — fully isolated from the Python sim.
- **DONE (branch `mls-handshake-hardening`):** replaced zenoh's 10 s default
  per-query timeout with an explicit 2 s timeout + exponential backoff (a lost
  query now costs ~2 s, not 10 s), made the committer fetch all key packages
  concurrently, and exposed a configurable `--mls-timeout-s` deadline (+ a
  `queries` count in the `mls_ready` metric). The cheap-retry + generous-deadline
  combination is what makes it loss-tolerant. See
  [results/mls-handshake-hardening.md](results/mls-handshake-hardening.md).

### Weak-link propagation calibration
- **Why:** EMANE was more pessimistic than the hand-rolled model on weak
  foliage-edge links (v4 @118 dB: our FSK says fine, EMANE handshake takes 11 s
  and fails). Calibrate the model to close the gap.
- **Files:** `propagation/foliage.py`, `propagation/diffraction.py`,
  `channel/linkbudget.py` (`fsk_per`), `validation/`.
- **Plan:**
  1. **Freeze a comparison set.** Reproduce the divergent EMANE-spike cells as a
     fixed handful of `(distance, foliage_depth, tx_power)` points straddling the
     weak-link edge (the v4 @118 dB point and neighbours). This is the regression
     target — check it into `validation/`.
  2. **Attribute the gap.** Per cell, compare EMANE's effective pathloss and PER
     against our `CompositePathloss.loss()` and `fsk_per()` separately. The gap is
     either in the **pathloss** (foliage/diffraction under-attenuating) or in the
     **PER curve near threshold** (our FSK cliff too sharp/soft) — decide which
     before touching knobs.
  3. **Calibrate physically, not by fudge factor.** Prefer adjusting
     physically-meaningful inputs: foliage *depth along the path* (a modeling
     choice, not the Weissberger form), single- vs multiple-knife-edge diffraction
     count, and the `fsk_per` Eb/N0→BER constants — over inserting an arbitrary
     offset.
  4. **Guardrail.** The `validation/` suite must still reproduce the published
     Friis / two-ray / Weissberger / ITU knife-edge curves — calibration cannot
     break the anchored references. Add the EMANE weak-link cells as new regression
     points alongside them.
- **Exit:** our effective pathloss/PER is within a stated band of EMANE across the
  weak-link set (target ≤3 dB effective), published curves still green, and the
  divergence is attributed (pathloss vs PER) in a short note.

## Deferred — EW-vector work (irrelevant while not modelling jammers)

Resume only when returning to jamming-resilience experiments.

### Reactive jamming ✅
- **Why:** a smarter adversary that senses transmissions and jams the active
  channel — tests the FHSS/FEC defenses against a harder threat.
- **Files:** `ew.py` (new `Jammer` kind) + `config.py` + a scenario.
- **DONE (branch `reactive-jamming`):** `kind: reactive` follower folded into the
  per-dwell binomial model as a lock probability (no sub-tick stepping);
  defence is a cliff at `T_dwell = τ`, FEC nearly irrelevant. Validated against a
  time-domain dwell simulation. See [results/reactive-jamming.md](results/reactive-jamming.md).

### Per-tick link-state logging
- **Why:** the viz agent's outstanding request — the map today *reconstructs* link
  quality from per-packet verdicts (`viz/data.py:link_quality()` bins
  delivered/attempts per window), so it can only colour links that actually carried
  traffic and is noisy at low load. It cannot show the **true** jammer footprint /
  SINR on idle links. The orchestrator already computes the ground-truth
  `build_table()` every tick — just persist it.
- **Files:** `orchestrator.py` (per-tick writer), `viz/data.py` + `viz/web`
  (reader + heatmap layer), a scenario/run flag.
- **Plan:**
  1. **Log the truth.** Opt-in `--log-linkstate` writes the per-tick `LinkState`
     table to `linkstate.parquet`: `t, src, dst, prx_dbm, noise_dbm, rho,
     jam_inchannel_dbm, delivery_prob, foliage_db, terrain_db, medium`. Use the
     analytical `LinkState.delivery_prob(len)` — the instantaneous true link
     quality, independent of whether a packet crossed.
  2. **Bound the cost.** The table is O(N²) rows/tick, so this is **off by default**
     (zero write cost on benchmark/sweep runs) and supports a decimation interval
     (`every k ticks`) since link-state changes slowly relative to the tick rate.
  3. **Viz reads truth, falls back to reconstruction.** `viz/data.py` grows a
     `link_state_truth()` reader that prefers `linkstate.parquet` when present
     (true SINR/PER heatmap + jammer-footprint layer) and falls back to the
     packet-reconstructed `link_quality()` when it's absent — the existing dashboard
     keeps working on old runs.
- **Note:** only meaningful with a jammer present (EW track).
- **Exit:** with a jammer active, the map shows continuous true per-link SINR/PER
  and the jammer footprint (including on idle links) from `linkstate.parquet`;
  feature off by default; packet-reconstruction fallback still works when the log
  is absent.

## Also deferred (from roadmap M5+)
- ~~R4 Isaac Sim mobility provider~~ ✅ **DONE 2026-06-13 (PR #10)** — see the
  as-built note in [`docs/ros2-integration-plan.md`](ros2-integration-plan.md#r4--isaac-sim-6-as-mobilityprovider).
- Routing comparisons, frequency-band sweeps, Sionna RT pathloss backend — now
  planned in [`docs/m5-plan.md`](m5-plan.md) (ordering, scope, exit criteria).
