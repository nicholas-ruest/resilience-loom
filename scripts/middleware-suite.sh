#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
cargo test --locked -p rl-auth -p rl-middleware -p rl-policy -p rl-regorus
