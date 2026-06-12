import os

import pytest


def pytest_collection_modifyitems(config, items):
    if os.geteuid() == 0:
        return
    skip = pytest.mark.skip(reason="requires root; run `pixi run sudo-test`")
    for item in items:
        if "sudo" in item.keywords:
            item.add_marker(skip)
