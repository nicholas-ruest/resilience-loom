use rl_errors::{MeshError, Result};
use sha2::{Digest, Sha256};
pub const DIMENSION: usize = 32;
pub fn embed(text: &str) -> Result<Vec<f32>> {
    if text.is_empty() {
        return Err(MeshError::Invalid("empty embedding".into()));
    }
    let h = Sha256::digest(text.as_bytes());
    Ok(h.iter().map(|b| f32::from(*b) / 255.0).collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dimension() {
        assert_eq!(embed("objective").unwrap().len(), DIMENSION);
    }
    #[test]
    fn rejects_empty_embedding_input() {
        assert!(embed("").is_err());
    }
}
