use rl_errors::{MeshError, Result};
use std::path::PathBuf;
#[derive(Clone, Debug)]
pub struct Config {
    pub bind: String,
    pub data_path: PathBuf,
    pub token: String,
}
impl Config {
    pub fn from_env() -> Result<Self> {
        let bind = std::env::var("MM_BIND").unwrap_or_else(|_| "127.0.0.1:18080".into());
        if !bind.contains(':') {
            return Err(MeshError::Invalid("MM_BIND".into()));
        }
        Ok(Self {
            bind,
            data_path: std::env::var("MM_DATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| "data/recovery-plans.rvf".into()),
            token: std::env::var("MM_TOKEN").unwrap_or_else(|_| "demo-token".into()),
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn defaults() {
        assert!(Config::from_env().unwrap().bind.ends_with("8080"))
    }
}
