# ADR-0016: Refusal and recovery as one workflow

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
A denial-only demonstration does not prove that an operator can correct evidence and complete the intended recovery safely.

## Decision
Model policy or validation refusal as an explicit state that preserves the submitted correlation context, then allow a corrected request to traverse the full workflow.

## Consequences and trade-offs
The UI displays the reason and offers recovery controls. The forbidden attempt leaves no durable plan or audit entry.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Policy tests and the journey named denial-recovery verify before/after counts and the successful retry.
