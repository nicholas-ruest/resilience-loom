# ADR-0013: Versioned typed recovery API

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Frontend and backend evolution must fail visibly when recovery plan fields or middleware errors change.

## Decision
Publish an OpenAPI v1 schema for health, create, list, and retrieve operations and keep Rust response types aligned with it.

## Consequences and trade-offs
Breaking changes require a new API version. Error envelopes always include code, message, and correlation ID.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Contract drift script, API serialization tests, and browser journeys against the v1 routes.
