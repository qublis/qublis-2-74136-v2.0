//! Core runtime types for Qublis v2.0 (2-74136).
//!
//! Defines the `Block`, `ConsensusEngine`, its configuration and errors,
//! as well as mock utilities for testing.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use qublis_qnum::QNum;
use qublis_ci_core::RewardWeights;
use thiserror::Error;

/// A block in the QMesh entropic DAG.
#[derive(Debug, Clone)]
pub struct Block {
    /// Unique block identifier (quantum number).
    pub id: QNum,
    /// Parent block identifiers.
    pub parents: Vec<QNum>,
    /// Arbitrary payload (transactions, state diffs, etc.).
    pub payload: Vec<u8>,
    /// Entropy score of this block.
    pub entropy: f64,
    /// Unix timestamp of block creation.
    pub timestamp: u64,
}

/// Errors returned by the consensus engine.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Failure during entanglement propagation.
    #[error("entanglement propagation failed")]
    Entanglement,
    /// Failure during causal reflection.
    #[error("causal reflection failed")]
    Causal,
}

/// Configuration parameters for the consensus engine.
#[derive(Debug, Clone)]
pub struct ConsensusEngineConfig {
    /// Entropy threshold at which blocks are considered final.
    pub entropy_finality: f64,
    /// Minimum allowed entropy threshold.
    pub min_entropy: f64,
    /// Maximum allowed entropy threshold.
    pub max_entropy: f64,
    /// Minimum number of tips to reference when building a new block.
    pub min_tips: usize,
    /// Maximum number of tips to reference.
    pub max_tips: usize,
    /// Weights used by NeuroFlux to compute rewards.
    pub reward_weights: RewardWeights,
}

impl Default for ConsensusEngineConfig {
    fn default() -> Self {
        ConsensusEngineConfig {
            entropy_finality: 10.0,
            min_entropy: 1.0,
            max_entropy: 50.0,
            min_tips: 1,
            max_tips: 8,
            reward_weights: RewardWeights {
                tps: 0.7,
                latency: 0.2,
                forks: 0.1,
            },
        }
    }
}

/// The consensus engine drives QMesh block production and tip‐selection,
/// and exposes hooks for entanglement propagation and causal reflection.
#[derive(Debug)]
pub struct ConsensusEngine {
    /// Consensus engine parameters.
    pub config: ConsensusEngineConfig,
    /// Current number of open tips.
    pub tip_count: usize,
    /// Observed average block latency (ms).
    pub avg_latency_ms: f64,
    /// Current cognitive entropy of the DAG.
    pub cognitive_entropy: f64,
    /// Current fork rate (fraction of orphaned blocks).
    pub fork_rate: f64,
    /// Measured transactions (or blocks) per second.
    pub measured_tps: u64,
    /// Target transactions (or blocks) per second.
    pub target_tps: u64,
    /// Maximum tolerated latency (ms) for reward calculation.
    pub max_latency_ms: f64,
    // Internal mocks/testing capacity:
    entanglement_capacity: usize,
    causal_reflection_capacity: usize,
}

impl ConsensusEngine {
    /// Create a new consensus engine with the given configuration.
    pub fn new(config: ConsensusEngineConfig) -> Self {
        ConsensusEngine {
            config,
            tip_count: 0,
            avg_latency_ms: 0.0,
            cognitive_entropy: 0.0,
            fork_rate: 0.0,
            measured_tps: 0,
            target_tps: 1,
            max_latency_ms: 1.0,
            entanglement_capacity: 0,
            causal_reflection_capacity: 0,
        }
    }

    /// Produce blocks for one epoch.  In a real implementation this
    /// would assemble transactions, compute entropy, broadcast the block, etc.
    pub fn produce_blocks(&mut self) {
        // no-op for scaffold; real code advances the DAG
    }

    /// Propagate entanglement across up to `max` branches.
    /// Returns the number of branches actually processed.
    pub fn propagate_entanglement(&mut self, max: usize) -> Result<usize, EngineError> {
        // In testing, return the mock capacity; otherwise cap at `max`.
        let n = self.entanglement_capacity.min(max);
        Ok(n)
    }

    /// Reflect causal dependencies up to `max_depth` hops.
    /// Returns the number of reflections performed.
    pub fn reflect_causal(&mut self, _max_depth: usize) -> Result<usize, EngineError> {
        Ok(self.causal_reflection_capacity)
    }

    /// Construct a mock engine for unit tests.
    pub fn mock() -> Self {
        ConsensusEngine::new(ConsensusEngineConfig::default())
    }

    /// Set the number of branches that `propagate_entanglement` will process.
    pub fn set_entanglement_capacity(&mut self, cap: usize) {
        self.entanglement_capacity = cap;
    }

    /// Set the number of items that `reflect_causal` will report.
    pub fn set_causal_reflection_capacity(&mut self, cap: usize) {
        self.causal_reflection_capacity = cap;
    }
}
