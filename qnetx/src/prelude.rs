//! QNetX Prelude
//! ------------------------
//! Common imports and re-exports for the QNetX entangled overlay mesh crate.

pub use crate::config::QNetXConfig;
pub use crate::quantum_mesh::QuantumMesh;
pub use crate::zero_prop::ZeroPropagator;
pub use crate::state_condenser::StateCondenser;
pub use crate::anomaly_filter::AnomalyFilter;
pub use crate::metrics::QNetXMetrics;
pub use crate::error::QNetXError;
pub use crate::types::{Dimension, ChannelId};

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn prelude_reexports_compile() {
        // Config
        let cfg: QNetXConfig = QNetXConfig::default();
        // Mesh
        let mesh = QuantumMesh::new(&cfg);
        // Zero propagator
        let mut zp = ZeroPropagator::new(&cfg);
        // State condenser
        let mut sc = StateCondenser::new(&cfg);
        // Anomaly filter
        let mut af = AnomalyFilter::new(&cfg);
        // Metrics
        let mut metrics = QNetXMetrics::new();
        // Error
        let err: QNetXError = QNetXError::HandshakeError("oops".into());
        assert!(err.to_string().contains("Handshake error"));

        // Use Dimension and ChannelId
        let dim = Dimension("X".into());
        let cid: ChannelId = vec![1,2,3];
        assert_eq!(format!("{}", dim), "Dimension(X)");
        assert_eq!(cid, vec![1,2,3]);

        // Basic synthesizing calls
        let mut state = QNum::from_digits(&[3,4]);
        zp.propagate(&mut state);
        // Condense on an empty mesh should error
        assert!(sc.condense_all(&mesh).is_err());
        // Detect anomalies on empty mesh
        let anomalies = af.detect(&mesh);
        assert!(anomalies.is_empty());

        // Record a metric
        metrics.record_channel_created();
        let prom = metrics.export_prometheus();
        assert!(prom.contains("qnetx_channels_created"));
    }
}
