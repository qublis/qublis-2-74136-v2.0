//! Consensus NeuroFlux integration for Qublis v2.0 (Universe 2-74136).
//!
//! Embeds a NeuroFlux reinforcement‐learning agent into the QMesh consensus loop,
//! enabling dynamic, real‐time tuning of consensus parameters based on observed
//! network metrics.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use qublis_qnum::QNum;
use qublis_ci_core::{NeuroFluxAgent, Action};
use crate::config::ConsensusConfig;
use crate::types::ConsensusEngine;
use crate::metrics::ConsensusMetrics;

/// NeuroFlux‐driven consensus adapter.
#[derive(Debug)]
pub struct ConsensusNeuroFlux {
    agent: NeuroFluxAgent,
    metrics: ConsensusMetrics,
}

impl ConsensusNeuroFlux {
    /// Initialize NeuroFlux within the consensus engine.
    ///
    /// # Arguments
    ///
    /// * `cfg` - Consensus configuration, containing NeuroFlux parameters.
    pub fn new(cfg: &ConsensusConfig) -> Self {
        // Initialize the NeuroFlux RL agent
        let mut agent = NeuroFluxAgent::new(&cfg.neuroflux);
        // Setup metrics collector
        let mut metrics = ConsensusMetrics::new();
        metrics.inc_counter("neuroflux_initialized", 1);
        ConsensusNeuroFlux { agent, metrics }
    }

    /// Execute one tick of the consensus loop with NeuroFlux optimization.
    ///
    /// Observes current state, selects and applies an action, runs the engine,
    /// computes reward, and updates the RL agent.
    ///
    /// # Arguments
    ///
    /// * `engine` - Mutable reference to the consensus engine.
    pub fn tick(&mut self, engine: &mut ConsensusEngine) {
        // 1. Observe current consensus state
        let state = self.collect_state(engine);

        // 2. Select an optimization action
        let action: Action = self.agent.select_action(&state);

        // 3. Apply the action to the consensus parameters
        self.apply_action(engine, &action);

        // 4. Produce blocks for one epoch
        engine.produce_blocks();

        // 5. Compute reward based on engine performance
        let reward = self.compute_reward(engine, &state);

        // 6. Update NeuroFlux agent with (state, action, reward)
        self.agent.learn(state, action, reward);

        // 7. Record metrics
        self.metrics.inc_counter("neuroflux_ticks", 1);
        self.metrics.record_observation(&state);
        self.metrics.record_reward(reward);
    }

    /// Gathers observable metrics from the engine into a single QNum.
    fn collect_state(&self, engine: &ConsensusEngine) -> QNum {
        // Pack tip_count, avg_latency_ms, cognitive_entropy, fork_rate*100
        QNum::from_digits(&[
            engine.tip_count as u8,
            engine.avg_latency_ms as u8,
            engine.cognitive_entropy as u8,
            (engine.fork_rate * 100.0) as u8,
        ])
    }

    /// Applies a NeuroFlux action to the engine's consensus configuration.
    fn apply_action(&self, engine: &mut ConsensusEngine, action: &Action) {
        let cfg = &mut engine.config;
        // Adjust entropy threshold
        cfg.entropy_finality = (cfg.entropy_finality + action.delta_entropy)
            .clamp(cfg.min_entropy, cfg.max_entropy);
        // Adjust tip count
        cfg.max_tips = ((cfg.max_tips as isize) + action.delta_tips)
            .clamp(cfg.min_tips as isize, cfg.max_tips as isize) as usize;
    }

    /// Computes a scalar reward from the engine's performance metrics.
    fn compute_reward(&self, engine: &ConsensusEngine, _prev_state: &QNum) -> f64 {
        let tps_ratio = engine.measured_tps as f64 / engine.target_tps as f64;
        let latency_penalty = engine.avg_latency_ms / engine.max_latency_ms;
        let fork_penalty = engine.fork_rate;
        engine.config.reward_weights.tps * tps_ratio
            - engine.config.reward_weights.latency * latency_penalty
            - engine.config.reward_weights.forks * fork_penalty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConsensusConfig;
    use crate::types::ConsensusEngine;
    use qublis_ci_core::RewardWeights;

    #[test]
    fn reward_calculation_example() {
        // Prepare a dummy ConsensusConfig with a dummy NeuroFluxConfig
        let cfg = ConsensusConfig {
            qmesh_config_path: "qmesh.toml".into(),
            neuroflux_enabled: true,
            neuroflux_config_path: None,
        };
        let mut cnf = ConsensusNeuroFlux::new(&cfg);

        // Mock a ConsensusEngine with sample metrics
        let mut engine = ConsensusEngine::mock();
        engine.measured_tps = 900;
        engine.target_tps = 1000;
        engine.avg_latency_ms = 50.0;
        engine.max_latency_ms = 100.0;
        engine.fork_rate = 0.02;
        engine.config.reward_weights = RewardWeights { tps: 1.0, latency: 1.0, forks: 1.0 };

        let state = cnf.collect_state(&engine);
        let reward = cnf.compute_reward(&engine, &state);

        // expected = 0.9 - 0.5 - 0.02 = 0.38
        assert!((reward - 0.38).abs() < 1e-6);
    }
}
