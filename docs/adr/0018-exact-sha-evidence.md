# ADR-0018: Exact-source evidence binding

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Passing commands are meaningless if receipts, screenshots, or semantic review refer to a different source tree.

## Decision
Record every suite against a clean immutable git SHA; hash stdout, stderr, reports, traces, screenshots, and selection artifacts in schema-v2 evidence.

## Consequences and trade-offs
Any source change requires fresh evidence. Generated evidence remains outside the checkout to avoid circular source hashes.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Canonical recorder and verifier tests plus manifest hash read-back.
