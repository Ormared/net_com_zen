# M5+ plan — routing, frequency bands, ray-traced propagation

Three independent follow-on features from the roadmap's `M5+` row. None depend on
each other, so they can run in parallel or in the order below — but the order is
chosen by **value-per-effort descending**, and each de-risks a question the next
one leans on.

| Phase | Feature | Layer | Effort | Why this order |
|---|---|---|---|---|
| **M5.1** ✅ | Frequency-band sweeps | harness + results | days | **DONE 2026-07-07:** carrier trades foliage margin for spectral agility; 915 MHz is the sweet spot; at 2.4/5.8 GHz the MLS group never forms — the band choice is a security outage before it is packet loss. See [results/freq-band-sweep.md](results/freq-band-sweep.md) |
| **M5.2** ✅ | Routing comparison layer | Python sim (`routing.py`) | 1–2 wk | **DONE 2026-07-07** (branch `m5-routing`): routing doubles quiet delivery on spread geometry and holds the physics ceiling under moderate jamming; linkstate ≈ flood at 14–19 % less cost, gossip dominated — worth a real agent implementation. See [results/routing-comparison.md](results/routing-comparison.md) |
| **M5.3** | Sionna RT pathloss backend | heavy pixi env + precompute | 2 wk + GPU | Fidelity upgrade behind the existing `PathlossProvider` interface. Mirrors the Isaac pattern (heavy env, offline precompute, cheap runtime lookup). Do last — most infra, least new *science* per hour. |

Guiding principle (same as the whole project): these are **model comparisons that
produce defensible curves**, not protocol/infra projects. Where a feature threatens
to become an implementation project (real MANET daemon, live RT per tick), the plan
scopes it back to the model layer and flags the heavier variant as a separate
follow-on.

---

## M5.1 — Frequency-band sweeps

**What already works.** The propagation stack is fully frequency-parametric today:
Friis and the two-ray crossover (`freespace.py`), Weissberger foliage
(`foliage.py`, `f_ghz**0.284`), and ITU knife-edge diffraction (`diffraction.py`,
via `lam = C/f_hz`) all take `f_hz`. `RadioProfile.freq_hz` flows through
`build_table` into every `LinkState`. So the *model* supports band comparison now;
the work is harness + interpretation, not new physics.

**The real tradeoff to surface.** Lower carriers (VHF/UHF) diffract around terrain
and penetrate foliage far better → longer NLOS range → sparser, better-connected
mesh. Higher carriers (S/C band) attenuate faster **but** a fixed *fractional*
bandwidth buys many more FHSS channels → better dilution against partial-band
jamming (smaller `rho` per barrage watt). The band choice trades **connectivity
range** against **spectral agility**. That's the curve M5.1 must draw.

**Design — two clean variants** so the two effects don't confound:

- **V1 (isolate propagation):** sweep carrier holding *absolute* channelization
  fixed (`hop.n_channels` and per-hop `bandwidth_hz` constant). Only pathloss
  moves. Shows the range/connectivity story.
- **V2 (isolate agility):** sweep carrier holding *fractional* bandwidth fixed
  (hop span / carrier constant → `n_channels` scales with carrier). Shows the
  anti-partial-band-jam story.

- **Bands:** a realistic tactical/ISM ladder — 150 MHz (VHF), 433 MHz & 915 MHz
  (UHF ISM), 2.4 GHz, 5.8 GHz. Keep tx power, antenna gain, and swarm geometry
  fixed across the ladder; the jammer's `bandwidth_hz` is matched to each variant's
  hop span so barrage coverage is comparable.
- **Harness:** `freq_hz` (and, for V2, a derived `n_channels`) become sweep axes in
  a new `scenarios/sweep_freq_band.yaml`; reuse the existing cartesian sweep +
  paired-seed replay + report harness unchanged.
- **Metrics:** the standard set — frame PDR, update delivery, mean AoI — vs carrier,
  crossed with jammer power and hop rate.

**Exit:** a results doc (`docs/results/freq-band-sweep.md`) with the
range-vs-agility tradeoff curves for both variants, and a one-line recommendation
per threat regime (quiet vs partial-band-jammed) on which band the swarm should
prefer. Validation suite unchanged (no model change).

**Risk:** cross-band comparisons are only fair if the *link budget* is held
honestly — antenna aperture and noise bandwidth both track frequency in reality.
Decision: hold antenna **gain** (dBi) and noise figure fixed and state explicitly
that we are comparing at equal EIRP and equal receiver NF, i.e. isolating
propagation + channelization, not front-end physics. Note the caveat in the doc.

---

## M5.2 — Routing comparison layer

**The gap.** The sim is single-hop today: `build_table` produces a full directed
per-link table and delivery is link-local (`LinkState.verdict`). When the swarm
spreads out or is partially jammed so some pairs have no viable *direct* link, the
current model just drops those pairs (`no_link`). Multi-hop routing is the classic
swarm answer — but we've never measured whether it beats direct single-hop under
*our* jamming model, or which routing policy wins. That's the M5.2 question.

**Scope decision — model layer, not a protocol.** Implementing a real MANET daemon
(OLSR/Babel) per netns is a project of its own and conflates routing with the
transport/QoS questions M3/R3 already answered. Instead, add a `routing.py` that
consumes the per-tick link table and computes **end-to-end** delivery under
different routing *policies*, integrating out over the same seeded per-link
uniforms. This isolates the routing variable cleanly and stays in the
model-comparison spirit.

