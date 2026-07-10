"""Offline Sionna RT precompute: scenario terrain -> ray-traced pathloss grid.

Runs ONLY in the standalone `sionna` pixi env (drjit/mitsuba stack):

    pixi run -e sionna sionna-precompute scenarios/foo.yaml -o grids/foo.npz

Mirrors the Isaac pattern: heavy GPU work happens once, offline; the runtime
provider (`sionna_grid.SionnaGridPathloss`) is a cheap numpy lookup in the
default env. The scene is the scenario's own terrain (heightmap or flat
ground) meshed as a triangle grid with a lossy ground material; receivers sit
at every rx cell centre at h_ant above the terrain — the analytical model's
convention — and the deterministic PathSolver (LoS + specular reflection +
wedge diffraction, coherently summed) gives the pathloss per (tx, cell) pair.
A Monte-Carlo RadioMapSolver was tried first and rejected: with both ends at
antenna height the geometry is grazing, and ray launching both misses the
ground-bounce interference and runs out of hits beyond ~300 m. Foliage is NOT
part of the scene — the runtime provider adds Weissberger analytically on top.

The mesh/scene builders below are pure numpy and unit-tested in the default
env; only `_trace()`/`main()` import sionna.
"""
from __future__ import annotations

import argparse
import json
import math
import sys
import tempfile
import time
from pathlib import Path

import numpy as np

from ..config import Scenario, load_scenario
from ..terrain import Terrain
from .sionna_grid import CAP_DB, SionnaGrid

# Classic ground constants (ITU-R P.527 "medium dry ground"), used via the
# generic radio-material bsdf: sionna's itu-radio-material ground types are
# only defined for 1-10 GHz, and the band ladder goes down to 150 MHz.
GROUND_EPS_R = 15.0
GROUND_SIGMA_S_M = 0.005
GROUND_THICKNESS_M = 0.5


def grid_mesh(terrain: Terrain, x_axis: np.ndarray, y_axis: np.ndarray,
              z_offset: float = 0.0) -> tuple[np.ndarray, np.ndarray]:
    """Triangulated heightfield over x_axis x y_axis vertex lines.

    Returns (vertices (N,3) float64, faces (M,3) int32). Faces wind CCW seen
    from +z so face normals point up.
    """
    nx, ny = len(x_axis), len(y_axis)
    xx, yy = np.meshgrid(x_axis, y_axis)  # (ny, nx)
    zz = np.array([[terrain.height(x, y) for x in x_axis] for y in y_axis])
    verts = np.column_stack([xx.ravel(), yy.ravel(), (zz + z_offset).ravel()])
    faces = []
    for j in range(ny - 1):
        for i in range(nx - 1):
            v00 = j * nx + i
            v10 = v00 + 1
            v01 = v00 + nx
            v11 = v01 + 1
            faces.append((v00, v10, v11))
            faces.append((v00, v11, v01))
    return verts, np.asarray(faces, dtype=np.int32)


def write_ply(path: str | Path, verts: np.ndarray, faces: np.ndarray) -> None:
    with open(path, "w") as f:
        f.write("ply\nformat ascii 1.0\n"
                f"element vertex {len(verts)}\n"
                "property float x\nproperty float y\nproperty float z\n"
                f"element face {len(faces)}\n"
                "property list uchar int vertex_indices\nend_header\n")
        for v in verts:
            f.write(f"{v[0]:.3f} {v[1]:.3f} {v[2]:.3f}\n")
        for a, b, c in faces:
            f.write(f"3 {a} {b} {c}\n")


def scene_xml(mesh_filename: str, eps_r: float = GROUND_EPS_R,
              sigma: float = GROUND_SIGMA_S_M,
              thickness_m: float = GROUND_THICKNESS_M) -> str:
    return f"""<scene version="2.1.0">
    <bsdf type="radio-material" id="ground">
        <float name="relative_permittivity" value="{eps_r}"/>
        <float name="conductivity" value="{sigma}"/>
        <float name="thickness" value="{thickness_m}"/>
    </bsdf>
    <shape type="ply" id="terrain">
        <string name="filename" value="{mesh_filename}"/>
        <boolean name="face_normals" value="true"/>
        <ref id="ground" name="bsdf"/>
    </shape>
</scene>
"""


def cell_axes(extent: tuple[float, float], n_cells: int) -> tuple[np.ndarray, np.ndarray]:
    """Rx cell-centre axes: n_cells x n_cells over the scenario extent."""
    dx, dy = extent[0] / n_cells, extent[1] / n_cells
    return (np.linspace(dx / 2, extent[0] - dx / 2, n_cells),
            np.linspace(dy / 2, extent[1] - dy / 2, n_cells))


def gain_to_pl_db(gain: np.ndarray) -> np.ndarray:
    """Linear path gain -> pathloss dB, CAP_DB where the tracer found nothing."""
    pl = np.full(gain.shape, CAP_DB, dtype=np.float32)
    hit = gain > 0
    pl[hit] = np.minimum(-10.0 * np.log10(gain[hit]), CAP_DB).astype(np.float32)
    return pl


