# ADR-0009: RVF durable recovery persistence

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Restart retrieval requires durable state without introducing an unapproved service-owned relational database.

## Decision
Persist recovery plan vectors and records through the RVF boundary with atomic replacement, then rebuild the in-memory index from the journal on startup.

## Consequences and trade-offs
The local file is portable and deterministic. Corruption is surfaced as a persistence failure rather than reset silently.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-rvf round-trip and corrupt-file tests; browser restart journey proves read-back after process replacement.
