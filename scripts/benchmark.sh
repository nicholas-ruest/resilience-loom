#!/usr/bin/env bash
set -euo pipefail
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/resilience-loom-target}
OUT_DIR=${DREAM_MACHINE_EVIDENCE_DIR:-/tmp/resilience-loom-benchmark}
mkdir -p "$OUT_DIR"
cargo run --locked -p rl-evaluation --example ablation > "$OUT_DIR/benchmark.json"
node -e 'const x=require(process.argv[1]);if(!x.find(r=>r.mode==="combined"&&r.success)||x.find(r=>r.mode==="without_rvf").restart_retrieval)process.exit(1);console.log("BENCHMARK_AND_ABLATION_PASS rows="+x.length)' "$OUT_DIR/benchmark.json"
