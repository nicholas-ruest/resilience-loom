# ADR-0004: Constrained response-cell optimizer

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Maximizing individual confidence alone produces oversized response cells and coordination delay during incidents.

## Decision
Evaluate mean capability confidence against a quadratic coordination cost, while task coverage and separation remain hard constraints.

## Consequences and trade-offs
The optimizer exposes a UtilityBreakdown rather than a scalar-only assertion. Small qualified teams win unless additional members materially improve coverage.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-optimizer tradeoff and invalid-cost tests; benchmark records utility and workflow latency.
