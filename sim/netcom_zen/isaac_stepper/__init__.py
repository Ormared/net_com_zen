"""Lockstep mobility stepper (plan R4): serves the isaac_mobility protocol
over a unix socket. Backends: isaac (headless SimulationApp, isaac pixi env)
or kinematic (pure waypoint math, any env — used by tests and dry runs)."""
