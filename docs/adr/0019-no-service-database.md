# ADR-0019: Authorized local storage boundary

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
The product needs restart durability but may not silently add a managed database, deployment, or external spend.

## Decision
Use project-owned local RVF and append-only audit files mounted through one explicit data directory. No network storage is contacted.

## Consequences and trade-offs
The topology is simple and Cloud Run compatible through a replaceable volume boundary, but horizontal multi-writer operation is not claimed.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Repository, RVF, audit tests and container restart journey.
