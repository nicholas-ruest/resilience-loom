#!/usr/bin/env bash
set -euo pipefail
: "${DREAM_MACHINE_EVIDENCE_DIR:?evidence directory required}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
E="$DREAM_MACHINE_EVIDENCE_DIR"
export DREAM_MACHINE_RUN_DIR="$E/runtime"
export MM_DATA="$E/runtime/recovery-plans.rvf"
mkdir -p "$E/runtime"
"$ROOT/scripts/stop.sh" || true
cleanup() { "$ROOT/scripts/stop.sh" || true; }
trap cleanup EXIT
"$ROOT/scripts/start.sh"
set +e
(cd "$ROOT/frontend" && ./node_modules/.bin/playwright test)
status=$?
set -e
"$ROOT/scripts/stop.sh" || true
sleep 1
grep '"correlation_id"' "$E/runtime/backend.log" > "$E/correlation-trace.jsonl"
node -e 'const fs=require("fs"),p=process.argv[1],rows=fs.readFileSync(p,"utf8").trim().split("\n").filter(Boolean).map(line=>JSON.parse(line)); const events=new Set(rows.map(row=>row.fields?.event)); for(const event of ["request.accepted","request.failed","upstreams.executed","persistence.written","persistence.read"]) if(!events.has(event)) throw new Error("missing correlated runtime event "+event); if(rows.some(row=>!row.fields?.correlation_id)) throw new Error("uncorrelated runtime row"); console.log("CORRELATION_TRACE_PASS rows="+rows.length);' "$E/correlation-trace.jsonl"
trace=$(find "$E/playwright-results" -type f -name trace.zip | sort | head -1)
test -n "$trace"
cp "$trace" "$E/browser-trace.zip"
test -s "$E/browser-report.json"
test -s "$E/desktop.png"
test -s "$E/mobile.png"
printf 'BROWSER_EVIDENCE_PASS report=%s trace=%s\n' "$E/browser-report.json" "$E/browser-trace.zip"
exit "$status"
