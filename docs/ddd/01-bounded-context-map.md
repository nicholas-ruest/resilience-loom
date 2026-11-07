# Bounded context map

## Model
Incident Evidence owns submitted symptoms and sensitivity; Recovery Planning owns task graphs, capability coverage, optimization, and budgets; Governance owns authentication and policy; Execution owns upstream composition; Audit Retrieval owns durable plans and hash-chain history.

## Invariants
A context can mutate only its own aggregate. Cross-context values carry plan and correlation IDs. Governance failure terminates the command before Execution and Audit Retrieval.

## Events and verification
IncidentAccepted, PlanProposed, PlanDenied, PlanIssued, and PlanRetrieved are correlated domain events. API journeys verify their order and absence after denial.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
