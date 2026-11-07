use rl_errors::{MeshError, Result};
use rl_repository::Repository;
use rl_ruvector::RecoveryPlanIndex;
use rl_types::RecoveryPlan;
use std::{path::PathBuf, sync::Mutex};
pub struct Orchestrator {
    repo: Repository,
    index: Mutex<RecoveryPlanIndex>,
}
impl Orchestrator {
    pub fn open(path: PathBuf) -> Result<Self> {
        let repo = Repository::open(path)?;
        let mut index = RecoveryPlanIndex::new()?;
        for m in repo.list()? {
            index.insert(&m.id, rl_embedding::embed(&m.objective.text)?)?
        }
        Ok(Self {
            repo,
            index: Mutex::new(index),
        })
    }
    pub fn create(
        &self,
        text: &str,
        sensitivity: &str,
        correlation_id: String,
    ) -> Result<RecoveryPlan> {
        if let Some(existing) = self
            .repo
            .list()?
            .into_iter()
            .find(|m| m.correlation_id == correlation_id)
        {
            return Ok(existing);
        }
        let objective = rl_objective::validate(text, sensitivity)?;
        let tasks = rl_task_graph::decompose(&objective.text)?;
        if !rl_task_graph::acyclic(&tasks) {
            return Err(MeshError::Invalid("task cycle".into()));
        }
        let registry = rl_seed::deterministic_agents();
        let _coverage = rl_capability::coverage(&tasks, &registry)?;
        let team = rl_team::assemble(&tasks, &registry)?;
        let separated = rl_team::separated(&team);
        let risk =
            rl_calibration::conformal_risk(&team.iter().map(|a| a.score).collect::<Vec<_>>(), 0.1)?;
        if rl_calibration::should_abstain(risk, 0.25) {
            return Err(MeshError::Denied("calibration abstained".into()));
        }
        rl_policy::authorize(&objective.sensitivity, separated, risk)?;
        let _utility = rl_optimizer::utility(&team, 0.01)?;
        let _budgets = rl_budget::allocate(tasks.len(), 90)?;
        let contract_hash = rl_contract::digest(&objective, &tasks, &team)?;
        rl_contract::verify(&contract_hash, &objective, &tasks, &team)?;
        let _scopes = rl_contract::scopes(&tasks);
        let mut recovery_plan =
            RecoveryPlan::new(objective, tasks, team, risk, contract_hash, correlation_id);
        let _evaluation = rl_evaluation::assess(&recovery_plan, 0.25)
            .map_err(|error| MeshError::Invalid(format!("evaluation: {error:?}")))?;
        let upstream_analysis = rl_upstream_runtime::analyze(text, &recovery_plan.correlation_id)
            .map_err(MeshError::Dependency)?;
        rl_telemetry::record_recovery(
            &recovery_plan.correlation_id,
            &upstream_analysis.dspy.priority,
        )
        .map_err(MeshError::Dependency)?;
        recovery_plan.upstream_analysis = Some(upstream_analysis);
        let analogues = {
            let idx = self
                .index
                .lock()
                .map_err(|_| MeshError::Dependency("index lock".into()))?;
            rl_recall::recall(&idx, &recovery_plan.objective.text)?
        };
        if !analogues.is_empty() {
            recovery_plan.status = "issued-with-analogue".into()
        }
        self.repo.save(&recovery_plan)?;
        self.index
            .lock()
            .map_err(|_| MeshError::Dependency("index lock".into()))?
            .insert(
                &recovery_plan.id,
                rl_embedding::embed(&recovery_plan.objective.text)?,
            )?;
        Ok(recovery_plan)
    }
    pub fn list(&self) -> Result<Vec<RecoveryPlan>> {
        self.repo.list()
    }
    pub fn get(&self, id: &str) -> Result<RecoveryPlan> {
        self.repo.get(id)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertical_slice() {
        let d = tempfile::tempdir().unwrap();
        let o = Orchestrator::open(d.path().join("x.rvf")).unwrap();
        let m = o
            .create(
                "Investigate service latency and verify remediation",
                "internal",
                "test-corr".into(),
            )
            .unwrap();
        assert_eq!(o.get(&m.id).unwrap().agents.len(), 3);
        assert!(o
            .create(
                "Investigate service latency and verify remediation",
                "restricted",
                "c".into()
            )
            .is_err())
    }
}
