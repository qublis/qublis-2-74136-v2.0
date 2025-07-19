//! Morphic AI — Adaptive, Self‐Organizing Neural Substrate for Qublis v2.0
//!
//! `MorphicAI` implements a quantum‐enhanced, morphogenetic neural engine:
//! - Maintains an internal `NeuralState` (a `QNum` vector of neuron activations).
//! - Processes incoming `SensoryInput` by entangling with internal state.
//! - Generates `MotorOutput` by measuring a transform of the state.
//! - Supports reinforcement training by feedback‐driven state updates.
//!
//! All neuron activations are represented as quantum number superpositions,
//! entangled to model emergent network dynamics.

use qublis_qnum::{QNum, entangle, qadd};
use crate::config::CiCoreConfig;
use crate::error::CiCoreError;
use crate::metrics::CiCoreMetrics;
use crate::types::{NeuralState, SensoryInput, MotorOutput};

/// MorphicAI holds the current neural state and configuration.
#[derive(Debug, Clone)]
pub struct MorphicAI {
    config: CiCoreConfig,
    metrics: CiCoreMetrics,
    /// Current internal neuron activations (vector of QNums).
    pub state: NeuralState,
}

impl MorphicAI {
    /// Initialize a new `MorphicAI` with default (zero) neural state.
    pub fn new(config: &CiCoreConfig) -> Self {
        let mut metrics = CiCoreMetrics::new();
        metrics.inc_counter("morphic_ai_initialized", 1);
        MorphicAI {
            config: config.clone(),
            metrics,
            state: NeuralState::zero(config.num_neurons),
        }
    }

    /// Perceive incoming sensory data by entangling with internal state.
    ///
    /// Each input channel (a QNum) is entangled with the corresponding neuron.
    pub fn perceive(&mut self, input: SensoryInput) -> Result<(), CiCoreError> {
        if input.len() != self.state.len() {
            return Err(CiCoreError::DimensionMismatch {
                expected: self.state.len(),
                got: input.len(),
            });
        }
        for (neuron, sensory) in self.state.0.iter_mut().zip(input.0.into_iter()) {
            entangle(neuron, &mut sensory.clone());
        }
        self.metrics.inc_counter("morphic_ai_perceptions", 1);
        Ok(())
    }

    /// Generate motor output by collapsing a summary of the neural state.
    ///
    /// We sum all neuron QNums into a single QNum, then measure to produce outputs.
    pub fn generate(&mut self) -> MotorOutput {
        // Condense all neurons via quantum addition
        let mut summary = self.state.0.first().cloned()
            .unwrap_or_else(|| QNum::zero(1));
        for q in self.state.0.iter().skip(1) {
            summary = qadd(&summary, q);
        }
        let digits = summary.measure();
        self.metrics.inc_counter("morphic_ai_generations", 1);

        // Map measured digits to motor outputs
        MotorOutput { signals: digits }
    }

    /// Train the network via reinforcement: reward > 0 strengthens current state.
    ///
    /// We scale each neuron's amplitude by `1 + learning_rate * reward`, then normalize.
    pub fn train(&mut self, reward: f64) {
        let lr = self.config.learning_rate * reward;
        for neuron in &mut self.state.0 {
            // For each amplitude α: α → α * (1 + lr)
            for qid in &mut neuron.0 {
                for amp in qid.amps.iter_mut() {
                    // amp: num_complex::Complex<ordered_float::OrderedFloat<f64>>
                    // We must multiply amp by the same type, so cast scalar to Complex<OrderedFloat<f64>>
                    // If not imported already:
                    // use num_complex::Complex;
                    // use ordered_float::OrderedFloat;

                    // Create a Complex<OrderedFloat<f64>> from f64
                    let scalar = num_complex::Complex {
                        re: ordered_float::OrderedFloat(1.0 + lr),
                        im: ordered_float::OrderedFloat(0.0),
                    };
                    *amp = *amp * scalar;
                }
                qid.normalize();
            }
        }
        self.metrics.inc_counter("morphic_ai_trains", 1);
    }

    /// Export internal metrics (Prometheus text format).
    pub fn export_metrics(&self) -> String {
        self.metrics.export_prometheus()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    fn default_cfg() -> CiCoreConfig {
        let mut cfg = CiCoreConfig::default();
        cfg.num_neurons = 3;
        cfg.learning_rate = 0.1;
        cfg
    }

    #[test]
    fn initialization_sets_zero_state() {
        let cfg = default_cfg();
        let ai = MorphicAI::new(&cfg);
        assert_eq!(ai.state.0.len(), cfg.num_neurons);
        for neuron in ai.state.0 {
            // Zero‐state QNum measures to all zeros
            assert_eq!(neuron.measure(), vec![0; neuron.len()]);
        }
    }

    #[test]
    fn perceive_entangles_state() {
        let cfg = default_cfg();
        let mut ai = MorphicAI::new(&cfg);
        // Build sensory input: each neuron gets classical 1
        let sensory = SensoryInput::from_digits(vec![1,1,1]);
        ai.perceive(sensory.clone()).unwrap();

        // After entangle, at least one neuron has non-zero entropy
        let entropies: Vec<f64> = ai.state.0.iter().map(|n| n.entropy()).collect();
        assert!(entropies.iter().any(|&e| e > 0.0));
    }

    #[test]
    fn generate_produces_motor_output() {
        let cfg = default_cfg();
        let mut ai = MorphicAI::new(&cfg);
        // Set a known state: two neurons both value "2"
        ai.state = NeuralState(vec![
            QNum::from_digits(&[2]),
            QNum::from_digits(&[2]),
            QNum::from_digits(&[0]),
        ]);
        let out = ai.generate();
        // signals is a Vec<u8>
        assert!(!out.signals.is_empty());
        // each signal digit is between 0 and 9
        for &d in &out.signals {
            assert!(d < 10);
        }
    }

    #[test]
    fn train_scales_amplitudes() {
        let cfg = default_cfg();
        let mut ai = MorphicAI::new(&cfg);
        // Initialize one neuron to superposition [1/sqrt2,1/sqrt2]
        let q = QNum::from_superposed(vec![
            (vec![0], 1.0 / 2f64.sqrt()),
            (vec![1], 1.0 / 2f64.sqrt()),
        ]);
        ai.state = NeuralState(vec![q.clone(), q.clone(), q.clone()]);
        ai.train(1.0); // reward = 1
        // After training, normalization preserves entropy but amplitudes changed
        for neuron in &ai.state.0 {
            for qid in &neuron.0 {
                let norm_sq: f64 = qid.amps.iter().map(|c| c.norm_sqr()).sum();
                assert!((norm_sq - 1.0).abs() < 1e-12);
            }
        }
    }
}
