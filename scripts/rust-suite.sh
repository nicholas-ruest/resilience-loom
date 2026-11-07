#!/usr/bin/env bash
set -euo pipefail
export RESILIENCE_LOOM_UPSTREAM_SCRIPT="$PWD/upstreams/resilience-engine.mjs"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo deny check
