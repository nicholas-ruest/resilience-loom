# Capability assurance

## Model
CoverageReport joins graph roles to response agents and exposes covered count, total count, ratio, and assigned identities.

## Invariants
A report cannot be created for an empty graph. Full coverage is mandatory; partial coverage cannot be rounded to success or repaired after policy.

## Events and verification
CapabilityAssessed precedes optimization. Unit tests and rendered response topology show the exact assignment evidence.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
