"""Preflight: verify the host can run scenarios (architecture.md)."""
import importlib.util
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
    ok &= check("`ethtool` available", os.path.exists("/usr/sbin/ethtool"),
                "install ethtool (needed to disable veth checksum offload)")
    if os.geteuid() == 0:
        probe = subprocess.run(["ip", "netns", "add", "ncz-probe"], capture_output=True)
        subprocess.run(["ip", "netns", "del", "ncz-probe"], capture_output=True)
        ok &= check("netns create/delete", probe.returncode == 0)
    else:
        check("root", False,
              "scenario runs need root: `sudo $(which python) -m netcom_zen.run ...`")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
