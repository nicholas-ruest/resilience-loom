#!/usr/bin/env bash
set -euo pipefail
./scripts/rust-suite.sh
./scripts/frontend-suite.sh
./scripts/check-contract.sh
DREAM_MACHINE_RUN_DIR=/tmp/resilience-loom-slice ./scripts/start.sh
trap './scripts/stop.sh' EXIT
curl -fsS http://127.0.0.1:18080/readyz
