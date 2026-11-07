# ADR-0001: Task graph before recovery-cell selection

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Incident prose mixes symptoms, dependencies, and remediation intent; assigning responders before resolving that structure creates unsafe parallel work.

## Decision
Parse every recovery objective into an acyclic role-labelled task graph before selecting the response cell. Reject missing dependencies and cycles at the domain boundary.

## Consequences and trade-offs
This adds deterministic planning latency but makes ownership, budgets, and separation-of-duty inspectable. The orchestrator may not persist a plan until graph validation passes.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-task-graph behavior and cycle tests plus the primary and denial journeys prove graph construction and no-write failure.
