# M1 Channel Core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and validate the channel core of net_com_zen: mobility, propagation, FHSS/jammer models, the netns dataplane, the per-packet channel forwarder, and the model validation suite (roadmap M1).

**Architecture:** A single Python scenario engine (one clock, 10 Hz ticks) computes per-link state (SINR/PER/delay) from vehicle positions, terrain/foliage, and jammers. Real traffic flows between per-node Linux network namespaces through a userspace AF_PACKET forwarder that gives every frame a seeded, logged verdict. See `docs/architecture.md` and `docs/models.md` — they are the spec; this plan implements them.

**Tech Stack:** Python 3.11+ (pixi), numpy, pydantic v2, pyyaml, pyarrow, pytest, eclipse-zenoh (smoke test only). Rust agent is M2 — not in this plan.

**Conventions:** positions in meters (local ENU, x/y), frequencies Hz, power dBm, loss dB, time seconds. Tests requiring root are marked `@pytest.mark.sudo` and auto-skip when not root.

---

### Task 1: Project scaffolding

**Files:**
- Create: `pixi.toml`, `pyproject.toml`, `sim/netcom_zen/__init__.py`, `tests/__init__.py`, `tests/conftest.py`, `tests/test_package.py`, `.gitignore`

- [ ] **Step 1: Create branch**

```bash
git checkout -b m1-channel-core
```

- [ ] **Step 2: Write `pixi.toml`**

```toml
[project]
name = "net_com_zen"
channels = ["conda-forge"]
platforms = ["linux-64"]

[dependencies]
python = ">=3.11,<3.13"
numpy = ">=1.26"
pydantic = ">=2.7"
pyyaml = ">=6"
pytest = ">=8"
pyarrow = ">=16"
pip = "*"

[pypi-dependencies]
netcom-zen = { path = ".", editable = true }
eclipse-zenoh = ">=1.0"

[tasks]
test = "pytest tests -v"
sudo-test = "sudo $(which python) -m pytest tests -v -m sudo"
validate = "pytest validation -v"
setup-check = "python -m netcom_zen.setup_check"
```

- [ ] **Step 3: Write `pyproject.toml`**

```toml
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[project]
name = "netcom-zen"
version = "0.1.0"
description = "EW-resilient swarm comms simulation environment"
requires-python = ">=3.11"

[tool.hatch.build.targets.wheel]
packages = ["sim/netcom_zen"]

[tool.pytest.ini_options]
markers = ["sudo: requires root/CAP_NET_ADMIN; run via `pixi run sudo-test`"]
```

- [ ] **Step 4: Write package skeleton and `.gitignore`**

`sim/netcom_zen/__init__.py`:
```python
__version__ = "0.1.0"
```

`tests/__init__.py`: empty file.

`tests/conftest.py`:
```python
import os

import pytest


def pytest_collection_modifyitems(config, items):
    if os.geteuid() == 0:
        return
    skip = pytest.mark.skip(reason="requires root; run `pixi run sudo-test`")
    for item in items:
        if "sudo" in item.keywords:
            item.add_marker(skip)
```

`.gitignore`:
```
.pixi/
__pycache__/
*.egg-info/
results/
validation/report.md
```

`tests/test_package.py`:
```python
import netcom_zen


def test_version():
    assert netcom_zen.__version__ == "0.1.0"
```

- [ ] **Step 5: Install env and run test**

Run: `pixi install && pixi run test`
Expected: 1 passed.

- [ ] **Step 6: Commit**

```bash
git add pixi.toml pixi.lock pyproject.toml sim/ tests/ .gitignore
git commit -m "chore: pixi + package scaffolding"
```
(`pixi.lock` is committed: it pins the environment for reproducible benchmark runs.)

---

### Task 2: Scenario config schema

**Files:**
- Create: `sim/netcom_zen/config.py`
- Test: `tests/test_config.py`

- [ ] **Step 1: Write the failing test**

`tests/test_config.py`:
```python
import pytest
from pydantic import ValidationError

from netcom_zen.config import Scenario, load_scenario

MINIMAL = """
name: t
duration_s: 5
seed: 42
radio:
  freq_hz: 433.0e6
  bandwidth_hz: 250.0e3
  tx_power_dbm: 27.0
  data_rate_bps: 250.0e3
  hop: {n_channels: 50, hop_rate_hz: 100.0}
nodes:
  - {id: v1, waypoints: [[100, 500]]}
  - {id: v2, waypoints: [[200, 500]]}
"""


def test_minimal_scenario_parses(tmp_path):
    p = tmp_path / "s.yaml"
    p.write_text(MINIMAL)
    s = load_scenario(p)
    assert s.tick_hz == 10.0 and s.radio.noise_figure_db == 7.0
    assert s.nodes[0].speed_mps == 5.0


def test_tx_power_swap_cap():
    with pytest.raises(ValidationError):
        Scenario.model_validate(
            {"name": "t", "duration_s": 1,
             "radio": {"freq_hz": 1, "bandwidth_hz": 1, "tx_power_dbm": 40,
                       "data_rate_bps": 1, "hop": {"n_channels": 2, "hop_rate_hz": 1}},
             "nodes": [{"id": "a", "waypoints": [[0, 0]]},
                       {"id": "b", "waypoints": [[1, 1]]}]})


def test_duplicate_node_ids_rejected(tmp_path):
    p = tmp_path / "s.yaml"
    p.write_text(MINIMAL.replace("id: v2", "id: v1"))
    with pytest.raises(ValidationError):
        load_scenario(p)
```

- [ ] **Step 2: Run test, verify it fails**

Run: `pixi run test`
Expected: FAIL with `ModuleNotFoundError: No module named 'netcom_zen.config'`

- [ ] **Step 3: Implement `sim/netcom_zen/config.py`**

```python
from __future__ import annotations

from pathlib import Path
from typing import Literal

import yaml
from pydantic import BaseModel, Field, model_validator


class HopConfig(BaseModel):
    n_channels: int = Field(gt=1)
    hop_rate_hz: float = Field(gt=0)


class RadioProfile(BaseModel):
    freq_hz: float = Field(gt=0)
    bandwidth_hz: float = Field(gt=0)  # per hop channel
    tx_power_dbm: float = Field(le=30.0)  # SWaP cap: 1 W (README)
    antenna_gain_dbi: float = 0.0
    noise_figure_db: float = 7.0
    data_rate_bps: float = Field(gt=0)
    hop: HopConfig


class NodeConfig(BaseModel):
    id: str
    waypoints: list[tuple[float, float]] = Field(min_length=1)
    speed_mps: float = Field(gt=0, default=5.0)


class FoliageRect(BaseModel):
    x_min: float
    x_max: float
    y_min: float
    y_max: float

    @model_validator(mode="after")
    def _ordered(self):
        if self.x_min >= self.x_max or self.y_min >= self.y_max:
            raise ValueError("foliage rect min must be < max")
        return self


class EnvironmentConfig(BaseModel):
    heightmap: Path | None = None  # .npy of heights (m), row=y, col=x
    extent_m: tuple[float, float] = (1000.0, 1000.0)
    foliage: list[FoliageRect] = []


class JammerConfig(BaseModel):
    id: str
    kind: Literal["barrage", "spot", "sweep"]
    position: tuple[float, float]
    tx_power_dbm: float
    start_s: float = 0.0
    stop_s: float | None = None
    channels: list[int] = []          # spot: jammed hop-channel indices
    bandwidth_hz: float | None = None  # barrage: total jammed bandwidth

    @model_validator(mode="after")
    def _kind_params(self):
        if self.kind == "spot" and not self.channels:
            raise ValueError("spot jammer requires channels")
        if self.kind == "barrage" and not self.bandwidth_hz:
            raise ValueError("barrage jammer requires bandwidth_hz")
        return self


class Scenario(BaseModel):
    name: str
    duration_s: float = Field(gt=0)
    tick_hz: float = 10.0
    seed: int = 0
    radio: RadioProfile
    nodes: list[NodeConfig] = Field(min_length=2, max_length=8)
    environment: EnvironmentConfig = EnvironmentConfig()
    jammers: list[JammerConfig] = []

    @model_validator(mode="after")
    def _unique_ids(self):
        ids = [n.id for n in self.nodes]
        if len(set(ids)) != len(ids):
            raise ValueError("duplicate node ids")
        return self


def load_scenario(path: str | Path) -> Scenario:
    return Scenario.model_validate(yaml.safe_load(Path(path).read_text()))
```

- [ ] **Step 4: Run tests, verify pass**

Run: `pixi run test`
Expected: all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/config.py tests/test_config.py
git commit -m "feat: pydantic scenario schema with fail-fast validation"
```

---

### Task 3: Terrain (heightmap, profile, foliage depth)

**Files:**
- Create: `sim/netcom_zen/terrain.py`
- Test: `tests/test_terrain.py`

- [ ] **Step 1: Write the failing test**

`tests/test_terrain.py`:
```python
import numpy as np
import pytest

