//! Zero Propagator for QNetX — Qublis v2.0
//!
//! The `ZeroPropagator` entangles each channel’s quantum‐number state (`QNum`)
//! with a zero-state QNum, propagating “zero amplitude modes” across the network.
//!
//! This can be used to inject or diffuse zero‐mode information for consistency
//! checks, anomaly damping, or initialization routines.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use crate::config::QNetXConfig;
use crate::metrics::QNetXMetrics;
use qublis_qnum::{QNum, entangle};

/// Zero propagator: mixes each channel’s state with a zero‐mode QNum.
#[derive(Clone, Debug)]
pub struct ZeroPropagator {
    config: QNetXConfig,
    metrics: QNetXMetrics,
}

impl ZeroPropagator {
    /// Create a new `ZeroPropagator` using the given configuration.
    pub fn new(config: &QNetXConfig) -> Self {
        ZeroPropagator {
            config: config.clone(),
            metrics: QNetXMetrics::new(),
        }
    }

    /// Entangle the provided `state` with a zero QNum of the same length.
    ///
    /// This “propagates” zero‐mode amplitudes across the QNum, increasing
    /// its joint entropy and diffusing information uniformly.
    pub fn propagate(&mut self, state: &mut QNum) {
        // Build a zero QNum matching the digit‐length of `state`
        let mut zero = QNum::zero(state.len());
        // Perform a Bell‐like entanglement between `state` and `zero`
        entangle(state, &mut zero);
        // Record a zero‐propagation event
        self.metrics.inc_counter("zero_propagations", 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;
    use num_complex::Complex;

    #[test]
    fn zero_propagator_preserves_classical_states() {
        let cfg = QNetXConfig::default();
        let mut zp = ZeroPropagator::new(&cfg);

        // A purely classical QNum should remain unchanged
        let mut qnum = QNum::from_digits(&[5, 7, 3]);
        zp.propagate(&mut qnum);
        assert_eq!(qnum.measure(), vec![5, 7, 3]);
    }

    #[test]
    fn zero_propagator_increases_entropy_on_superposed_states() {
        let cfg = QNetXConfig::default();
        let mut zp = ZeroPropagator::new(&cfg);

        // Create a superposed QNum: equal mix of |1⟩ and |2⟩
        let states = vec![
            (vec![1], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
            (vec![2], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
        ];
        let mut qnum = QNum::from_superposed(states);
        let before = qnum.entropy();

        zp.propagate(&mut qnum);
        let after = qnum.entropy();

        assert!(after >= before, "Entropy should not decrease after zero-propagation");
    }
}
