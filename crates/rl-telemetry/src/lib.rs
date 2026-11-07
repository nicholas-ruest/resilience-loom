use opentelemetry::{
    trace::{Span, Tracer, TracerProvider},
    KeyValue,
};
use opentelemetry_sdk::trace::SdkTracerProvider;
#[derive(Clone, Debug, PartialEq)]
pub struct TelemetryRecord {
    pub correlation_id: String,
    pub priority: String,
    pub operation: &'static str,
}
pub fn record_recovery(correlation_id: &str, priority: &str) -> Result<TelemetryRecord, String> {
    if correlation_id.trim().is_empty() {
        return Err("correlation_id must not be empty".into());
    }
    let provider = SdkTracerProvider::builder().build();
    let tracer = provider.tracer("resilience-loom");
    let mut span = tracer.start("recovery_plan.create");
    span.set_attribute(KeyValue::new(
        "recovery.correlation_id",
        correlation_id.to_string(),
    ));
    span.set_attribute(KeyValue::new("recovery.priority", priority.to_string()));
    span.add_event(
        "selected-upstreams-composed",
        vec![KeyValue::new("upstream.count", 7_i64)],
    );
    span.end();
    provider
        .shutdown()
        .map_err(|error| format!("telemetry shutdown: {error}"))?;
    Ok(TelemetryRecord {
        correlation_id: correlation_id.into(),
        priority: priority.into(),
        operation: "recovery_plan.create",
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn records_a_correlated_recovery_span() {
        let record = record_recovery("corr-17", "critical").unwrap();
        assert_eq!(record.operation, "recovery_plan.create");
    }
    #[test]
    fn rejects_uncorrelated_telemetry() {
        assert!(record_recovery("", "critical").is_err());
    }
}
