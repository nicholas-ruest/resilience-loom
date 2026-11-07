# ADR-0017: Bounded upstream execution

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Five Node-based product integrations could hang the Rust request path or emit unbounded output during an incident.

## Decision
Run the locked upstream engine with a request timeout, bounded JSON stdout contract, inherited correlation ID, and explicit non-zero exit mapping.

## Consequences and trade-offs
The architecture keeps one executable boundary while preserving distinct upstream responsibilities. Timeout or parse failure aborts before persistence.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-upstream-runtime success, timeout/invalid-output tests and all readiness failure receipts.
