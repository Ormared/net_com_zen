"""Model validation against published/analytical reference values (ADR-0001:
this suite is what makes hand-rolled benchmark results defensible)."""
import numpy as np
import pytest

from netcom_zen.channel.linkbudget import fsk_per, noise_dbm
from netcom_zen.propagation.diffraction import knife_edge_loss_db
from netcom_zen.propagation.foliage import weissberger_db
from netcom_zen.propagation.freespace import friis_db

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
