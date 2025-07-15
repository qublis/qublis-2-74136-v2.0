//! Deployment Prelude
//!
//! Convenient imports and re‐exports for the `qublis-deploy` crate (Qublis v2.0).
#![deny(missing_docs)]
#![forbid(unsafe_code)]

pub use crate::config::{load_toml_config, ConfigError};
pub use crate::ci_fork_launcher::{CiForkConfig, CiForkError, run as run_ci_fork_launcher};
pub use crate::qnetx_node_bootstrap::{BootstrapConfig, BootstrapError, run as run_qnetx_node_bootstrap};
pub use crate::types::{CiForkConfig as _, BootstrapConfig as _};
pub use crate::error::DeployError;
pub use crate::metrics::DeployMetrics;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::runtime::Runtime;
    use std::io::Write;

    #[test]
    fn prelude_reexports_compile() {
        // Config loader
        #[derive(serde::Deserialize)]
        struct TestCfg { foo: String }
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "foo = \"bar\"").unwrap();
        let cfg: TestCfg = load_toml_config(file.path()).unwrap();
        assert_eq!(cfg.foo, "bar");

        // CI fork types
        let cf = CiForkConfig {
            upstream_repo: "https://git".into(),
            branch: "main".into(),
            forks: 1,
            output_dir: "./ci".into(),
            run_tests: false,
        };
        assert_eq!(cf.forks, 1);

        // Bootstrap types
        let bc = BootstrapConfig {
            node_name: "n1".into(),
            listen_addr: "/ip4/0.0.0.0/tcp/30333".into(),
            bootstrap_peers: vec![],
            chain_spec: "spec.json".into(),
            base_path: "./nodes".into(),
            dev_mode: true,
        };
        assert!(bc.dev_mode);

        // DeployError conversion
        let cfg_err: ConfigError = std::io::Error::new(std::io::ErrorKind::Other, "e").into();
        let _e: DeployError = cfg_err.into();

        // Metrics
        let mut m = DeployMetrics::new();
        m.record_config_load();
        let prom = m.export_prometheus();
        assert!(prom.contains("deploy_config_loads"));
    }

    #[tokio::test]
    async fn run_functions_exist() {
        // Cannot actually clone, but ensure functions exist
        let _ = run_ci_fork_launcher("nonexistent.toml").await.err();
        let _ = run_qnetx_node_bootstrap("nonexistent.toml").await.err();
    }
}
