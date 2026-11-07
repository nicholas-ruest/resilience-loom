#!/usr/bin/env bash
set -euo pipefail
grep -q '/v1/recovery-plans' openapi.yaml
grep -q '/v1/recovery-plans' frontend/src/client.ts
echo CONTRACT_PASS
