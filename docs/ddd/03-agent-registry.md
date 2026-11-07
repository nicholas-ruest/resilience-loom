# Response agent registry

## Model
Agent is a synthetic response capability record with stable identity, display name, role set, and calibrated score. The registry is a deterministic seed, not an identity provider.

## Invariants
Selection is role-specific and chooses the highest score for that role. Unknown roles fail rather than reuse a nearby capability. Distinct governance roles require distinct identities.

## Events and verification
AgentSelected records task, role, and agent ID. Registry and capability tests prove selection and missing-role failure.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
