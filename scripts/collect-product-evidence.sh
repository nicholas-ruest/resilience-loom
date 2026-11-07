#!/usr/bin/env bash
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"
E=${DREAM_MACHINE_EVIDENCE_DIR:?external evidence directory required}
GATE=(node scripts/dream-machine-product-gate.mjs record "$ROOT" "$E")
export PATH=/home/ruv/.local/share/dream-machine/runtime/node-v24.21.0-linux-x64/bin:/home/ruv/.cargo/bin:$PATH
export PLAYWRIGHT_BROWSERS_PATH=${PLAYWRIGHT_BROWSERS_PATH:-/home/ruv/.cache/ms-playwright}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
status=0
record() {
  local suite="$1"; shift
  "${GATE[@]}" "$suite" -- "$@" || status=1
}
record rust bash scripts/rust-suite.sh
record backend cargo metadata --locked --offline --format-version 1
record frontend bash scripts/frontend-suite.sh
record api bash scripts/api-suite.sh
record middleware bash scripts/middleware-suite.sh
record integrations bash scripts/integrations-suite.sh
record benchmark bash scripts/benchmark.sh
record ablations bash scripts/ablations-suite.sh
record containers bash scripts/container-suite.sh
record browser bash scripts/browser-suite.sh
if (( status != 0 )); then
  printf 'EVIDENCE_COLLECTION_FAILED\n' >&2
  exit "$status"
fi
node scripts/build-product-evidence.mjs
printf 'PRODUCT_EVIDENCE_COLLECTED source=%s\n' "$DREAM_MACHINE_SOURCE_SHA"
