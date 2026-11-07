# ADR-0006: Regorus policy anti-corruption layer

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Raw Rego values and engine errors must not leak into the recovery domain or frontend contract.

## Decision
Keep Microsoft Regorus behind rl-regorus and rl-policy. Convert typed sensitivity, separation, and risk inputs into one policy decision with an explicit reason.

## Consequences and trade-offs
The service can replace policy engines without changing application types. Malformed policy is a dependency error; a false result is a domain denial.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-regorus allow and malformed-policy tests, readiness probes, and middleware policy tests.