from netcom_zen.terrain import FoliageRegion, Terrain


def test_flat_terrain_height_zero():
    t = Terrain(extent_m=(1000, 1000))
    assert t.height(500, 500) == 0.0


def test_heightmap_lookup():
    hm = np.array([[0.0, 0.0], [0.0, 50.0]])  # row=y, col=x
    t = Terrain(extent_m=(100, 100), heightmap=hm)
    assert t.height(99, 99) == 50.0
    assert t.height(0, 0) == 0.0


def test_profile_endpoints():
    t = Terrain(extent_m=(1000, 1000))
    prof = t.profile((0, 0), (1000, 1000), n=16)
    assert len(prof) == 16 and prof[0] == 0.0


def test_foliage_depth_full_crossing():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=400, x_max=600, y_min=0, y_max=1000)])
    assert t.foliage_depth((0, 500), (1000, 500)) == pytest.approx(200.0)


def test_foliage_depth_miss():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=400, x_max=600, y_min=0, y_max=100)])
    assert t.foliage_depth((0, 500), (1000, 500)) == 0.0
```

- [ ] **Step 2: Run test, verify it fails**

Run: `pixi run test`
Expected: FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/terrain.py`**

```python
from __future__ import annotations

import math
from dataclasses import dataclass

import numpy as np


@dataclass(frozen=True)
class FoliageRegion:
    x_min: float
    x_max: float
    y_min: float
    y_max: float


class Terrain:
    """Heightmap + land-cover. Flat ground (z=0) when no heightmap given."""

    def __init__(self, extent_m: tuple[float, float],
                 heightmap: np.ndarray | None = None,
                 foliage: list[FoliageRegion] | None = None):
        self.extent = extent_m
        self.heightmap = heightmap
        self.foliage = list(foliage or [])

    def height(self, x: float, y: float) -> float:
        if self.heightmap is None:
            return 0.0
        h, w = self.heightmap.shape
        i = min(max(int(round(y / self.extent[1] * (h - 1))), 0), h - 1)
        j = min(max(int(round(x / self.extent[0] * (w - 1))), 0), w - 1)
        return float(self.heightmap[i, j])

    def profile(self, p1: tuple[float, float], p2: tuple[float, float],
                n: int = 64) -> np.ndarray:
        xs = np.linspace(p1[0], p2[0], n)
        ys = np.linspace(p1[1], p2[1], n)
        return np.array([self.height(x, y) for x, y in zip(xs, ys)])

    def foliage_depth(self, p1: tuple[float, float], p2: tuple[float, float]) -> float:
        """Total length of segment p1->p2 inside foliage regions (Liang-Barsky)."""
        dx, dy = p2[0] - p1[0], p2[1] - p1[1]
        seg = math.hypot(dx, dy)
        if seg == 0:
            return 0.0
        total = 0.0
        for r in self.foliage:
            t0, t1 = 0.0, 1.0
            inside = True
            for p, q in ((-dx, p1[0] - r.x_min), (dx, r.x_max - p1[0]),
                         (-dy, p1[1] - r.y_min), (dy, r.y_max - p1[1])):
                if p == 0:
                    if q < 0:
                        inside = False
                        break
                else:
                    t = q / p
                    if p < 0:
                        t0 = max(t0, t)
                    else:
                        t1 = min(t1, t)
            if inside and t0 < t1:
                total += (t1 - t0) * seg
        return total
```

- [ ] **Step 4: Run tests, verify pass**

Run: `pixi run test` — Expected: all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/terrain.py tests/test_terrain.py
git commit -m "feat: terrain heightmap, path profile, foliage depth"
```

---

### Task 4: Mobility (waypoint vehicle)

**Files:**
- Create: `sim/netcom_zen/mobility.py`
- Test: `tests/test_mobility.py`

- [ ] **Step 1: Write the failing test**

`tests/test_mobility.py`:
```python
import math

from netcom_zen.mobility import Pose, WaypointVehicle


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


def test_loops_waypoints():
    v = WaypointVehicle(waypoints=[(0, 0), (10, 0)], speed_mps=10)
    xs = [v.step(0.1).x for _ in range(100)]
    assert min(xs) > -5 and max(xs) < 15  # stays around the loop
```

- [ ] **Step 2: Run test, verify it fails**

Run: `pixi run test` — Expected: FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/mobility.py`**

```python
from __future__ import annotations

import math
from dataclasses import dataclass


@dataclass
class Pose:
    x: float
    y: float
    heading: float  # radians, CCW from +x


class WaypointVehicle:
    """Turn-rate-limited unicycle following waypoints in a loop.

    MobilityProvider seam (ADR-0002): anything with step(dt) -> Pose fits.
    """

    ARRIVE_M = 2.0

    def __init__(self, waypoints, speed_mps: float = 5.0,
                 max_turn_rate: float = math.radians(60)):
        self.waypoints = [tuple(map(float, w)) for w in waypoints]
        self.speed = speed_mps
        self.max_turn = max_turn_rate
        self.x, self.y = self.waypoints[0]
        self._target = 1 % len(self.waypoints)
        self.heading = self._bearing_to(self.waypoints[self._target])

    def _bearing_to(self, wp) -> float:
        return math.atan2(wp[1] - self.y, wp[0] - self.x)

    def step(self, dt: float) -> Pose:
        wp = self.waypoints[self._target]
        if math.hypot(wp[0] - self.x, wp[1] - self.y) < self.ARRIVE_M:
            self._target = (self._target + 1) % len(self.waypoints)
            wp = self.waypoints[self._target]
        err = (self._bearing_to(wp) - self.heading + math.pi) % (2 * math.pi) - math.pi
        lim = self.max_turn * dt
        self.heading += max(-lim, min(lim, err))
        self.x += self.speed * dt * math.cos(self.heading)
        self.y += self.speed * dt * math.sin(self.heading)
        return Pose(self.x, self.y, self.heading)
```

- [ ] **Step 4: Run tests, verify pass** — `pixi run test`, all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/mobility.py tests/test_mobility.py
git commit -m "feat: waypoint unicycle mobility behind MobilityProvider seam"
```

---

### Task 5: Propagation baseline (Friis, two-ray)

**Files:**
- Create: `sim/netcom_zen/propagation/__init__.py` (empty for now), `sim/netcom_zen/propagation/freespace.py`
- Test: `tests/test_freespace.py`

- [ ] **Step 1: Write the failing test**

`tests/test_freespace.py`:
```python
import pytest

from netcom_zen.propagation.freespace import baseline_db, crossover_m, friis_db, two_ray_db


def test_friis_known_value():
    # 1 km @ 2.4 GHz: 20log10(1000)+20log10(2.4e9)-147.55 = 100.05 dB
    assert friis_db(1000, 2.4e9) == pytest.approx(100.05, abs=0.02)


def test_friis_monotonic_and_guarded():
    assert friis_db(0.1, 2.4e9) == friis_db(1.0, 2.4e9)  # d clamped to 1 m
    assert friis_db(2000, 2.4e9) > friis_db(1000, 2.4e9)


def test_two_ray_known_value():
    # 40log10(1000) - 20log10(1.5*1.5) = 120 - 7.04 = 112.96 dB
    assert two_ray_db(1000, 1.5, 1.5) == pytest.approx(112.96, abs=0.02)


def test_baseline_switches_at_crossover():
    f, h = 2.4e9, 1.5
    dc = crossover_m(f, h, h)
    assert baseline_db(dc * 0.5, f, h, h) == friis_db(dc * 0.5, f)
    assert baseline_db(dc * 2.0, f, h, h) == two_ray_db(dc * 2.0, h, h)
```

- [ ] **Step 2: Run test, verify it fails** — `pixi run test`, FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/propagation/freespace.py`**

```python
import math

C = 299_792_458.0


def friis_db(d_m: float, f_hz: float) -> float:
    d = max(d_m, 1.0)
    return 20 * math.log10(d) + 20 * math.log10(f_hz) - 147.55


def two_ray_db(d_m: float, h_tx_m: float, h_rx_m: float) -> float:
    d = max(d_m, 1.0)
    return 40 * math.log10(d) - 20 * math.log10(h_tx_m * h_rx_m)


def crossover_m(f_hz: float, h_tx_m: float, h_rx_m: float) -> float:
    return 4 * math.pi * h_tx_m * h_rx_m * f_hz / C


def baseline_db(d_m: float, f_hz: float, h_tx_m: float = 1.5, h_rx_m: float = 1.5) -> float:
    if d_m < crossover_m(f_hz, h_tx_m, h_rx_m):
        return friis_db(d_m, f_hz)
    return two_ray_db(d_m, h_tx_m, h_rx_m)
```

Also create empty `sim/netcom_zen/propagation/__init__.py`.

