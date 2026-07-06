"""P5 netem link-emulation plumbing (docs/dds-topology-plan.md P5).

Pure unit tests: no root required, no subprocesses spawned, no network I/O.
Covers netem_args() token building, NetemConfig field validation, the Scenario
cross-field validators, the NetnsTopology constructor signature, and the
model_dump manifest round-trip.
"""
from __future__ import annotations

import pytest
from pydantic import ValidationError

from netcom_zen.config import NetemConfig, Scenario
from netcom_zen.netns import NetnsTopology, netem_args


# ---------------------------------------------------------------------------
# netem_args — pure token builder
# ---------------------------------------------------------------------------

class TestNetemArgs:
    def test_all_zero_returns_empty(self):
        """No-op case: when every knob is 0 there is nothing to pass to tc and
        the qdisc add call is skipped entirely."""
        assert netem_args() == []
        assert netem_args(0.0, 0.0, 0.0, 0.0) == []

    def test_delay_only(self):
        # single delay knob: one keyword + one value token
        assert netem_args(delay_ms=20.0) == ["delay", "20.0ms"]

    def test_delay_and_jitter(self):
        # jitter token follows the base delay value immediately (no keyword)
        assert netem_args(delay_ms=20.0, jitter_ms=5.0) == [
            "delay", "20.0ms", "5.0ms"]

    def test_loss_only(self):
        assert netem_args(loss_pct=1.0) == ["loss", "1.0%"]

    def test_rate_only(self):
        assert netem_args(rate_mbit=50.0) == ["rate", "50.0mbit"]

    def test_full_combo_exact_tokens(self):
        """Exact token list for the example in docs/dds-topology-plan.md P5:
        delay 20ms with 5ms jitter, 1% loss, 50 mbit."""
        tokens = netem_args(delay_ms=20.0, jitter_ms=5.0,
                            loss_pct=1.0, rate_mbit=50.0)
        assert tokens == [
            "delay", "20.0ms", "5.0ms",
            "loss", "1.0%",
            "rate", "50.0mbit",
        ]

    def test_jitter_with_zero_delay_is_dropped(self):
        """tc netem requires a base delay for jitter; when delay_ms==0 the
        jitter token must be silently omitted (defence in depth — the Scenario
        validator normally prevents this combination from reaching setup())."""
        assert netem_args(delay_ms=0.0, jitter_ms=5.0) == []


# ---------------------------------------------------------------------------
# NetemConfig — field-level validation
# ---------------------------------------------------------------------------

class TestNetemConfig:
    def test_defaults_all_zero(self):
        c = NetemConfig()
        assert c.delay_ms == 0.0
        assert c.jitter_ms == 0.0
        assert c.loss_pct == 0.0
        assert c.rate_mbit == 0.0

    def test_valid_values_accepted(self):
        c = NetemConfig(delay_ms=10.0, jitter_ms=2.0,
                        loss_pct=0.5, rate_mbit=100.0)
        assert c.delay_ms == 10.0 and c.rate_mbit == 100.0

    def test_negative_delay_rejected(self):
        # ge=0 on delay_ms
        with pytest.raises(ValidationError):
            NetemConfig(delay_ms=-1.0)

    def test_loss_pct_over_100_rejected(self):
        # le=100 on loss_pct
        with pytest.raises(ValidationError):
            NetemConfig(loss_pct=150.0)

    def test_loss_pct_exactly_100_accepted(self):
        # boundary: 100% loss is a valid (if extreme) emulation knob
        c = NetemConfig(loss_pct=100.0)
        assert c.loss_pct == 100.0


# ---------------------------------------------------------------------------
# Scenario — netem cross-field validators
# ---------------------------------------------------------------------------

# Minimal dicts reused across validator tests to keep each test focused on
# the one constraint it exercises.
_RADIO = {
    "freq_hz": 433.0e6,
    "bandwidth_hz": 250.0e3,
    "tx_power_dbm": 27.0,
    "data_rate_bps": 250.0e3,
    "hop": {"n_channels": 50, "hop_rate_hz": 100.0},
}
_NODES = [
    {"id": "v1", "waypoints": [[0, 0]]},
    {"id": "v2", "waypoints": [[0, 0]]},
]


