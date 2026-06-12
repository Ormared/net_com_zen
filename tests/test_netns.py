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
