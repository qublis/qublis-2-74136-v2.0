//! Common Types for deploy crate — Qublis v2.0
//!
//! Re-exports configuration types and errors for deployment commands.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::ci_fork_launcher::{CiForkConfig, CiForkError};
pub use crate::qnetx_node_bootstrap::{BootstrapConfig, BootstrapError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reexports_compile() {
        // Test CiForkConfig
        let cf = CiForkConfig {
            upstream_repo: "https://github.com/qublis/qublis.git".into(),
            branch: "main".into(),
            forks: 2,
            output_dir: "./ci_forks".into(),
            run_tests: true,
        };
        assert_eq!(cf.forks, 2);
        // Test BootstrapConfig
        let bc = BootstrapConfig {
            node_name: "node1".into(),
            listen_addr: "/ip4/0.0.0.0/tcp/30333".into(),
            bootstrap_peers: vec!["/ip4/1.2.3.4/tcp/30333".into()],
            chain_spec: "spec.json".into(),
            base_path: "./nodes".into(),
            dev_mode: false,
        };
        assert_eq!(bc.node_name, "node1");
    }
}
