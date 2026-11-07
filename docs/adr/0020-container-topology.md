# ADR-0020: Container topology with Node upstream runtime

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
The Rust backend executes locked npm integrations, so a Rust-only image would pass compilation yet fail the real product path.

## Decision
Build a backend image containing the Rust service, Node 24 runtime, locked upstream packages, policy, and static schema; pair it with a separately built frontend and shared health checks.

## Consequences and trade-offs
The image is larger than a Rust-only binary but faithfully matches production behavior. Dependency installation remains locked and offline-capable after build.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Podman Compose build/readiness test and primary journey against the container topology.
