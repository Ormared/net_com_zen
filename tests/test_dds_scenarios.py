"""Phase B3 validation: dds_scenarios.py and the files it generates.

Run after generating the scenario files::

    PYTHONNOUSERSITE=1 .pixi/envs/default/bin/python \\
        -m netcom_zen.harness.dds_scenarios
    PYTHONNOUSERSITE=1 .pixi/envs/default/bin/python \\
        -m pytest tests/test_dds_scenarios.py -q
"""
from __future__ import annotations

import copy
from pathlib import Path

import pytest
import yaml
from pydantic import ValidationError

from netcom_zen.config import Scenario, load_scenario
from netcom_zen.harness.dds_scenarios import (
    DDS_N_VALUES,
    DDS_RMW_VALUES,
    _sweep_dict,
    bridge_scenario,
    generate_all,
)
from netcom_zen.harness.sweep import apply_override, expand

# Filesystem location for the generated files (relative to project root,
# which is where pytest is invoked from).
_SCENARIOS_DDS = Path("scenarios/dds")


# ---------------------------------------------------------------------------
# bridge_scenario() — pure-function tests, no filesystem I/O
# ---------------------------------------------------------------------------

class TestBridgeScenario:
    @pytest.mark.parametrize("n", DDS_N_VALUES)
    @pytest.mark.parametrize("rmw", DDS_RMW_VALUES)
    def test_validates_for_full_grid(self, n: int, rmw: str) -> None:
        """Every (N, RMW) pair in the benchmark grid must produce a valid Scenario."""
        d = bridge_scenario(n, rmw)
        s = Scenario.model_validate(d)
        assert len(s.nodes) == n
        assert s.substrate == "bridge"
        assert s.ros2.rmw == rmw
        assert s.workload == "ros2"

    def test_node_ids_d1_to_dN(self) -> None:
        d = bridge_scenario(4, "zenoh")
        s = Scenario.model_validate(d)
        assert [nd.id for nd in s.nodes] == ["d1", "d2", "d3", "d4"]

    def test_each_node_has_one_waypoint_at_origin(self) -> None:
        # waypoints must be ≥1 (schema); [[0,0]] is the canonical no-op for bridge
        d = bridge_scenario(4, "zenoh")
        s = Scenario.model_validate(d)
        for node in s.nodes:
            assert node.waypoints == [(0.0, 0.0)]

    def test_no_jammers_in_dict(self) -> None:
        # The DDS benchmark has no EW vector; jammers list must be absent/empty.
        d = bridge_scenario(8, "fastrtps")
        assert d.get("jammers", []) == []

    def test_over_bridge_cap_raises_validation_error(self) -> None:
        """N=97 exceeds the bridge cap of 96; Scenario.model_validate must raise."""
        with pytest.raises(ValidationError):
            bridge_scenario(97, "zenoh")

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_custom_workload_params_round_trip(self, n: int) -> None:
        d = bridge_scenario(
            n, "cyclonedds",
            duration_s=60.0,
            period_ms=100,
            payload_bytes=512,
            reliability="best_effort",
        )
        s = Scenario.model_validate(d)
        assert s.duration_s == 60.0
        assert s.ros2.period_ms == 100
        assert s.ros2.payload_bytes == 512
        assert s.ros2.reliability == "best_effort"


# ---------------------------------------------------------------------------
# generate_all() — round-trip against a temp dir (no real filesystem side-effects)
# ---------------------------------------------------------------------------

class TestGenerateAll:
    def test_correct_filenames_written(self, tmp_path: Path) -> None:
        written = generate_all(tmp_path)
        names = {p.name for p in written}
        for n in DDS_N_VALUES:
            assert f"bridge_n{n}.yaml" in names, f"missing bridge_n{n}.yaml"
            assert f"sweep_n{n}.yaml" in names, f"missing sweep_n{n}.yaml"

    def test_total_file_count(self, tmp_path: Path) -> None:
        # 2 files (base + sweep) × 6 N values = 12
        written = generate_all(tmp_path)
        assert len(written) == len(DDS_N_VALUES) * 2

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_base_scenario_validates(self, n: int, tmp_path: Path) -> None:
        generate_all(tmp_path)
        s = load_scenario(tmp_path / f"bridge_n{n}.yaml")
        assert len(s.nodes) == n
        assert s.substrate == "bridge"
        assert s.ros2.rmw == "zenoh"  # base always uses zenoh

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_sweep_references_existing_base(self, n: int, tmp_path: Path) -> None:
        generate_all(tmp_path)
        sweep = yaml.safe_load((tmp_path / f"sweep_n{n}.yaml").read_text())
        base_path = tmp_path / sweep["base"]
        assert base_path.exists(), f"sweep_n{n}.yaml references missing {sweep['base']}"

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_sweep_expands_to_exactly_3_cells(self, n: int, tmp_path: Path) -> None:
        # 3 RMW values × 1 seed = 3 cells per N
        generate_all(tmp_path)
        sweep = yaml.safe_load((tmp_path / f"sweep_n{n}.yaml").read_text())
        cells = expand(sweep)
        assert len(cells) == len(DDS_RMW_VALUES)

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_sweep_cells_each_validate(self, n: int, tmp_path: Path) -> None:
        """All 3 sweep cells for each N must produce a valid Scenario."""
        generate_all(tmp_path)
        sweep = yaml.safe_load((tmp_path / f"sweep_n{n}.yaml").read_text())
        base_path = tmp_path / sweep["base"]
        base = yaml.safe_load(base_path.read_text())
        for overrides, seed in expand(sweep):
            cfg = copy.deepcopy(base)
            for k, v in overrides.items():
                apply_override(cfg, k, v)
            cfg["seed"] = seed
            Scenario.model_validate(cfg)


# ---------------------------------------------------------------------------
# Generated file fixtures on disk (scenarios/dds/) — skipped on fresh clone
# ---------------------------------------------------------------------------

class TestGeneratedFilesOnDisk:
    """Validate the files that were written to scenarios/dds/ by the CLI.

    These tests are skipped if the directory does not yet exist (i.e. on a
    fresh clone before running ``python -m netcom_zen.harness.dds_scenarios``).
    They complement TestGenerateAll by exercising the *actual* committed files
    rather than a temp-dir copy.
    """

    @pytest.fixture(autouse=True)
    def _require_generated(self) -> None:
        if not _SCENARIOS_DDS.exists():
            pytest.skip("scenarios/dds/ not present — run dds_scenarios first")

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_on_disk_base_validates(self, n: int) -> None:
        path = _SCENARIOS_DDS / f"bridge_n{n}.yaml"
        assert path.exists(), f"missing {path}"
        s = load_scenario(path)
        assert len(s.nodes) == n
        assert s.substrate == "bridge"

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_on_disk_sweep_references_existing_base(self, n: int) -> None:
        sweep_path = _SCENARIOS_DDS / f"sweep_n{n}.yaml"
        assert sweep_path.exists(), f"missing {sweep_path}"
        sweep = yaml.safe_load(sweep_path.read_text())
        base_path = _SCENARIOS_DDS / sweep["base"]
        assert base_path.exists(), f"sweep_n{n}.yaml references missing {sweep['base']}"

    @pytest.mark.parametrize("n", DDS_N_VALUES)
    def test_on_disk_sweep_expands_to_3_cells(self, n: int) -> None:
        sweep = yaml.safe_load((_SCENARIOS_DDS / f"sweep_n{n}.yaml").read_text())
        assert len(expand(sweep)) == len(DDS_RMW_VALUES)
