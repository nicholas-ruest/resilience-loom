# Durable plan and audit

## Model
The RecoveryPlan aggregate owns objective, tasks, cell, risk, contract hash, upstream analysis, status, and correlation ID. Repository coordinates RVF persistence and hash-chained audit.

## Invariants
A plan is durable only after both record and audit append succeed. Startup rebuilds analogue recall from validated records. Corrupt storage is never silently reset.

## Events and verification
PlanPersisted and AuditAppended are visible in correlated traces; restart retrieval proves the aggregate survives process replacement.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
