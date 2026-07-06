"""Reactive (follower) jammer model cross-checked against a direct time-domain
dwell simulation, and the packet-level consequence (fast hopping outruns a
follower jammer) verified through the shared binomial-CDF survival model."""
import numpy as np
import pytest

from netcom_zen.channel.fhss import jam_survival_prob
from netcom_zen.config import JammerConfig
from netcom_zen.ew import Jammer


def _reactive(react_latency_s, detection_prob=1.0):
    return Jammer(JammerConfig(
        id="jr", kind="reactive", position=(0.0, 0.0), tx_power_dbm=30.0,
        react_latency_s=react_latency_s, detection_prob=detection_prob))


def test_lock_prob_matches_time_domain_simulation():
    # Direct timing model: the transmitter holds each channel for T_dwell. A
    # follower needs react_latency_s to sense+retune, so it jams the tail
    # window [i*T + tau, (i+1)*T). A symbol dropped at a uniform-random instant
    # in the dwell is erased iff it lands in that window AND the jammer's
    # (Bernoulli) sensing succeeded. Empirical erasure rate must match
    # detection_prob * dwell_exposure — the value occupancy() reports as rho.
    rng = np.random.default_rng(3)
    hop_rate, tau, p_det, trials = 500.0, 0.001, 0.8, 400_000
    t_dwell = 1.0 / hop_rate
    j = _reactive(tau, detection_prob=p_det)

    offset = rng.random(trials) * t_dwell          # symbol instant within dwell
    in_window = offset >= tau                       # jammer locked on in time
    sensed = rng.random(trials) < p_det             # detection succeeded
    sim = float(np.mean(in_window & sensed))

    rho, frac = j.occupancy(n_channels=64, channel_bw_hz=250e3, hop_rate_hz=hop_rate)
    assert frac == 1.0                              # all power on the active channel
    assert sim == pytest.approx(rho, abs=3 * np.sqrt(sim * (1 - sim) / trials))


def test_fast_hopping_outruns_the_follower():
    # Once the hop period drops below the sense+tune latency the follower can
    # never catch a dwell: exposure -> 0, lock prob -> 0, packets survive.
    j = _reactive(react_latency_s=0.002)  # 2 ms to sense + retune
    slow = j.occupancy(64, 250e3, hop_rate_hz=100.0)[0]   # T_dwell = 10 ms
    fast = j.occupancy(64, 250e3, hop_rate_hz=500.0)[0]   # T_dwell =  2 ms
    assert slow == pytest.approx(0.8)   # catches 80% of each slow dwell
    assert fast == pytest.approx(0.0)   # tx has hopped away before it locks


def test_detection_prob_caps_lock_rate():
    # Even a jammer fast enough to catch every dwell only lands its configured
    # sensing reliability fraction of them.
    j = _reactive(react_latency_s=0.0, detection_prob=0.5)
    rho, _ = j.occupancy(64, 250e3, hop_rate_hz=100.0)
    assert rho == pytest.approx(0.5)


def test_reactive_survival_flips_with_hop_rate():
    # Packet-level: the reactive lock prob feeds the same binomial-CDF survival
    # model as partial-band jamming. Against a follower, faster hopping both
    # shrinks the per-dwell lock prob and (with FEC) spreads residual hits, so
    # survival rises monotonically with hop rate — the opposite of a follower's
    # intent to punish agility.
    j = _reactive(react_latency_s=0.0015)
    # packet spans k dwells; k grows with hop rate for a fixed-duration packet
    def survival(hop_rate, k, fec=0.25):
        rho = j.occupancy(64, 250e3, hop_rate_hz=hop_rate)[0]
        return jam_survival_prob(rho, k, fec)
    s_slow = survival(200.0, k=1)     # T_dwell 5 ms: catches 70% of the one dwell
    s_med = survival(500.0, k=3)      # T_dwell 2 ms: catches 25%, spread over 3
    s_fast = survival(1000.0, k=6)    # T_dwell 1 ms < 1.5 ms: never locks
    assert s_slow < s_med < s_fast
    assert s_fast == pytest.approx(1.0)
