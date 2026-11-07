# ADR-0003: Selective conformal abstention

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
A recovery planner must refuse when team confidence is outside calibrated bounds even if policy syntax allows the request.

## Decision
Calculate conformal risk from the selected cell and abstain above the accepted risk threshold before upstream execution or persistence.

## Consequences and trade-offs
The product may decline a legitimate request, but unsafe confidence cannot trigger downstream effects. Operators receive a recoverable domain error rather than a low-quality plan.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-calibration quantile and invalid-score tests; denial journey verifies the durable plan count is unchanged.
