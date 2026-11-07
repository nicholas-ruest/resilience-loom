# ADR-0015: Deterministic synthetic incident seed

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
A reproducible product demonstration needs realistic incident inputs without importing sensitive production data.

## Decision
Provide a deterministic seed that creates stable response agents and a representative checkout-latency objective through the same public API used by the browser.

## Consequences and trade-offs
Seed and reset are safe to repeat and never bypass policy or upstream integrations. Real tenants are an explicit non-goal.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-seed tests, reset script, and primary browser journey.
