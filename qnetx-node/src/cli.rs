//! CLI definitions for the QNetX validator node.
//!
//! Defines the `Cli` struct and `Command` enum for `init`, `run`, and `status` subcommands.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Qublis v2.0 QNetX Validator Node CLI
#[derive(Parser, Debug)]
#[clap(
    name = "qublis-qnetx-node",
    version,
    about = "Extended QNetX validator node for Qublis v2.0 (2-74136)"
)]
pub struct Cli {
    /// Logging level: trace, debug, info, warn, error
    #[clap(
        long,
        default_value = "info",
        env = "RUST_LOG",
        help = "Set the logging level (also via RUST_LOG)"
    )]
    pub log_level: String,

    /// Subcommand to execute
    #[clap(subcommand)]
    pub command: Command,
}

/// Available CLI commands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initialize data directories, keys, and default configurations
    Init {
        /// Path to the node configuration TOML file
        #[clap(long, parse(from_os_str), help = "Path to config file")]
        config: PathBuf,

        /// Base directory in which to create data, keys, etc.
        #[clap(long, parse(from_os_str), help = "Base path for node data")]
        base_path: PathBuf,
    },

    /// Run the validator node (P2P networking, consensus, telemetry)
    Run {
        /// Path to the node configuration TOML file
        #[clap(long, parse(from_os_str), help = "Path to config file")]
        config: PathBuf,

        /// Base directory where node data and keys are stored
        #[clap(long, parse(from_os_str), help = "Base path for node data")]
        base_path: PathBuf,
    },

    /// Query the current status of the running node
    Status {
        /// Path to the node configuration TOML file
        #[clap(long, parse(from_os_str), help = "Path to config file")]
        config: PathBuf,

        /// Base directory where node data and keys are stored
        #[clap(long, parse(from_os_str), help = "Base path for node data")]
        base_path: PathBuf,
    },
}
