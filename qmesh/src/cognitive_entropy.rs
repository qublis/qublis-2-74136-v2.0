//! Cognitive Entropy Analysis for QMesh — Qublis v2.0
//!
//! Provides routines to compute and track entropy patterns across an
//! EntropicDag.  Includes per-node entropies, global metrics (mean,
//! variance, max), and a sliding‐window history for trend analysis.

use crate::{
    config::QMeshConfig,
    entropic_dag::EntropicDag,
    metrics::QMeshMetrics,
    types::NodeId,
};
use std::collections::{HashMap, VecDeque};

/// A report of cognitive entropy metrics for a single analysis run.
#[derive(Clone, Debug)]
pub struct CognitiveReport {
    /// Entropy per node.
    pub node_entropies: HashMap<NodeId, f64>,
    /// Global entropy (sum of node entropies).
    pub global_entropy: f64,
    /// Mean entropy across nodes.
    pub mean_entropy: f64,
    /// Variance of entropy across nodes.
    pub variance_entropy: f64,
    /// Maximum node entropy.
    pub max_entropy: f64,
}

/// CognitiveEntropy tracks and analyzes entropy in a QMesh entropic DAG.
pub struct CognitiveEntropy {
    config: QMeshConfig,
    metrics: QMeshMetrics,
    /// History of global entropy values (sliding window).
    history: VecDeque<f64>,
    /// Maximum history length.
    window_size: usize,
}

impl CognitiveEntropy {
    /// Create a new analyzer with the given configuration.
    ///
    /// `config.history_window` (if set) defines the sliding window length;
    /// otherwise defaults to 10.
    pub fn new(config: &QMeshConfig) -> Self {
        let window_size = config
            .history_window
            .unwrap_or(10)
            .max(1);
        CognitiveEntropy {
            config: config.clone(),
            metrics: QMeshMetrics::new(),
            history: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    /// Perform a full analysis run on the given `dag`, updating metrics
    /// and history, and returning a `CognitiveReport`.
    pub fn analyze(&mut self, dag: &EntropicDag) -> CognitiveReport {
        // Compute per-node entropies
        let mut node_entropies = HashMap::new();
        for idx in dag.graph.node_indices() {
            let data = &dag.graph[idx];
            let ent = dag.graph[idx].state.entropy();
            node_entropies.insert(data.id.clone(), ent);
            self.metrics.inc_counter("node_entropy_computed", 1);
        }

        // Global metrics
        let global_entropy = node_entropies.values().sum::<f64>();
        let n = node_entropies.len() as f64;
        let mean_entropy = if n > 0.0 { global_entropy / n } else { 0.0 };
        let variance_entropy = if n > 0.0 {
            node_entropies.values()
                .map(|e| (e - mean_entropy).powi(2))
                .sum::<f64>() / n
        } else {
            0.0
        };
        let max_entropy = node_entropies
            .values()
            .cloned()
            .fold(0.0f64, f64::max);

        // Record metrics
        self.metrics.set_gauge("global_entropy", global_entropy);
        self.metrics.set_gauge("mean_entropy", mean_entropy);
        self.metrics.set_gauge("variance_entropy", variance_entropy);
        self.metrics.set_gauge("max_entropy", max_entropy);
        self.metrics.inc_counter("analysis_runs", 1);

        // Update sliding window history
        self.history.push_back(global_entropy);
        if self.history.len() > self.window_size {
            self.history.pop_front();
        }
        self.metrics.set_gauge("history_length", self.history.len() as f64);

        CognitiveReport {
            node_entropies,
            global_entropy,
            mean_entropy,
            variance_entropy,
            max_entropy,
        }
    }

    /// Retrieve the history of global entropy values.
    pub fn history(&self) -> Vec<f64> {
        self.history.iter().cloned().collect()
    }

    /// Compute the mean of the history window, if non-empty.
    pub fn history_mean(&self) -> Option<f64> {
        if self.history.is_empty() {
            None
        } else {
            let sum: f64 = self.history.iter().sum();
            Some(sum / (self.history.len() as f64))
        }
    }

    /// Compute the variance of the history window, if non-empty.
    pub fn history_variance(&self) -> Option<f64> {
        let mean = self.history_mean()?;
        let n = self.history.len() as f64;
        let var = self.history.iter()
            .map(|e| (e - mean).powi(2))
            .sum::<f64>() / n;
        Some(var)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::QMeshConfig;
    use crate::entropic_dag::EntropicDag;
    use qublis_qnum::QNum;

    /// Build a DAG with given node IDs and classical states.
    fn build_simple_dag(ids: &[&str], digits: u8) -> EntropicDag {
        let cfg = QMeshConfig::default();
        let mut dag = EntropicDag::new(&cfg);
        for &id in ids {
            dag.add_node(id.to_string(), QNum::from_digits(&[digits]));
        }
        dag
    }

    #[test]
    fn test_analyze_empty_dag() {
        let cfg = QMeshConfig::default();
        let mut ce = CognitiveEntropy::new(&cfg);
        let dag = EntropicDag::new(&cfg);
        let report = ce.analyze(&dag);

        assert_eq!(report.global_entropy, 0.0);
        assert_eq!(report.mean_entropy, 0.0);
        assert_eq!(report.variance_entropy, 0.0);
        assert_eq!(report.max_entropy, 0.0);
        assert!(report.node_entropies.is_empty());
        assert_eq!(ce.history(), vec![0.0]);
    }

    #[test]
    fn test_analyze_uniform_states() {
        let cfg = QMeshConfig {
            history_window: Some(3),
            ..Default::default()
        };
        let mut ce = CognitiveEntropy::new(&cfg);
        // Build DAG with three nodes all |5⟩ -> entropy 0 each
        let mut dag = build_simple_dag(&["A","B","C"], 5);
        let report = ce.analyze(&dag);

        assert_eq!(report.global_entropy, 0.0);
        assert_eq!(report.mean_entropy, 0.0);
        assert_eq!(report.variance_entropy, 0.0);
        assert_eq!(report.max_entropy, 0.0);
        assert_eq!(ce.history(), vec![0.0]);
    }

    #[test]
    fn test_history_statistics() {
        let cfg = QMeshConfig {
            history_window: Some(2),
            ..Default::default()
        };
        let mut ce = CognitiveEntropy::new(&cfg);
        // Push two values via analyze
        let mut dag1 = build_simple_dag(&["X"], 1);
        ce.analyze(&dag1); // global=0
        let mut dag2 = build_simple_dag(&["Y"], 2);
        // Make Y superposed to yield entropy >0
        dag2.add_edge(dag2.add_node("Y".into(), QNum::from_digits(&[1])), dag2.add_node("Z".into(), QNum::from_digits(&[2])), 1.0).unwrap();
        let report2 = ce.analyze(&dag2);
        assert!(report2.global_entropy > 0.0);

        // History length should be 2
        let hist = ce.history();
        assert_eq!(hist.len(), 2);
        assert_eq!(ce.history_mean(), Some((0.0 + report2.global_entropy) / 2.0));
        assert!(ce.history_variance().unwrap() >= 0.0);
    }
}
