//! Unit tests for the entanglement propagation loop (`EntanglementLoop`).

use crate::entanglement_loop::EntanglementLoop;
use crate::config::EntanglementConfig;
use crate::types::ConsensusEngine;

#[test]
fn initialization_records_initialized_metric() {
    let cfg = EntanglementConfig { interval_ms: 123, max_branches: 7 };
    let el = EntanglementLoop::new(&cfg);
    let metrics = el.export_metrics();
    assert!(
        metrics.contains("entanglement_loop_initialized 1"),
        "should record initialization"
    );
}

#[test]
fn tick_before_interval_skipped() {
    // Use a large interval so tick() does nothing
    let cfg = EntanglementConfig { interval_ms: 1_000_000, max_branches: 3 };
    let mut el = EntanglementLoop::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    el.tick(&mut engine).expect("tick should not error");
    let metrics = el.export_metrics();
    assert!(
        !metrics.contains("entanglement_ticks"),
        "no tick metric should be recorded before interval"
    );
}

#[test]
fn tick_after_interval_propagates() {
    // Zero interval so tick() always runs
    let cfg = EntanglementConfig { interval_ms: 0, max_branches: 5 };
    let mut el = EntanglementLoop::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    // Mock engine to process exactly 4 branches
    engine.set_entanglement_capacity(4);
    el.tick(&mut engine).expect("tick should not error");
    let metrics = el.export_metrics();
    assert!(
        metrics.contains("entanglement_ticks 1"),
        "should record one tick after interval"
    );
    assert!(
        metrics.contains("entanglement_branches_processed 4"),
        "should record number of branches processed"
    );
}
