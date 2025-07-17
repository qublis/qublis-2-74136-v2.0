//! Unit tests for the CausalReflector module.

use crate::causal_reflector::CausalReflector;
use crate::config::CausalReflectorConfig;
use crate::types::ConsensusEngine;

#[test]
fn new_reflector_records_enabled_metric() {
    let cfg = CausalReflectorConfig { enabled: true, max_depth: 5 };
    let reflector = CausalReflector::new(&cfg);
    let metrics = reflector.export_metrics();
    assert!(
        metrics.contains("causal_reflector_enabled 1"),
        "should record 'causal_reflector_enabled' when enabled"
    );
}

#[test]
fn new_reflector_records_disabled_metric() {
    let cfg = CausalReflectorConfig { enabled: false, max_depth: 5 };
    let reflector = CausalReflector::new(&cfg);
    let metrics = reflector.export_metrics();
    assert!(
        metrics.contains("causal_reflector_disabled 1"),
        "should record 'causal_reflector_disabled' when disabled"
    );
}

#[test]
fn reflect_noop_when_disabled() {
    let cfg = CausalReflectorConfig { enabled: false, max_depth: 3 };
    let mut reflector = CausalReflector::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    // Should do nothing and not record causal_reflections
    assert!(reflector.reflect(&mut engine).is_ok());
    let metrics = reflector.export_metrics();
    assert!(
        !metrics.contains("causal_reflections"),
        "no 'causal_reflections' metric when disabled"
    );
}

#[test]
fn reflect_traverses_and_records() {
    let cfg = CausalReflectorConfig { enabled: true, max_depth: 4 };
    let mut reflector = CausalReflector::new(&cfg);
    let mut engine = ConsensusEngine::mock();
    // Mock engine to reflect exactly 7 items
    engine.set_causal_reflection_capacity(7);
    assert!(reflector.reflect(&mut engine).is_ok());
    let metrics = reflector.export_metrics();
    assert!(
        metrics.contains("causal_reflections 1"),
        "should record one causal_reflections tick"
    );
    assert!(
        metrics.contains("causal_reflected_count 7"),
        "should record the number of reflected items"
    );
}

#[test]
fn export_metrics_includes_all_counters_and_gauges() {
    let cfg = CausalReflectorConfig { enabled: true, max_depth: 2 };
    let reflector = CausalReflector::new(&cfg);
    let output = reflector.export_metrics();
    assert!(
        output.lines().any(|l| l.starts_with("causal_reflector_enabled ")),
        "export should include the enabled counter"
    );
    assert!(
        output.lines().any(|l| l.starts_with("causal_reflections ")),
        "export should include the reflections counter (even if zero)"
    );
}
