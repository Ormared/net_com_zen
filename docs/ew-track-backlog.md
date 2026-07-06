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
- **Files:** `propagation.py` / `terrain.py` + `validation/`.
- **Goal:** tune the foliage/edge attenuation; validation suite still reproduces
  published curves.

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
- **Why:** the viz agent's outstanding request — log per-tick link state so the
  Canvas dashboard map shows the real jammer footprint / SINR.
- **Files:** `orchestrator.py` (per-tick logging) + `viz/`.
- **Note:** only meaningful with a jammer present (EW track).

## Also deferred (from roadmap M5+)
- R4 Isaac Sim mobility provider (see `docs/ros2-integration-plan.md`).
- Routing comparisons, frequency-band sweeps, Sionna RT pathloss backend.
