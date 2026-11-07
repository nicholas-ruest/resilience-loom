# ADR-0010: Hash-chained recovery audit

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
A plan list alone cannot show whether a denial caused a hidden write or whether a stored result was later altered.

## Decision
Append each accepted recovery plan to a sequence-numbered SHA-256 chain whose previous hash is part of the next entry.

## Consequences and trade-offs
The audit is tamper-evident and denial can be proven by unchanged chain length. Recovery from a truncated chain is intentionally a hard error.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
rl-audit append/load/tamper tests; denial journey compares persisted counts and correlation trace hashes.