**Policies to compare** (all fed the same `build_table` output per tick):

- **direct** — current behaviour; one hop, drop if the direct link is down. Baseline.
- **shortest-viable-path (link-state)** — per tick, route each pair over the path
  maximising end-to-end delivery probability (product of per-hop `delivery_prob`);
  Dijkstra on `-log(delivery_prob)` edge weights. The "smart centralised router"
  upper bound for a single path.
- **flooding / epidemic** — a message arrives if *any* path of up-links exists;
  reliability upper bound, bandwidth cost ignored (report the hop-count / expected
  transmissions as the cost axis so the tradeoff is visible).
- **gossip with TTL** — probabilistic forward with a hop limit; the realistic
  middle ground between direct and flooding.

**Metrics:** end-to-end update delivery and AoI vs jammer power, per policy, with a
**cost** column (mean transmissions per delivered update) so the reliability gain is
never reported without its bandwidth price. Crossed with swarm spread (a geometry
axis) since routing only helps once direct links start failing.

**Files:** new `sim/netcom_zen/routing.py` (pure, consumes the link table);
orchestrator/report gain a `routing: direct|linkstate|flood|gossip` axis; new
`scenarios/sweep_routing.yaml`; unit tests cross-checking end-to-end delivery
against hand-computed small topologies.

**Exit:** `docs/results/routing-comparison.md` showing where multi-hop beats
direct (and by how much, at what transmission cost) as a function of swarm spread ×
jammer power. If routing shows a large, robust win, that *justifies* a follow-on to
implement a real routing layer in the Rust agent — logged as a candidate, not
committed here.

**Risk:** the seeded-uniform replay must extend cleanly to multi-hop (each hop
draws from its own `link_rng(seed, src, dst)` stream, so paths stay replayable).
Verify determinism holds across policies before drawing conclusions.

---

## M5.3 — Sionna RT pathloss backend

**What it is.** Sionna RT (NVIDIA's differentiable radio ray-tracer) replaces the
analytical `CompositePathloss` (Friis + two-ray + Weissberger + knife-edge) with
ray-traced pathloss over an actual 3D scene — the propagation analog of what Isaac
did for mobility. Multipath, real building/terrain geometry, and material
reflectivity come out of the geometry instead of closed-form approximations.

**Design — slot in behind the existing interface.** ADR-0001 fixed the pathloss
contract as EMANE-shaped: `(tx_xy, rx_xy, freq_hz) -> PathlossBreakdown`. A
`SionnaPathloss` implements the same `.loss()` interface, so `build_table` is
untouched. Follow the Isaac pattern exactly:

- **Heavy standalone pixi env** (`sionna`, with the CUDA/`mitsuba`/`tensorflow`
  stack) — never in the default env, like `isaac`.
- **Offline precompute, cached lookup.** Sionna solves a **coverage map** /
  path set over the scene *once* (per band, per scene) into a pathloss grid;
  runtime just does a CPU-only bilinear lookup per `(tx_xy, rx_xy)`. The per-tick
  loop stays cheap and needs no GPU — the same "precompute then serve" split that
  kept Isaac's stepper out of the hot loop.
- **Scene reuse.** The scene comes from the existing DEM terrain (and, later, the
  Isaac scene geometry), so trajectories, mobility, and propagation all respect the
  same world.

**Validation is the point.** A ray-tracer is only worth the weight if we show what
it buys. Cross-check `SionnaPathloss` against the analytical model in regimes where
they *must* agree (free-space, flat-earth two-ray) — agreement validates the
pipeline — then quantify where they diverge (urban multipath, terrain shadowing,
foliage) — divergence is the fidelity gain. Add these as `validation/` cases;
published-curve anchors stay green.

**Files:** `sim/netcom_zen/propagation/sionna_backend.py` (env-gated import + grid
lookup); a precompute CLI (`pixi run -e sionna sionna-precompute <scene> <band>`);
a `pathloss: composite|sionna` selector on the run config; `validation/` cross-check
cases.

**Exit:** an existing scenario runs end-to-end with the Sionna-sourced pathloss
grid (same metrics pipeline), plus a doc
(`docs/results/sionna-vs-analytical.md`) showing agreement in free-space/two-ray and
the divergence envelope in multipath/shadowed geometry — the evidence that the
ray-traced backend changes conclusions (or, honestly, that it doesn't for our
scenarios, in which case the analytical model stands vindicated).

**Risks / open questions:**
- **Precompute granularity:** a full pairwise grid is O(cells²); start with a
  per-tx coverage map (receiver grid per transmitter position) and interpolate,
  matching how Sionna's coverage-map API is shaped. Bound the grid so precompute is
  minutes, not hours.
- **Static vs mobile geometry:** the cached grid assumes fixed scene geometry.
  Vehicles moving through it is fine (lookup by position); vehicles *as reflectors*
  is not captured — state that limitation, same class of caveat as the analytical
  model's.
- **Env conflicts:** Sionna's TF/Mitsuba/CUDA pins will conflict with the
  conda-solved default env exactly like isaacsim did → standalone env, no default
  feature.
