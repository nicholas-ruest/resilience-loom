use rl_types::RecoveryPlan;
#[derive(Debug, Clone)]
pub struct Metrics {
    pub coverage: f64,
    pub risk: f64,
    pub team_size: usize,
}
pub fn measure(m: &RecoveryPlan) -> Metrics {
    Metrics {
        coverage: m.tasks.len() as f64 / m.tasks.len().max(1) as f64,
        risk: m.risk,
        team_size: m.agents.len(),
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationError {
    EmptyTaskGraph,
    EmptyResponseTeam,
    RiskAboveBound,
}
pub fn assess(plan: &RecoveryPlan, maximum_risk: f64) -> Result<Metrics, EvaluationError> {
    if plan.tasks.is_empty() {
        return Err(EvaluationError::EmptyTaskGraph);
    }
    if plan.agents.is_empty() {
        return Err(EvaluationError::EmptyResponseTeam);
    }
    if plan.risk > maximum_risk {
        return Err(EvaluationError::RiskAboveBound);
    }
    Ok(measure(plan))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metric_shape() {
        let m = RecoveryPlan::new(
            rl_types::Objective {
                text: "x".into(),
                sensitivity: "public".into(),
            },
            vec![],
            vec![],
            0.2,
            "h".into(),
            "c".into(),
        );
        assert_eq!(measure(&m).team_size, 0);
        assert!(matches!(
            assess(&m, 0.25),
            Err(EvaluationError::EmptyTaskGraph)
        ));
        let valid = RecoveryPlan::new(
            rl_types::Objective {
                text: "recover payments".into(),
                sensitivity: "internal".into(),
            },
            vec![rl_types::Task {
                id: "t1".into(),
                role: "operator".into(),
                depends_on: vec![],
            }],
            vec![rl_types::Agent {
                id: "a1".into(),
                name: "Atlas".into(),
                roles: vec!["operator".into()],
                score: 0.9,
            }],
            0.1,
            "contract".into(),
            "corr".into(),
        );
        let metrics = assess(&valid, 0.25).unwrap();
        assert_eq!(metrics.team_size, 1);
        assert_eq!(metrics.coverage, 1.0);
    }
}
