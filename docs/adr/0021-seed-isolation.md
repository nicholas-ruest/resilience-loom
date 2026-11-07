# ADR-0021: Test namespace isolation

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Concurrent or repeated evidence runs must not share plans, ports, or traces because that would make denial and retrieval assertions ambiguous.

## Decision
Assign every run a unique test namespace, data directory, correlation prefix, and port set; reset only that namespace.

## Consequences and trade-offs
Artifacts can be compared deterministically and cleanup cannot erase another run. The namespace is recorded in manifest metadata.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Start/reset scripts and container/browser suite receipts.
