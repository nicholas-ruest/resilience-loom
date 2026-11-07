use rl_agent::by_role;
use rl_errors::{MeshError, Result};
use rl_types::{Agent, Task};

#[derive(Debug, Clone, PartialEq)]
pub struct CoverageReport {
    pub covered_tasks: usize,
    pub total_tasks: usize,
    pub ratio: f64,
    pub assigned_agents: Vec<String>,
}

pub fn assess(tasks: &[Task], agents: &[Agent]) -> Result<CoverageReport> {
    if tasks.is_empty() {
        return Err(MeshError::Invalid(
            "capability assessment needs tasks".into(),
        ));
    }
    let assigned_agents = tasks
        .iter()
        .map(|task| by_role(agents, &task.role).map(|agent| agent.id.clone()))
        .collect::<Result<Vec<_>>>()?;
    Ok(CoverageReport {
        covered_tasks: assigned_agents.len(),
        total_tasks: tasks.len(),
        ratio: assigned_agents.len() as f64 / tasks.len() as f64,
        assigned_agents,
    })
}

pub fn coverage(tasks: &[Task], agents: &[Agent]) -> Result<f64> {
    Ok(assess(tasks, agents)?.ratio)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(role: &str) -> Task {
        Task {
            id: role.into(),
            role: role.into(),
            depends_on: vec![],
        }
    }

    #[test]
    fn reports_assignments_for_every_task() {
        let a = rl_agent::synthetic_registry();
        let report = assess(&[task("reviewer"), task("operator")], &a).unwrap();
        assert_eq!(report.covered_tasks, 2);
        assert_eq!(report.ratio, 1.0);
        assert_eq!(report.assigned_agents.len(), 2);
    }

    #[test]
    fn rejects_uncovered_or_empty_task_sets() {
        let a = rl_agent::synthetic_registry();
        assert!(assess(&[], &a).is_err());
        assert!(assess(&[task("nonexistent")], &a).is_err());
    }
}
