//! Prometheus‐style in‐memory metrics collector for the Qublis runtime.
//!
//! Supports simple counters and gauges, with text‐format export for Prometheus scraping.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::sync::Mutex;

/// In‐memory metrics collector.
#[derive(Debug)]
pub struct RuntimeMetrics {
    counters: Mutex<HashMap<String, u64>>,
    gauges: Mutex<HashMap<String, f64>>,
}

impl RuntimeMetrics {
    /// Create a new, empty metrics collector.
    pub fn new() -> Self {
        RuntimeMetrics {
            counters: Mutex::new(HashMap::new()),
            gauges: Mutex::new(HashMap::new()),
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

    /// Get a snapshot of the named counter, if present.
    pub fn get_counter(&self, name: &str) -> Option<u64> {
        let ctrs = self.counters.lock().unwrap();
        ctrs.get(name).copied()
    }

    /// Get a snapshot of the named gauge, if present.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_and_gauge_workflow() {
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
