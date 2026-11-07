use rl_errors::{MeshError, Result};
pub const POLICY: &str = include_str!("../../../policies/recovery_plan.rego");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow { reason: String },
    Deny { reason: String },
}

pub fn decide(sensitivity: &str, separated: bool, risk: f64) -> Result<PolicyDecision> {
    if !risk.is_finite() || !(0.0..=1.0).contains(&risk) {
        return Err(MeshError::Invalid(
            "risk must be between zero and one".into(),
        ));
    }
    if rl_regorus::allow(POLICY, sensitivity, separated, risk)? {
        Ok(PolicyDecision::Allow {
            reason: "embedded policy accepted separated low-risk recovery".into(),
        })
    } else {
        Ok(PolicyDecision::Deny {
            reason: format!(
                "policy denied sensitivity={sensitivity}, separated={separated}, risk={risk:.3}"
            ),
        })
    }
}

pub fn authorize(sensitivity: &str, separated: bool, risk: f64) -> Result<()> {
    match decide(sensitivity, separated, risk)? {
        PolicyDecision::Allow { .. } => Ok(()),
        PolicyDecision::Deny { reason } => Err(MeshError::Denied(reason)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_separated_low_risk_recovery_allowed() {
        assert!(matches!(
            decide("internal", true, 0.1).unwrap(),
            PolicyDecision::Allow { .. }
        ));
    }

    #[test]
    fn restricted_or_invalid_risk_denied() {
        assert!(matches!(
            decide("restricted", true, 0.1).unwrap(),
            PolicyDecision::Deny { .. }
        ));
        assert!(decide("internal", true, 1.2).is_err());
    }
}
