#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export PATH=/home/ruv/.local/share/dream-machine/runtime/node-v24.21.0-linux-x64/bin:/home/ruv/.cargo/bin:$PATH
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
RUN_DIR=${DREAM_MACHINE_RUN_DIR:-/tmp/resilience-loom-runtime}
DATA_PATH=${MM_DATA:-$RUN_DIR/recovery-plans.rvf}
mkdir -p "$RUN_DIR"
auth=(-H 'Authorization: Bearer demo-token')
body='{"objective":"Restore this governed recovery_plan after a real service restart","sensitivity":"internal"}'
response=$(curl -fsS "${auth[@]}" -H 'Content-Type: application/json' -d "$body" http://127.0.0.1:18080/v1/recovery-plans)
id=$(printf '%s' "$response" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>console.log(JSON.parse(s).id))')
kill "$(cat /tmp/resilience-loom-backend.pid)"
for _ in $(seq 1 30); do curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 || break; sleep 1; done
MM_BIND=127.0.0.1:18080 MM_DATA="$DATA_PATH" MM_TOKEN=demo-token RUST_LOG=info cargo run --locked -p rl-cli >>"$RUN_DIR/backend.log" 2>&1 &
echo $! > /tmp/resilience-loom-backend.pid
for _ in $(seq 1 60); do curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 && break; sleep 1; done
curl -fsS "${auth[@]}" http://127.0.0.1:18080/v1/recovery-plans | node -e 'let s="";const id=process.argv[1];process.stdin.on("data",d=>s+=d).on("end",()=>{const a=JSON.parse(s);if(!a.some(x=>x.id===id))process.exit(1);console.log("RESTART_RETRIEVAL_PASS "+id)})' "$id"
