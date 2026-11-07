use rl_agent::by_role;
use rl_errors::Result;
use rl_types::{Agent, Task};
pub fn assemble(tasks: &[Task], registry: &[Agent]) -> Result<Vec<Agent>> {
    let mut out = Vec::new();
    for t in tasks {
        let a = by_role(registry, &t.role)?.clone();
        if !out.iter().any(|x: &Agent| x.id == a.id) {
            out.push(a)
        }
    }
    Ok(out)
}
pub fn separated(team: &[Agent]) -> bool {
    let reviewers: Vec<_> = team
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == "reviewer"))
        .collect();
    let operators: Vec<_> = team
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == "operator"))
        .collect();
    reviewers
        .iter()
        .all(|r| operators.iter().all(|o| r.id != o.id))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duties_separate() {
        assert!(separated(&rl_agent::synthetic_registry()));
    }
    #[test]
    fn rejects_a_team_with_an_unstaffed_task() {
        let tasks = vec![Task {
            id: "command".into(),
            role: "commander".into(),
            depends_on: vec![],
        }];
        assert!(assemble(&tasks, &rl_agent::synthetic_registry()).is_err());
    }
}
