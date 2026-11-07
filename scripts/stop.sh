#!/usr/bin/env bash
set -euo pipefail
for f in /tmp/resilience-loom-backend.pid /tmp/resilience-loom-frontend.pid;do if test -f "$f";then kill "$(cat "$f")" 2>/dev/null || true;rm -f "$f";fi;done
