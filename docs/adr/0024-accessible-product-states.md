# ADR-0024: Accessible incident command surface

Status: Accepted for Resilience Loom on 2026-11-06.

## Context
A generic form hides recovery state and fails operators using keyboard, small screens, or assistive technology.

## Decision
Implement command board, response topology, and evidence ledger views with semantic landmarks, focusable controls, responsive layout, and explicit loading, empty, success, and error states.

## Consequences and trade-offs
The frontend displays real backend and upstream data. Visual density is balanced against incident-time scanability.

## Failure semantics
A failure at this boundary returns a typed error with the request correlation ID and does not authorize a durable recovery-plan write.

## Verification
Frontend unit/accessibility assertions plus desktop and mobile Playwright screenshots.
