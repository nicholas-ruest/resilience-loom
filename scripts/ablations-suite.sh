#!/usr/bin/env bash
set -euo pipefail
./scripts/benchmark.sh
node - <<'NODE'
const rows = require(process.env.DREAM_MACHINE_EVIDENCE_DIR + '/benchmark.json');
const names = new Set(rows.map(row => row.mode));
for (const name of ['combined','without_ruvector','without_rvf','without_regorus','without_calibration']) {
  if (!names.has(name)) throw new Error('missing ablation '+name);
}
console.log('ABLATIONS_PASS '+[...names].sort().join(','));
NODE
