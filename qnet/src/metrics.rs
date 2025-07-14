//! QNet Metrics Collector
//!
//! Collects and exports metrics for the QNet routing, relay, and teleportation subsystems.
//! Domain-specific metrics include relay attempts, successes, path lengths, hops, teleport attempts, etc.

use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::Serialize;

/// A snapshot of counters and gauges at a point in time.
#[derive(Debug, Clone, Serialize)]
struct MetricSnapshot {
    /// Milliseconds since UNIX epoch.
    timestamp: u128,
    /// Monotonic counters.
    counters: HashMap<String, u64>,
    /// Instantaneous gauges.
    gauges: HashMap<String, f64>,
}

/// Collector for QNet metrics.
#[derive(Clone, Debug)]
pub struct QNetMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl QNetMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        QNetMetrics {
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

    /// Export all recorded snapshots as pretty‐printed JSON.
    pub fn export_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.snapshots)
            .map_err(|e| format!("Metrics JSON export failed: {}", e))
    }

    /// Export current counters and gauges in Prometheus text format.
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        // counters
        for (k, v) in &self.counters {
            out.push_str(&format!("qnet_{} {}\n", k, v));
        }
        // gauges
        for (k, v) in &self.gauges {
            out.push_str(&format!("qnet_{} {}\n", k, v));
        }
        // uptime gauge
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("qnet_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain‐specific metrics ===

    /// Record a relay attempt.
    pub fn record_relay_attempt(&mut self) {
        self.inc_counter("relay_attempts", 1);
    }

    /// Record a successful relay.
    pub fn record_relay_success(&mut self) {
        self.inc_counter("relay_successes", 1);
    }

    /// Record the path length selected (number of hops).
    pub fn record_path_length(&mut self, length: usize) {
        // Use a gauge for last path length
        self.set_gauge("last_path_length", length as f64);
        // Histogram or counter for path lengths could be added as needed
    }

    /// Record a hop in hop-by-hop or teleport fallback.
    pub fn record_hop(&mut self) {
        self.inc_counter("hops", 1);
    }

    /// Record a teleport attempt.
    pub fn record_teleport_attempt(&mut self) {
        self.inc_counter("teleport_attempts", 1);
    }

    /// Record a successful teleport.
    pub fn record_teleport_success(&mut self) {
        self.inc_counter("teleport_successes", 1);
    }

    /// Record a failed teleport.
    pub fn record_teleport_failure(&mut self) {
        self.inc_counter("teleport_failures", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counter_gauge_and_export() {
        let mut m = QNetMetrics::new();
        m.inc_counter("relay_attempts", 2);
        m.set_gauge("last_path_length", 3.5);
        m.record_snapshot();

        // Prometheus export should contain metrics
        let prom = m.export_prometheus();
        assert!(prom.contains("qnet_relay_attempts 2"));
        assert!(prom.contains("qnet_last_path_length 3.5"));
        assert!(prom.contains("qnet_uptime_seconds"));

        // JSON export contains a snapshot array
        let json = m.export_json().expect("export json");
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        let snap = &arr[0];
        assert_eq!(snap["counters"]["relay_attempts"].as_u64(), Some(2));
        assert_eq!(snap["gauges"]["last_path_length"].as_f64(), Some(3.5));
    }

    #[test]
    fn domain_specific_records() {
        let mut m = QNetMetrics::new();
        m.record_relay_attempt();
        m.record_relay_success();
        m.record_path_length(4);
        m.record_hop();
        m.record_teleport_attempt();
        m.record_teleport_success();
        m.record_teleport_failure();

        assert_eq!(m.counters["relay_attempts"], 1);
        assert_eq!(m.counters["relay_successes"], 1);
        assert_eq!(m.counters["hops"], 1);
        assert_eq!(m.counters["teleport_attempts"], 1);
        assert_eq!(m.counters["teleport_successes"], 1);
        assert_eq!(m.counters["teleport_failures"], 1);
        assert_eq!(*m.gauges.get("last_path_length").unwrap(), 4.0);
    }
}
