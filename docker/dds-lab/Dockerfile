# dds-lab container image
#
# Provides a portable pixi environment for cross-host DDS benchmark runs
# (substrate=lan). The repo itself is BIND-MOUNTED at /work at run time so
# the image stays generic — no repo code is baked in.  A long-lived container
# is kept alive with `sleep infinity` and nodes are spawned via `docker exec`.
#
# The pixi ros2 env inside the container is the exact same RoboStack Jazzy
# build used on the local machine (same pixi.toml / pixi.lock), which keeps
# the Fast DDS / Cyclone / Zenoh versions bit-for-bit identical across hosts.
# That parity is what makes cross-host latency and throughput numbers directly
# comparable.

FROM ghcr.io/prefix-dev/pixi:noble

# Tools required by the deploy workflow and the orchestrator teardown path:
#   openssh-client — ssh / scp for communicating with the local machine
#   rsync          — collect per-node result JSONL files back to the local host
#   procps         — pkill / pgrep used by _run_lan finally-block cleanup
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        openssh-client \
        rsync \
        procps && \
    rm -rf /var/lib/apt/lists/*

# The bind mount (-v ~/net_com_zen:/work) provides the repo at /work.
# pixi uses PIXI_HOME or the CWD's pixi.toml; /work is where pixi.toml lives.
WORKDIR /work
