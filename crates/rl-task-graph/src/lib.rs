use rl_errors::{MeshError, Result};
use rl_types::Task;
pub fn decompose(text: &str) -> Result<Vec<Task>> {
    if text.trim().is_empty() {
        return Err(MeshError::Invalid("empty objective".into()));
    }
    Ok(vec![
        Task {
            id: "research".into(),
            role: "researcher".into(),
            depends_on: vec![],
        },
        Task {
            id: "verify".into(),
            role: "reviewer".into(),
            depends_on: vec!["research".into()],
        },
        Task {
            id: "deliver".into(),
            role: "operator".into(),
            depends_on: vec!["verify".into()],
        },
    ])
}
pub fn acyclic(tasks: &[Task]) -> bool {
    tasks.iter().enumerate().all(|(i, t)| {
        t.depends_on
            .iter()
            .all(|d| tasks[..i].iter().any(|x| &x.id == d))
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph_is_ordered() {
        let g = decompose("long enough objective").unwrap();
        assert!(acyclic(&g))
    }
    #[test]
    fn rejects_empty_objective_and_forward_edges() {
        assert!(decompose("   ").is_err());
        let invalid = vec![Task {
            id: "later".into(),
            role: "operator".into(),
            depends_on: vec!["missing".into()],
        }];
        assert!(!acyclic(&invalid));
    }
}
