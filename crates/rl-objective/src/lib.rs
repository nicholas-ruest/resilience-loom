use rl_errors::{MeshError, Result};
use rl_types::Objective;

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectiveProfile {
    pub objective: Objective,
    pub tokens: Vec<String>,
    pub urgency: u8,
}

pub fn profile(text: &str, sensitivity: &str) -> Result<ObjectiveProfile> {
    let objective = validate(text, sensitivity)?;
    let tokens = objective
        .text
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>();
    let urgency = if tokens
        .iter()
        .any(|token| token.contains("outage") || token.contains("critical"))
    {
        3
    } else if tokens
        .iter()
        .any(|token| token.contains("degraded") || token.contains("latency"))
    {
        2
    } else {
        1
    };
    Ok(ObjectiveProfile {
        objective,
        tokens,
        urgency,
    })
}

pub fn validate(text: &str, sensitivity: &str) -> Result<Objective> {
    let t = text.trim();
    if t.len() < 12 {
        return Err(MeshError::Invalid(
            "objective must be at least 12 characters".into(),
        ));
    }
    if !["public", "internal", "restricted"].contains(&sensitivity) {
        return Err(MeshError::Invalid("unknown sensitivity".into()));
    }
    Ok(Objective {
        text: t.into(),
        sensitivity: sensitivity.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_critical_recovery_objectives() {
        let result = profile("Recover critical checkout outage", "internal").unwrap();
        assert_eq!(result.urgency, 3);
        assert!(result.tokens.contains(&"checkout".into()));
    }

    #[test]
    fn rejects_tiny_or_unknown_sensitivity() {
        assert!(profile("tiny", "public").is_err());
        assert!(profile("Recover checkout safely", "secret").is_err());
    }
}
