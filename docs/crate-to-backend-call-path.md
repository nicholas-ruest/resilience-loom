# Crate-to-backend production call paths

The launched binary is `rl-cli`. It constructs `rl-api::serve`, whose versioned handlers call `rl-orchestrator::Orchestrator`. The following packages are counted as meaningful product crates because Cargo normal-dependency edges and concrete function calls place them in that launched production graph.

| Crate | Production caller | Owned behavior | Failure behavior exercised |
|---|---|---|---|
| rl-api | rl-cli | Versioned routes, middleware ordering, readiness | rejects malformed and unauthenticated requests |
| rl-orchestrator | rl-api | Complete recovery-plan transaction | aborts before persistence on policy/upstream failure |
| rl-objective | rl-orchestrator | Incident objective normalization | rejects empty/oversized objectives |
| rl-task-graph | rl-orchestrator | Dependency-ordered recovery graph | rejects cycles and missing tasks |
| rl-agent | rl-seed | Response-cell member model | rejects invalid identity/capabilities |
| rl-capability | rl-orchestrator | Evidence-backed capability assurance | rejects stale or insufficient evidence |
| rl-calibration | rl-orchestrator | Confidence calibration/abstention | abstains below the calibrated threshold |
| rl-team | rl-orchestrator | Separation-of-duty cell formation | rejects unstaffed required roles |
| rl-optimizer | rl-orchestrator | Constrained responder assignment | reports infeasible assignments |
| rl-budget | rl-orchestrator | Bounded response effort | rejects exhausted or negative budget |
| rl-contract | rl-orchestrator | Governed execution contract | rejects digest mismatch after plan assembly |
| rl-policy | rl-orchestrator | Pre-effect authorization decision | denies restricted incidents |
| rl-embedding | rl-orchestrator | Deterministic incident embedding | rejects invalid input dimensionality |
| rl-ruvector | rl-orchestrator | RuVector analogue persistence/search | rejects dimension mismatch |
| rl-recall | rl-orchestrator | Ranked recovery analogue recall | rejects empty query evidence |
| rl-audit | rl-repository | Hash-chained audit events | detects broken chain continuity |
| rl-repository | rl-orchestrator | RVF durable plan repository | reports corrupt/missing records |
| rl-evaluation | rl-orchestrator | Plan completeness and calibrated-risk acceptance | rejects empty tasks, empty teams, and excess risk before upstream effects |
| rl-upstream-runtime | rl-orchestrator | AgentDB/DSPy/Sublinear/Ruflo execution | propagates asserted dependency failures |
| rl-telemetry | rl-orchestrator | OpenTelemetry correlation spans | verifies closed-span/export behavior |

`rl-seed` remains a production-reachable deterministic fixture provider but is explicitly not counted because it delegates registry construction without owning a separate product responsibility.

## Request paths

- Primary: React command view -> checked client -> `POST /v1/recovery-plans` -> all middleware -> orchestrator -> all seven upstreams -> repository/audit -> rendered topology and upstream proof.
- Denial/recovery: sensitivity control -> Regorus denial -> zero repository mutation -> corrected sensitivity -> normal primary path.
- Durable retrieval: saved plan -> backend process termination and restart -> `GET /v1/recovery-plans` -> RVF read -> evidence-ledger render.

Reciprocal ADR/DDD source mappings and source hashes are emitted in the schema-2 `composition_report`; this map explains the same edges for human review.