def _scenario(**kw) -> dict:
    """Build a minimal valid scenario dict, merging caller-supplied keys."""
    return {"name": "t", "duration_s": 5.0, "radio": _RADIO,
            "nodes": _NODES, **kw}


class TestScenarioNetemValidator:
    def test_netem_on_bridge_validates(self):
        # Happy path: netem is only meaningful on the bridge substrate (no RF
        # model, no remote tc — just a kernel bridge between veths).
        s = Scenario.model_validate(
            _scenario(substrate="bridge",
                      netem={"delay_ms": 20.0, "jitter_ms": 5.0,
                             "loss_pct": 1.0, "rate_mbit": 50.0}))
        assert s.netem is not None
        assert s.netem.delay_ms == 20.0

    def test_netem_on_channel_raises(self):
        # channel has its own RF/pathloss model; tc netem is semantically
        # redundant and would shadow the EW model's link-quality decisions.
        with pytest.raises(ValidationError, match="substrate='bridge'"):
            Scenario.model_validate(
                _scenario(substrate="channel",
                          netem={"delay_ms": 10.0}))

    def test_netem_on_lan_raises(self):
        # lan runs on real NICs; the orchestrator does not manage tc on remote
        # hosts (no root ssh), so netem on lan is not supported.
        with pytest.raises(ValidationError):
            Scenario.model_validate(
                _scenario(substrate="lan",
                          hosts={"local": {"addr": "192.168.1.1",
                                           "iface": "eth0", "ssh": ""}},
                          netem={"delay_ms": 10.0}))

    def test_jitter_without_delay_raises(self):
        # tc netem treats jitter as a perturbation on a base delay; without a
        # delay the qdisc command would error at runtime.
        with pytest.raises(ValidationError, match="delay_ms > 0"):
            Scenario.model_validate(
                _scenario(substrate="bridge",
                          netem={"delay_ms": 0.0, "jitter_ms": 5.0}))

    def test_netem_none_is_always_valid(self):
        # Omitting netem (None) should never trigger the substrate validator.
        s = Scenario.model_validate(_scenario(substrate="bridge"))
        assert s.netem is None
        s2 = Scenario.model_validate(_scenario(substrate="channel"))
        assert s2.netem is None


# ---------------------------------------------------------------------------
# NetnsTopology constructor — accepts netem without running anything
# ---------------------------------------------------------------------------

class TestNetnsTopologyConstructor:
    def test_accepts_netem_token_list(self):
        """Constructor must store the token list without calling setup()
        (which requires root and real netns / tc)."""
        tokens = ["delay", "20.0ms", "5.0ms", "loss", "1.0%"]
        topo = NetnsTopology(["v1", "v2"], bridge=True, netem=tokens)
        assert topo.netem == tokens
        assert topo.bridge is True

    def test_default_netem_is_none(self):
        # When no netem tokens are provided the topology runs with the ideal
        # kernel bridge link (no qdisc).
        topo = NetnsTopology(["v1", "v2"])
        assert topo.netem is None

    def test_netem_none_on_bridge_is_allowed(self):
        topo = NetnsTopology(["v1", "v2"], bridge=True, netem=None)
        assert topo.netem is None

    def test_empty_token_list_stored_as_is(self):
        # netem_args() returns [] when all knobs are 0; the orchestrator passes
        # None in that case, but the constructor must accept [] if called directly.
        topo = NetnsTopology(["v1"], bridge=True, netem=[])
        assert topo.netem == []


# ---------------------------------------------------------------------------
# Scenario manifest round-trip
# ---------------------------------------------------------------------------

class TestManifestRoundTrip:
    def test_netem_survives_model_dump(self):
        """netem must survive a model_dump() round-trip so the manifest JSON
        written by _run_bridge faithfully records the link-emulation knobs
        used in each run."""
        s = Scenario.model_validate(
            _scenario(substrate="bridge",
                      netem={"delay_ms": 20.0, "jitter_ms": 5.0,
                             "loss_pct": 1.0, "rate_mbit": 50.0}))
        d = s.model_dump()
        assert d["netem"] == {
            "delay_ms": 20.0,
            "jitter_ms": 5.0,
            "loss_pct": 1.0,
            "rate_mbit": 50.0,
        }

    def test_netem_none_dumps_as_none(self):
        s = Scenario.model_validate(_scenario(substrate="bridge"))
        d = s.model_dump()
        assert d["netem"] is None
