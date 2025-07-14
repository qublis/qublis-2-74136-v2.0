//! Probabilistic QNet Router
//!
//! This module provides `Router`, which uses the Quantum Number System (`QNum`)
//! to represent a superposition of k-shortest paths between two nodes, and then
//! measures (collapses) that superposition to select a single path at relay time.

use std::collections::{HashMap, VecDeque};
use num_complex::Complex;
use qublis_qnum::{QNum};
use crate::{
    config::QNetConfig,
    error::QNetError,
    types::{NodeId, Path},
};

/// `Router` holds the network graph and configuration for path selection.
#[derive(Clone, Debug)]
pub struct Router {
    /// How many candidate paths to superpose
    pub config: QNetConfig,
    /// Adjacency list: node → neighbors
    graph: HashMap<NodeId, Vec<NodeId>>,
}

impl Router {
    /// Create a new Router with the given QNetConfig.
    pub fn new(config: &QNetConfig) -> Self {
        Router {
            config: config.clone(),
            graph: HashMap::new(),
        }
    }

    /// Add an undirected edge between two nodes in the graph.
    pub fn add_edge(&mut self, a: NodeId, b: NodeId) {
        self.graph.entry(a.clone()).or_default().push(b.clone());
        self.graph.entry(b).or_default().push(a);
    }

    /// Compute up to `k` shortest simple paths from `src` to `dst` using BFS.
    fn k_shortest_paths(&self, src: &NodeId, dst: &NodeId, k: usize) -> Vec<Path> {
        let mut results = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(vec![src.clone()]);

        while let Some(path) = queue.pop_front() {
            if let Some(last) = path.last() {
                if last == dst {
                    results.push(path.clone());
                    if results.len() >= k {
                        break;
                    }
                    continue;
                }
                if let Some(neighbors) = self.graph.get(last) {
                    for nbr in neighbors {
                        if !path.contains(nbr) {
                            let mut new_path = path.clone();
                            new_path.push(nbr.clone());
                            queue.push_back(new_path);
                        }
                    }
                }
            }
        }

        results
    }

    /// Return a `QNum` superposition over up to `k_paths` candidate routes.
    ///
    /// Each path is assigned equal amplitude; the basis states encode
    /// the path index in fixed-width decimal digits.
    pub fn route_qnum(&self, src: &NodeId, dst: &NodeId) -> QNum {
        let paths = self.k_shortest_paths(src, dst, self.config.k_paths);
        let k = paths.len().max(1);
        // Determine width in decimal digits to encode indices [0..k)
        let width = ((k as f64).log10().ceil() as usize).max(1);

        // Build superposed states: (digits_of_index, amplitude)
        let amplitude = Complex::new(1.0, 0.0) / (k as f64).sqrt();
        let states: Vec<(Vec<u8>, Complex<f64>)> = (0..k)
            .map(|i| {
                let mut digits = vec![0u8; width];
                let mut idx = i;
                for d in (0..width).rev() {
                    digits[d] = (idx % 10) as u8;
                    idx /= 10;
                }
                (digits, amplitude)
            })
            .collect();

        QNum::from_superposed(states)
    }

    /// Collapse the `QNum` to select one path, returning it or an error if none.
    pub fn route(&self, src: &NodeId, dst: &NodeId) -> Result<Path, QNetError> {
        let paths = self.k_shortest_paths(src, dst, self.config.k_paths);
        if paths.is_empty() {
            return Err(QNetError::NoPath(src.clone(), dst.clone()));
        }

        // Measure superposed path indices
        let mut qnum = self.route_qnum(src, dst);
        let digits = qnum.measure();
        let index = digits.iter().fold(0usize, |acc, &d| acc * 10 + d as usize);

        // If the measured index is out of bounds, wrap around
        let chosen = paths.get(index % paths.len())
            .cloned()
            .ok_or_else(|| QNetError::NoPath(src.clone(), dst.clone()))?;

        Ok(chosen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::QNetConfig;

    fn build_simple_graph() -> Router {
        let mut cfg = QNetConfig::default();
        cfg.k_paths = 4;
        let mut r = Router::new(&cfg);
        // Graph: A—B—C, A—D—C
        r.add_edge("A".into(), "B".into());
        r.add_edge("B".into(), "C".into());
        r.add_edge("A".into(), "D".into());
        r.add_edge("D".into(), "C".into());
        r
    }

    #[test]
    fn test_k_shortest_paths_count() {
        let r = build_simple_graph();
        let paths = r.k_shortest_paths(&"A".into(), &"C".into(), 3);
        // Should find two distinct simple paths: A-B-C and A-D-C
        assert_eq!(paths.len(), 2);
        assert!(paths.contains(&vec!["A".into(), "B".into(), "C".into()]));
        assert!(paths.contains(&vec!["A".into(), "D".into(), "C".into()]));
    }

    #[test]
    fn test_route_qnum_superposition() {
        let r = build_simple_graph();
        let qnum = r.route_qnum(&"A".into(), &"C".into());
        // QNum length should encode max k=4 in width 1 digit
        assert_eq!(qnum.len(), 1);
        // Measuring repeatedly yields indices 0 or 1 (two paths)
        let mut seen = std::collections::HashSet::new();
        for _ in 0..100 {
            let mut copy = qnum.clone();
            seen.insert(copy.measure()[0]);
        }
        assert!(seen.contains(&0) && seen.contains(&1));
    }

    #[test]
    fn test_route_selects_valid_path() {
        let r = build_simple_graph();
        let path = r.route(&"A".into(), &"C".into()).unwrap();
        assert!(path == vec!["A".into(), "B".into(), "C".into()] ||
                path == vec!["A".into(), "D".into(), "C".into()]);
    }

    #[test]
    fn test_route_no_path_error() {
        let cfg = QNetConfig { k_paths: 2, ..Default::default() };
        let r = Router::new(&cfg);
        let err = r.route(&"X".into(), &"Y".into()).unwrap_err();
        matches!(err, QNetError::NoPath(_, _));
    }
}
