# Incident objective and task graph

## Model
The IncidentObjective aggregate normalizes user intent, sensitivity, and urgency. A RecoveryTask graph decomposes it into research, review, and operation nodes with explicit dependencies.

## Invariants
Objectives shorter than the minimum or with unknown sensitivity do not exist. Task IDs are unique, every dependency resolves, and the graph is acyclic before cell formation.

## Events and verification
ObjectiveProfiled and TaskGraphValidated feed planning. Validation tests cover short input, unknown sensitivity, missing dependencies, and cycles.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