- [ ] **Step 4: Run tests, verify pass** — `pixi run test`, all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/propagation/ tests/test_freespace.py
git commit -m "feat: Friis + two-ray baseline pathloss"
```

---

### Task 6: Foliage loss (Weissberger)

**Files:**
- Create: `sim/netcom_zen/propagation/foliage.py`
- Test: `tests/test_foliage.py`

- [ ] **Step 1: Write the failing test**

`tests/test_foliage.py`:
```python
import pytest

from netcom_zen.propagation.foliage import weissberger_db


def test_zero_depth():
    assert weissberger_db(900e6, 0.0) == 0.0


def test_short_depth_regime():
    # d<=14 m: 0.45 * f_GHz^0.284 * d ; 0.45*0.9^0.284*10 = 4.368 dB
    assert weissberger_db(900e6, 10.0) == pytest.approx(4.368, abs=0.01)


def test_long_depth_regime():
    # 14<d<=400: 1.33 * f_GHz^0.284 * d^0.588 ; @0.9GHz,100m = 19.36 dB
    assert weissberger_db(900e6, 100.0) == pytest.approx(19.36, abs=0.05)


def test_depth_clamped_at_400m():
    assert weissberger_db(900e6, 1000.0) == weissberger_db(900e6, 400.0)
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/propagation/foliage.py`**

```python
def weissberger_db(f_hz: float, depth_m: float) -> float:
    """Weissberger Modified Exponential Decay model (foliage depth <= 400 m)."""
    if depth_m <= 0:
        return 0.0
    f_ghz = f_hz / 1e9
    d = min(depth_m, 400.0)
    if d <= 14.0:
        return 0.45 * f_ghz**0.284 * d
    return 1.33 * f_ghz**0.284 * d**0.588
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/propagation/foliage.py tests/test_foliage.py
git commit -m "feat: Weissberger foliage excess loss"
```

---

### Task 7: Terrain diffraction (single knife-edge)

**Files:**
- Create: `sim/netcom_zen/propagation/diffraction.py`
- Test: `tests/test_diffraction.py`

- [ ] **Step 1: Write the failing test**

`tests/test_diffraction.py`:
```python
import numpy as np
import pytest

from netcom_zen.propagation.diffraction import knife_edge_loss_db


def test_grazing_edge_six_db():
    # obstacle exactly at LOS height (v=0) -> J(0) = 6.03 dB (classic result)
    prof = np.array([0.0, 1.5, 0.0])
    loss = knife_edge_loss_db(prof, d_m=1000, h_tx=1.5, h_rx=1.5, f_hz=433e6)
    assert loss == pytest.approx(6.03, abs=0.02)


def test_deep_shadow_large_loss():
    prof = np.array([0.0, 30.0, 0.0])
    loss = knife_edge_loss_db(prof, d_m=1000, h_tx=1.5, h_rx=1.5, f_hz=433e6)
    assert loss > 20.0


def test_clear_path_small_loss():
    prof = np.zeros(16)
    loss = knife_edge_loss_db(prof, d_m=1000, h_tx=1.5, h_rx=1.5, f_hz=433e6)
    assert 0.0 <= loss < 7.0  # near-grazing flat earth gives a few dB


def test_degenerate_inputs():
    assert knife_edge_loss_db(np.zeros(2), 100, 1.5, 1.5, 433e6) == 0.0
    assert knife_edge_loss_db(np.zeros(16), 0.0, 1.5, 1.5, 433e6) == 0.0
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/propagation/diffraction.py`**

```python
import math

import numpy as np

from .freespace import C


def knife_edge_loss_db(profile: np.ndarray, d_m: float,
                       h_tx: float, h_rx: float, f_hz: float) -> float:
    """Single dominant knife-edge (ITU-R P.526 approximation J(v)).

    profile: terrain heights along the path, endpoints included.
    Antennas sit h_tx/h_rx above the endpoint terrain heights.
    """
    n = len(profile)
    if n < 3 or d_m <= 0:
        return 0.0
    lam = C / f_hz
    z_tx = float(profile[0]) + h_tx
    z_rx = float(profile[-1]) + h_rx
    v_max = -math.inf
    for i in range(1, n - 1):
        d1 = d_m * i / (n - 1)
        d2 = d_m - d1
        los = z_tx + (z_rx - z_tx) * i / (n - 1)
        h = float(profile[i]) - los
        v_max = max(v_max, h * math.sqrt(2 * d_m / (lam * d1 * d2)))
    if v_max <= -0.78:
        return 0.0
    return 6.9 + 20 * math.log10(math.sqrt((v_max - 0.1) ** 2 + 1) + v_max - 0.1)
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/propagation/diffraction.py tests/test_diffraction.py
git commit -m "feat: single knife-edge terrain diffraction (P.526 J(v))"
```

---

### Task 8: Composite pathloss provider

**Files:**
- Modify: `sim/netcom_zen/propagation/__init__.py`
- Test: `tests/test_composite_pathloss.py`

- [ ] **Step 1: Write the failing test**

`tests/test_composite_pathloss.py`:
```python
import pytest

from netcom_zen.propagation import CompositePathloss, PathlossBreakdown
from netcom_zen.propagation.foliage import weissberger_db
from netcom_zen.propagation.freespace import baseline_db
from netcom_zen.terrain import FoliageRegion, Terrain


def test_open_field_equals_baseline():
    pl = CompositePathloss(Terrain(extent_m=(1000, 1000)))
    bd = pl.loss((0, 500), (300, 500), 433e6)
    assert isinstance(bd, PathlossBreakdown)
    assert bd.foliage_db == 0.0
    assert bd.fspl_db == pytest.approx(baseline_db(300, 433e6), abs=0.01)
    assert bd.total_db == bd.fspl_db + bd.terrain_db + bd.foliage_db


def test_forest_adds_weissberger():
    t = Terrain(extent_m=(1000, 1000),
                foliage=[FoliageRegion(x_min=100, x_max=200, y_min=0, y_max=1000)])
    pl = CompositePathloss(t)
    bd = pl.loss((0, 500), (300, 500), 433e6)
    assert bd.foliage_db == pytest.approx(weissberger_db(433e6, 100.0), abs=0.01)
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ImportError`

- [ ] **Step 3: Implement in `sim/netcom_zen/propagation/__init__.py`**

```python
from __future__ import annotations

import math
from dataclasses import dataclass

from ..terrain import Terrain
from .diffraction import knife_edge_loss_db
from .foliage import weissberger_db
from .freespace import baseline_db


@dataclass(frozen=True)
class PathlossBreakdown:
    fspl_db: float
    terrain_db: float
    foliage_db: float

    @property
    def total_db(self) -> float:
        return self.fspl_db + self.terrain_db + self.foliage_db


class CompositePathloss:
    """PathlossProvider: (tx_xy, rx_xy, freq_hz) -> breakdown. EMANE-shaped (ADR-0001)."""

    def __init__(self, terrain: Terrain, antenna_height_m: float = 1.5):
        self.terrain = terrain
        self.h_ant = antenna_height_m

    def loss(self, tx: tuple[float, float], rx: tuple[float, float],
             f_hz: float) -> PathlossBreakdown:
        d = math.hypot(rx[0] - tx[0], rx[1] - tx[1])
        fspl = baseline_db(d, f_hz, self.h_ant, self.h_ant)
        prof = self.terrain.profile(tx, rx)
        terr = knife_edge_loss_db(prof, d, self.h_ant, self.h_ant, f_hz)
        fol = weissberger_db(f_hz, self.terrain.foliage_depth(tx, rx))
        return PathlossBreakdown(fspl, terr, fol)
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/propagation/__init__.py tests/test_composite_pathloss.py
git commit -m "feat: composite pathloss with per-component breakdown"
```

---

### Task 9: Link budget (noise, SINR, FSK PER waterfall)

**Files:**
- Create: `sim/netcom_zen/channel/__init__.py` (empty), `sim/netcom_zen/channel/linkbudget.py`
- Test: `tests/test_linkbudget.py`

- [ ] **Step 1: Write the failing test**

`tests/test_linkbudget.py`:
```python
import pytest

from netcom_zen.channel.linkbudget import (fsk_per, noise_dbm, per_threshold_sinr_db,
                                           sinr_db)


def test_noise_floor():
    # -174 + 10log10(250e3) + 7 = -113.02 dBm
    assert noise_dbm(250e3, 7.0) == pytest.approx(-113.02, abs=0.02)


def test_sinr_no_interference():
    assert sinr_db(-90, -113) == pytest.approx(23.0, abs=0.01)


def test_sinr_with_equal_interferer():
    # interferer equal to noise halves the denominator margin: -3.01 dB shift
    assert sinr_db(-90, -113, [-113]) == pytest.approx(19.99, abs=0.02)


def test_fsk_per_known_value():
    # SINR 10 dB, 32-byte packet: BER=0.5*exp(-5)=3.369e-3, PER=1-(1-BER)^256=0.5785
    assert fsk_per(10.0, 32) == pytest.approx(0.5785, abs=1e-3)


