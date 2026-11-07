# ADR-0012: Idempotency and correlation propagation

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Retries during an incident can otherwise create duplicate durable plans and traces that cannot be joined across browser, API, Rust, and Node boundaries.

## Decision
Use a caller-supplied correlation ID as the idempotency key. Return an existing plan for the same key and propagate it through middleware, telemetry, upstream commands, audit, and response headers.

## Consequences and trade-offs
Retry safety costs a repository lookup but prevents duplicate downstream effects. Missing IDs are generated once at ingress.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-middleware, rl-orchestrator idempotency tests, and the correlated raw journey trace.
