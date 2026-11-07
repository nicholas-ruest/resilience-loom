use rl_errors::{MeshError, Result};
use rl_types::{Agent, Objective, Task};
use sha2::{Digest, Sha256};
pub fn digest(objective: &Objective, tasks: &[Task], team: &[Agent]) -> Result<String> {
    let bytes = serde_json::to_vec(&(objective, tasks, team))
        .map_err(|e| rl_errors::MeshError::Invalid(e.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}
pub fn verify(expected: &str, objective: &Objective, tasks: &[Task], team: &[Agent]) -> Result<()> {
    let actual = digest(objective, tasks, team)?;
    if actual != expected {
        return Err(MeshError::Invalid(
            "recovery contract digest mismatch".into(),
        ));
    }
    Ok(())
}
pub fn scopes(tasks: &[Task]) -> Vec<String> {
    tasks
        .iter()
        .map(|t| format!("recovery_plan:{}:execute", t.id))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        let o = Objective {
            text: "x".into(),
            sensitivity: "public".into(),
        };
        let hash = digest(&o, &[], &[]).unwrap();
        assert_eq!(hash, digest(&o, &[], &[]).unwrap());
        assert!(verify(&hash, &o, &[], &[]).is_ok());
    }
    #[test]
    fn rejects_tampered_contract_digest() {
        let objective = Objective {
            text: "recover".into(),
            sensitivity: "internal".into(),
        };
        assert!(verify("tampered", &objective, &[], &[]).is_err());
    }
}
