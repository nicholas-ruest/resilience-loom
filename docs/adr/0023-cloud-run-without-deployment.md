# ADR-0023: Cloud Run configuration without deployment

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Delivery requires a production-shaped rollback plan, but this run has no authority to deploy or incur cloud spend.

## Decision
Provide declarative service configuration, health probes, immutable image placeholders, and rollback instructions without invoking cloud APIs.

## Consequences and trade-offs
Operators can review the topology and later bind approved images. Local containers remain the executed delivery proof.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Deployment manifest validation and documentation review; no deploy command exists in verification.
