# Recovery saga

## Model
The application service executes validate, graph, capability, cell, calibration, policy, contract, upstreams, telemetry, recall, persistence, and response rendering as one governed saga.

## Invariants
Steps before persistence are side-effect free except bounded telemetry. Any failure cancels remaining steps. A saved plan is returned only after read-back confirms its identifier.

## Events and verification
SagaStarted, step events, SagaDenied, and SagaCompleted share one correlation ID. Primary and denial journeys verify order and compensation.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
