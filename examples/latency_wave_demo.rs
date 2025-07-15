//! examples/latency_wave_demo.rs
//!
//! A minimal example demonstrating how to run the LatencyWave simulator
//! from the Qublis v2.0 simulation suite.

use qublis_sim::config::SimConfig;
use qublis_sim::latency_wave::LatencyWave;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a simulation configuration with defaults.
    let mut config = SimConfig::default();

    // Optionally override defaults here or load from a TOML file:
    // config = SimConfig::load("sim_config.toml")?;
    config.duration_secs = 5;        // simulate for 5 seconds
    config.latency_mean_ms = 100.0;  // average latency of 100 ms
    config.latency_stddev_ms = 20.0; // standard deviation of 20 ms

    // Instantiate the LatencyWave simulator
    let mut simulator = LatencyWave::new(&config);

    // Run the simulation
    let profile = simulator.simulate()?;

    // Display results
    println!("=== Latency Wave Simulation ===");
    println!("Duration:              {} seconds", config.duration_secs);
    println!("Configured mean (ms):  {:.2}", profile.mean_ms);
    println!("Configured stddev (ms):{:.2}", profile.stddev_ms);
    println!("Time-series latency samples:");
    for (second, latency) in &profile.samples {
        println!("  second {:>2}: {:.2} ms", second, latency);
    }

    // Export and print internal Prometheus-style metrics
    println!("\n=== Prometheus Metrics ===");
    println!("{}", simulator.export_metrics());

    Ok(())
}
