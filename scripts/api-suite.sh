#!/usr/bin/env bash
set -euo pipefail
./scripts/check-contract.sh
cargo test --locked -p rl-api
