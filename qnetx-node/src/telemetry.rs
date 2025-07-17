//! Telemetry server for the `qublis-qnetx-node` CLI.
//!
//! Starts a simple HTTP endpoint to serve Prometheus‐style metrics at the
//! address configured in `TelemetryConfig.prometheus_bind`.

use crate::config::TelemetryConfig;
use crate::error::NodeError;
use crate::metrics;
use std::io::Write;
use std::net::TcpListener;
use std::thread;

/// Start the telemetry HTTP server.
///
/// Binds to `cfg.prometheus_bind` (e.g. "0.0.0.0:9300") and serves the
/// current metrics in Prometheus text format on every incoming connection.
pub fn start(cfg: &TelemetryConfig) -> Result<(), NodeError> {
    // Bind to the configured address
    let bind_addr = &cfg.prometheus_bind;
    let listener = TcpListener::bind(bind_addr)
        .map_err(|e| NodeError::Other(format!(
            "Failed to bind telemetry endpoint at {}: {}",
            bind_addr, e
        )))?;

    // Spawn a thread to handle incoming scrape requests
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    // Gather all metrics as Prometheus text
                    let body = metrics::export_prometheus();
                    // Form a minimal HTTP response
                    let response = format!(
                        "HTTP/1.1 200 OK\r\n\
                         Content-Type: text/plain; version=0.0.4\r\n\
                         Content-Length: {}\r\n\r\n\
                         {}",
                        body.len(),
                        body
                    );
                    // Ignore any write errors on a per‐connection basis
                    let _ = stream.write_all(response.as_bytes());
                }
                Err(_) => {
                    // Ignore errors accepting connections
                    continue;
                }
            }
        }
    });

    log::info!("Telemetry endpoint listening on {}", bind_addr);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::RuntimeMetrics;
    use std::{thread, time::Duration};
    use std::io::{Read, Write};

    /// A dummy TelemetryConfig for testing.
    fn make_cfg() -> TelemetryConfig {
        TelemetryConfig {
            prometheus_bind: "127.0.0.1:0".into(), // let OS assign a port
        }
    }

    #[test]
    fn telemetry_server_responds_with_metrics() {
        // Initialize metrics for test
        RuntimeMetrics::new().inc_counter("test_counter", 5);
        let cfg = make_cfg();
        // Bind to ephemeral port
        let listener = TcpListener::bind(&cfg.prometheus_bind).unwrap();
        let addr = listener.local_addr().unwrap();
        drop(listener);

        let mut cfg = cfg;
        cfg.prometheus_bind = addr.to_string();
        // Start server
        start(&cfg).unwrap();

        // Give the server a moment to start
        thread::sleep(Duration::from_millis(50));

        // Connect and read response
        let mut socket = std::net::TcpStream::connect(addr).unwrap();
        socket.write_all(b"GET /metrics HTTP/1.1\r\n\r\n").unwrap();
        let mut buf = String::new();
        socket.read_to_string(&mut buf).unwrap();

        // Check that our test counter appears in the body
        assert!(buf.contains("test_counter 5"));
    }
}
