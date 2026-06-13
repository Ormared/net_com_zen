"""Isaac Sim 6 stepper backend (plan R4): flat ground + one dynamic cuboid
per vehicle, velocity-controlled along the scenario waypoints by the same
unicycle law as WaypointVehicle — but the controller runs on PhysX ground
truth, so positions are Isaac-integrated, not seed-exact.

What physics is actually live (the channel only consumes XY poses, so the scene
is deliberately minimal — "poses are all the channel needs", plan R4):
  - Gravity + ground contact: real. Each cuboid is a rigid body with mass; the
    GroundPlane collider is an infinite plane, so bodies rest on it everywhere
    (verified: a body at (900,900), far off the visual mesh, settles at z=0.5).
  - Vehicle-vehicle collision: effectively suppressed. step() hard-sets each
    body's XY velocity from its controller every frame, so the commanded motion
    overrides any contact response in the plane — crossing vehicles pass
    through rather than collide. Only Z is left to physics. This is intentional
    (vehicles are placeholders for poses); richer dynamics / terrain are
    deferred (plan R4: "terrain mesh from the existing DEM later").

Import only after SimulationApp is running (isaacsim.core needs the app);
__main__ owns that ordering.
"""
from __future__ import annotations

import math

import numpy as np
from isaacsim.core.api import World
from isaacsim.core.api.objects import DynamicCuboid
from isaacsim.core.api.objects.ground_plane import GroundPlane

from ..mobility import WaypointVehicle

VEHICLE_Z = 0.5  # half the cuboid height: spawn resting on the plane


class IsaacBackend:
    def __init__(self, physics_dt: float, vehicles: dict,
                 extent: tuple[float, float] | None = None,
                 render: bool = False):
        self.dt = physics_dt
        self.render = render  # GUI viewport (--gui); headless runs skip it
        self.world = World(physics_dt=physics_dt, rendering_dt=physics_dt,
                           stage_units_in_meters=1.0)
        self._add_ground(extent)
        self._ctl: dict[str, WaypointVehicle] = {}
        self._body: dict[str, DynamicCuboid] = {}
        for nid, cfg in vehicles.items():
            ctl = WaypointVehicle(cfg["waypoints"], cfg["speed_mps"])
            self._ctl[nid] = ctl
            self._body[nid] = self.world.scene.add(DynamicCuboid(
                prim_path=f"/World/veh_{nid}", name=f"veh_{nid}",
                position=np.array([ctl.x, ctl.y, VEHICLE_Z]),
                scale=np.array([2.0, 1.0, 1.0]), mass=10.0))
        self.world.reset()
        self._warmup()

    def _add_ground(self, extent: tuple[float, float] | None) -> None:
        # GroundPlane's collider is an infinite plane, but its visual mesh
        # defaults to a 100 m square at the origin. Scenario vehicles run in
        # [0, extent], so with the default they appear to drive off the world
        # into free space (they don't -- the infinite collider still holds
        # them, verified: a body at (900,900) rests at z=0.5). Size the visual
        # large enough to cover the whole scenario. It stays origin-centered:
        # GroundPlane's mesh doesn't honor an xform translate (geometry is in
        # absolute coords), so the field reaches +/-1.2*max(extent), which
        # contains [0, extent] with margin to spare.
        m = max(float(extent[0]), float(extent[1])) if extent else 1000.0
        self.world.scene.add(GroundPlane(prim_path="/World/ground",
                                         size=m * 1.2))

    def _warmup(self, frames: int = 4) -> None:
        # the first world.step() JIT-compiles warp kernels (~3.5 s on the test
        # host). Pay it here during scene build -- which the orchestrator does
        # before t=0 -- not on the first live tick, where it would stall the
        # channel. Snapshot and restore spawn state so poses() stays exact.
        spawn = {nid: b.get_world_pose() for nid, b in self._body.items()}
        for _ in range(frames):
            self.world.step(render=False)
        for nid, b in self._body.items():
            pos, orient = spawn[nid]
            b.set_world_pose(position=pos, orientation=orient)
            b.set_linear_velocity(np.zeros(3))
            b.set_angular_velocity(np.zeros(3))

    def poses(self) -> dict[str, list[float]]:
        out = {}
        for nid, body in self._body.items():
            pos, _ = body.get_world_pose()
            # heading is the controller's (orientation is not torque-driven);
            # position is PhysX's — the channel only consumes positions
            out[nid] = [float(pos[0]), float(pos[1]), self._ctl[nid].heading]
        return out

    def step(self, frames: int) -> dict[str, list[float]]:
        for _ in range(frames):
            for nid, body in self._body.items():
                ctl = self._ctl[nid]
                pos, _ = body.get_world_pose()
                # controller state tracks PhysX truth; its own integration
                # is a one-frame prediction overwritten here
                ctl.x, ctl.y = float(pos[0]), float(pos[1])
                ctl.step(self.dt)
                vz = float(body.get_linear_velocity()[2])  # gravity settles z
                body.set_linear_velocity(np.array([
                    ctl.speed * math.cos(ctl.heading),
                    ctl.speed * math.sin(ctl.heading), vz]))
            self.world.step(render=self.render)
        return self.poses()

    def close(self) -> None:
        # full teardown: the next init builds a fresh World (sweep reuse)
        World.clear_instance()
