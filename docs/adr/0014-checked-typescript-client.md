# ADR-0014: Checked TypeScript recovery client

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Handwritten ad hoc fetch calls allow endpoint, header, and response drift to reach the browser unnoticed.

## Decision
Centralize all network access in a typed TypeScript client checked against the committed OpenAPI schema. The client attaches auth and correlation headers and normalizes errors.

## Consequences and trade-offs
Views cannot bypass middleware or use mocked first-party routes in acceptance tests. Client compilation is part of every product verification.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Frontend typecheck, client unit tests, check-contract.sh, and Playwright network assertions.
