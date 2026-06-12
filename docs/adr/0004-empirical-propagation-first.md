# ADR-0004: Empirical propagation models first; Sionna RT as optional backend

**Status:** accepted
**Date:** 2026-06-12

## Context

Sionna RT is a GPU ray tracer computing path loss/CIR from explicit 3D scene
geometry. Forests — a primary operating environment — are volumetric scatterers,
hostile to ray optics: published Sionna foliage work uses hand-modeled tree meshes
calibrated mainly at mmWave, while this system likely operates at UHF/low-SHF for
penetration. Benchmarking also wants *parameterized environment classes* (forest
density, terrain roughness) to sweep statistically, not one specific 3D scene.
Empirical models (Weissberger, ITU-R P.833, two-ray, knife-edge/ITM) are the
established standard at these bands and cost microseconds.

## Decision

Ship empirical models first (see models.md), behind the `PathlossProvider`
interface. Sionna RT — or recorded real-world traces — can be added later as
alternative providers for site-specific validation of a real deployment area.

## Consequences

- (+) No GPU/scene-authoring dependency; environment parameters are sweepable.
- (+) Models validated against published curves in `validation/`.
- (−) No site-specific multipath fidelity until/unless an RT backend is added.
