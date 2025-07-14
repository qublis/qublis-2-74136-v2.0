//! Retrochain Tracker for QMesh — Qublis v2.0
//!
//! Tracks the sequence of entropic states (QNums) associated with successive
//! blocks, and lets you retrieve the “retrochain” history or compute entropic
//! differences along that chain.

use crate::config::QMeshConfig;
use crate::metrics::QMeshMetrics;
use qublis_qnum::QNum;
use std::collections::HashMap;

/// Identifier for a block in the retrochain.
pub type BlockId = String;

/// `RetrochainTracker` records a linear sequence of `(BlockId, QNum)` states
/// and provides methods to retrieve the chain up to any recorded block, as
/// well as to compute entropy diffs along that chain.
#[derive(Clone, Debug)]
pub struct RetrochainTracker {
    config: QMeshConfig,
    metrics: QMeshMetrics,
    chain: Vec<(BlockId, QNum)>,
    index: HashMap<BlockId, usize>,
}

impl RetrochainTracker {
    /// Create a new, empty `RetrochainTracker`.
    pub fn new(config: &QMeshConfig) -> Self {
        RetrochainTracker {
            config: config.clone(),
            metrics: QMeshMetrics::new(),
            chain: Vec::new(),
            index: HashMap::new(),
        }
    }

    /// Record the entropic state `state` for `block`.
    ///
    /// If `block` was recorded before, this is a no-op.
    pub fn record_block(&mut self, block: BlockId, state: QNum) {
        if self.index.contains_key(&block) {
            return;
        }
        let idx = self.chain.len();
        self.chain.push((block.clone(), state));
        self.index.insert(block, idx);
        self.metrics.inc_counter("blocks_recorded", 1);
    }

    /// Retrieve the recorded `QNum` state for `block`, if any.
    pub fn get_state(&self, block: &BlockId) -> Option<&QNum> {
        self.index
            .get(block)
            .and_then(|&i| self.chain.get(i).map(|(_, q)| q))
    }

    /// Return the retrochain for `block`: a vector of `(BlockId, QNum)` from
    /// genesis (first recorded) up through `block` inclusive.
    ///
    /// If `block` is not recorded, returns `None`.
    pub fn retrochain(&mut self, block: &BlockId) -> Option<Vec<(BlockId, QNum)>> {
        let &i = self.index.get(block)?;
        let subchain = self.chain[0..=i]
            .iter()
            .cloned()
            .collect();
        self.metrics.inc_counter("retrochain_retrieved", 1);
        Some(subchain)
    }

    /// Compute the entropic differences for each block in the retrochain of
    /// `block`: returns `Some(Vec<(BlockId, entropy)>)`, or `None` if
    /// `block` is unknown.
    pub fn diffs(&mut self, block: &BlockId) -> Option<Vec<(BlockId, f64)>> {
        let chain = self.retrochain(block)?;
        let diffs = chain
            .into_iter()
            .map(|(b, q)| {
                let ent = q.entropy();
                (b, ent)
            })
            .collect();
        self.metrics.inc_counter("diffs_computed", 1);
        Some(diffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::{QNum, Qid};
    use num_complex::Complex;

    fn make_qnum(digit: u8) -> QNum {
        QNum::from_digits(&[digit])
    }

    #[test]
    fn test_record_and_get_state() {
        let cfg = QMeshConfig::default();
        let mut rt = RetrochainTracker::new(&cfg);

        rt.record_block("genesis".into(), make_qnum(1));
        rt.record_block("block2".into(), make_qnum(2));

        assert_eq!(rt.get_state(&"genesis".into()).unwrap().measure(), vec![1]);
        assert_eq!(rt.get_state(&"block2".into()).unwrap().measure(), vec![2]);
        assert!(rt.get_state(&"unknown".into()).is_none());
    }

    #[test]
    fn test_retrochain_sequence() {
        let cfg = QMeshConfig::default();
        let mut rt = RetrochainTracker::new(&cfg);

        rt.record_block("A".into(), make_qnum(5));
        rt.record_block("B".into(), make_qnum(6));
        rt.record_block("C".into(), make_qnum(7));

        let chain = rt.retrochain(&"C".into()).unwrap();
        let ids: Vec<_> = chain.iter().map(|(b, _)| b.clone()).collect();
        assert_eq!(ids, vec!["A".to_string(), "B".to_string(), "C".to_string()]);

        // Unknown block returns None
        assert!(rt.retrochain(&"X".into()).is_none());
    }

    #[test]
    fn test_diffs_computation() {
        let cfg = QMeshConfig::default();
        let mut rt = RetrochainTracker::new(&cfg);

        // Superposed QNum for block P: equal 0 & 1 → entropy ln2
        let mut amps = [Complex::new(0.0,0.0); 10];
        amps[0] = Complex::new(1.0/2f64.sqrt(), 0.0);
        amps[1] = Complex::new(1.0/2f64.sqrt(), 0.0);
        let super_q = QNum::new(amps);

        rt.record_block("P".into(), make_qnum(0));
        rt.record_block("Q".into(), super_q.clone());

        let diffs = rt.diffs(&"Q".into()).unwrap();
        // First entry entropy = 0, second = ln(2)
        assert_eq!(diffs[0].0, "P");
        assert!((diffs[0].1 - 0.0).abs() < 1e-12);
        assert_eq!(diffs[1].0, "Q");
        assert!((diffs[1].1 - (2f64).ln()).abs() < 1e-6);

        // Unknown block returns None
        assert!(rt.diffs(&"Z".into()).is_none());
    }
}
