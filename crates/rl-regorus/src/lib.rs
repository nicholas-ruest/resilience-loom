use regorus::{Engine, Value};
use rl_errors::{MeshError, Result};
pub fn allow(policy: &str, sensitivity: &str, separated: bool, risk: f64) -> Result<bool> {
    let mut e = Engine::new();
    e.add_policy("recovery_plan.rego".into(), policy.into())
        .map_err(|x| MeshError::Dependency(format!("Regorus policy: {x}")))?;
    let input =
        serde_json::json!({"sensitivity":sensitivity,"separation_of_duty":separated,"risk":risk});
    e.set_input(
        Value::from_json_str(&input.to_string()).map_err(|x| MeshError::Invalid(x.to_string()))?,
    );
    let v = e
        .eval_rule("data.recovery_plan_mesh.recovery_plan.allow".into())
        .map_err(|x| MeshError::Dependency(format!("Regorus eval: {x}")))?;
    Ok(v == Value::Bool(true))
}
#[cfg(test)]
mod tests {
    use super::*;
    const P: &str = include_str!("../../../policies/recovery_plan.rego");
    #[test]
    fn real_policy() {
        assert!(allow(P, "internal", true, 0.1).unwrap());
        assert!(!allow(P, "restricted", true, 0.1).unwrap())
    }
}
