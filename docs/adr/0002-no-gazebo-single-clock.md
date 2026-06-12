# ADR-0002: No physics co-simulator; single-clock Python mobility

**Status:** accepted
**Date:** 2026-06-12

## Context

SynchroSim-style co-simulation (Gazebo + network simulator) requires synchronizing
two clocks; the SynchroSim paper's contribution is exactly that sync algorithm, and
the ~10–11 % packet-loss/delay fidelity improvement it reports is the magnitude of
error clock skew injects. Gazebo's value (contact physics, sensors, articulated
bodies) does not feed any metric we measure — link budgets need position/velocity/
heading at ~10 Hz over a heightmap.

## Decision

Mobility is a bicycle-kinematics waypoint follower inside the scenario engine: one
process, one clock, deterministic and seedable. It sits behind a `MobilityProvider`
interface; Gazebo/ROS 2 can replace it later (e.g. for autonomy-in-the-loop) without
touching propagation, channel, or harness code.

## Consequences

- (+) The entire clock-sync problem class vanishes; runs are reproducible.
- (+) GB-scale dependency, SDF authoring, plugin churn, and headless-CI pain avoided.
- (−) No dynamic effects (slip, suspension) on trajectories — irrelevant to link
  budgets at these update rates.
