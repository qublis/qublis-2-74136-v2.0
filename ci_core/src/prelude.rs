//! CI‐Core Prelude
//!
//! Common imports and re‐exports for the Conscious AI core crate (Qublis v2.0).
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::config::CiCoreConfig;
pub use crate::error::CiCoreError;
pub use crate::metrics::CiCoreMetrics;

pub use crate::morphic_ai::MorphicAI;
pub use crate::moral_regulator::MoralRegulator;
pub use crate::collective_sync::CollectiveSync;

pub use crate::types::{
    NeuralState,
    SensoryInput,
    MotorOutput,
    AgentId,
    AgentState,
    SyncMessage,
};

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn prelude_reexports_compile() {
        // Config
        let cfg: CiCoreConfig = CiCoreConfig::default();
        // Error
        let err: CiCoreError = CiCoreError::SyncError("err".into());
        assert!(err.to_string().contains("sync error"));

        // Metrics
        let mut metrics = CiCoreMetrics::new();
        metrics.inc_counter("foo", 1);
        let prom = metrics.export_prometheus();
        assert!(prom.contains("ci_core_foo 1"));

        // MorphicAI
        let mut ai = MorphicAI::new(&cfg);
        let sensory = SensoryInput::from_digits(vec![1,2,3]);
        let _ = ai.perceive(sensory.clone());
        let out = ai.generate();
        assert!(!out.signals.is_empty());

        // MoralRegulator
        let mut mr = MoralRegulator::new(&cfg);
        let allowed = mr.enforce(out.clone());
        // Since no principles, should allow
        assert!(allowed.is_ok());

        // CollectiveSync
        let mut cs = CollectiveSync::new(&cfg);
        cs.register_agent("A".into(), AgentState { state: QNum::from_digits(&[5]) }).unwrap();
        let snap = cs.snapshot();
        assert_eq!(snap.get("A"), Some(&vec![5]));

        // Types
        let ns = NeuralState::zero(cfg.num_neurons);
        assert_eq!(ns.len(), cfg.num_neurons);
        let si = SensoryInput::from_digits(vec![7]);
        assert_eq!(si.len(), 1);
        let mo = MotorOutput { signals: vec![9] };
        assert_eq!(mo.signals, vec![9]);
        let msg = SyncMessage { from: "A".into(), state: QNum::from_digits(&[3]) };
        assert_eq!(msg.from, "A");
    }
}
