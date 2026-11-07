# INTEGRATION_MATRIX

Every counted project is pinned to one canonical repository and performs a distinct responsibility in the issued recovery-plan workflow. RVF is part of the RuVector upstream and is not counted twice.

| Canonical upstream | Immutable revision / package | License | Executed product boundary | Owning crate | Success + failure evidence | Ablation |
|---|---|---|---|---|---|---|
| ruvnet/RuVector | `e3d82b19683a08c6392a2534bafc90927ca690c7` / `ruvector-core=2.4.0` | MIT | analogue insert/search plus RVF durable write/read | rl-ruvector, rl-rvf, rl-repository | dimension failure, round-trip, restart journey | without analogue recall or durability |
| ruvnet/agentdb | `96c4780cd8b7d743e2745216e5ad036059ffc25a` / `agentdb=3.7.0` | MIT | RVF-WASM runtime evidence memory and nearest-hit search | rl-upstream-runtime | hit assertion and wrong-dimension failure | without evidence-memory hit |
| ruvnet/dspy.ts | `fbaa7113fde231f26be051a8dc704571e8b2a8a5` / `dspy.ts=2.2.0` | MIT | compile a typed recovery program from demonstrations | rl-upstream-runtime | compiled priority and missing-input failure | without optimized priority |
| ruvnet/ruflo | `b7bc909d5f0d60c74fc965113af2d5af3d506938` / `ruflo=3.72.2` | MIT | initialize hierarchical incident coordination | rl-upstream-runtime | swarm ID and invalid-command failure | without coordination identity |
| ruvnet/sublinear-time-solver | `41175473700b7be65ce5e834dad12de3f20da15f` / `@ruvnet/sublinear-core=1.0.0` | MIT | solve constrained intervention allocation | rl-upstream-runtime | convergence assertion and invalid-method failure | without solver allocation |
| microsoft/regorus | `b94fc6bbda74d5d626321254bc4f2d8a9dddded9` / `regorus=0.13.0` | MIT | embedded policy allow/deny before downstream effects | rl-regorus, rl-policy | allow/deny and malformed-policy readiness | without policy decision |
| open-telemetry/opentelemetry-rust | `285dc925f98403ff426acc70968f104dc820d4f2` / `opentelemetry=0.31.0` | Apache-2.0 | correlated application and upstream spans | rl-telemetry | span assertion and invalid trace-ID failure | without correlation span |

The disabled local `@mlflow/tracking` compatibility package only satisfies an unavailable optional DSPy peer. It always throws when invoked and is excluded from dependency counts and product responsibilities.
