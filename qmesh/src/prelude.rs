//! QMesh Prelude
//! ------------------------
//! Common imports and re‐exports for the QMesh entropic DAG, cognitive entropy,
//! and retrochain tracking crate (Qublis v2.0).

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::config::QMeshConfig;
pub use crate::entropic_dag::EntropicDag;
pub use crate::cognitive_entropy::CognitiveEntropy;
pub use crate::retrochain_tracker::RetrochainTracker;
pub use crate::error::QMeshError;
pub use crate::metrics::QMeshMetrics;
pub use crate::types::NodeId;

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn prelude_reexports_compile() {
        // Config
        let cfg: QMeshConfig = QMeshConfig::default();
        // Create an EntropicDag
        let mut dag = EntropicDag::new(&cfg);
        assert_eq!(dag.node_count(), 0);
        // CognitiveEntropy requires an EntropicDag
        let mut analyzer = CognitiveEntropy::new(&cfg);
        let report = analyzer.analyze(&dag);
        assert_eq!(report.global_entropy, 0.0);
        // RetrochainTracker
        let mut tracker = RetrochainTracker::new(&cfg);
        tracker.record_block("blk1".into(), QNum::from_digits(&[1]));
        assert!(tracker.get_state(&"blk1".into()).is_some());
        // Error
        let err: QMeshError = QMeshError::CycleDetected;
        assert!(err.to_string().contains("cycle detected"));
        // Metrics
        let mut metrics = QMeshMetrics::new();
        metrics.record_node_added();
        let prom = metrics.export_prometheus();
        assert!(prom.contains("qmesh_nodes_added"));
        // NodeId alias
        let nid: NodeId = "nodeX".into();
        assert_eq!(nid, "nodeX");
    }
}