def _trace(scenario: Scenario, terrain: Terrain, tx_x: np.ndarray,
           tx_y: np.ndarray, n_cells: int, h_ant: float,
           max_depth: int, margin_m: float, mesh_cells: int,
           tx_chunk: int = 8) -> np.ndarray:
    """Trace the pathloss grid with the deterministic PathSolver. sionna env
    only. Path gain per (tx, rx cell) is the coherent sum of LoS + specular
    reflections + wedge diffraction — two-ray interference and terrain
    shadowing come out of the geometry."""
    import mitsuba as mi  # noqa: F401  (selects the variant before sionna)
    from sionna.rt import (PathSolver, PlanarArray, Receiver, Transmitter,
                           load_scene)

    extent = terrain.extent
    tmp = Path(tempfile.mkdtemp(prefix="sionna_scene_"))
    # ground extends past the extent so edge tx/rx still have terrain below
    gx = np.linspace(-margin_m, extent[0] + margin_m, mesh_cells + 1)
    gy = np.linspace(-margin_m, extent[1] + margin_m, mesh_cells + 1)
    write_ply(tmp / "ground.ply", *grid_mesh(terrain, gx, gy))
    (tmp / "scene.xml").write_text(scene_xml("ground.ply"))

    scene = load_scene(str(tmp / "scene.xml"))
    scene.frequency = scenario.radio.freq_hz
    scene.tx_array = PlanarArray(num_rows=1, num_cols=1, pattern="iso",
                                 polarization="V")
    scene.rx_array = scene.tx_array
    cx, cy = cell_axes(extent, n_cells)
    for ci, y in enumerate(cy):          # row-major: cy outer, matches pl_db
        for cj, x in enumerate(cx):
            scene.add(Receiver(name=f"rx{ci * n_cells + cj}", position=[
                float(x), float(y), terrain.height(x, y) + h_ant]))
    solver = PathSolver()

    tx_pos = [(x, y) for y in tx_y for x in tx_x]  # row-major: iy outer
    pl = np.empty((len(tx_pos), n_cells, n_cells), dtype=np.float32)
    for lo in range(0, len(tx_pos), tx_chunk):
        chunk = tx_pos[lo:lo + tx_chunk]
        names = []
        for k, (x, y) in enumerate(chunk):
            name = f"tx{lo + k}"
            scene.add(Transmitter(name=name, position=[
                float(x), float(y), terrain.height(x, y) + h_ant]))
            names.append(name)
        t0 = time.monotonic()
        paths = solver(scene, max_depth=max_depth, los=True,
                       specular_reflection=True, diffraction=True,
                       edge_diffraction=False, refraction=False,
                       diffuse_reflection=False)
        a, _ = paths.cir(out_type="numpy")
        # a: [num_rx, rx_ant, num_tx, tx_ant, num_paths, num_time]
        amp = a[:, 0, :, 0, :, 0].sum(axis=-1)     # coherent path sum
        gain = np.abs(amp.T) ** 2                  # (num_tx, num_rx)
        pl[lo:lo + len(chunk)] = gain_to_pl_db(
            gain.reshape(len(chunk), n_cells, n_cells))
        for name in names:
            scene.remove(name)
        print(f"  tx {lo + len(chunk)}/{len(tx_pos)} "
              f"({time.monotonic() - t0:.1f}s/chunk)", flush=True)
    return pl.reshape(len(tx_y), len(tx_x), n_cells, n_cells)


def main(argv: list[str] | None = None) -> None:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("scenario", help="scenario yaml (terrain + radio.freq_hz)")
    ap.add_argument("-o", "--out", required=True, help="output .npz grid")
    ap.add_argument("--tx-grid", type=int, default=9,
                    help="coarse tx grid is N x N over the extent")
    ap.add_argument("--rx-cells", type=int, default=100,
                    help="rx map is N x N cells over the extent")
    ap.add_argument("--mesh-cells", type=int, default=128,
                    help="ground mesh resolution (cells per axis)")
    ap.add_argument("--max-depth", type=int, default=3)
    ap.add_argument("--margin-m", type=float, default=100.0)
    ap.add_argument("--h-ant", type=float, default=1.5)
    args = ap.parse_args(argv)

    scenario = load_scenario(args.scenario)
    env = scenario.environment
    hm = np.load(env.heightmap) if env.heightmap else None
    terrain = Terrain(env.extent_m, hm)  # foliage stays analytic (runtime)

    tx_x = np.linspace(0.0, env.extent_m[0], args.tx_grid)
    tx_y = np.linspace(0.0, env.extent_m[1], args.tx_grid)
    cx, cy = cell_axes(env.extent_m, args.rx_cells)

    print(f"tracing {args.tx_grid}x{args.tx_grid} tx grid, "
          f"{args.rx_cells}x{args.rx_cells} rx cells @ "
          f"{scenario.radio.freq_hz/1e6:.0f} MHz ...", flush=True)
    t0 = time.monotonic()
    pl = _trace(scenario, terrain, tx_x, tx_y, args.rx_cells, args.h_ant,
                args.max_depth, args.margin_m, args.mesh_cells)
    import sionna.rt as srt
    grid = SionnaGrid(
        freq_hz=scenario.radio.freq_hz, h_ant_m=args.h_ant,
        tx_x=tx_x, tx_y=tx_y, cell_x=cx, cell_y=cy, pl_db=pl,
        meta=dict(scenario=str(args.scenario),
                  solver="PathSolver los+specular+diffraction, coherent sum",
                  max_depth=args.max_depth,
                  material=f"ground eps_r={GROUND_EPS_R} sigma={GROUND_SIGMA_S_M}",
                  mesh_cells=args.mesh_cells, margin_m=args.margin_m,
                  sionna_rt=getattr(srt, "__version__", "unknown"),
                  elapsed_s=round(time.monotonic() - t0, 1)))
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    grid.save(args.out)
    print(f"wrote {args.out} in {time.monotonic() - t0:.0f}s")


if __name__ == "__main__":
    sys.exit(main())
