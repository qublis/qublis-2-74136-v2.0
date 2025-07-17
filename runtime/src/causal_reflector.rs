//! Causal reflector for multi‐dimensional consistency in Qublis v2.0.
//!
//! The CausalReflector ensures that causal dependencies across
//! infinite branching dimensions are enforced in the consensus engine.
//! It traverses ancestor paths up to a configured depth and "reflects"
//! state updates to maintain consistency across all branches.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::time::Instant;

use crate::config::CausalReflectorConfig;
use crate::error::RuntimeError;
use crate::metrics::RuntimeMetrics;
use crate::types::ConsensusEngine;

/// The causal reflector driver.
#[derive(Debug)]
pub struct CausalReflector {
    enabled: bool,
    max_depth: usize,
    last_run: Instant,
    metrics: RuntimeMetrics,
}

impl CausalReflector {
    /// Creates a new `CausalReflector` from configuration.
    ///
    /// # Arguments
    ///
    /// * `cfg` - Configuration specifying whether causal reflection
    ///   is enabled and the maximum depth to traverse.
    pub fn new(cfg: &CausalReflectorConfig) -> Self {
        let mut metrics = RuntimeMetrics::new();
        if cfg.enabled {
            metrics.inc_counter("causal_reflector_enabled", 1);
        } else {
            metrics.inc_counter("causal_reflector_disabled", 1);
        }
        CausalReflector {
            enabled: cfg.enabled,
            max_depth: cfg.max_depth,
            last_run: Instant::now(),
            metrics,
        }
    }

    /// Perform one round of causal reflection on the consensus engine.
    ///
    /// If disabled, this is a no‐op.
    /// Otherwise, traverses up to `max_depth` ancestors for each new block
    /// and ensures state updates are propagated across all branches.
    ///
    /// # Errors
    ///
    /// Returns `RuntimeError` if the engine's causal reflection fails.
    pub fn reflect(&mut self, engine: &mut ConsensusEngine) -> Result<(), RuntimeError> {
        if !self.enabled {
            return Ok(());
        }
        // Kick off causal reflection in the engine
        let reflected = engine.reflect_causal(self.max_depth)?;
        // Record metrics
        self.metrics.inc_counter("causal_reflections", 1);
        self.metrics.set_gauge("causal_reflected_count", reflected as f64);
        self.last_run = Instant::now();
        Ok(())
    }

    /// Export collected metrics in Prometheus text format.
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::CausalReflectorConfig;
    use crate::types::ConsensusEngine;

    #[test]
    fn new_reflector_records_state() {
        let cfg = CausalReflectorConfig { enabled: true, max_depth: 10 };
        let reflector = CausalReflector::new(&cfg);
        assert_eq!(reflector.metrics.get_counter("causal_reflector_enabled"), Some(&1));
        assert_eq!(reflector.enabled, true);
        assert_eq!(reflector.max_depth, 10);
    }

    #[test]
    fn reflect_noop_when_disabled() {
        let cfg = CausalReflectorConfig { enabled: false, max_depth: 5 };
        let mut reflector = CausalReflector::new(&cfg);
        let mut engine = ConsensusEngine::mock();
        // Should not call engine.reflect_causal
        assert!(reflector.reflect(&mut engine).is_ok());
        assert_eq!(reflector.metrics.get_counter("causal_reflections"), None);
    }

    #[test]
    fn reflect_traverses_and_records() {
        let cfg = CausalReflectorConfig { enabled: true, max_depth: 3 };
        let mut reflector = CausalReflector::new(&cfg);
        let mut engine = ConsensusEngine::mock();
        // Configure the mock to reflect exactly 7 items
        engine.set_causal_reflection_capacity(7);
        assert!(reflector.reflect(&mut engine).is_ok());
        assert_eq!(reflector.metrics.get_counter("causal_reflections"), Some(&1));
        assert_eq!(
            reflector.metrics.get_gauge("causal_reflected_count"),
            Some(&7.0)
        );
    }

    #[test]
    fn export_prometheus_contains_metrics() {
        let cfg = CausalReflectorConfig { enabled: true, max_depth: 1 };
        let reflector = CausalReflector::new(&cfg);
        let output = reflector.export_metrics();
        assert!(output.contains("causal_reflector_enabled 1"));
    }
}
