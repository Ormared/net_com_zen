import math

from netcom_zen.mobility import Pose, WaypointMobility, WaypointVehicle


def test_moves_toward_waypoint():
    v = WaypointVehicle(waypoints=[(0, 0), (100, 0)], speed_mps=10)
    p = v.step(1.0)
    assert isinstance(p, Pose)
    assert p.x == 10.0 and abs(p.y) < 1e-9


def test_turn_rate_limited():
    v = WaypointVehicle(waypoints=[(0, 0), (100, 0)], speed_mps=10,
                        max_turn_rate=math.radians(30))
    v.heading = math.pi / 2  # facing +y, target is +x
    v.step(0.1)
    assert abs(v.heading - (math.pi / 2 - math.radians(3))) < 1e-9


def test_waypoint_mobility_aggregates():
    class N:
        def __init__(self, id, waypoints, speed_mps):
            self.id, self.waypoints, self.speed_mps = id, waypoints, speed_mps

    m = WaypointMobility([N("a", [(0, 0), (100, 0)], 10.0),
                          N("b", [(5, 5)], 0.1)])
    p0 = m.poses()
    assert p0["a"].x == 0.0 and p0["b"].x == 5.0  # poses() doesn't advance
    p1 = m.step(1.0)
    assert p1["a"].x == 10.0
    assert m.poses()["a"].x == 10.0  # poses() reflects the last step


def test_loops_waypoints():
    # turning radius = speed / max_turn_rate ~ 9.5 m; vehicle must stay within
    # the waypoint span plus its turning circle, never running away
    v = WaypointVehicle(waypoints=[(0, 0), (10, 0)], speed_mps=10)
    r_turn = v.speed / v.max_turn
    poses = [v.step(0.1) for _ in range(200)]
    assert all(-2 * r_turn - 2 < p.x < 10 + 2 * r_turn + 2 for p in poses)
    assert all(abs(p.y) < 2 * r_turn + 2 for p in poses)
