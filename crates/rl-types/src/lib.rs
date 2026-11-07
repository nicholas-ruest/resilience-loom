use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    pub text: String,
    pub sensitivity: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: String,
    pub role: String,
    pub depends_on: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
    pub score: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AgentDbEvidence {
    pub backend: String,
    pub indexed: usize,
    pub analogue: String,
    pub similarity: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DspyEvidence {
    pub priority: String,
    pub demonstrations: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SublinearEvidence {
    pub converged: bool,
    pub iterations: usize,
    #[serde(rename = "servicePressure")]
    pub service_pressure: Vec<f64>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RufloEvidence {
    pub initialized: bool,
    pub topology: String,
    #[serde(rename = "swarmId")]
    pub swarm_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct UpstreamAnalysis {
    pub agentdb: AgentDbEvidence,
    pub dspy: DspyEvidence,
    pub sublinear: SublinearEvidence,
    pub ruflo: RufloEvidence,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RecoveryPlan {
    pub id: String,
    pub objective: Objective,
    pub tasks: Vec<Task>,
    pub agents: Vec<Agent>,
    pub risk: f64,
    pub contract_hash: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub correlation_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_analysis: Option<UpstreamAnalysis>,
}
impl RecoveryPlan {
    pub fn new(
        objective: Objective,
        tasks: Vec<Task>,
        agents: Vec<Agent>,
        risk: f64,
        contract_hash: String,
        correlation_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            objective,
            tasks,
            agents,
            risk,
            contract_hash,
            created_at: Utc::now(),
            status: "issued".into(),
            correlation_id,
            upstream_analysis: None,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateRecoveryPlan {
    pub objective: String,
    pub sensitivity: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub correlation_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
    pub persistence: String,
    pub policy: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analogue {
    pub recovery_plan_id: String,
    pub distance: f64,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recovery_plan_has_identity() {
        let m = RecoveryPlan::new(
            Objective {
                text: "x".into(),
                sensitivity: "internal".into(),
            },
            vec![],
            vec![],
            0.1,
            "h".into(),
            "c".into(),
        );
        assert!(!m.id.is_empty())
    }
}
