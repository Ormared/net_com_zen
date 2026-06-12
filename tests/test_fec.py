"""FEC / interleaving model: erasure-correction over FHSS dwells."""
import math

import pytest

from netcom_zen.channel.fhss import jam_survival_prob, packet_loss_prob
from netcom_zen.channel.linkstate import LinkState


def test_no_fec_reduces_to_all_clean():
    # f=0: survive only if every dwell clean -> (1-p)^k
    for p, k in [(0.2, 5), (0.5, 3), (0.1, 10)]:
        assert jam_survival_prob(p, k, 0.0) == pytest.approx((1 - p) ** k)


def test_survival_matches_binomial_cdf():
    # f=0.4, k=10 -> e=4; P(B<=4), B~Binom(10,0.3)
    p, k, f = 0.3, 10, 0.4
    e = int(f * k)
    expected = sum(math.comb(k, b) * p**b * (1 - p) ** (k - b)
                   for b in range(e + 1))
    assert jam_survival_prob(p, k, f) == pytest.approx(expected, abs=1e-12)


def test_fec_flips_hop_rate_advantage():
    # The headline: at a fixed jammed fraction p below the code rate, MORE dwells
    # (fast hopping) raises survival with FEC, but lowers it without FEC.
    p = 0.2
    no_fec_slow = jam_survival_prob(p, 1, 0.0)    # k=1 (slow hop)
    no_fec_fast = jam_survival_prob(p, 40, 0.0)   # k=40 (fast hop)
    fec_slow = jam_survival_prob(p, 1, 0.4)
    fec_fast = jam_survival_prob(p, 40, 0.4)
    assert no_fec_fast < no_fec_slow              # no FEC: fast hopping worse
    assert fec_fast > fec_slow                    # FEC: fast hopping better
    assert fec_fast > 0.95                         # p < f -> nearly always survives


def test_fec_useless_when_jammed_fraction_exceeds_code():
    # p > f: even fast hopping with FEC fails (law of large numbers)
    assert jam_survival_prob(0.6, 60, 0.3) < 0.05


def test_survival_edge_cases():
    assert jam_survival_prob(0.0, 10, 0.0) == 1.0     # no jamming
    assert jam_survival_prob(1.0, 10, 0.0) == 0.0     # all dwells jammed, no FEC
    assert jam_survival_prob(1.0, 10, 1.0) == 1.0     # f>=1 corrects everything


def test_packet_loss_prob_with_fec():
    # PER = 1 - (1-per_clear)*survive
    surv = jam_survival_prob(0.2, 8, 0.5)
    assert packet_loss_prob(0.1, 0.2, 1.0, 8, 0.5) == pytest.approx(
        1 - (1 - 0.1) * surv)


def test_linkstate_fec_reduces_jam_drops():
    base = dict(src="a", dst="b", prx_dbm=-80, noise_dbm=-113,
                jam_inchannel_dbm=-60, data_rate_bps=250e3, hop_rate_hz=1000,
                prop_delay_s=1e-6, foliage_db=0.0, terrain_db=0.0)
    # strong partial-band jammer, fast hopping; count jam verdicts over the
    # seeded stream with and without FEC
    import numpy as np

    def jam_rate(fec):
        st = LinkState(rho=0.3, fec_fraction=fec, **base)
        rng = np.random.default_rng(1)
        v = [st.verdict(64, 0.999, rng.random()) for _ in range(2000)]
        return v.count("jam") / len(v)

    assert jam_rate(0.5) < jam_rate(0.0)   # FEC cuts jam-caused drops
