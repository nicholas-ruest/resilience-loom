# Failure and recovery model

## Model
Failures are classified as validation, authentication, authorization, policy denial, upstream dependency, timeout, and persistence integrity errors. The API maps them to consistent typed envelopes.

## Invariants
Forbidden requests cause no repository or audit write. Recoverable errors retain user input and correlation context; corrupt durable state requires operator intervention.

## Events and verification
FailureObserved includes class and correlation ID without secrets. Browser recovery and focused middleware tests assert both the failure and corrected completion.

## Boundary rule
Types cross this boundary through explicit domain values; upstream library types and storage representations remain inside their owning adapters.
