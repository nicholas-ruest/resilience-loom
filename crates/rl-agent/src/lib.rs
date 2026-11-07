use rl_errors::{MeshError, Result};
use rl_types::Agent;
pub fn synthetic_registry() -> Vec<Agent> {
    vec![
        Agent {
            id: "a-research".into(),
            name: "Atlas".into(),
            roles: vec!["researcher".into()],
            score: 0.92,
        },
        Agent {
            id: "a-review".into(),
            name: "Prism".into(),
            roles: vec!["reviewer".into()],
            score: 0.88,
        },
        Agent {
            id: "a-operate".into(),
            name: "Forge".into(),
            roles: vec!["operator".into()],
            score: 0.86,
        },
    ]
}
pub fn by_role<'a>(agents: &'a [Agent], role: &str) -> Result<&'a Agent> {
    agents
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == role))
        .max_by(|a, b| a.score.total_cmp(&b.score))
        .ok_or_else(|| MeshError::Dependency(format!("no agent for {role}")))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_role() {
        assert_eq!(
            by_role(&synthetic_registry(), "reviewer").unwrap().name,
            "Prism"
        );
    }
    #[test]
    fn rejects_unstaffed_role() {
        assert!(by_role(&synthetic_registry(), "commander").is_err());
    }
}
