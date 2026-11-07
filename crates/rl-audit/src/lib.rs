use rl_errors::{MeshError, Result};
use rl_rvf::RvfArchive;
use rl_types::RecoveryPlan;
use sha2::{Digest, Sha256};
use std::path::Path;
#[derive(serde::Serialize, serde::Deserialize)]
struct Envelope {
    recovery_plan: RecoveryPlan,
    previous: String,
    hash: String,
}
pub fn append(
    path: &Path,
    seq: u64,
    recovery_plan: &RecoveryPlan,
    previous: &str,
) -> Result<String> {
    let expected_previous = last_hash(path)?;
    if previous != expected_previous {
        return Err(MeshError::Persistence(
            "audit chain predecessor mismatch".into(),
        ));
    }
    let body = serde_json::to_vec(&(recovery_plan, previous))
        .map_err(|e| MeshError::Persistence(e.to_string()))?;
    let hash = hex::encode(Sha256::digest(body));
    let e = Envelope {
        recovery_plan: recovery_plan.clone(),
        previous: previous.into(),
        hash: hash.clone(),
    };
    RvfArchive::append(
        path,
        seq,
        &serde_json::to_vec(&e).map_err(|x| MeshError::Persistence(x.to_string()))?,
    )?;
    Ok(hash)
}
fn read_envelopes(path: &Path) -> Result<Vec<Envelope>> {
    let records = RvfArchive::read_all(path)?;
    let mut envelopes = Vec::with_capacity(records.len());
    let mut previous_hash = String::new();
    for (_, bytes) in records {
        let envelope: Envelope = serde_json::from_slice(&bytes)
            .map_err(|error| MeshError::Persistence(error.to_string()))?;
        let body = serde_json::to_vec(&(&envelope.recovery_plan, &envelope.previous))
            .map_err(|error| MeshError::Persistence(error.to_string()))?;
        let expected_hash = hex::encode(Sha256::digest(body));
        if envelope.previous != previous_hash || envelope.hash != expected_hash {
            return Err(MeshError::Persistence(
                "audit chain verification failed".into(),
            ));
        }
        previous_hash = envelope.hash.clone();
        envelopes.push(envelope);
    }
    Ok(envelopes)
}
pub fn last_hash(path: &Path) -> Result<String> {
    Ok(read_envelopes(path)?
        .last()
        .map(|entry| entry.hash.clone())
        .unwrap_or_default())
}
pub fn load(path: &Path) -> Result<Vec<RecoveryPlan>> {
    Ok(read_envelopes(path)?
        .into_iter()
        .map(|entry| entry.recovery_plan)
        .collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn appends_and_verifies_a_hash_chain() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("audit.rvf");
        let plan = RecoveryPlan::new(
            rl_types::Objective {
                text: "recover".into(),
                sensitivity: "internal".into(),
            },
            vec![],
            vec![],
            0.1,
            "contract".into(),
            "corr".into(),
        );
        let hash = append(&path, 1, &plan, "").unwrap();
        assert_eq!(last_hash(&path).unwrap(), hash);
        assert_eq!(load(&path).unwrap().len(), 1);
        assert!(append(&path, 2, &plan, "wrong-predecessor").is_err());
    }
}
