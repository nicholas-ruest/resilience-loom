#!/usr/bin/env bash
set -euo pipefail
export RESILIENCE_LOOM_UPSTREAM_SCRIPT="$PWD/upstreams/resilience-engine.mjs"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
payload='{"incident":"database payment outage with queue saturation","correlation_id":"integration-final"}'
printf '%s' "$payload" | node upstreams/resilience-engine.mjs analyze
for dependency in agentdb dspy sublinear ruflo; do node upstreams/resilience-engine.mjs failure "$dependency"; done
cargo test --locked -p rl-upstream-runtime -p rl-ruvector -p rl-rvf -p rl-regorus -p rl-telemetry
printf 'INTEGRATIONS_PASS ruvnet=5 enterprise=2\n'
