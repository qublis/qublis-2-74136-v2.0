//! Unit tests for the Consensus NeuroFlux integration.
//!
//! Verifies state collection, action application, and reward computation.

use crate::consensus_neuroflux::ConsensusNeuroFlux;
use crate::config::ConsensusConfig;
use crate::types::ConsensusEngine;
use qublis_ci_core::Action;
use qublis_qnum::QNum;

fn make_cfg() -> ConsensusConfig {
    ConsensusConfig {
        qmesh_config_path: "qmesh.toml".into(),
        neuroflux_enabled: true,
        neuroflux_config_path: None,
    }
}

#[test]
fn collect_state_packs_metrics_into_qnum() {
    let cfg = make_cfg();
    let mut cnf = ConsensusNeuroFlux::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    engine.tip_count = 5;
    engine.avg_latency_ms = 10.0;
    engine.cognitive_entropy = 20.0;
    engine.fork_rate = 0.03; // *100 = 3
    let state = cnf.collect_state(&engine);
    let expected = QNum::from_digits(&[5, 10, 20, 3]);
    assert_eq!(state, expected, "collect_state should encode [tip,lat,entropy,fork*100]");
}

#[test]
fn apply_action_clamps_to_config_bounds() {
    let cfg = make_cfg();
    let mut cnf = ConsensusNeuroFlux::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    // set known config bounds
    engine.config.min_entropy = 1.0;
    engine.config.max_entropy = 50.0;
    engine.config.entropy_finality = 10.0;
    engine.config.min_tips = 1;
    engine.config.max_tips = 8;
    engine.config.max_tips = 4;

    // action tries to exceed both upper and lower bounds
    let action = Action { delta_entropy: 100.0, delta_tips: 10 };
    cnf.apply_action(&mut engine, &action);
    // entropy should clamp to 50.0
    assert_eq!(engine.config.entropy_finality, 50.0);
    // tips should clamp to max_tips = 8
    assert_eq!(engine.config.max_tips, 8);

    // negative delta to go below min bounds
    let action2 = Action { delta_entropy: -100.0, delta_tips: -10 };
    cnf.apply_action(&mut engine, &action2);
    // entropy clamps to min_entropy = 1.0
    assert_eq!(engine.config.entropy_finality, 1.0);
    // tips clamp to min_tips = 1
    assert_eq!(engine.config.max_tips, 1);
}

#[test]
fn compute_reward_agrees_with_manual_formula() {
    let cfg = make_cfg();
    let mut cnf = ConsensusNeuroFlux::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    // set sample performance
    engine.measured_tps = 900;
    engine.target_tps = 1000;
    engine.avg_latency_ms = 50.0;
    engine.max_latency_ms = 100.0;
    engine.fork_rate = 0.02;
    // override reward weights to unity for simplicity
    engine.config.reward_weights.tps = 1.0;
    engine.config.reward_weights.latency = 1.0;
    engine.config.reward_weights.forks = 1.0;

    let state = cnf.collect_state(&engine);
    let r = cnf.compute_reward(&engine, &state);
    // expected: 0.9 (TPS ratio) - 0.5 (latency penalty) - 0.02 (fork penalty) = 0.38
    assert!((r - 0.38).abs() < 1e-6, "reward should match manual calculation");
}

#[test]
fn tick_runs_without_error() {
    let cfg = make_cfg();
    let mut cnf = ConsensusNeuroFlux::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    // ensure engine has some capacity so that learn() is exercised
    engine.measured_tps = 500;
    engine.target_tps = 1000;
    // call tick; should not panic
    cnf.tick(&mut engine);
}
