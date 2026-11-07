# ADR-0025: Explicit operational limitations

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
Prototype evidence becomes misleading when local identity, synthetic incidents, single-writer storage, or embedded policy are presented as production guarantees.

## Decision
Keep limitations in the README, architecture, security model, and acceptance review, and fail semantic review if claims exceed executed evidence.

## Consequences and trade-offs
This reduces marketing breadth but makes the build usable as an honest engineering baseline. Future production work has named migration boundaries.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Semantic review, README read-back, and final exact-head CI artifact inspection.
