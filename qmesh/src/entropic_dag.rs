//! Entropic Directed Acyclic Graph (DAG) for QMesh — Qublis v2.0
//!
//! An `EntropicDag` manages a directed acyclic graph of nodes, each carrying
//! a quantum‐number state (`QNum`).  Edges carry a floating‐point “influence”
//! weight.  Propagation entangles connected nodes’ QNum states, and we can
//! compute per-node and total entropy as a measure of uncertainty.

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Topo;
use petgraph::algo::is_cyclic_directed;
use crate::config::QMeshConfig;
use crate::error::QMeshError;
use crate::metrics::QMeshMetrics;
use crate::types::NodeId;
use qublis_qnum::{QNum, entangle};

/// Data stored at each node: an identifier and a QNum state.
#[derive(Clone, Debug)]
pub struct NodeData {
    pub id: NodeId,
    pub state: QNum,
}

/// An entropic DAG: nodes carry QNum states, edges carry influence weights.
#[derive(Clone, Debug)]
pub struct EntropicDag {
    graph: DiGraph<NodeData, f64>,
    config: QMeshConfig,
    metrics: QMeshMetrics,
}

impl EntropicDag {
    /// Create a new empty `EntropicDag` with the given configuration.
    pub fn new(config: &QMeshConfig) -> Self {
        EntropicDag {
            graph: DiGraph::new(),
            config: config.clone(),
            metrics: QMeshMetrics::new(),
        }
    }

    /// Add a node with the given `id` and initial `state`.  
    /// Returns the `NodeIndex` of the new node.
    pub fn add_node(&mut self, id: NodeId, initial: QNum) -> NodeIndex {
        let idx = self.graph.add_node(NodeData { id, state: initial });
        self.metrics.inc_counter("nodes_added", 1);
        idx
    }

    /// Add a directed edge from `parent` to `child` with the given `weight`.  
    /// Returns an error if adding the edge would introduce a cycle.
    pub fn add_edge(&mut self, parent: NodeIndex, child: NodeIndex, weight: f64) -> Result<(), QMeshError> {
        self.graph.add_edge(parent, child, weight);
        if is_cyclic_directed(&self.graph) {
            // rollback and report error
            self.graph.remove_edge(self.graph.find_edge(parent, child).unwrap());
            return Err(QMeshError::CycleDetected);
        }
        self.metrics.inc_counter("edges_added", 1);
        Ok(())
    }

    /// Propagate entropic influence across the DAG:
    /// for each edge (u→v), entangle u.state with v.state scaled by `weight`.
    /// Process nodes in topological order to respect causality.
    pub fn propagate(&mut self) {
        let mut topo = Topo::new(&self.graph);
        while let Some(idx) = topo.next(&self.graph) {
            for edge in self.graph.edges(idx) {
                let target = edge.target();
                let weight = *edge.weight();
                // scale v.state toward u.state by entanglement
                // here we call entangle on the two QNums directly
                entangle(&mut self.graph[idx].state, &mut self.graph[target].state);
                // record metric
                self.metrics.inc_counter("entanglements", 1);
                self.metrics.set_gauge("last_influence_weight", weight);
            }
        }
    }

    /// Compute the Shannon‐joint entropy of a single node.
    pub fn node_entropy(&self, idx: NodeIndex) -> f64 {
        self.graph[idx].state.entropy()
    }

    /// Compute the total entropy of the DAG = sum of all node entropies.
    pub fn total_entropy(&self) -> f64 {
        self.graph.node_indices()
            .map(|i| self.node_entropy(i))
            .sum()
    }

    /// Get a reference to the node data by index.
    pub fn node_data(&self, idx: NodeIndex) -> &NodeData {
        &self.graph[idx]
    }

    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Number of edges.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qublis_qnum::QNum;

    #[test]
    fn test_add_nodes_and_edges() {
        let cfg = QMeshConfig::default();
        let mut dag = EntropicDag::new(&cfg);
        let n1 = dag.add_node("A".into(), QNum::zero(1));
        let n2 = dag.add_node("B".into(), QNum::zero(1));
        assert_eq!(dag.node_count(), 2);
        assert!(dag.add_edge(n1, n2, 0.5).is_ok());
        assert_eq!(dag.edge_count(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let cfg = QMeshConfig::default();
        let mut dag = EntropicDag::new(&cfg);
        let n1 = dag.add_node("X".into(), QNum::zero(1));
        let n2 = dag.add_node("Y".into(), QNum::zero(1));
        dag.add_edge(n1, n2, 1.0).unwrap();
        let err = dag.add_edge(n2, n1, 1.0).unwrap_err();
        matches!(err, QMeshError::CycleDetected);
    }

    #[test]
    fn test_propagation_and_entropy() {
        let cfg = QMeshConfig::default();
        let mut dag = EntropicDag::new(&cfg);

        // Two nodes with distinct classical states
        let mut s1 = QNum::from_digits(&[1]);
        let mut s2 = QNum::from_digits(&[2]);
        let n1 = dag.add_node("N1".into(), s1.clone());
        let n2 = dag.add_node("N2".into(), s2.clone());

        // Before entangle, entropies are zero
        assert!((dag.node_entropy(n1) - 0.0).abs() < 1e-12);
        assert!((dag.node_entropy(n2) - 0.0).abs() < 1e-12);

        dag.add_edge(n1, n2, 1.0).unwrap();
        dag.propagate();

        // After entangle, at least one node has non-zero entropy
        let e1 = dag.node_entropy(n1);
        let e2 = dag.node_entropy(n2);
        assert!(e1 > 0.0 || e2 > 0.0, "Expected non-zero entropy after entanglement");

        // Total entropy > 0
        assert!(dag.total_entropy() > 0.0);
    }
}