def test_per_threshold_inverse():
    thr = per_threshold_sinr_db(32, target_per=0.5)
    assert fsk_per(thr, 32) == pytest.approx(0.5, abs=1e-6)
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/channel/linkbudget.py`** (and empty `channel/__init__.py`)

```python
import math


def noise_dbm(bandwidth_hz: float, noise_figure_db: float) -> float:
    return -174.0 + 10 * math.log10(bandwidth_hz) + noise_figure_db


def dbm_to_mw(dbm: float) -> float:
    return 10 ** (dbm / 10)


def mw_to_dbm(mw: float) -> float:
    return 10 * math.log10(mw)


def sinr_db(prx_dbm: float, noise_dbm_val: float,
            interferer_dbm: list[float] = ()) -> float:
    denom = dbm_to_mw(noise_dbm_val) + sum(dbm_to_mw(i) for i in interferer_dbm)
    return prx_dbm - mw_to_dbm(denom)


def fsk_per(sinr_db_val: float, length_bytes: int) -> float:
    """Noncoherent BFSK: BER = 0.5*exp(-SNR/2); PER over 8L bits (models.md)."""
    snr = 10 ** (sinr_db_val / 10)
    ber = 0.5 * math.exp(-snr / 2)
    return 1.0 - (1.0 - ber) ** (8 * length_bytes)


def per_threshold_sinr_db(length_bytes: int, target_per: float = 0.5) -> float:
    """SINR at which PER == target (closed-form inverse of fsk_per)."""
    ber = 1.0 - (1.0 - target_per) ** (1.0 / (8 * length_bytes))
    snr = -2.0 * math.log(2.0 * ber)
    return 10 * math.log10(snr)
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/channel/ tests/test_linkbudget.py
git commit -m "feat: link budget - noise, SINR aggregation, FSK PER waterfall"
```

---

### Task 10: FHSS hop-collision model

**Files:**
- Create: `sim/netcom_zen/channel/fhss.py`
- Test: `tests/test_fhss.py`, `validation/test_fhss_analytical.py`, `validation/__init__.py` (empty)

- [ ] **Step 1: Write the failing tests**

`tests/test_fhss.py`:
```python
import pytest

from netcom_zen.channel.fhss import dwells_per_packet, packet_loss_prob


def test_dwells_at_least_one():
    # 32B @ 250kbps = 1.024 ms; 100 hop/s dwell = 10 ms -> 1 dwell
    assert dwells_per_packet(32, 250e3, 100) == 1


def test_dwells_fast_hopping():
    # 1.024 ms packet, 1 ms dwell -> 2 dwells
    assert dwells_per_packet(32, 250e3, 1000) == 2


def test_loss_prob_formula():
    # PER_total = 1 - (1-per_clear)*(1-rho*lost)^k
    assert packet_loss_prob(0.0, 0.2, 1.0, 3) == pytest.approx(1 - 0.8**3)
    assert packet_loss_prob(0.1, 0.0, 1.0, 5) == pytest.approx(0.1)
```

`validation/test_fhss_analytical.py` (Monte-Carlo cross-check — validation suite):
```python
import numpy as np
import pytest

from netcom_zen.channel.fhss import packet_loss_prob


def test_formula_matches_dwell_simulation():
    rng = np.random.default_rng(7)
    rho, k, trials = 0.15, 4, 200_000
    hits = rng.random((trials, k)) < rho
    sim = float(np.mean(hits.any(axis=1)))
    formula = packet_loss_prob(0.0, rho, 1.0, k)
    assert sim == pytest.approx(formula, abs=3 * np.sqrt(formula * (1 - formula) / trials))
```

- [ ] **Step 2: Run, verify both fail**

Run: `pixi run test && pixi run validate`
Expected: FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/channel/fhss.py`**

```python
import math


def dwells_per_packet(length_bytes: int, data_rate_bps: float,
                      hop_rate_hz: float) -> int:
    t_pkt = 8 * length_bytes / data_rate_bps
    return max(1, math.ceil(t_pkt * hop_rate_hz))


def packet_loss_prob(per_clear: float, rho: float,
                     dwell_lost_if_jammed: float, k: int) -> float:
    """models.md: PER_total = 1 - (1-PER_thermal) * (1-rho_eff)^k."""
    p_dwell_bad = rho * dwell_lost_if_jammed
    return 1.0 - (1.0 - per_clear) * (1.0 - p_dwell_bad) ** k
```

- [ ] **Step 4: Run, verify pass** — `pixi run test && pixi run validate`, all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/channel/fhss.py tests/test_fhss.py validation/
git commit -m "feat: statistical FHSS hop-collision model + Monte-Carlo validation"
```

---

### Task 11: Jammers (barrage, spot, sweep)

**Files:**
- Create: `sim/netcom_zen/ew.py`
- Test: `tests/test_ew.py`

- [ ] **Step 1: Write the failing test**

`tests/test_ew.py`:
```python
import pytest

from netcom_zen.config import JammerConfig
from netcom_zen.ew import Jammer


def mk(kind, **kw):
    base = {"id": "j1", "kind": kind, "position": (0, 0), "tx_power_dbm": 30}
    return Jammer(JammerConfig(**base, **kw))


def test_active_window():
    j = mk("spot", channels=[0, 1], start_s=10, stop_s=20)
    assert not j.active(5) and j.active(10) and j.active(19.9) and not j.active(20)


def test_spot_occupancy():
    j = mk("spot", channels=[0, 1, 2, 3])
    rho, frac = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == pytest.approx(4 / 50)
    assert frac == pytest.approx(1 / 4)  # power split across its 4 channels


def test_spot_ignores_out_of_band_channels():
    j = mk("spot", channels=[0, 999])
    rho, _ = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == pytest.approx(1 / 50)


def test_barrage_occupancy():
    j = mk("barrage", bandwidth_hz=12.5e6)
    rho, frac = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == 1.0
    assert frac == pytest.approx(250e3 / 12.5e6)  # in-channel share of psd


def test_sweep_occupancy():
    j = mk("sweep")
    rho, frac = j.occupancy(n_channels=50, channel_bw_hz=250e3)
    assert rho == pytest.approx(1 / 50) and frac == 1.0
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/ew.py`**

```python
from __future__ import annotations

from .config import JammerConfig


class Jammer:
    """Position-aware jammer. occupancy() returns (rho, in_channel_power_fraction):
    rho = fraction of hop channels affected at any instant;
    fraction = share of the jammer's TX power landing inside one affected channel.
    Sweep is modeled statistically as one uniformly-random jammed channel (models.md).
    """

    def __init__(self, cfg: JammerConfig):
        self.cfg = cfg
        self.position = cfg.position
        self.tx_power_dbm = cfg.tx_power_dbm

    def active(self, t: float) -> bool:
        return self.cfg.start_s <= t and (self.cfg.stop_s is None or t < self.cfg.stop_s)

    def occupancy(self, n_channels: int, channel_bw_hz: float) -> tuple[float, float]:
        c = self.cfg
        if c.kind == "spot":
            in_band = {ch for ch in c.channels if 0 <= ch < n_channels}
            return len(in_band) / n_channels, 1.0 / max(len(c.channels), 1)
        if c.kind == "barrage":
            return 1.0, min(1.0, channel_bw_hz / c.bandwidth_hz)
        # sweep
        return 1.0 / n_channels, 1.0
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/ew.py tests/test_ew.py
git commit -m "feat: barrage/spot/sweep jammer occupancy models"
```

---

### Task 12: Link-state table + per-packet verdicts + deterministic replay

**Files:**
- Create: `sim/netcom_zen/channel/linkstate.py`
- Test: `tests/test_linkstate.py`

- [ ] **Step 1: Write the failing test**

`tests/test_linkstate.py`:
```python
import numpy as np
import pytest

from netcom_zen.channel.linkstate import LinkState, build_table, link_rng
from netcom_zen.config import JammerConfig, RadioProfile
from netcom_zen.ew import Jammer
from netcom_zen.propagation import CompositePathloss
from netcom_zen.terrain import Terrain

RADIO = RadioProfile(freq_hz=433e6, bandwidth_hz=250e3, tx_power_dbm=27,
                     data_rate_bps=250e3,
                     hop={"n_channels": 50, "hop_rate_hz": 100})


def make_state(**kw):
    base = dict(src="a", dst="b", prx_dbm=-80, noise_dbm=-113, rho=0.0,
                jam_inchannel_dbm=None, data_rate_bps=250e3, hop_rate_hz=100,
                prop_delay_s=1e-6, foliage_db=0.0, terrain_db=0.0)
    base.update(kw)
    return LinkState(**base)


def test_strong_link_delivers():
    st = make_state()
    assert st.verdict(64, 0.999, 0.999) == "deliver"


