#!/bin/bash
set -euo pipefail

# Build docs
just docs

# Cross-compile for Linux
cross build --release --target x86_64-unknown-linux-musl

# Rsync binary
rsync -avz ../../target/x86_64-unknown-linux-musl/release/server $TARGET_NODE:/root/hotpath-backend/server

# Rsync static assets
rsync -azr --delete html/ $TARGET_NODE:/root/hotpath-backend/html
rsync -azr --delete assets/ $TARGET_NODE:/root/hotpath-backend/assets
