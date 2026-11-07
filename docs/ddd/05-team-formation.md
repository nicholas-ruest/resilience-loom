# Response cell formation

## Model
ResponseCell is the ordered set of researcher, reviewer, and operator agents chosen for one plan. UtilityBreakdown evaluates confidence against coordination cost.

## Invariants
Every task role is covered, identities are unique where separation is required, and utility inputs are finite and non-negative. Formation has no persistence side effect.

## Events and verification
CellFormed and CellRejected are mutually exclusive. Team and optimizer tests cover both paths.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