def test_weak_link_drops_range():
    st = make_state(prx_dbm=-120)
    assert st.verdict(64, 0.5, 0.5) == "range"


def test_foliage_attribution():
    st = make_state(prx_dbm=-120, foliage_db=25.0)
    assert st.verdict(64, 0.5, 0.5) == "foliage"


def test_strong_jammer_drops_jam():
    st = make_state(rho=1.0, jam_inchannel_dbm=-60)
    assert st.verdict(64, 0.999, 0.5) == "jam"


def test_build_table_directed_pairs():
    pl = CompositePathloss(Terrain(extent_m=(1000, 1000)))
    jam = Jammer(JammerConfig(id="j", kind="spot", position=(500, 0),
                              tx_power_dbm=30, channels=[0, 1]))
    table = build_table({"a": (0, 0), "b": (100, 0)}, [jam], t=0.0,
                        radio=RADIO, pathloss=pl)
    assert set(table) == {("a", "b"), ("b", "a")}
    st = table[("a", "b")]
    assert st.rho == pytest.approx(2 / 50)
    assert st.jam_inchannel_dbm is not None


def test_replay_determinism():
    st = make_state(prx_dbm=-105, rho=0.3, jam_inchannel_dbm=-95)

    def run():
        rng = link_rng(seed=42, src="a", dst="b")
        return [st.verdict(64, rng.random(), rng.random()) for _ in range(500)]

    assert run() == run()
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/channel/linkstate.py`**

```python
from __future__ import annotations

import math
import zlib
from dataclasses import dataclass

import numpy as np

from ..config import RadioProfile
from ..ew import Jammer
from ..propagation import CompositePathloss
from .fhss import dwells_per_packet
from .linkbudget import dbm_to_mw, fsk_per, noise_dbm, per_threshold_sinr_db, sinr_db


def link_rng(seed: int, src: str, dst: str) -> np.random.Generator:
    """Deterministic per-link RNG stream (replay mechanism, benchmarking.md)."""
    return np.random.default_rng([seed, zlib.crc32(f"{src}>{dst}".encode())])


@dataclass(frozen=True)
class LinkState:
    src: str
    dst: str
    prx_dbm: float
    noise_dbm: float
    rho: float                      # fraction of hop channels jammed
    jam_inchannel_dbm: float | None  # received jammer power in a jammed channel
    data_rate_bps: float
    hop_rate_hz: float
    prop_delay_s: float
    foliage_db: float
    terrain_db: float

    def verdict(self, length_bytes: int, u_thermal: float, u_jam: float) -> str:
        """'deliver' or drop cause. u_*: uniforms from the seeded per-link stream."""
        s_clear = sinr_db(self.prx_dbm, self.noise_dbm)
        if u_thermal < fsk_per(s_clear, length_bytes):
            return "foliage" if self.foliage_db > max(self.terrain_db, 3.0) else "range"
        if self.rho > 0 and self.jam_inchannel_dbm is not None:
            s_jam = sinr_db(self.prx_dbm, self.noise_dbm, [self.jam_inchannel_dbm])
            lost = 1.0 if s_jam < per_threshold_sinr_db(length_bytes) else 0.0
            k = dwells_per_packet(length_bytes, self.data_rate_bps, self.hop_rate_hz)
            if u_jam < 1.0 - (1.0 - self.rho * lost) ** k:
                return "jam"
        return "deliver"


def build_table(positions: dict[str, tuple[float, float]], jammers: list[Jammer],
                t: float, radio: RadioProfile,
                pathloss: CompositePathloss) -> dict[tuple[str, str], LinkState]:
    """Per-tick directed link-state table. M1 simplification: the single
    highest-impact jammer per link is applied (documented in models.md)."""
    n0 = noise_dbm(radio.bandwidth_hz, radio.noise_figure_db)
    active = [j for j in jammers if j.active(t)]
    table: dict[tuple[str, str], LinkState] = {}
    for src, sp in positions.items():
        for dst, dp in positions.items():
            if src == dst:
                continue
            bd = pathloss.loss(sp, dp, radio.freq_hz)
            prx = radio.tx_power_dbm + 2 * radio.antenna_gain_dbi - bd.total_db
            rho, jam_dbm, impact = 0.0, None, 0.0
            for j in active:
                r, frac = j.occupancy(radio.hop.n_channels, radio.bandwidth_hz)
                jl = pathloss.loss(j.position, dp, radio.freq_hz)
                p = j.tx_power_dbm - jl.total_db + 10 * math.log10(frac)
                if r * dbm_to_mw(p) > impact:
                    rho, jam_dbm, impact = r, p, r * dbm_to_mw(p)
            d = math.hypot(dp[0] - sp[0], dp[1] - sp[1])
            table[(src, dst)] = LinkState(
                src=src, dst=dst, prx_dbm=prx, noise_dbm=n0, rho=rho,
                jam_inchannel_dbm=jam_dbm, data_rate_bps=radio.data_rate_bps,
                hop_rate_hz=radio.hop.hop_rate_hz, prop_delay_s=d / 3e8,
                foliage_db=bd.foliage_db, terrain_db=bd.terrain_db)
    return table
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/channel/linkstate.py tests/test_linkstate.py
git commit -m "feat: link-state table, cause-labeled verdicts, seeded replay"
```

---

### Task 13: Packet log (parquet)

**Files:**
- Create: `sim/netcom_zen/metrics.py`
- Test: `tests/test_metrics.py`

- [ ] **Step 1: Write the failing test**

`tests/test_metrics.py`:
```python
import pyarrow.parquet as pq

from netcom_zen.metrics import PacketLog, PacketRecord


def test_log_roundtrip(tmp_path):
    log = PacketLog()
    log.add(PacketRecord(t=0.1, src="a", dst="b", length=64,
                         verdict="delivered", delay_s=0.002))
    log.add(PacketRecord(t=0.2, src="b", dst="a", length=64,
                         verdict="jam", delay_s=0.0))
    out = tmp_path / "packets.parquet"
    log.to_parquet(out)
    tbl = pq.read_table(out)
    assert tbl.num_rows == 2
    assert tbl.column("verdict").to_pylist() == ["delivered", "jam"]
```

- [ ] **Step 2: Run test, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/metrics.py`**

```python
from __future__ import annotations

from dataclasses import dataclass, fields
from pathlib import Path


@dataclass(frozen=True)
class PacketRecord:
    t: float
    src: str
    dst: str
    length: int
    verdict: str  # delivered | range | foliage | jam | queue | no_link | tx_error
    delay_s: float


class PacketLog:
    def __init__(self):
        self.records: list[PacketRecord] = []

    def add(self, rec: PacketRecord) -> None:
        self.records.append(rec)

    def to_parquet(self, path: str | Path) -> None:
        import pyarrow as pa
        import pyarrow.parquet as pq
        names = [f.name for f in fields(PacketRecord)]
        cols = {n: [getattr(r, n) for r in self.records] for n in names}
        pq.write_table(pa.table(cols), path)
```

- [ ] **Step 4: Run tests, verify pass** — all PASS

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/metrics.py tests/test_metrics.py
git commit -m "feat: per-packet verdict log with parquet export"
```

---

### Task 14: Netns plumbing

**Files:**
- Create: `sim/netcom_zen/netns.py`
- Test: `tests/test_netns.py` (sudo-marked)

- [ ] **Step 1: Write the failing test**

`tests/test_netns.py`:
```python
import subprocess

import pytest

from netcom_zen.netns import NetnsTopology, sweep_stale


@pytest.mark.sudo
def test_setup_teardown_roundtrip():
    topo = NetnsTopology(["v1", "v2"])
    topo.setup()
    try:
        ns = subprocess.run(["ip", "netns", "list"], capture_output=True,
                            text=True).stdout
        for name in topo.ns_names.values():
            assert name in ns
        assert len(topo.macs) == 2
        assert all(len(m) == 6 for m in topo.macs.values())
    finally:
        topo.teardown()
    ns = subprocess.run(["ip", "netns", "list"], capture_output=True, text=True).stdout
    assert all(name not in ns for name in topo.ns_names.values())


@pytest.mark.sudo
def test_sweep_removes_stale():
    topo = NetnsTopology(["v1"])
    topo.setup()  # deliberately not torn down
    sweep_stale()
    ns = subprocess.run(["ip", "netns", "list"], capture_output=True, text=True).stdout
    assert topo.ns_names["v1"] not in ns
```

- [ ] **Step 2: Run, verify fail (as root)**

Run: `pixi run sudo-test`
Expected: FAIL with `ModuleNotFoundError` (non-root `pixi run test` shows them SKIPPED — also verify that)

- [ ] **Step 3: Implement `sim/netcom_zen/netns.py`**

```python
from __future__ import annotations

import json
import secrets
import subprocess

PREFIX = "ncz"


def _run(*args: str) -> str:
    return subprocess.run(args, check=True, capture_output=True, text=True).stdout


