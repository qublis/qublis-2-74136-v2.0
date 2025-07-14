//! Core data types for CI‐Core — Qublis v2.0
//!
//! Defines the primary shared types used by the MorphicAI, MoralRegulator,
//! and CollectiveSync modules.

use qublis_qnum::QNum;
use serde::{Deserialize, Serialize};

/// Internal neural substrate state: a vector of neuron activation QNums.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeuralState(pub Vec<QNum>);

impl NeuralState {
    /// Create a zero‐state with `num_neurons` neurons (each initialized to |0⟩).
    pub fn zero(num_neurons: usize) -> Self {
        NeuralState(vec![QNum::zero(1); num_neurons])
    }

    /// Number of neurons in the state.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Sensory input to `MorphicAI`: a vector of `QNum` signals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SensoryInput(pub Vec<QNum>);

impl SensoryInput {
    /// Build a `SensoryInput` from a list of classical digits,
    /// each turned into a single‐digit `QNum`.
    pub fn from_digits(digits: Vec<u8>) -> Self {
        let vec = digits.into_iter()
            .map(|d| QNum::from_digits(&[d]))
            .collect();
        SensoryInput(vec)
    }

    /// Number of input channels.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Motor output from `MorphicAI`: a vector of classical u8 signals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotorOutput {
    pub signals: Vec<u8>,
}

/// Identifier for a distributed AI agent.
pub type AgentId = String;

/// Holds the quantum state of an agent for `CollectiveSync`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentState {
    pub state: QNum,
}

/// Message sent between agents, carrying a `QNum` state payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncMessage {
    pub from: AgentId,
    pub state: QNum,
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn neural_state_zero_and_len() {
        let ns = NeuralState::zero(4);
        assert_eq!(ns.len(), 4);
        for q in ns.0 {
            // each neuron measures to [0]
            assert_eq!(q.measure(), vec![0]);
        }
    }

    #[test]
    fn sensory_input_from_digits_and_len() {
        let digits = vec![1, 2, 7];
        let si = SensoryInput::from_digits(digits.clone());
        assert_eq!(si.len(), 3);
        // each channel is a single‐digit QNum
        let measures: Vec<_> = si.0.iter().map(|q| q.measure()[0]).collect();
        assert_eq!(measures, digits);
    }

    #[test]
    fn motor_output_signals() {
        let out = MotorOutput { signals: vec![3, 5, 9] };
        assert_eq!(out.signals, vec![3, 5, 9]);
    }

    #[test]
    fn agent_state_and_sync_message() {
        let q = QNum::from_digits(&[4, 2]);
        let agent = AgentState { state: q.clone() };
        assert_eq!(agent.state.measure(), q.measure());

        let msg = SyncMessage { from: "agent1".into(), state: q.clone() };
        assert_eq!(msg.from, "agent1");
        assert_eq!(msg.state.measure(), q.measure());
    }
}
