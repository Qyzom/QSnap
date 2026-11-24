#!/usr/bin/env bash
set -e

echo "=== Building QSnap Studio Linux Release ==="
cargo build --release --manifest-path "$(dirname "$0")/../../Cargo.toml"

BIN_PATH="$(dirname "$0")/../../target/release/qsnap"
if [ -f "$BIN_PATH" ]; then
    echo "Release binary ready at: $BIN_PATH"
    ls -lh "$BIN_PATH"
else
    echo "Error: Binary not found!"
    exit 1
fi
