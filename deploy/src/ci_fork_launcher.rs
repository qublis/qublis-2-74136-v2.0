//! CI Fork Launcher for Qublis v2.0
//!
//! Reads a TOML configuration defining one or more CI “forks” of the Qublis
//! repository, clones each fork into a separate directory, optionally checks out
//! a branch, runs tests, and prepares them for parallel CI execution.

use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;
use tokio::process::Command;

/// Configuration for the CI Fork Launcher.
#[derive(Debug, Deserialize)]
struct CiForkConfig {
    /// Git URL of the upstream repository to fork.
    pub upstream_repo: String,
    /// Branch to check out in each fork (default: "main").
    #[serde(default = "default_branch")]
    pub branch: String,
    /// Number of parallel forks to create (default: 1).
    #[serde(default = "default_num_forks")]
    pub forks: usize,
    /// Directory under which to create fork directories (default: "./ci_forks").
    #[serde(default = "default_output_dir")]
    pub output_dir: String,
    /// Whether to run `cargo test` in each fork after cloning (default: false).
    #[serde(default)]
    pub run_tests: bool,
}

fn default_branch() -> String { "main".into() }
fn default_num_forks() -> usize { 1 }
fn default_output_dir() -> String { "./ci_forks".into() }

/// Errors returned by the CI Fork Launcher.
#[derive(Debug, Error)]
pub enum CiForkError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Git command failed: {0}")]
    Git(String),
}

/// Entry point for the `ci_fork_launcher` binary.
///
/// # Arguments
///
/// * `config_path` - Path to a TOML file defining the `CiForkConfig`.
///
pub async fn run(config_path: &str) -> Result<(), CiForkError> {
    // Load and parse the config
    let toml_str = fs::read_to_string(config_path)?;
    let cfg: CiForkConfig = toml::from_str(&toml_str)?;
    println!("Loaded CI fork config: {:#?}", cfg);

    // Ensure the output directory exists
    tokio::fs::create_dir_all(&cfg.output_dir).await?;

    // For each fork, clone and set up
    for i in 0..cfg.forks {
        let fork_name = format!("fork_{}", i + 1);
        let fork_dir = Path::new(&cfg.output_dir).join(&fork_name);
        println!("Cloning fork {} into {:?}", i + 1, fork_dir);

        // git clone <upstream_repo> <fork_dir>
        let status = Command::new("git")
            .arg("clone")
            .arg(&cfg.upstream_repo)
            .arg(&fork_dir)
            .status()
            .await
            .map_err(|e| CiForkError::Git(format!("failed to spawn git clone: {}", e)))?;
        if !status.success() {
            return Err(CiForkError::Git(format!(
                "git clone failed for fork {} (exit code {:?})",
                i + 1,
                status.code()
            )));
        }

        // git checkout <branch>
        let status = Command::new("git")
            .arg("-C")
            .arg(&fork_dir)
            .arg("checkout")
            .arg(&cfg.branch)
            .status()
            .await
            .map_err(|e| CiForkError::Git(format!("failed to spawn git checkout: {}", e)))?;
        if !status.success() {
            return Err(CiForkError::Git(format!(
                "git checkout '{}' failed in fork {}",
                cfg.branch,
                i + 1
            )));
        }

        // Optionally run tests
        if cfg.run_tests {
            println!("Running tests for fork {}", i + 1);
            let status = Command::new("cargo")
                .arg("test")
                .current_dir(&fork_dir)
                .status()
                .await
                .map_err(|e| CiForkError::Git(format!("failed to spawn cargo test: {}", e)))?;
            if !status.success() {
                eprintln!(
                    "Warning: tests failed for fork {} (exit code {:?})",
                    i + 1,
                    status.code()
                );
            }
        }
    }

    println!("All {} CI forks prepared in '{}'.", cfg.forks, cfg.output_dir);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::runtime::Runtime;

    #[test]
    fn parse_default_config() {
        let toml = r#"
            upstream_repo = "https://github.com/YourOrg/qublis.git"
        "#;
        let cfg: CiForkConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.upstream_repo, "https://github.com/YourOrg/qublis.git");
        assert_eq!(cfg.branch, "main");
        assert_eq!(cfg.forks, 1);
        assert_eq!(cfg.output_dir, "./ci_forks");
        assert!(!cfg.run_tests);
    }

    #[test]
    fn parse_full_config() {
        let toml = r#"
            upstream_repo = "git@github.com:YourOrg/qublis.git"
            branch = "develop"
            forks = 3
            output_dir = "/tmp/forks"
            run_tests = true
        "#;
        let cfg: CiForkConfig = toml::from_str(toml).unwrap();
        assert_eq!(cfg.branch, "develop");
        assert_eq!(cfg.forks, 3);
        assert_eq!(cfg.output_dir, "/tmp/forks");
        assert!(cfg.run_tests);
    }

    #[test]
    fn run_errors_on_missing_config() {
        let rt = Runtime::new().unwrap();
        let res = rt.block_on(run("nonexistent.toml"));
        assert!(matches!(res, Err(CiForkError::Io(_))));
    }
}