class NetnsTopology:
    """One netns + veth pair per node. Host side carries no IP; the channel
    forwarder attaches AF_PACKET sockets there (architecture.md dataplane)."""

    def __init__(self, node_ids: list[str]):
        self.run_id = secrets.token_hex(2)
        self.nodes = list(node_ids)
        self.ns_names = {n: f"{PREFIX}-{self.run_id}-{i}" for i, n in enumerate(self.nodes)}
        self.host_ifaces = {n: f"{PREFIX}{self.run_id}h{i}" for i, n in enumerate(self.nodes)}
        self.addrs = {n: f"10.99.0.{i + 1}" for i, n in enumerate(self.nodes)}
        self.macs: dict[str, bytes] = {}

    def setup(self) -> None:
        sweep_stale()
        for i, n in enumerate(self.nodes):
            ns, host = self.ns_names[n], self.host_ifaces[n]
            inner = f"{PREFIX}{self.run_id}n{i}"
            _run("ip", "netns", "add", ns)
            _run("ip", "link", "add", host, "type", "veth", "peer", "name", inner)
            _run("ip", "link", "set", inner, "netns", ns)
            _run("ip", "-n", ns, "addr", "add", f"{self.addrs[n]}/24", "dev", inner)
            _run("ip", "-n", ns, "link", "set", inner, "up")
            _run("ip", "-n", ns, "link", "set", "lo", "up")
            _run("ip", "link", "set", host, "up")
            info = json.loads(_run("ip", "-n", ns, "-j", "link", "show", inner))
            self.macs[n] = bytes.fromhex(info[0]["address"].replace(":", ""))

    def teardown(self) -> None:
        for ns in self.ns_names.values():
            subprocess.run(["ip", "netns", "del", ns], capture_output=True)


def sweep_stale() -> None:
    """Idempotent cleanup of leftovers from crashed runs (architecture.md)."""
    out = subprocess.run(["ip", "-j", "netns", "list"],
                         capture_output=True, text=True).stdout
    for entry in json.loads(out or "[]"):
        if entry.get("name", "").startswith(f"{PREFIX}-"):
            subprocess.run(["ip", "netns", "del", entry["name"]], capture_output=True)
```

(Deleting a netns destroys its interfaces; the host-side veth dies with its peer.)

- [ ] **Step 4: Run, verify pass**

Run: `pixi run sudo-test` — Expected: both PASS. `pixi run test` — Expected: both SKIPPED.

- [ ] **Step 5: Commit**

```bash
git add sim/netcom_zen/netns.py tests/test_netns.py
git commit -m "feat: run-scoped netns/veth topology with stale sweep"
```

---

### Task 15: Channel forwarder

**Files:**
- Create: `sim/netcom_zen/channel/forwarder.py`
- Test: `tests/test_forwarder.py` (unit, no root) and `tests/test_forwarder_integration.py` (sudo)

- [ ] **Step 1: Write the failing unit test**

`tests/test_forwarder.py`:
```python
import asyncio

from netcom_zen.channel.forwarder import ChannelForwarder
from netcom_zen.metrics import PacketLog
from tests.test_linkstate import make_state


class FakeTopo:
    nodes = ["a", "b"]
    host_ifaces = {}
    macs = {"a": b"\xaa" * 6, "b": b"\xbb" * 6}


def mk_fwd(table):
    log = PacketLog()
    fwd = ChannelForwarder(FakeTopo(), seed=42, log=log)
    fwd.update_links(table)
    return fwd, log


def run_process(fwd, t, src, dst, frame):
    async def go():
        fwd._process(t, src, dst, frame)
        await asyncio.sleep(0.05)
    asyncio.run(go())


def test_drop_is_logged_with_cause():
    fwd, log = mk_fwd({("a", "b"): make_state(prx_dbm=-150)})
    run_process(fwd, 0.0, "a", "b", b"\xbb" * 6 + b"\xaa" * 6 + b"x" * 52)
    assert [r.verdict for r in log.records] == ["range"]


def test_unknown_link_logged():
    fwd, log = mk_fwd({})
    run_process(fwd, 0.0, "a", "b", b"\xbb" * 6 + b"\xaa" * 6 + b"x" * 52)
    assert [r.verdict for r in log.records] == ["no_link"]


def test_queue_overflow_drops():
    st = make_state(data_rate_bps=1000.0)  # 64B frame = 0.512 s serialization
    fwd, log = mk_fwd({("a", "b"): st})
    frame = b"\xbb" * 6 + b"\xaa" * 6 + b"x" * 52

    async def go():
        for _ in range(3):  # 3rd frame queues past MAX_QUEUE_S = 0.5
            fwd._process(0.0, "a", "b", frame)
    asyncio.run(go())
    assert "queue" in [r.verdict for r in log.records]


def test_mac_targeting():
    fwd, _ = mk_fwd({})
    bcast = b"\xff" * 6 + b"\xaa" * 6
    ucast = b"\xbb" * 6 + b"\xaa" * 6
    assert fwd._targets("a", bcast) == ["b"]
    assert fwd._targets("a", ucast) == ["b"]
    assert fwd._targets("a", b"\xcc" * 6 + b"\xaa" * 6) == []
```

- [ ] **Step 2: Run, verify it fails** — FAIL with `ModuleNotFoundError`

- [ ] **Step 3: Implement `sim/netcom_zen/channel/forwarder.py`**

```python
from __future__ import annotations

import asyncio
import socket
import time

from ..metrics import PacketLog, PacketRecord
from .linkstate import LinkState, link_rng

ETH_P_ALL = 0x0003
PACKET_OUTGOING = 4  # sll_pkttype: frames we injected; must not re-process


class ChannelForwarder:
    """Userspace dataplane: every inter-node frame gets a seeded, logged verdict."""

    MAX_QUEUE_S = 0.5

    def __init__(self, topo, seed: int, log: PacketLog, clock=time.monotonic):
        self.topo = topo
        self.seed = seed
        self.log = log
        self.clock = clock
        self.t0 = clock()
        self._table: dict[tuple[str, str], LinkState] = {}
        self._rngs = {}
        self._busy_until: dict[tuple[str, str], float] = {}
        self._socks: dict[str, socket.socket] = {}

    def start(self) -> None:
        loop = asyncio.get_running_loop()
        for node, iface in self.topo.host_ifaces.items():
            s = socket.socket(socket.AF_PACKET, socket.SOCK_RAW, socket.htons(ETH_P_ALL))
            s.bind((iface, 0))
            s.setblocking(False)
            self._socks[node] = s
            loop.add_reader(s.fileno(), self._on_readable, node, s)

    def stop(self) -> None:
        loop = asyncio.get_running_loop()
        for s in self._socks.values():
            loop.remove_reader(s.fileno())
            s.close()

    def update_links(self, table: dict[tuple[str, str], LinkState]) -> None:
        self._table = table  # atomic swap (GIL)

    def _rng(self, link: tuple[str, str]):
        if link not in self._rngs:
            self._rngs[link] = link_rng(self.seed, *link)
        return self._rngs[link]

    def _targets(self, src: str, frame: bytes) -> list[str]:
        dst_mac = frame[:6]
        if dst_mac[0] & 1:  # broadcast/multicast
            return [n for n in self.topo.nodes if n != src]
        return [n for n, m in self.topo.macs.items() if m == dst_mac and n != src]

    def _on_readable(self, src: str, s: socket.socket) -> None:
        while True:
            try:
                frame, addr = s.recvfrom(65535)
            except BlockingIOError:
                return
            if addr[2] == PACKET_OUTGOING:
                continue  # our own injected frame echoing back
            t = self.clock() - self.t0
            for dst in self._targets(src, frame):
                self._process(t, src, dst, frame)

    def _process(self, t: float, src: str, dst: str, frame: bytes) -> None:
        link = (src, dst)
        st = self._table.get(link)
        if st is None:
            self.log.add(PacketRecord(t, src, dst, len(frame), "no_link", 0.0))
            return
        ser = len(frame) * 8 / st.data_rate_bps
        start = max(t, self._busy_until.get(link, 0.0))
        if start - t > self.MAX_QUEUE_S:
            self.log.add(PacketRecord(t, src, dst, len(frame), "queue", 0.0))
            return
        self._busy_until[link] = start + ser  # airtime consumed even by losses
        rng = self._rng(link)
        verdict = st.verdict(len(frame), rng.random(), rng.random())
        if verdict != "deliver":
            self.log.add(PacketRecord(t, src, dst, len(frame), verdict, 0.0))
            return
        delay = (start - t) + ser + st.prop_delay_s
        asyncio.get_running_loop().call_later(delay, self._deliver, t, src, dst,
                                              frame, delay)

    def _deliver(self, t: float, src: str, dst: str, frame: bytes,
                 delay: float) -> None:
        try:
            self._socks[dst].send(frame)
            self.log.add(PacketRecord(t, src, dst, len(frame), "delivered", delay))
        except (OSError, KeyError):
            self.log.add(PacketRecord(t, src, dst, len(frame), "tx_error", 0.0))
