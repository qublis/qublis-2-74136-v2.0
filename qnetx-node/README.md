````markdown
# qublis-qnetx-node

**Extended QNetX Validator Node Binary for Qublis v2.0 (2-74136)**

A standalone CLI application implementing the QNetX overlay validator node. Provides peer-to-peer networking, entropic DAG consensus (QMesh), NeuroFlux optimization, telemetry, and administrative commands.

---

## Table of Contents

- [Features](#features)  
- [Prerequisites](#prerequisites)  
- [Installation](#installation)  
- [Configuration](#configuration)  
- [Usage](#usage)  
  - [`init`](#init)  
  - [`run`](#run)  
  - [`status`](#status)  
- [Logging & Metrics](#logging--metrics)  
- [Examples](#examples)  
- [Development](#development)  
- [License](#license)  

---

## Features

- **QNetX Overlay**: Quantum-inspired entropic DAG propagation, zero-propagation, state condensation, anomaly filtering.  
- **NeuroFlux Integration**: Real-time consensus and network optimization via reinforcement learning.  
- **CLI Commands**: `init`, `run`, `status` for lifecycle management.  
- **Configuration**: TOML-based, with schema validation.  
- **Metrics & Telemetry**: Prometheus endpoint, structured logs.  

---

## Prerequisites

- Rust **1.88.0** (toolchain via `rustup`)  
- `wasm32-unknown-unknown` target (for QBLang contracts)  
- Network ports:  
  - **30333** TCP for P2P  
  - **9944** TCP for JSON-RPC (optional)  
  - **9300–9500** TCP for Prometheus metrics  

---

## Installation

Build from source:

```bash
git clone https://github.com/YourOrg/qublis-2-74136-v2.0.git
cd qublis-2-74136-v2.0/qnetx-node
cargo build --release
# Binary: target/release/qublis-qnetx-node
````

Or install via Cargo (if published):

```bash
cargo install --git https://github.com/YourOrg/qublis-2-74136-v2.0.git --bin qublis-qnetx-node --locked
```

---

## Configuration

Create a TOML config file (e.g. `config/node.toml`):

```toml
# Node identity
node_name       = "validator-1"
listen_addr     = "/ip4/0.0.0.0/tcp/30333"

# Peers to bootstrap from
bootstrap_peers = [
  "/ip4/203.0.113.5/tcp/30333",
  "/ip4/203.0.113.6/tcp/30333"
]

# QMesh consensus parameters
[consensus]
qmesh_config_path      = "../qmesh/config/default.toml"
neuroflux_enabled      = true
neuroflux_config_path  = "../ci_core/neuroflux.toml"

# Prometheus metrics
[metrics]
port    = 9400
enabled = true

# Logging
[logging]
level   = "info"   # trace, debug, info, warn, error
format  = "json"   # json, plain
```

A sample schema is provided in `config/schema.json`.

---

## Usage

```bash
# Show help
qublis-qnetx-node --help
```

### `init`

Initialize data directories, keys, and default configs:

```bash
qublis-qnetx-node init \
  --config config/node.toml \
  --base-path /var/lib/qublis/node1
```

Creates `base-path`, copies default QMesh specs, and generates node keys.

### `run`

Start the validator node:

```bash
qublis-qnetx-node run \
  --config config/node.toml \
  --base-path /var/lib/qublis/node1
```

Runs P2P networking, consensus loop with NeuroFlux, and exposes JSON-RPC & metrics.

### `status`

Query runtime status without stopping:

```bash
qublis-qnetx-node status \
  --config config/node.toml \
  --base-path /var/lib/qublis/node1
```

Outputs current tip count, TPS, average latency, and NeuroFlux metrics.

---

## Logging & Metrics

* **Logs** respect the `logging.level` and `logging.format` settings.
* **Prometheus** metrics available at `http://<host>:<metrics.port>/metrics`.

Key metrics:

* `qnetx_peers_connected_total`
* `qmesh_blocks_produced_total`
* `neuroflux_ticks_total`
* `neuroflux_rewards_sum`
* `entanglement_branches_processed`

---

## Examples

Run a local test node:

```bash
# Initialize
qublis-qnetx-node init \
  --config config/local.toml \
  --base-path ./data/node-local

# Run
qublis-qnetx-node run \
  --config config/local.toml \
  --base-path ./data/node-local
```

---

## Development

* Code is formatted with `rustfmt` and linted with `clippy`.

* Tests cover CLI parsing, config loading, key runtime components.

* To run tests:

  ```bash
  cd qnetx-node
  cargo test
  ```

* Contributions welcome via pull requests against the `main` branch.

---

## License

**Proprietary.** All rights reserved by the Qublis Consortium. Unauthorized copying, modification, or distribution is prohibited.

```
::contentReference[oaicite:0]{index=0}
```
