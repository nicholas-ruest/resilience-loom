# Resilience Loom

![RecoveryPlan formation](docs/assets/recovery_plan-flow.svg)

Resilience Loom is an incident-recovery control plane that turns correlated symptoms into a governed, explainable and restart-durable recovery plan. Its Rust application service decomposes the incident, calibrates a separated response cell, executes seven real upstream project boundaries, authorizes the plan with embedded Rego, persists it through RuVector/RVF, and renders the full evidence path in a multi-view React command surface.

## Run the prototype

Prerequisites: Linux, Rust 1.99, Node 20.20+, and Chromium for Playwright.

```bash
npm ci --prefix frontend
npx --prefix frontend playwright install chromium
./scripts/start.sh
```

Open [http://127.0.0.1:5173](http://127.0.0.1:5173). The API readiness endpoint is `http://127.0.0.1:18080/readyz`. Stop with `./scripts/stop.sh`; reset deterministic test state with `./scripts/reset.sh`.

## Product journeys

1. **Issue a governed recovery plan:** the command board displays task topology, response cell, contract hash, AgentDB evidence hit, DSPy priority, Ruflo coordination ID, Sublinear allocation, RuVector recall and correlated telemetry.
2. **Deny and recover:** choose `restricted`; Regorus denies before any upstream execution or RVF/audit write. The journey proves the saved count is unchanged, then an `internal` retry succeeds.
3. **Restore after restart:** the browser creates a plan, refreshes, terminates and restarts the real Rust service, then retrieves the same durable plan and audit evidence.

## Composition

| Ingredient | Pinned version | Executed responsibility | Evidence |
|---|---:|---|---|
| RuVector / RVF | core 2.4.0 / runtime 0.3.2 | analogue search and durable recovery-plan write/read | `rl-ruvector`, `rl-rvf`, restart journey |
| AgentDB | 3.7.0 | RVF-WASM incident evidence memory | readiness and upstream-runtime tests |
| DSPy.ts | 2.2.0 | typed recovery-program compilation | readiness and primary rendered result |
| Ruflo | 3.72.2 | hierarchical response coordination | readiness and primary rendered result |
| Sublinear Solver | 1.0.0 | constrained intervention allocation | readiness and primary rendered result |
| Microsoft Regorus | 0.13.0 | Rego authorization and denial | `rl-regorus`, denial journey |
| OpenTelemetry Rust | 0.31.0 | correlated spans across application and upstream boundary | `rl-telemetry`, correlation trace |

No single ingredient can correlate evidence, compile a response, solve allocation, coordinate the cell, enforce policy, persist the outcome and render its audit trail. The application-level composition lives in `rl-orchestrator`; the versioned Axum boundary lives in `rl-api`.

## Verification

```bash
./scripts/verify-prototype.sh
```

The script runs structural checks, formatting, strict Clippy, locked Rust tests, Cargo deny, npm audit, frontend typecheck/lint/unit/build, contract drift, executed ingredient ablations, both Playwright viewports against the real API, and restart retrieval. Raw evidence is under `evidence/`.

## Workspace and design

The semantic count conservatively includes 20 product crates with substantive behavior, real production callers, and success/failure tests. The [architecture](docs/architecture.md) maps all 29 Rust crates to a bounded context, caller and acceptance evidence. See the [integration matrix](docs/integration-matrix.md), [prototype acceptance map](docs/prototype-acceptance.md), [25 ADRs](docs/adr/index.md), [12 DDD documents](docs/ddd/index.md), and [semantic review](evidence/semantic-review.md).

![Recovery loop](docs/assets/recovery-loop.svg)

## Deployment and limits

Docker Compose supplies the same frontend/API topology. `deploy/cloudrun-service.yaml` and `deploy/rollback.sh` are configuration only: no cloud deployment or spend was performed. Local RVF is single-instance prototype persistence; production requires an authorized shared RuVector/RVF/Cloud SQL boundary and IAM-scoped secret delivery. Prototype auth uses a local bearer token from environment configuration, never browser service credentials. The Cargo policy carries one documented non-exploit maintenance exception inherited from RuVector; see `docs/security.md`.
