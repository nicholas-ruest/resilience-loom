# Recovery plan governance

## Model
GovernanceDecision combines authenticated principal, objective sensitivity, separation evidence, calibrated risk, and embedded Rego result.

## Invariants
Only an explicit Allow can reach upstream execution. Invalid risk is a validation failure; policy false is a denial with reason; policy engine failure is a dependency error.

## Events and verification
PlanAuthorized or PlanDenied carries the correlation ID. Middleware and Regorus tests plus denial journey verify no downstream write.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
