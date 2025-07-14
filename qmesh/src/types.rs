//! Core data types for QMesh — Qublis v2.0
//!
//! Defines identifiers used throughout the QMesh crate, such as node IDs
//! in the entropic DAG.

/// Unique identifier for a node in the Entropic DAG.
pub type NodeId = String;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nodeid_alias_behaves_as_string() {
        let id: NodeId = "node_42".to_string();
        assert_eq!(id, "node_42");
        // You can use it wherever a String is expected:
        fn takes_string(s: String) -> String { s }
        let returned: String = takes_string(id.clone());
        assert_eq!(returned, id);
    }
}
