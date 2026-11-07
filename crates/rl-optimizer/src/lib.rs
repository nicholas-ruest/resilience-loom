use rl_errors::{MeshError, Result};
use rl_types::Agent;

#[derive(Debug, Clone, PartialEq)]
pub struct UtilityBreakdown {
    pub mean_confidence: f64,
    pub coordination_cost: f64,
    pub utility: f64,
}

pub fn evaluate(team: &[Agent], cost_weight: f64) -> Result<UtilityBreakdown> {
    if team.is_empty() || !cost_weight.is_finite() || cost_weight < 0.0 {
        return Err(MeshError::Invalid("invalid team or cost weight".into()));
    }
    let mean_confidence = team.iter().map(|agent| agent.score).sum::<f64>() / team.len() as f64;
    let coordination_cost = cost_weight * (team.len() * team.len()) as f64;
    Ok(UtilityBreakdown {
        mean_confidence,
        coordination_cost,
        utility: mean_confidence - coordination_cost,
    })
}

pub fn utility(team: &[Agent], cost_weight: f64) -> Result<f64> {
    Ok(evaluate(team, cost_weight)?.utility)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_confidence_and_coordination_tradeoff() {
        let a = rl_agent::synthetic_registry();
        let result = evaluate(&a, 0.01).unwrap();
        assert!(result.utility < result.mean_confidence);
        assert!(result.coordination_cost > 0.0);
    }

    #[test]
    fn rejects_empty_or_negative_cost_models() {
        assert!(evaluate(&[], 0.01).is_err());
        assert!(evaluate(&rl_agent::synthetic_registry(), -1.0).is_err());
    }
}
