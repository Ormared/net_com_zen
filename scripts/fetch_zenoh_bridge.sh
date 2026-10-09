#!/usr/bin/env bash
# Fetch the zenoh-bridge-ros2dds standalone binary into vendor/.
#
# Why vendored rather than a pixi dependency: zenoh-plugin-ros2dds is not
# packaged in conda-forge or robostack-jazzy (checked 2026-08-22), so the
# upstream GitHub release binary is the only distribution channel. It lands in
# vendor/ (repo-relative) because the dds-lab container bind-mounts the repo at
# /work -- that makes the same bit-for-bit binary available on every LAN host
# with no image rebuild, matching the parity argument in docker/dds-lab/Dockerfile.
#
# The binary is statically linked (libc/libm/libgcc only) and embeds its own
# CycloneDDS via cyclors -- it does NOT use the ros2 pixi env's DDS libraries.
set -euo pipefail

VERSION="${ZENOH_BRIDGE_VERSION:-1.10.0}"
TARGET="x86_64-unknown-linux-gnu"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="$REPO_ROOT/vendor/zenoh-bridge-ros2dds"
BIN="$DEST/zenoh-bridge-ros2dds"

if [ -x "$BIN" ] && [ "${FORCE:-0}" != "1" ]; then
    echo "already present: $BIN ($("$BIN" --version 2>&1 | head -1))"
    exit 0
fi

URL="https://github.com/eclipse-zenoh/zenoh-plugin-ros2dds/releases/download/${VERSION}/zenoh-plugin-ros2dds-${VERSION}-${TARGET}-standalone.zip"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "fetching $URL"
curl -fsSL -o "$TMP/bridge.zip" "$URL"
unzip -q -o "$TMP/bridge.zip" -d "$TMP"
mkdir -p "$DEST"
install -m 0755 "$TMP/zenoh-bridge-ros2dds" "$BIN"
# The .so is the zenohd plugin form of the same code; unused by the standalone
# bridge but kept so a future router-plugin variant needs no second download.
install -m 0644 "$TMP/libzenoh_plugin_ros2dds.so" "$DEST/" 2>/dev/null || true

# Record provenance next to the binary: the version is otherwise invisible in
# a result manifest, and bridge behaviour is version-sensitive.
"$BIN" --version > "$DEST/VERSION" 2>&1 || echo "$VERSION" > "$DEST/VERSION"
sha256sum "$BIN" | awk '{print $1}' > "$DEST/SHA256"

echo "installed $BIN"
cat "$DEST/VERSION"
