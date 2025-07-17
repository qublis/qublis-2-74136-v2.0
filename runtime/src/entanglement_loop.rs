//! Entanglement loop for quantum‐inspired state propagation in Qublis v2.0.
//!
//! Periodically propagates “entanglement” (quantum‐style correlations) across
//! the consensus engine’s multi-dimensional branches, exploring up to
//! `max_branches` per cycle and recording metrics.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::time::{Duration, Instant};

use crate::config::EntanglementConfig;
use crate::error::RuntimeError;
use crate::metrics::RuntimeMetrics;
use crate::types::ConsensusEngine;

/// Periodic entanglement propagation loop.
#[derive(Debug)]
pub struct EntanglementLoop {
    interval: Duration,
    max_branches: usize,
    last_run: Instant,
    metrics: RuntimeMetrics,
}

impl EntanglementLoop {
    /// Create a new `EntanglementLoop` from the given config.
    ///
    /// # Arguments
    ///
    /// * `cfg` – entanglement‐loop parameters from `RuntimeConfig`.
    pub fn new(cfg: &EntanglementConfig) -> Self {
        let interval = Duration::from_millis(cfg.interval_ms);
        let mut metrics = RuntimeMetrics::new();
        metrics.inc_counter("entanglement_loop_initialized", 1);
        EntanglementLoop {
            interval,
            max_branches: cfg.max_branches,
            last_run: Instant::now(),
            metrics,
        }
    }

    /// Attempt to run an entanglement propagation tick.
    ///
    /// Only actually propagates if `interval` has elapsed since `last_run`.
    /// Propagates up to `max_branches` branches in the consensus engine,
    /// then records counters and gauges.
    ///
    /// # Errors
    ///
    /// Returns `RuntimeError` if the underlying engine propagation fails.
    pub fn tick(&mut self, engine: &mut ConsensusEngine) -> Result<(), RuntimeError> {
        let now = Instant::now();
        if now.duration_since(self.last_run) < self.interval {
            // Not yet time for the next propagation
            return Ok(())
        }

        // Perform entanglement propagation in the consensus engine
        // (returns number of branches actually processed)
        let processed = engine.propagate_entanglement(self.max_branches)?;
        
        // Record metrics
        self.metrics.inc_counter("entanglement_ticks", 1);
        self.metrics.set_gauge("entanglement_branches_processed", processed as f64);

        // Update timer
        self.last_run = now;
        Ok(())
    }

    /// Export internal metrics in Prometheus text format.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EntanglementConfig;
    use crate::types::ConsensusEngine;

    #[test]
    fn initialization_sets_values() {
        let cfg = EntanglementConfig { interval_ms: 123, max_branches: 7 };
        let el = EntanglementLoop::new(&cfg);
        assert_eq!(el.interval.as_millis(), 123);
        assert_eq!(el.max_branches, 7);
        assert_eq!(el.metrics.get_counter("entanglement_loop_initialized"), Some(&1));
    }

    #[test]
    fn tick_before_interval_skips() {
        let cfg = EntanglementConfig { interval_ms: 1_000_000, max_branches: 3 };
        let mut el = EntanglementLoop::new(&cfg);
        let mut engine = ConsensusEngine::mock();
        // Engine.mock() should not record propagation if not called
        assert!(el.tick(&mut engine).is_ok());
        assert_eq!(el.metrics.get_counter("entanglement_ticks"), Some(&0));
    }

    #[test]
    fn tick_after_interval_propagates() {
        let cfg = EntanglementConfig { interval_ms: 0, max_branches: 5 };
        let mut el = EntanglementLoop::new(&cfg);
        let mut engine = ConsensusEngine::mock();
        // Configure mock to process exactly 4 branches when asked
        engine.set_entanglement_capacity(4);
        assert!(el.tick(&mut engine).is_ok());
        assert_eq!(el.metrics.get_counter("entanglement_ticks"), Some(&1));
        assert_eq!(
            el.metrics.get_gauge("entanglement_branches_processed"),
            Some(&4.0)
        );
    }
}
