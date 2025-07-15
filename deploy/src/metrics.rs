//! Deployment Metrics Collector for Qublis‐deploy — Qublis v2.0
//!
//! Collects and exports Prometheus-style metrics for the deployment tools,
//! covering configuration loads, CI fork operations, test runs, and QNetX
//! node bootstrapping events.

use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::Serialize;

/// A timestamped snapshot of counters and gauges.
#[derive(Debug, Clone, Serialize)]
struct MetricSnapshot {
    /// Milliseconds since UNIX epoch.
    timestamp: u128,
    /// Monotonic counters.
    counters: HashMap<String, u64>,
    /// Instantaneous gauges.
    gauges: HashMap<String, f64>,
}

/// Collector for deployment metrics.
#[derive(Clone, Debug)]
pub struct DeployMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl DeployMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        DeployMetrics {
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

    /// Set a gauge (instantaneous metric) to `value`.
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

    /// Export all recorded snapshots as pretty-printed JSON.
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| format!("Metrics JSON export failed: {}", e))
    }

    /// Export current counters and gauges in Prometheus text format.
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.counters {
            out.push_str(&format!("deploy_{} {}\n", k, v));
        }
        for (k, v) in &self.gauges {
            out.push_str(&format!("deploy_{} {}\n", k, v));
        }
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("deploy_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain-specific metrics ===

    /// Record loading of a deployment configuration file.
    pub fn record_config_load(&mut self) {
        self.inc_counter("config_loads", 1);
    }

    /// Record creation of a CI fork.
    pub fn record_ci_fork_created(&mut self) {
        self.inc_counter("ci_forks_created", 1);
    }

    /// Record execution of `cargo test` in a fork.
    pub fn record_ci_tests_run(&mut self) {
        self.inc_counter("ci_forks_tests_run", 1);
    }

    /// Record start of a QNetX node bootstrap.
    pub fn record_node_bootstrap_started(&mut self) {
        self.inc_counter("node_bootstraps_started", 1);
    }

    /// Record successful completion of a QNetX node bootstrap.
    pub fn record_node_bootstrap_completed(&mut self) {
        self.inc_counter("node_bootstraps_completed", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counter_gauge_and_export() {
        let mut m = DeployMetrics::new();
        m.inc_counter("foo", 2);
        m.set_gauge("bar", 3.14);
        m.record_snapshot();

        let prom = m.export_prometheus();
        assert!(prom.contains("deploy_foo 2"));
        assert!(prom.contains("deploy_bar 3.14"));
        assert!(prom.contains("deploy_uptime_seconds"));

        let json = m.export_json().unwrap();
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["counters"]["foo"].as_u64(), Some(2));
        assert_eq!(arr[0]["gauges"]["bar"].as_f64(), Some(3.14));
    }

    #[test]
    fn domain_specific_records() {
        let mut m = DeployMetrics::new();
        m.record_config_load();
        m.record_ci_fork_created();
        m.record_ci_tests_run();
        m.record_node_bootstrap_started();
        m.record_node_bootstrap_completed();

        assert_eq!(m.counters["config_loads"], 1);
        assert_eq!(m.counters["ci_forks_created"], 1);
        assert_eq!(m.counters["ci_forks_tests_run"], 1);
        assert_eq!(m.counters["node_bootstraps_started"], 1);
        assert_eq!(m.counters["node_bootstraps_completed"], 1);
    }
}