```

- [ ] **Step 4: Run unit tests, verify pass** — `pixi run test`, all PASS

- [ ] **Step 5: Write the sudo integration test**

`tests/test_forwarder_integration.py`:
```python
import asyncio
import subprocess

import pytest

from netcom_zen.channel.forwarder import ChannelForwarder
from netcom_zen.channel.linkstate import LinkState
from netcom_zen.metrics import PacketLog
from netcom_zen.netns import NetnsTopology


def good_link(src, dst):
    return LinkState(src=src, dst=dst, prx_dbm=-60, noise_dbm=-113, rho=0.0,
                     jam_inchannel_dbm=None, data_rate_bps=1e6, hop_rate_hz=100,
                     prop_delay_s=1e-6, foliage_db=0.0, terrain_db=0.0)


@pytest.mark.sudo
def test_udp_through_channel():
    topo = NetnsTopology(["v1", "v2"])
    topo.setup()
    try:
        log = PacketLog()

        async def go():
            fwd = ChannelForwarder(topo, seed=1, log=log)
            fwd.update_links({("v1", "v2"): good_link("v1", "v2"),
                              ("v2", "v1"): good_link("v2", "v1")})
            fwd.start()
            recv = subprocess.Popen(
                ["ip", "netns", "exec", topo.ns_names["v2"], "python", "-c",
                 "import socket;s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM);"
                 "s.bind(('10.99.0.2',9000));s.settimeout(5);print(len(s.recv(100)))"],
                stdout=subprocess.PIPE, text=True)
            await asyncio.sleep(1.0)
            for _ in range(5):
                subprocess.run(
                    ["ip", "netns", "exec", topo.ns_names["v1"], "python", "-c",
                     "import socket;s=socket.socket(socket.AF_INET,socket.SOCK_DGRAM);"
                     "s.sendto(b'x'*64,('10.99.0.2',9000))"], check=True)
                await asyncio.sleep(0.2)
            out, _ = recv.communicate(timeout=10)
            assert out.strip() == "64"
            fwd.stop()

        asyncio.run(go())
        assert any(r.verdict == "delivered" for r in log.records)
    finally:
        topo.teardown()
```

- [ ] **Step 6: Run, verify pass** — `pixi run sudo-test`, all PASS (ARP resolves through the forwarder via broadcast; first UDP packets may race ARP, hence 5 sends)

- [ ] **Step 7: Commit**

```bash
git add sim/netcom_zen/channel/forwarder.py tests/test_forwarder.py tests/test_forwarder_integration.py
git commit -m "feat: AF_PACKET channel forwarder with verdicts, queueing, delivery"
```

---

### Task 16: Orchestrator + smoke scenario

**Files:**
- Create: `sim/netcom_zen/orchestrator.py`, `scenarios/smoke_2node.yaml`, `sim/netcom_zen/run.py`
- Test: `tests/test_orchestrator.py` (sudo)

- [ ] **Step 1: Write `scenarios/smoke_2node.yaml`**

```yaml
name: smoke-2node
duration_s: 5
tick_hz: 10
seed: 42
radio:
  freq_hz: 433.0e6
  bandwidth_hz: 250.0e3
  tx_power_dbm: 27.0
  noise_figure_db: 7.0
  data_rate_bps: 250.0e3
  hop: {n_channels: 50, hop_rate_hz: 100.0}
nodes:
  - {id: v1, waypoints: [[100, 500]], speed_mps: 0.1}
  - {id: v2, waypoints: [[200, 500]], speed_mps: 0.1}
environment:
  extent_m: [1000, 1000]
```

- [ ] **Step 2: Write the failing test**

`tests/test_orchestrator.py`:
```python
import asyncio
import json
import subprocess

import pyarrow.parquet as pq
import pytest

from netcom_zen.config import load_scenario
from netcom_zen.orchestrator import ScenarioEngine


@pytest.mark.sudo
def test_smoke_run_produces_artifacts(tmp_path):
    scenario = load_scenario("scenarios/smoke_2node.yaml")
    engine = ScenarioEngine(scenario, out_dir=tmp_path)

    async def go():
        run_task = asyncio.create_task(engine.run())
        await asyncio.wait_for(engine.ready.wait(), timeout=10)
        ns1 = engine.topo.ns_names["v1"]
        # generate traffic: ping v2 from v1 through the channel
        subprocess.run(["ip", "netns", "exec", ns1, "ping", "-c", "3", "-W", "2",
                        "10.99.0.2"], check=True, capture_output=True)
        await run_task

    asyncio.run(go())
    tbl = pq.read_table(tmp_path / "packets.parquet")
    assert tbl.num_rows > 0
    assert "delivered" in set(tbl.column("verdict").to_pylist())
    manifest = json.loads((tmp_path / "manifest.json").read_text())
    assert manifest["scenario"]["name"] == "smoke-2node"
    assert manifest["seed"] == 42 and "git_hash" in manifest
    assert manifest["timing_ok"] is True  # tick loop kept up with wall clock
```

- [ ] **Step 3: Run, verify it fails** — `pixi run sudo-test`, FAIL with `ModuleNotFoundError`

- [ ] **Step 4: Implement `sim/netcom_zen/orchestrator.py`**

```python
from __future__ import annotations

import asyncio
import json
import subprocess
import time
from pathlib import Path

import numpy as np

from .channel.forwarder import ChannelForwarder
from .channel.linkstate import build_table
from .config import Scenario
from .ew import Jammer
from .metrics import PacketLog
from .mobility import WaypointVehicle
from .netns import NetnsTopology
from .propagation import CompositePathloss
from .terrain import FoliageRegion, Terrain


def _git_hash() -> str:
    try:
        return subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True,
                              text=True, check=True).stdout.strip()
    except (subprocess.CalledProcessError, FileNotFoundError):
        return "unknown"


class ScenarioEngine:
    """Owns the single clock: 10 Hz tick loop driving mobility -> pathloss ->
    link-state -> forwarder (architecture.md)."""

    def __init__(self, scenario: Scenario, out_dir: str | Path):
        self.scenario = scenario
        self.out_dir = Path(out_dir)
        self.ready = asyncio.Event()
        self.topo: NetnsTopology | None = None

    def _build(self) -> None:
        env = self.scenario.environment
        hm = np.load(env.heightmap) if env.heightmap else None
        terrain = Terrain(env.extent_m, hm,
                          [FoliageRegion(**f.model_dump()) for f in env.foliage])
        self.pathloss = CompositePathloss(terrain)
        self.vehicles = {n.id: WaypointVehicle(n.waypoints, n.speed_mps)
                         for n in self.scenario.nodes}
        self.jammers = [Jammer(j) for j in self.scenario.jammers]

    async def run(self) -> None:
        self._build()
        self.topo = NetnsTopology([n.id for n in self.scenario.nodes])
        self.topo.setup()
        log = PacketLog()
        fwd = ChannelForwarder(self.topo, self.scenario.seed, log)
        try:
            fwd.start()
            self.ready.set()
            dt = 1.0 / self.scenario.tick_hz
            t = 0.0
            wall0 = time.monotonic()
            max_tick_lag = 0.0  # timing integrity (architecture.md): sim time vs wall clock
            while t < self.scenario.duration_s:
                poses = {nid: v.step(dt) for nid, v in self.vehicles.items()}
                positions = {nid: (p.x, p.y) for nid, p in poses.items()}
                fwd.update_links(build_table(positions, self.jammers, t,
                                             self.scenario.radio, self.pathloss))
                await asyncio.sleep(dt)
                t += dt
                max_tick_lag = max(max_tick_lag, (time.monotonic() - wall0) - t)
            fwd.stop()
        finally:
            self.topo.teardown()
        self.out_dir.mkdir(parents=True, exist_ok=True)
        log.to_parquet(self.out_dir / "packets.parquet")
        (self.out_dir / "manifest.json").write_text(json.dumps({
            "scenario": self.scenario.model_dump(mode="json"),
            "seed": self.scenario.seed,
            "git_hash": _git_hash(),
            "max_tick_lag_s": max_tick_lag,
            "timing_ok": max_tick_lag < dt,
        }, indent=2))
```

- [ ] **Step 5: Implement CLI `sim/netcom_zen/run.py`**

```python
import argparse
import asyncio
from pathlib import Path

from .config import load_scenario
from .orchestrator import ScenarioEngine


def main() -> None:
    ap = argparse.ArgumentParser(description="Run a net_com_zen scenario (needs root)")
    ap.add_argument("scenario")
    ap.add_argument("-o", "--out", default="results/latest")
    args = ap.parse_args()
    engine = ScenarioEngine(load_scenario(args.scenario), Path(args.out))
    asyncio.run(engine.run())


if __name__ == "__main__":
    main()
