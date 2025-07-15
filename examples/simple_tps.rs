//! examples/simple_tps.rs
//!
//! A minimal example demonstrating how to run the TPS simulator
//! from the Qublis v2.0 simulation suite.

use qublis_sim::config::SimConfig;
use qublis_sim::tps_simulator::TpsSimulator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a simulation configuration with defaults.
    let mut config = SimConfig::default();

    // Optionally override defaults here or load from a TOML file:
    // config = SimConfig::load("sim_config.toml")?;
    config.duration_secs = 10;   // simulate 10 seconds
    config.target_tps = 1_000;   // target 1,000 transactions per second

    // Instantiate the TPS simulator
    let mut simulator = TpsSimulator::new(&config);

    // Run the simulation
    let result = simulator.simulate()?;

    // Display results
    println!("=== Simple TPS Simulation ===");
    println!("Target TPS:    {}", result.target_tps);
    println!("Average TPS:   {:.2}", result.average_tps);
    println!("Time-series samples:");
    for (second, tps) in &result.samples {
        println!("  second {:>2}: {:.2} TPS", second, tps);
    }

    // Export and print internal Prometheus-style metrics
    println!("\n=== Prometheus Metrics ===");
    println!("{}", simulator.export_metrics());

    Ok(())
}
