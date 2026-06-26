# EW Track — Backlog (deferred & optional-parallel)

Items split out of the [DDS benchmark plan](dds-benchmark-plan.md) on 2026-06-27.
None are on the DDS critical path. They belong to the **EW / jamming-resilience
track** (the `substrate: channel` dataplane: `channel/forwarder.py`, `ew.py`,
`propagation.py`, `terrain.py`).

## Optional — parallel-safe (separate worktrees, zero file overlap with DDS work)

Spin these up only to keep the EW track warm while the DDS benchmark runs. Each
touches a disjoint file set, so they merge cleanly alongside `dds-rmw-benchmark`.

### MLS handshake hardening
- **Why:** EMANE spike found the OpenMLS queryable-based handshake is not
  loss-tolerant (~10× AoI inflation under EMANE loss; full fail on a weak
  foliage-edge link).
- **Files:** `agent/src/` (Rust) only — fully isolated from the Python sim.
- **Goal:** make group join/commit loss-tolerant (retry/timeout, or a
  loss-tolerant handshake path). Re-run an EMANE cell to confirm.

### Weak-link propagation calibration
- **Why:** EMANE was more pessimistic than the hand-rolled model on weak
  foliage-edge links (v4 @118 dB: our FSK says fine, EMANE handshake takes 11 s
  and fails). Calibrate the model to close the gap.
- **Files:** `propagation.py` / `terrain.py` + `validation/`.
- **Goal:** tune the foliage/edge attenuation; validation suite still reproduces
  published curves.

## Deferred — EW-vector work (irrelevant while not modelling jammers)

Resume only when returning to jamming-resilience experiments.

### Reactive jamming
- **Why:** a smarter adversary that senses transmissions and jams the active
  channel — tests the FHSS/FEC defenses against a harder threat.
- **Files:** `ew.py` (new `Jammer` kind) + `config.py` + a scenario.
- **Sequencing:** after the channel-engine harness is current; benchmark the
  resilient stack (and, if desired, the DDS stack) against it.

### Per-tick link-state logging
- **Why:** the viz agent's outstanding request — log per-tick link state so the
  Canvas dashboard map shows the real jammer footprint / SINR.
- **Files:** `orchestrator.py` (per-tick logging) + `viz/`.
- **Note:** only meaningful with a jammer present (EW track).

## Also deferred (from roadmap M5+)
- R4 Isaac Sim mobility provider (see `docs/ros2-integration-plan.md`).
- Routing comparisons, frequency-band sweeps, Sionna RT pathloss backend.
