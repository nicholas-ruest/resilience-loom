use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rl_auth::principal;
use rl_config::Config;
use rl_errors::MeshError;
use rl_orchestrator::Orchestrator;
use rl_types::{ApiError, CreateRecoveryPlan, Health};
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
#[derive(Clone)]
struct App {
    orch: Arc<Orchestrator>,
    token: String,
}
fn fail(e: MeshError, c: String) -> Response {
    tracing::warn!(correlation_id = %c, event = "request.failed", error = %e, "correlated request failed");
    let (status, code) = match e {
        MeshError::Invalid(_) => (StatusCode::UNPROCESSABLE_ENTITY, "invalid"),
        MeshError::Denied(_) => (StatusCode::FORBIDDEN, "denied"),
        MeshError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
        MeshError::NotFound(_) => (StatusCode::NOT_FOUND, "not_found"),
        _ => (StatusCode::SERVICE_UNAVAILABLE, "dependency"),
    };
    (
        status,
        Json(ApiError {
            code: code.into(),
            message: e.to_string(),
            correlation_id: c,
        }),
    )
        .into_response()
}
#[allow(clippy::result_large_err)]
fn ctx(h: &HeaderMap, s: &App) -> Result<String, Response> {
    let c = rl_middleware::correlation(h.get("x-correlation-id").and_then(|x| x.to_str().ok()));
    rl_middleware::authorize(
        h.get("authorization").and_then(|x| x.to_str().ok()),
        &s.token,
    )
    .map_err(|e| fail(e, c.clone()))?;
    let _p = principal(&s.token);
    Ok(c)
}
async fn health() -> Json<Health> {
    Json(Health {
        status: "ready".into(),
        persistence: "rvf".into(),
        policy: "regorus".into(),
    })
}
async fn create(State(s): State<App>, h: HeaderMap, Json(r): Json<CreateRecoveryPlan>) -> Response {
    let c = match ctx(&h, &s) {
        Ok(x) => x,
        Err(r) => return r,
    };
    tracing::info!(correlation_id = %c, event = "request.accepted", sensitivity = %r.sensitivity, "correlated recovery request accepted");
    match s.orch.create(&r.objective, &r.sensitivity, c.clone()) {
        Ok(m) => {
            if let Some(upstream) = &m.upstream_analysis {
                tracing::info!(correlation_id = %c, event = "upstreams.executed", plan_id = %m.id, agentdb = %upstream.agentdb.backend, dspy = %upstream.dspy.priority, sublinear = upstream.sublinear.converged, ruflo = %upstream.ruflo.topology, ruvnet_count = 5, enterprise_count = 2, "selected upstreams executed");
            }
            tracing::info!(correlation_id = %c, event = "persistence.written", plan_id = %m.id, "recovery plan persisted");
            (StatusCode::CREATED, Json(m)).into_response()
        }
        Err(e) => fail(e, c),
    }
}
async fn list(State(s): State<App>, h: HeaderMap) -> Response {
    let c = match ctx(&h, &s) {
        Ok(x) => x,
        Err(r) => return r,
    };
    match s.orch.list() {
        Ok(v) => {
            tracing::info!(correlation_id = %c, event = "persistence.read", records = v.len(), "recovery plans read");
            Json(v).into_response()
        }
        Err(e) => fail(e, c),
    }
}
async fn get_one(State(s): State<App>, h: HeaderMap, Path(id): Path<String>) -> Response {
    let c = match ctx(&h, &s) {
        Ok(x) => x,
        Err(r) => return r,
    };
    match s.orch.get(&id) {
        Ok(v) => {
            tracing::info!(correlation_id = %c, event = "persistence.read", plan_id = %v.id, records = 1, "recovery plan read");
            Json(v).into_response()
        }
        Err(e) => fail(e, c),
    }
}
pub async fn serve(config: Config) -> rl_errors::Result<()> {
    let orch = Arc::new(Orchestrator::open(config.data_path)?);
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(health))
        .route("/v1/recovery-plans", post(create).get(list))
        .route("/v1/recovery-plans/{id}", get(get_one))
        .layer(CorsLayer::permissive())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(App {
            orch,
            token: config.token,
        });
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .map_err(|e| MeshError::Dependency(e.to_string()))?;
    axum::serve(listener, app)
        .await
        .map_err(|e| MeshError::Dependency(e.to_string()))
}
#[cfg(test)]
mod tests {
    #[test]
    fn versioned_contract() {
        let contract = include_str!("../../../openapi.yaml");
        assert!(contract.contains("/v1/recovery-plans"));
        assert!(!contract.contains("\n  /recovery-plans:"));
    }
}
