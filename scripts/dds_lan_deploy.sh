#!/usr/bin/env bash
# scripts/dds_lan_deploy.sh <ssh-target>
#
# Prepare the long-lived dds-lab container on a remote host for substrate=lan
# benchmark runs.  Run this once (or after a pixi.lock / Dockerfile change)
# before invoking any lan scenario — the orchestrator's _run_lan assumes the
# container already exists and the ros2 env is installed.
#
# What this script does (five numbered steps that match the log output):
#   1. rsync the repo to remote ~/net_com_zen, excluding blobs that are either
#      host-specific (.pixi, results) or unnecessary (git history, caches).
#   2. Build the dds-lab Docker image on the remote if the Dockerfile changed
#      since the last build (detected via a md5 label baked into the image).
#   3. (Re)create the container: docker rm -f then docker run -d --network host
#      with the bind mount ~/net_com_zen:/work.
#   4. Install the ros2 pixi env inside the container only when pixi.lock
#      changed since the last install (stamp file in ~/net_com_zen/).
#   5. Verify the env by importing rclpy inside the container.
#
# Prerequisites on the remote:
#   - docker (or a podman-shim that responds to `docker` commands) accessible
#     WITHOUT sudo (user in the podman/docker group).
#   - rsync installed.
#   - SSH key auth from the local machine (no password prompts).
#
# Usage:
#   bash scripts/dds_lan_deploy.sh orm_small_nix@192.168.1.9

set -euo pipefail

target="${1:?Usage: $0 <ssh-target>}"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "==> [1/5] Syncing repo to ${target}:~/net_com_zen ..."
# --delete keeps the remote in sync (removes files deleted locally).
# Exclude blobs that are host-specific or would waste bandwidth:
#   .pixi    — rebuilt inside the container from pixi.lock
#   results/ — written by the orchestrator, owned by each host
#   agent/target — Rust build artefacts (not needed inside the container)
rsync -az --delete \
    --exclude='.git' \
    --exclude='.pixi' \
    --exclude='results' \
    --exclude='agent/target' \
    --exclude='__pycache__' \
    --exclude='.pytest_cache' \
    "${repo_root}/" \
    "${target}:~/net_com_zen/"

echo "==> [2/5] Checking dds-lab image on ${target} ..."
# Embed the Dockerfile md5 as a Docker image label at build time.  On a
# subsequent run we read that label back and skip the (slow) build if the
# Dockerfile has not changed.  docker image inspect exits non-zero when the
# image does not exist, so the || echo '' fallback treats that as a mismatch.
local_md5="$(md5sum "${repo_root}/docker/dds-lab/Dockerfile" | cut -d' ' -f1)"
remote_md5="$(ssh "$target" \
    "docker image inspect dds-lab \
     --format '{{index .Config.Labels \"dockerfile_md5\"}}' 2>/dev/null \
     || echo ''")"

if [ "$local_md5" != "$remote_md5" ]; then
    echo "    Dockerfile changed or image missing — building dds-lab ..."
    ssh "$target" \
        "docker build \
            --label dockerfile_md5=${local_md5} \
            -t dds-lab \
            ~/net_com_zen/docker/dds-lab"
else
    echo "    Image is up to date — skipping build."
fi

echo "==> [3/5] (Re)creating container dds-lab on ${target} ..."
# Always recreate so the bind-mount source and any env var changes take effect.
# --network host: the container shares the host NIC directly, which is required
# for DDS multicast discovery and for zenoh router TCP listen on the LAN IP.
ssh "$target" \
    "docker rm -f dds-lab 2>/dev/null; \
     docker run -d \
         --name dds-lab \
         --network host \
         -v ~/net_com_zen:/work \
         -w /work \
         dds-lab sleep infinity"

echo "==> [4/5] Installing ros2 pixi env (checking pixi.lock stamp) ..."
# Rerunning `pixi install` on an unchanged lock is cheap but still takes a
# few seconds.  A md5 stamp file avoids that on repeat deploys.
lock_hash="$(md5sum "${repo_root}/pixi.lock" | cut -d' ' -f1)"
remote_stamp="$(ssh "$target" \
    "cat ~/net_com_zen/.pixi_env_stamp 2>/dev/null || echo ''")"

if [ "$lock_hash" != "$remote_stamp" ]; then
    echo "    pixi.lock changed — running pixi install -e ros2 (may take minutes) ..."
    ssh "$target" "docker exec dds-lab pixi install -e ros2"
    # Write the new stamp so we skip install on the next unchanged deploy.
    ssh "$target" "printf '%s' '${lock_hash}' > ~/net_com_zen/.pixi_env_stamp"
else
    echo "    pixi.lock unchanged — skipping pixi install."
fi

echo "==> [5/5] Verifying ros2 env on ${target} ..."
ssh "$target" \
    "docker exec dds-lab \
     .pixi/envs/ros2/bin/python -c 'import rclpy; print(\"rclpy OK\")'"

echo ""
echo "==> dds-lab is ready on ${target}."
echo "    zenohd: \$(docker exec dds-lab ls .pixi/envs/ros2/lib/rmw_zenoh_cpp/rmw_zenohd)"
