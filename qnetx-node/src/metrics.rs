//! In‐memory metrics collector for the `qublis-qnetx-node` CLI.
//!
//! Provides a global `RuntimeMetrics` instance and helper functions to
//! increment counters, set gauges, and export all metrics in Prometheus
//! text format.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// In‐memory metrics collector.
/// 
/// Supports counters and gauges, with Prometheus‐style text export.
#[derive(Debug)]
pub struct RuntimeMetrics {
    counters: Mutex<HashMap<String, u64>>,
    gauges:   Mutex<HashMap<String, f64>>,
}

impl RuntimeMetrics {
    /// Create a new, empty metrics collector.
    pub fn new() -> Self {
        RuntimeMetrics {
            counters: Mutex::new(HashMap::new()),
            gauges:   Mutex::new(HashMap::new()),
        }
    }

    /// Increment the named counter by `value`.
    ///
    /// Creates the counter if it does not yet exist.
    pub fn inc_counter(&self, name: &str, value: u64) {
        let mut ctrs = self.counters.lock().unwrap();
        let entry = ctrs.entry(name.to_string()).or_insert(0);
        *entry = entry.saturating_add(value);
    }

    /// Set the named gauge to `value`.
    ///
    /// Creates the gauge if it does not yet exist.
    pub fn set_gauge(&self, name: &str, value: f64) {
        let mut gs = self.gauges.lock().unwrap();
        gs.insert(name.to_string(), value);
    }

    /// Get the current value of a counter, if present.
    #[cfg(test)]
    pub fn get_counter(&self, name: &str) -> Option<u64> {
        let ctrs = self.counters.lock().unwrap();
        ctrs.get(name).copied()
    }

    /// Get the current value of a gauge, if present.
    #[cfg(test)]
    pub fn get_gauge(&self, name: &str) -> Option<f64> {
        let gs = self.gauges.lock().unwrap();
        gs.get(name).copied()
    }

    /// Export all metrics in Prometheus text format.
    ///
    /// Each line is: `<metric_name> <value>\n`
    pub fn export_prometheus(&self) -> String {
        let mut out = String::new();
        {
            let ctrs = self.counters.lock().unwrap();
            for (k, v) in ctrs.iter() {
                out.push_str(&format!("{} {}\n", k, v));
            }
        }
        {
            let gs = self.gauges.lock().unwrap();
            for (k, v) in gs.iter() {
                out.push_str(&format!("{} {}\n", k, v));
            }
        }
        out
    }
}

/// Global, singleton metrics collector for the qnetx-node.
lazy_static! {
    static ref METRICS: RuntimeMetrics = RuntimeMetrics::new();
}

/// Increment the named counter in the global metrics.
pub fn inc_counter(name: &str, value: u64) {
    METRICS.inc_counter(name, value);
}

/// Set the named gauge in the global metrics.
pub fn set_gauge(name: &str, value: f64) {
    METRICS.set_gauge(name, value);
}

/// Export all global metrics in Prometheus text format.
pub fn export_prometheus() -> String {
    METRICS.export_prometheus()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_and_gauge_workflow() {
        // Use a fresh, local instance for testing:
        let m = RuntimeMetrics::new();
        assert_eq!(m.get_counter("foo"), None);
        m.inc_counter("foo", 1);
        assert_eq!(m.get_counter("foo"), Some(1));
        m.inc_counter("foo", 3);
        assert_eq!(m.get_counter("foo"), Some(4));

        assert_eq!(m.get_gauge("bar"), None);
        m.set_gauge("bar", 2.5);
        assert_eq!(m.get_gauge("bar"), Some(2.5));
        m.set_gauge("bar", 7.5);
        assert_eq!(m.get_gauge("bar"), Some(7.5));
    }

    #[test]
    fn export_prometheus_contains_all() {
        let m = RuntimeMetrics::new();
        m.inc_counter("c1", 2);
        m.set_gauge("g1", 3.14);
        let txt = m.export_prometheus();
        assert!(txt.contains("c1 2"));
        assert!(txt.contains("g1 3.14"));
    }
}
