use rl_audit::{append, last_hash, load};
use rl_errors::{MeshError, Result};
use rl_types::RecoveryPlan;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
pub struct Repository {
    path: PathBuf,
    seq: AtomicU64,
}
impl Repository {
    pub fn open(path: PathBuf) -> Result<Self> {
        let n = load(&path)?.len() as u64;
        Ok(Self {
            path,
            seq: AtomicU64::new(n),
        })
    }
    pub fn save(&self, m: &RecoveryPlan) -> Result<()> {
        let seq = self.seq.fetch_add(1, Ordering::SeqCst) + 1;
        let previous = last_hash(&self.path)?;
        append(&self.path, seq, m, &previous)?;
        Ok(())
    }
    pub fn list(&self) -> Result<Vec<RecoveryPlan>> {
        load(&self.path)
    }
    pub fn get(&self, id: &str) -> Result<RecoveryPlan> {
        self.list()?
            .into_iter()
            .find(|m| m.id == id)
            .ok_or_else(|| MeshError::NotFound(id.into()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing() {
        let d = tempfile::tempdir().unwrap();
        let r = Repository::open(d.path().join("x.rvf")).unwrap();
        assert!(r.get("nope").is_err());
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
        r.save(&plan).unwrap();
        assert_eq!(r.get(&plan.id).unwrap().id, plan.id);
        assert_eq!(r.list().unwrap().len(), 1);
    }
}
