#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export TMPDIR=${TMPDIR:-/tmp/resilience-loom-container-tmp}
mkdir -p "$TMPDIR"
if command -v podman >/dev/null 2>&1; then runtime=podman; elif command -v docker >/dev/null 2>&1; then runtime=docker; else echo 'no container runtime' >&2; exit 1; fi
tag=${DREAM_MACHINE_SOURCE_SHA:-local}
backend="resilience-loom-backend:${tag:0:12}"
frontend="resilience-loom-frontend:${tag:0:12}"
bname="rl-backend-${tag:0:8}"
fname="rl-frontend-${tag:0:8}"
cleanup() { "$runtime" rm -f "$bname" "$fname" >/dev/null 2>&1 || true; }
trap cleanup EXIT
cleanup
./scripts/stop.sh || true
"$runtime" build --network=host --no-cache -f Dockerfile.backend -t "$backend" .
"$runtime" build --network=host --no-cache -f Dockerfile.frontend -t "$frontend" .
"$runtime" run -d --network=host --name "$bname" -e MM_BIND=127.0.0.1:18080 -e MM_DATA=/tmp/recovery-plans.rvf -e MM_TOKEN=demo-token "$backend"
"$runtime" run -d --network=host --name "$fname" "$frontend"
for _ in $(seq 1 90); do
  if curl -fsS http://127.0.0.1:18080/readyz >/dev/null 2>&1 && curl -fsS http://127.0.0.1:18081/ >/dev/null 2>&1; then break; fi
  sleep 2
done
curl -fsS http://127.0.0.1:18080/readyz
curl -fsS http://127.0.0.1:18081/ | grep -q '<div id="root">'
response=$(curl -fsS -H 'Authorization: Bearer demo-token' -H 'Content-Type: application/json' -d '{"objective":"Container topology recovery drill","sensitivity":"internal"}' http://127.0.0.1:18080/v1/recovery-plans)
printf '%s' "$response" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{const x=JSON.parse(s);if(!x.id||x.upstream_analysis?.ruflo?.topology!=="hierarchical")process.exit(1);console.log("CONTAINER_PRODUCT_PASS "+x.id)})'
"$runtime" inspect "$bname" "$fname" --format '{{.Config.Image}} {{.State.Status}}'
