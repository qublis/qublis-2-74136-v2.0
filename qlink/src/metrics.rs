//! QLink Metrics Collector
//!
//! Collects and exports Prometheus‐style metrics for the QLink crate,
//! covering identity registration, revocation, principles, consent, and mutation events.

use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::Serialize;

/// A timestamped snapshot of counters and gauges.
#[derive(Debug, Clone, Serialize)]
struct MetricSnapshot {
    timestamp: u128,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
}

/// Collector for QLink metrics.
#[derive(Clone, Debug)]
pub struct QLinkMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl QLinkMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        QLinkMetrics {
            start: Instant::now(),
            counters: HashMap::new(),
            gauges: HashMap::new(),
            snapshots: Vec::new(),
        }
    }

    /// Increment a named counter by `value`.
    pub fn inc_counter(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    /// Set a gauge to `value`.
    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }

    /// Record a snapshot of current counters and gauges.
    pub fn record_snapshot(&mut self) {
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.snapshots.push(MetricSnapshot {
            timestamp: ts,
            counters: self.counters.clone(),
            gauges: self.gauges.clone(),
        });
    }

    /// Export recorded snapshots as pretty‐printed JSON.
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| format!("Metrics JSON export failed: {}", e))
    }

    /// Export current counters and gauges in Prometheus text format.
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.counters {
            out.push_str(&format!("qlink_{} {}\n", k, v));
        }
        for (k, v) in &self.gauges {
            out.push_str(&format!("qlink_{} {}\n", k, v));
        }
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("qlink_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain‐specific metrics ===

    /// Record registration of a new identity.
    pub fn record_identities_registered(&mut self) {
        self.inc_counter("identities_registered", 1);
    }

    /// Record revocation of an identity.
    pub fn record_identities_revoked(&mut self) {
        self.inc_counter("identities_revoked", 1);
    }

    /// Record addition of an ethical principle.
    pub fn record_principles_added(&mut self) {
        self.inc_counter("principles_added", 1);
    }

    /// Record entanglement of two principles.
    pub fn record_principles_entangled(&mut self) {
        self.inc_counter("principles_entangled", 1);
    }

    /// Record evaluation of the ethics lattice.
    pub fn record_lattice_evaluations(&mut self) {
        self.inc_counter("lattice_evaluations", 1);
    }

    /// Record a consent request.
    pub fn record_consents_requested(&mut self) {
        self.inc_counter("consents_requested", 1);
    }

    /// Record a granted consent.
    pub fn record_consents_granted(&mut self) {
        self.inc_counter("consents_granted", 1);
    }

    /// Record a denied consent.
    pub fn record_consents_denied(&mut self) {
        self.inc_counter("consents_denied", 1);
    }

    /// Record revocation of consent.
    pub fn record_consents_revoked(&mut self) {
        self.inc_counter("consents_revoked", 1);
    }

    /// Record that a policy update was recorded.
    pub fn record_policy_updates_recorded(&mut self) {
        self.inc_counter("policy_updates_recorded", 1);
    }

    /// Record that a policy update was applied.
    pub fn record_policy_updates_applied(&mut self) {
        self.inc_counter("policy_updates_applied", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counter_gauge_and_export() {
        let mut m = QLinkMetrics::new();
        m.inc_counter("foo", 2);
        m.set_gauge("bar", 3.14);
        m.record_snapshot();

        let prom = m.export_prometheus();
        assert!(prom.contains("qlink_foo 2"));
        assert!(prom.contains("qlink_bar 3.14"));
        assert!(prom.contains("qlink_uptime_seconds"));

        let json = m.export_json().unwrap();
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["counters"]["foo"].as_u64(), Some(2));
        assert_eq!(arr[0]["gauges"]["bar"].as_f64(), Some(3.14));
    }

    #[test]
    fn domain_specific_records() {
        let mut m = QLinkMetrics::new();
        m.record_identities_registered();
        m.record_identities_revoked();
        m.record_principles_added();
        m.record_principles_entangled();
        m.record_lattice_evaluations();
        m.record_consents_requested();
        m.record_consents_granted();
        m.record_consents_denied();
        m.record_consents_revoked();
        m.record_policy_updates_recorded();
        m.record_policy_updates_applied();

        assert_eq!(m.counters["identities_registered"], 1);
        assert_eq!(m.counters["identities_revoked"], 1);
        assert_eq!(m.counters["principles_added"], 1);
        assert_eq!(m.counters["principles_entangled"], 1);
        assert_eq!(m.counters["lattice_evaluations"], 1);
        assert_eq!(m.counters["consents_requested"], 1);
        assert_eq!(m.counters["consents_granted"], 1);
        assert_eq!(m.counters["consents_denied"], 1);
        assert_eq!(m.counters["consents_revoked"], 1);
        assert_eq!(m.counters["policy_updates_recorded"], 1);
        assert_eq!(m.counters["policy_updates_applied"], 1);
    }
}
