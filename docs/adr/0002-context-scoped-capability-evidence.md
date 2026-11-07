# ADR-0002: Context-scoped capability evidence

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
An operator qualified for telemetry triage is not automatically qualified for policy review or execution. Global scores hide the evidence needed for a specific incident task.

## Decision
Compute coverage per task role and return the assigned agent identifiers in a typed CoverageReport. Missing roles are dependency failures, not silent substitutions.

## Consequences and trade-offs
Recovery plans expose why a cell is adequate and fail closed when the seeded registry cannot cover a role. This favors explicit evidence over opportunistic assignment.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-capability success and uncovered-role tests; the rendered team view exposes the assignments.