```

- [ ] **Step 6: Run, verify pass** — `pixi run sudo-test`, all PASS

- [ ] **Step 7: Commit**

```bash
git add sim/netcom_zen/orchestrator.py sim/netcom_zen/run.py scenarios/ tests/test_orchestrator.py
git commit -m "feat: scenario engine tick loop, run CLI, smoke scenario"
```

---

### Task 17: Zenoh smoke test (M1 exit criterion)

**Files:**
- Create: `tests/helpers/zsub.py`, `tests/helpers/zpub.py`
- Test: `tests/test_zenoh_smoke.py` (sudo)

- [ ] **Step 1: Write helper scripts**

`tests/helpers/zsub.py`:
```python
"""Zenoh subscriber: listens on its netns IP, prints received count after 8 s."""
import sys
import time

import zenoh

conf = zenoh.Config()
conf.insert_json5("listen/endpoints", '["tcp/0.0.0.0:7447"]')
conf.insert_json5("scouting/multicast/enabled", "false")
count = 0


def cb(sample):
    global count
    count += 1


with zenoh.open(conf) as session:
    session.declare_subscriber("ncz/test", cb)
    time.sleep(8)
print(count)
sys.exit(0)
```

`tests/helpers/zpub.py`:
```python
"""Zenoh publisher: connects to the subscriber endpoint, sends 20 messages."""
import sys
import time

import zenoh

conf = zenoh.Config()
conf.insert_json5("connect/endpoints", f'["tcp/{sys.argv[1]}:7447"]')
conf.insert_json5("scouting/multicast/enabled", "false")
with zenoh.open(conf) as session:
    time.sleep(1)
    for i in range(20):
        session.put("ncz/test", f"msg{i}".encode())
        time.sleep(0.2)
```

- [ ] **Step 2: Write the failing test**

`tests/test_zenoh_smoke.py`:
```python
import asyncio
import subprocess
import sys

import pytest

from netcom_zen.config import load_scenario
from netcom_zen.orchestrator import ScenarioEngine


@pytest.mark.sudo
def test_zenoh_pubsub_through_channel(tmp_path):
    scenario = load_scenario("scenarios/smoke_2node.yaml")
    scenario = scenario.model_copy(update={"duration_s": 12.0})
    engine = ScenarioEngine(scenario, out_dir=tmp_path)

    async def go():
        run_task = asyncio.create_task(engine.run())
        await asyncio.wait_for(engine.ready.wait(), timeout=10)
        ns1, ns2 = engine.topo.ns_names["v1"], engine.topo.ns_names["v2"]
        sub = subprocess.Popen(["ip", "netns", "exec", ns1, sys.executable,
                                "tests/helpers/zsub.py"],
                               stdout=subprocess.PIPE, text=True)
        await asyncio.sleep(1.5)
        subprocess.run(["ip", "netns", "exec", ns2, sys.executable,
                        "tests/helpers/zpub.py", "10.99.0.1"], check=True, timeout=30)
        out, _ = sub.communicate(timeout=30)
        assert int(out.strip()) > 0  # zenoh messages crossed the emulated channel
        await run_task

    asyncio.run(go())
```

- [ ] **Step 3: Run, verify it fails, then passes**

Run: `pixi run sudo-test`
Expected first: FAIL (missing helpers) → after files exist: PASS.
Note: `sys.executable` under sudo must be the pixi env python — `pixi run sudo-test` preserves it.

- [ ] **Step 4: Commit**

```bash
git add tests/helpers/ tests/test_zenoh_smoke.py
git commit -m "test: zenoh pub/sub through emulated channel (M1 exit criterion)"
```

---

### Task 18: setup-check + validation report

**Files:**
- Create: `sim/netcom_zen/setup_check.py`, `validation/report.py`, `validation/test_propagation_curves.py`

- [ ] **Step 1: Write validation curve tests**

`validation/test_propagation_curves.py`:
```python
"""Model validation against published/analytical reference values (ADR-0001:
this suite is what makes hand-rolled benchmark results defensible)."""
import pytest

from netcom_zen.channel.linkbudget import fsk_per, noise_dbm
from netcom_zen.propagation.diffraction import knife_edge_loss_db
from netcom_zen.propagation.foliage import weissberger_db
from netcom_zen.propagation.freespace import friis_db

import numpy as np

REFERENCES = [
    # (description, computed, expected, tolerance)
    ("Friis 1 km @ 2.4 GHz", lambda: friis_db(1000, 2.4e9), 100.05, 0.02),
    ("Friis 100 m @ 433 MHz", lambda: friis_db(100, 433e6), 65.18, 0.05),
    ("Weissberger 100 m foliage @ 900 MHz", lambda: weissberger_db(900e6, 100), 19.36, 0.05),
    ("Weissberger 50 m foliage @ 2.4 GHz", lambda: weissberger_db(2.4e9, 50), 17.02, 0.10),
    ("Knife-edge grazing (v=0) = 6 dB", lambda: knife_edge_loss_db(
        np.array([0.0, 1.5, 0.0]), 1000, 1.5, 1.5, 433e6), 6.03, 0.02),
    ("Thermal noise 250 kHz NF7", lambda: noise_dbm(250e3, 7.0), -113.02, 0.02),
    ("FSK PER @ 10 dB, 32 B", lambda: fsk_per(10.0, 32), 0.5785, 0.001),
]


@pytest.mark.parametrize("desc,fn,expected,tol",
                         REFERENCES, ids=[r[0] for r in REFERENCES])
def test_reference(desc, fn, expected, tol):
    assert fn() == pytest.approx(expected, abs=tol)
```

(Derivations for the two non-classic entries: `friis_db(100, 433e6) = 40 + 172.73 − 147.55 = 65.18`;
`weissberger(2.4 GHz, 50 m) = 1.33 · 2.4^0.284 · 50^0.588 = 1.33 · 1.2823 · 9.98 = 17.02`.)

- [ ] **Step 2: Write `validation/report.py`**

```python
"""Render validation/report.md from the reference table."""
from pathlib import Path

from test_propagation_curves import REFERENCES


def main() -> None:
    lines = ["# Model Validation Report", "",
             "| Case | Computed | Expected | Tolerance | OK |",
             "|---|---|---|---|---|"]
    ok_all = True
    for desc, fn, expected, tol in REFERENCES:
        got = fn()
        ok = abs(got - expected) <= tol
        ok_all &= ok
        lines.append(f"| {desc} | {got:.3f} | {expected} | ±{tol} | {'✅' if ok else '❌'} |")
    Path(__file__).with_name("report.md").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))
    raise SystemExit(0 if ok_all else 1)


if __name__ == "__main__":
    main()
```

Update `pixi.toml` validate task:
```toml
validate = "pytest validation -v && python validation/report.py"
```

- [ ] **Step 3: Write `sim/netcom_zen/setup_check.py`**

```python
"""Preflight: verify the host can run scenarios (architecture.md)."""
import importlib
import os
import shutil
import subprocess
import sys


def check(name: str, ok: bool, hint: str = "") -> bool:
    print(f"  [{'ok' if ok else 'FAIL'}] {name}" + (f" — {hint}" if not ok and hint else ""))
    return ok


def main() -> None:
    ok = True
    for mod in ("numpy", "pydantic", "yaml", "pyarrow", "zenoh"):
        ok &= check(f"import {mod}", importlib.util.find_spec(mod) is not None,
                    "run `pixi install`")
    ok &= check("`ip` available", shutil.which("ip") is not None, "install iproute2")
    if os.geteuid() == 0:
        probe = subprocess.run(["ip", "netns", "add", "ncz-probe"], capture_output=True)
        subprocess.run(["ip", "netns", "del", "ncz-probe"], capture_output=True)
        ok &= check("netns create/delete", probe.returncode == 0)
    else:
        check("root", False, "scenario runs need root: `sudo $(which python) -m netcom_zen.run ...`")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
```

- [ ] **Step 4: Run everything**

Run: `pixi run test && pixi run validate && pixi run setup-check && pixi run sudo-test`
Expected: all green (setup-check exits 0 as non-root with the root hint printed; rerun under sudo if unsure).

- [ ] **Step 5: Update `docs/models.md` known-limits note**

Append to the "Known fidelity limits" list in `docs/models.md`:
```markdown
- M1: only the single highest-impact jammer is applied per link per tick;
  multi-jammer power summation is future work.
```

- [ ] **Step 6: Commit**

```bash
git add validation/ sim/netcom_zen/setup_check.py pixi.toml docs/models.md
git commit -m "feat: setup-check preflight + model validation report"
```

---

## M1 exit checklist (from docs/roadmap.md)

- [ ] `pixi run test` — all unit tests pass
- [ ] `pixi run validate` — validation report clean
- [ ] `pixi run sudo-test` — netns, forwarder UDP, orchestrator smoke, **zenoh pub/sub through the engine** all pass
- [ ] Deterministic replay test green (`test_replay_determinism`)
