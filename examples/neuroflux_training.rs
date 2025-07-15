//! examples/neuroflux_training.rs
//!
//! A minimal example demonstrating how to run the NeuroFlux optimization
//! simulation from the Qublis v2.0 simulation suite.

use qublis_sim::config::SimConfig;
use qublis_sim::neuroflux_simulator::NeuroFluxSimulator;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a simulation configuration with defaults.
    let mut config = SimConfig::default();

    // Enable NeuroFlux and set number of iterations
    config.neuroflux_enabled = true;
    config.neuroflux_iterations = 100;

    // Instantiate the NeuroFlux simulator
    let mut simulator = NeuroFluxSimulator::new(&config);

    // Run the NeuroFlux optimization simulation
    let result = simulator.simulate()?;

    // Display results
    println!("=== NeuroFlux Optimization Simulation ===");
    println!("Iterations performed: {}", result.iterations);
    println!("Best metric found:    {:.6}", result.best_metric);
    println!("Progress per iteration (first 10 shown):");
    for (i, metric) in result.progress.iter().take(10) {
        println!("  iter {:>3}: {:.6}", i, metric);
    }
    if result.progress.len() > 10 {
        println!("  ... ({} more iterations)", result.progress.len() - 10);
    }

    // Export and print internal Prometheus-style metrics
    println!("\n=== Prometheus Metrics ===");
    println!("{}", simulator.export_metrics());

    Ok(())
}
