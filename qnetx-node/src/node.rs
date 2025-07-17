//! Core node runtime for the `qublis-qnetx-node` CLI.
//!
//! This module ties together P2P networking (QNet/QNetX), the QMesh consensus
//! engine with NeuroFlux optimization, quantum-inspired entanglement propagation,
//! causal reflection, JSON-RPC, and metrics/telemetry.

use crate::config::NodeConfig;
use crate::error::NodeError;
use crate::telemetry;
use crate::metrics;
use std::{path::Path, time::Duration};
use tokio::signal;
use tokio::time;
use log::{error, info};

// QNet primitives
use qublis_qnet::{Router, Relay, TeleportCore};
// QNetX overlay
use qublis_qnetx::{QuantumMeshOverlay, ZeroPropagation, StateCondenser, AnomalyFilter};
// Runtime modules
use qublis_runtime::{
    ConsensusNeuroFlux, EntanglementLoop, CausalReflector,
    WasmExecutor, RuntimeConfig,
    types::ConsensusEngineConfig,
};

/// Run the QNetX validator node.
///
/// Spawns tasks for:
/// - P2P network (QNet + QNetX overlay)
/// - Consensus loop with NeuroFlux
/// - Entanglement propagation loop
/// - Causal reflection loop
/// - JSON-RPC server (if enabled)
/// - Metrics & telemetry
pub async fn run(cfg: &NodeConfig, base_path: &Path) -> Result<(), NodeError> {
    // 1. Initialize telemetry & metrics endpoints
    telemetry::start(&cfg.telemetry)?;
    metrics::start(&cfg.metrics)?;

    // 2. Build QNet router/relay/teleport
    let mut router = Router::builder()
        .listen_addr(&cfg.listen_addr)
        .bootstrap_peers(cfg.bootstrap_peers.clone())
        .build()?;
    let mut relay = Relay::new();
    let mut teleport = TeleportCore::new();

    // 3. Build QNetX overlay (QuantumMeshOverlay) atop QNet primitives
    let mut qnetx = QuantumMeshOverlay::builder()
        .router(router.clone())
        .relay(relay.clone())
        .teleport(teleport.clone())
        .build()?;

    info!("QNetX overlay initialized; listening on {}", &cfg.listen_addr);

    // 4. Load runtime configuration (for consensus, entanglement, causal, wasm)
    //    Here we assume a default RuntimeConfig; for production you may load from file.
    let rt_cfg = RuntimeConfig::load("runtime/config.toml")
        .map_err(NodeError::Config)?;

    // 5. Instantiate consensus engine and NeuroFlux adapter
    let engine_cfg = ConsensusEngineConfig::default();
    let mut engine = qublis_runtime::types::ConsensusEngine::new(engine_cfg);
    let mut consensus = ConsensusNeuroFlux::new(&rt_cfg.consensus);

    // 6. Instantiate entanglement propagation loop
    let mut ent_loop = EntanglementLoop::new(&rt_cfg.entanglement);

    // 7. Instantiate causal reflector
    let mut causal = CausalReflector::new(&rt_cfg.causal);

    // 8. Instantiate WASM executor (for future QBLang contract hooks)
    let wasm_exec = WasmExecutor::new(&rt_cfg.wasm);

    // 9. Spawn P2P networking task
    tokio::spawn(async move {
        if let Err(e) = qnetx.run().await {
            error!("QNetX networking error: {}", e);
        }
    });

    // 10. Spawn consensus + NeuroFlux loop (every 1s)
    tokio::spawn(async move {
        let mut tick = time::interval(Duration::from_secs(1));
        loop {
            tick.tick().await;
            consensus.tick(&mut engine);
        }
    });

    // 11. Spawn entanglement propagation loop (configured interval)
    tokio::spawn(async move {
        let mut tick = time::interval(Duration::from_millis(rt_cfg.entanglement.interval_ms));
        loop {
            tick.tick().await;
            if let Err(e) = ent_loop.tick(&mut engine) {
                error!("Entanglement loop error: {}", e);
            }
        }
    });

    // 12. Spawn causal reflection loop (every 2s)
    tokio::spawn(async move {
        let mut tick = time::interval(Duration::from_secs(2));
        loop {
            tick.tick().await;
            if let Err(e) = causal.reflect(&mut engine) {
                error!("Causal reflector error: {}", e);
            }
        }
    });

    info!("Node running. Press CTRL-C to shut down.");

    // 13. Wait for shutdown signal (CTRL-C)
    signal::ctrl_c().await.map_err(|e| NodeError::Other(format!("Signal error: {}", e)))?;
    info!("Shutdown signal received. Terminating.");

    // (Optional) perform graceful shutdown here

    Ok(())
}
