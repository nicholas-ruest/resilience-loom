# ADR-0005: Separation of duty before execution

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
The same responder must not both propose and approve a high-impact recovery action because that collapses the control boundary.

## Decision
Require distinct researcher, reviewer, and operator identities before policy authorization. Team formation rejects any cell that cannot meet the separation invariant.

## Consequences and trade-offs
Synthetic operation remains deterministic while still demonstrating independent roles. The invariant is enforced in domain code before Regorus and persistence.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-team assembly and duplicate-role failure tests plus the denial journey.
