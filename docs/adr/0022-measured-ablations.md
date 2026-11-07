# ADR-0022: Measured ingredient ablations

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
A seven-project composition is credible only if each ingredient changes a measurable product behavior or quality signal.

## Decision
Run a combined benchmark and remove each major ingredient through supported test controls, recording degraded recall, policy, coordination, optimization, or trace signals.

## Consequences and trade-offs
Ablations are evaluation-only and cannot become production fallbacks. The full workflow remains the acceptance target.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
scripts/benchmark.sh and ingredient-ablation receipt with deterministic JSON results.
