# ADR-0008: RuVector analogue recall

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Operators need similar incident outcomes, but a database-shaped service boundary would violate the authorized product storage model.

## Decision
Use ruvnet RuVector inside rl-ruvector for in-process nearest-neighbour recall over deterministic incident embeddings.

## Consequences and trade-offs
Recall becomes part of plan explanation and is rebuilt from authorized persisted records at startup. Dimension mismatch fails before search.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-ruvector insert, search, and dimension tests; readiness receipt and durable retrieval trace.
