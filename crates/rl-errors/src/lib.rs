#[derive(Debug, thiserror::Error)]
pub enum MeshError {
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("policy denied: {0}")]
    Denied(String),
    #[error("dependency failure: {0}")]
    Dependency(String),
    #[error("persistence failure: {0}")]
    Persistence(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
}
pub type Result<T> = std::result::Result<T, MeshError>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_error() {
        assert!(MeshError::Denied("x".into()).to_string().contains("denied"))
    }
}
