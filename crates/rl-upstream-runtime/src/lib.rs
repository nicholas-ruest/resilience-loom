use rl_types::UpstreamAnalysis;
use serde_json::json;
use std::{
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
fn script_path() -> PathBuf {
    std::env::var_os("RESILIENCE_LOOM_UPSTREAM_SCRIPT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("upstreams/resilience-engine.mjs"))
}
pub fn analyze(incident: &str, correlation_id: &str) -> Result<UpstreamAnalysis, String> {
    if incident.trim().is_empty() || correlation_id.trim().is_empty() {
        return Err("incident and correlation_id are required".into());
    }
    let mut child = Command::new("node")
        .arg(script_path())
        .arg("analyze")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("spawn upstream engine: {error}"))?;
    let payload = json!({"incident": incident, "correlation_id": correlation_id});
    child
        .stdin
        .take()
        .ok_or_else(|| "upstream stdin unavailable".to_string())?
        .write_all(payload.to_string().as_bytes())
        .map_err(|error| format!("write upstream request: {error}"))?;
    let output = child
        .wait_with_output()
        .map_err(|error| format!("wait for upstream engine: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "upstream engine failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("decode upstream analysis: {error}"))
}
pub fn assert_dependency_failure(dependency: &str) -> Result<String, String> {
    let output = Command::new("node")
        .arg(script_path())
        .args(["failure", dependency])
        .output()
        .map_err(|error| format!("spawn failure probe: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "failure probe failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("decode failure probe: {error}"))?;
    if value.get("dependency").and_then(|v| v.as_str()) != Some(dependency)
        || value.get("asserted").and_then(|v| v.as_str()).is_none()
    {
        return Err("failure probe did not assert the requested boundary".into());
    }
    Ok(value["asserted"].as_str().unwrap_or_default().to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_upstreams_return_typed_analysis() {
        let report = analyze("payment database latency", "runtime-test").unwrap();
        assert_eq!(report.agentdb.backend, "agentdb-rvf-wasm");
        assert_eq!(report.dspy.demonstrations, 3);
        assert!(report.sublinear.converged);
        assert_eq!(report.ruflo.topology, "hierarchical");
    }
    #[test]
    fn every_node_boundary_has_a_failure_assertion() {
        for dependency in ["agentdb", "dspy", "sublinear", "ruflo"] {
            assert!(!assert_dependency_failure(dependency).unwrap().is_empty());
        }
    }
    #[test]
    fn empty_incident_is_rejected_before_side_effects() {
        assert!(analyze("", "runtime-test").is_err());
    }
}
