//! QNetX Metrics Collector
//!
//! Collects and exports metrics for the entangled‐overlay mesh (QNetX) subsystem.
//! Domain‐specific metrics include channel creations, entanglements, condensations,
//! and anomaly detections.

use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use serde::Serialize;

/// Snapshot of counters and gauges at a point in time.
#[derive(Debug, Clone, Serialize)]
struct MetricSnapshot {
    /// Milliseconds since UNIX epoch.
    timestamp: u128,
    /// Monotonic counters.
    counters: HashMap<String, u64>,
    /// Instantaneous gauges.
    gauges: HashMap<String, f64>,
}

/// Collector for QNetX metrics.
#[derive(Clone, Debug)]
pub struct QNetXMetrics {
    start: Instant,
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    snapshots: Vec<MetricSnapshot>,
}

impl QNetXMetrics {
    /// Create a new metrics collector.
    pub fn new() -> Self {
        QNetXMetrics {
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
            out.push_str(&format!("qnetx_{} {}\n", k, v));
        }
        // gauges
        for (k, v) in &self.gauges {
            out.push_str(&format!("qnetx_{} {}\n", k, v));
        }
        // uptime gauge
        let uptime = self.start.elapsed().as_secs_f64();
        out.push_str(&format!("qnetx_uptime_seconds {:.3}\n", uptime));
        out
    }

    // === Domain‐specific metrics ===

    /// Record creation of a new entangled channel.
    pub fn record_channel_created(&mut self) {
        self.inc_counter("channels_created", 1);
    }

    /// Record an entanglement operation on a channel.
    pub fn record_entanglement(&mut self) {
        self.inc_counter("entanglements", 1);
    }

    /// Record a full condensation of all channels.
    pub fn record_condense_all(&mut self) {
        self.inc_counter("condense_all", 1);
    }

    /// Record a grouped condensation by prefix.
    pub fn record_condense_by_prefix(&mut self) {
        self.inc_counter("condense_by_prefix", 1);
    }

    /// Record detection of an anomaly in channel state.
    pub fn record_anomaly_detected(&mut self) {
        self.inc_counter("anomalies_detected", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counter_gauge_and_export() {
        let mut m = QNetXMetrics::new();
        m.inc_counter("foo", 3);
        m.set_gauge("bar", 4.2);
        m.record_snapshot();

        let prom = m.export_prometheus();
        assert!(prom.contains("qnetx_foo 3"));
        assert!(prom.contains("qnetx_bar 4.2"));
        assert!(prom.contains("qnetx_uptime_seconds"));

        let json = m.export_json().unwrap();
        let arr: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["counters"]["foo"].as_u64(), Some(3));
        assert_eq!(arr[0]["gauges"]["bar"].as_f64(), Some(4.2));
    }

    #[test]
    fn domain_specific_records() {
        let mut m = QNetXMetrics::new();
        m.record_channel_created();
        m.record_entanglement();
        m.record_condense_all();
        m.record_condense_by_prefix();
        m.record_anomaly_detected();

        assert_eq!(m.counters["channels_created"], 1);
        assert_eq!(m.counters["entanglements"], 1);
        assert_eq!(m.counters["condense_all"], 1);
        assert_eq!(m.counters["condense_by_prefix"], 1);
        assert_eq!(m.counters["anomalies_detected"], 1);
    }
}
