# Recovery contract lifecycle

## Model
RecoveryContract binds objective, task graph, cell, budget, least-authority scopes, correlation ID, and a deterministic SHA-256 digest.

## Invariants
Digest inputs are ordered and immutable. Repeating the same correlation ID returns the existing issued plan. Scope expansion requires a new contract.

## Events and verification
ContractCreated, PlanIssued, and PlanReplayed distinguish creation from idempotent retrieval. Contract and orchestrator tests verify stable digests.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
