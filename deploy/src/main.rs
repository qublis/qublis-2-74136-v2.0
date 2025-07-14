// deploy/src/main.rs
//! Deployment CLI for Qublis v2.0
//!
//! Provides subcommands to launch CI forks and bootstrap QNetX nodes.

use clap::{Parser, Subcommand};

mod ci_fork_launcher;
mod qnetx_node_bootstrap;

/// Top‐level CLI definition.
#[derive(Parser)]
#[command(
    name = "qublis-deploy",
    version,
    about = "Deployment tools for Qublis v2.0",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Supported deployment subcommands.
#[derive(Subcommand)]
enum Commands {
    /// Launch a CI fork for Qublis deployment.
    CiForkLauncher {
        /// Path to the CI fork config file (TOML).
        #[arg(short, long, default_value = "ci_fork.toml")]
        config: String,
    },
    /// Bootstrap a QNetX node.
    QnetxNodeBootstrap {
        /// Path to the node bootstrap config file (TOML).
        #[arg(short, long, default_value = "bootstrap.toml")]
        config: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CiForkLauncher { config } => {
            // Delegates to src/ci_fork_launcher.rs
            ci_fork_launcher::run(&config).await?;
        }
        Commands::QnetxNodeBootstrap { config } => {
            // Delegates to src/qnetx_node_bootstrap.rs
            qnetx_node_bootstrap::run(&config).await?;
        }
    }

    Ok(())
}
