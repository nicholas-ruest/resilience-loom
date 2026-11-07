# ADR-0007: Least-authority execution scopes

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Recovery tasks need explicit resource scopes; granting the entire response cell a shared unrestricted capability makes denial evidence meaningless.

## Decision
Derive task-specific scopes from the validated graph and include their digest in the recovery contract. No scope is emitted for a rejected plan.

## Consequences and trade-offs
Scope generation is deterministic and auditable, at the cost of more contract material. The frontend renders the issued contract rather than inferring privileges.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-contract digest and scope tests; correlation trace records contract creation before persistence.
