# ADR-0011: Prototype session authentication

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
The browser needs a real authentication boundary without pretending that a static development token is production identity.

## Decision
Require a bearer token on protected versioned API routes and derive a stable prototype principal only after constant-path verification.

## Consequences and trade-offs
The mechanism is explicitly limited to local evaluation, while tests still exercise missing and incorrect credentials. Production deployment must replace it with workload identity.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-auth and API unauthorized tests; denial journey begins with a rejected token and then recovers.
