# Architecture

## Contexts
Objective Graph owns DAGs. Agent Registry owns identity and trust zones. Capability Assurance owns contextual samples and abstention. Team Formation owns coverage, budgets and optimization. Governance owns Regorus decisions. Contract Lifecycle owns least-authority state. Intelligence/Evidence owns RuVector recall, RVF durability and audit. Delivery owns HTTP, middleware and React.

## Runtime path
rl-cli -> rl-api -> rl-auth/rl-middleware -> rl-orchestrator -> objective/task/agent/capability/calibration/team/optimizer/budget/contract/policy/recall -> regorus/ruvector/embedding -> repository/audit/rvf.

## Crate map
Every crate has a typed API, caller or executable role, design link and focused or journey evidence.

| # | Crate | Responsibility | Context | Caller | Acceptance |
|---:|---|---|---|---|---|
| 1 | rl-types | shared value objects and DTOs | Shared Kernel | all domain crates | focused test + journey |
| 2 | rl-errors | typed errors | Shared Kernel | API/application | focused test + journey |
| 3 | rl-config | validated runtime configuration | Platform | rl-cli | focused test + journey |
| 4 | rl-objective | objective aggregate | Objective Graph | rl-orchestrator | focused test + journey |
| 5 | rl-task-graph | dependency-safe subtask DAG | Objective Graph | rl-orchestrator | focused test + journey |
| 6 | rl-agent | agent identity and trust zone | Agent Registry | rl-team | focused test + journey |
| 7 | rl-capability | contextual skill evidence | Capability Assurance | rl-calibration | focused test + journey |
| 8 | rl-calibration | selective conformal lower bound | Capability Assurance | rl-team | focused test + journey |
| 9 | rl-team | multi-role team formation | Team Formation | rl-orchestrator | focused test + journey |
| 10 | rl-optimizer | constrained utility search | Team Formation | rl-team | focused test + journey |
| 11 | rl-budget | hard cost/concurrency limits | Team Formation | rl-optimizer | focused test + journey |
| 12 | rl-contract | least-authority recovery_plan contract | Contract Lifecycle | rl-orchestrator | focused test + journey |
| 13 | rl-policy | typed policy decision | Governance | rl-orchestrator | focused test + journey |
| 14 | rl-regorus | Microsoft Regorus adapter | Governance | rl-policy | focused test + journey |
| 15 | rl-embedding | task/team embedding | Intelligence | rl-recall | focused test + journey |
| 16 | rl-ruvector | RuVector adapter | Intelligence | rl-recall | focused test + journey |
| 17 | rl-recall | analogue team recall | Intelligence | rl-orchestrator | focused test + journey |
| 18 | rl-rvf | RVF append/reopen/read | Evidence | rl-repository | focused test + journey |
| 19 | rl-audit | hash-chained evidence | Evidence | rl-repository | focused test + journey |
| 20 | rl-repository | single persistence boundary | Evidence | rl-orchestrator | focused test + journey |
| 21 | rl-auth | prototype principal | Delivery | rl-api | focused test + journey |
| 22 | rl-middleware | idempotency/correlation | Delivery | rl-api | focused test + journey |
| 23 | rl-orchestrator | delegation use cases | Application | rl-api/rl-cli | focused test + journey |
| 24 | rl-api | versioned Axum API | Delivery | rl-cli | focused test + journey |
| 25 | rl-seed | deterministic fixture provider (supporting, not counted) | Evaluation | rl-orchestrator | focused test + journey |
| 26 | rl-evaluation | plan completeness and calibrated-risk gate | Evaluation | rl-orchestrator | focused test + journey |
| 27 | rl-cli | service executable | Delivery | operator | focused test + journey |

## Topology
React/Vite proxies to stateless Axum. Only the API owns upstream adapters. RVF is the sole durable store. Browser code has no credentials. Production Cloud Run requires separately authorized shared RuVector/Cloud SQL.

## Failure order
Auth and validation precede orchestration. Budgets and calibration precede policy. Regorus denial precedes vector or RVF writes. Append failure prevents success. Timeouts and shutdown bound execution.
