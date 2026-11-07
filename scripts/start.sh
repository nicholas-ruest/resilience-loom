#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH=/home/ruv/.local/share/dream-machine/runtime/node-v24.21.0-linux-x64/bin:/home/ruv/.cargo/bin:$PATH
if ! test -x upstreams/node_modules/.bin/claude-flow; then npm --prefix upstreams ci --ignore-scripts --legacy-peer-deps --omit=optional --no-audit --no-fund; fi
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
RUN_DIR=${DREAM_MACHINE_RUN_DIR:-/tmp/resilience-loom-runtime}
mkdir -p "$RUN_DIR"
DATA_PATH=${MM_DATA:-$RUN_DIR/recovery-plans.rvf}
MM_BIND=127.0.0.1:18080 MM_DATA="$DATA_PATH" MM_TOKEN=demo-token RUST_LOG=info cargo run --locked -p rl-cli >"$RUN_DIR/backend.log" 2>&1 &
echo $! > /tmp/resilience-loom-backend.pid
(cd frontend && exec ./node_modules/.bin/vite --host 127.0.0.1 --port 5173) >"$RUN_DIR/frontend.log" 2>&1 &
echo $! > /tmp/resilience-loom-frontend.pid
for _ in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 && curl -fsS http://127.0.0.1:5173 >/dev/null 2>&1; then printf 'STACK_READY data=%s\n' "$DATA_PATH"; exit 0; fi
  sleep 1
done
printf 'readiness timeout\n' >&2
exit 1
