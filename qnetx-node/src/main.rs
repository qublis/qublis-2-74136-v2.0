//! Entry point for the QNetX validator node CLI.
//!
//! Provides `init`, `run`, and `status` commands to bootstrap, start, and inspect
//! a Qublis v2.0 QNetX validator node.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use clap::Parser;
use env_logger::Env;
use std::process;

mod cli;
mod config;
mod bootstrap;
mod node;
mod telemetry;
mod metrics;
mod error;
mod prelude;

use cli::{Cli, Command};
use error::NodeError;

/// Asynchronous main entry point.
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

/// Parses CLI args, initializes logging, and dispatches commands.
async fn run() -> Result<(), NodeError> {
    // Parse command‐line arguments
    let cli = Cli::parse();

    // Initialize logging (use RUST_LOG env var or fallback to level from CLI)
    env_logger::Builder::from_env(Env::default().default_filter_or(&cli.log_level))
        .format_timestamp_secs()
        .init();

    match cli.command {
        Command::Init { config, base_path } => {
            // Load and validate configuration
            let cfg = config::NodeConfig::load(&config)?;
            // Initialize directories, keys, and default files
            bootstrap::init(&cfg, &base_path)?;
            println!("Initialization successful: {}", base_path.display());
        }

        Command::Run { config, base_path } => {
            // Load config
            let cfg = config::NodeConfig::load(&config)?;
            // Start telemetry & metrics endpoints
            telemetry::start(&cfg.telemetry);
            metrics::start(&cfg.metrics);
            // Launch the validator node (P2P, consensus, NeuroFlux, etc.)
            node::run(&cfg, &base_path).await?;
        }

        Command::Status { config, base_path } => {
            // Load config
            let cfg = config::NodeConfig::load(&config)?;
            // Query and print node status
            let status = node::status(&cfg, &base_path)?;
            println!("{}", status);
        }
    }

    Ok(())
}
