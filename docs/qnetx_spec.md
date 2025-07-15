```markdown
# QNetX v2.0 Specification

This document defines the **QNetX** extended quantum networking layer for **Qublis v2.0** (2-74136). QNetX builds on QNet to provide entropic DAG overlays, zero-propagation, state condensation and anomaly filtering for high-throughput, resilient P2P communication.

---

## Table of Contents

1. [Overview](#overview)  
2. [Architecture](#architecture)  
3. [Core Components](#core-components)  
   - [QuantumMeshOverlay](#quantummeshoverlay)  
   - [ZeroPropagation](#zeropropagation)  
   - [StateCondenser](#statecondenser)  
   - [AnomalyFilter](#anomalyfilter)  
4. [Protocol & Message Formats](#protocol--message-formats)  
5. [Rust API](#rust-api)  
6. [Configuration](#configuration)  
7. [CLI Usage](#cli-usage)  
8. [Metrics & Performance](#metrics--performance)  
9. [Security & Privacy](#security--privacy)  
10. [Examples](#examples)  
11. [Extensibility](#extensibility)  

---

## Overview

QNetX extends QNet with:

- **Entropic DAG overlays** for block and message ordering (`QuantumMeshOverlay`).  
- **Zero-Propagation**: rapid convergence of state across peers (`ZeroPropagation`).  
- **State Condenser**: summarizes large state graphs into compact QNums (`StateCondenser`).  
- **Anomaly Filter**: detects and drops malicious or outlier messages (`AnomalyFilter`).  

These features enable Qublis to achieve billions of TPS, infinite-dimensional routing and self-healing under adversarial conditions.

---

## Architecture

```

+------------------------------------------------+
\|                    Application                 |
+------------------------------------------------+
\|    Consensus (QMesh)    |    AI / CI-Core      |
+------------------------------------------------+
\|                QNetX Extended Overlay         |
\|  ┌────────────────────────────────────────────┐ |
\|  │ QuantumMeshOverlay  ZeroPropagation       │ |
\|  │ StateCondenser      AnomalyFilter         │ |
\|  └────────────────────────────────────────────┘ |
+------------------------------------------------+
\|                     QNet                      |
+------------------------------------------------+
\|                  Transport                    |
+------------------------------------------------+

````

QNetX resides above QNet, consuming its transport and basic routing, and adds high-level mesh, propagation and filtering functions.

---

## Core Components

### QuantumMeshOverlay

- Maintains an **entropic DAG** of messages/blocks.  
- Each message is a node annotated with an **entropy score** (QNum).  
- Peers exchange **mesh snapshots** to converge on a shared DAG.  
- Provides `mesh.propagate()` and `mesh.fetch_missing()` APIs.

### ZeroPropagation

- Implements a “zero-state” convergence protocol:  
  - Each peer periodically broadcasts a **zero QNum** heartbeat.  
  - Recipients merge zeros to drive state distances toward 0.  
- Ensures **rapid network-wide state agreement** in O(log N) rounds.

### StateCondenser

- Summarizes large DAGs or message sets into a single QNum:  
  - Applies **quantum addition** (`qadd`) across all node QNums.  
  - Normalizes and measures to produce a compact fingerprint.  
- Enables lightweight state exchange for bootstrapping or audits.

### AnomalyFilter

- Monitors incoming messages for statistical outliers:  
  - Tracks recent **entropy deltas** per peer.  
  - Applies quantum thresholding to flag anomalies.  
- Drops or quarantines suspicious messages to protect the mesh.

---

## Protocol & Message Formats

All QNetX messages are framed over QNet envelopes with a `msg_type` field:

```text
enum QNetXType : u8 {
  MESH_SNAPSHOT     = 0x10,
  ZERO_HEARTBEAT    = 0x11,
  STATE_CONDENSE    = 0x12,
  ANOMALY_ALERT     = 0x13
}

struct QNetXEnvelope {
  u8     version;
  u8     qnetx_type;
  QNum   origin;
  u32    payload_len;
  bytes  payload;
}
````

* **MESH\_SNAPSHOT** payload: serialized DAG delta (CBOR).
* **ZERO\_HEARTBEAT** payload: empty (zero stamp).
* **STATE\_CONDENSE** payload: condensed QNum bytes.
* **ANOMALY\_ALERT** payload: `(peer_qid, anomaly_score)`.

---

## Rust API

```rust
use qublis_qnetx::{QuantumMeshOverlay, ZeroPropagation, StateCondenser, AnomalyFilter, QNetXConfig};

let cfg = QNetXConfig::load("qnetx.toml")?;
let mut mesh = QuantumMeshOverlay::new(&cfg);
mesh.propagate().await?;

let mut zprop = ZeroPropagation::new(&cfg);
zprop.broadcast_zero().await?;

let condenser = StateCondenser::new(&cfg);
let fingerprint: QNum = condenser.condense(&mesh).await?;

let mut filter = AnomalyFilter::new(&cfg);
filter.inspect(&envelope).await?;
```

### Key Types

* `QNetXConfig` – loaded from TOML.
* `QuantumMeshOverlay` – DAG maintenance.
* `ZeroPropagation` – convergence engine.
* `StateCondenser` – summarization tool.
* `AnomalyFilter` – runtime message filter.

---

## Configuration

```toml
# qnetx.toml
listen_addr           = "0.0.0.0:30334"
mesh_gossip_interval  = 100      # ms
heartbeat_interval_ms = 50
condense_batch_size   = 1000
anomaly_threshold     = 0.001
enable_zero_propagate = true
enable_mesh_snapshots = true
```

* `mesh_gossip_interval`: time between DAG exchanges.
* `heartbeat_interval_ms`: zero heartbeat frequency.
* `condense_batch_size`: max nodes per condense run.
* `anomaly_threshold`: maximum allowed entropy delta.

---

## CLI Usage

```bash
# Start a QNetX node
qublis-qnetx-node --config qnetx.toml

# Trigger a manual mesh snapshot
qublis-qnetx-node mesh snapshot

# Condense current mesh state
qublis-qnetx-node mesh condense
```

---

## Metrics & Performance

Prometheus exporter on port `9200` with metrics:

* `qnetx_mesh_nodes`
* `qnetx_mesh_edges`
* `qnetx_zero_heartbeats`
* `qnetx_condense_ops`
* `qnetx_anomalies_detected`

Performance targets:

* **>50M messages/sec** on 64-core cluster
* **<0.1 ms** condense round-trip
* **Anomaly detection** latency <1 ms

---

## Security & Privacy

* **End-to-end encryption** inherited from QNet.
* **Peer authentication** via QNum handshake.
* **AnomalyFilter** protects mesh integrity under adversarial load.

---

## Examples

### Mesh Snapshot

```bash
qublis-qnetx-node mesh snapshot > snapshot.cbor
```

### Zero Propagation Test

```bash
qublis-qnetx-node zero broadcast --once
```

---

## Extensibility

* **Custom DAG store**: implement `DagStorage` trait.
* **Plugin filters**: register new `AnomalyFilter` strategies.
* **Alternate condense algorithms**: swap in custom summarizers via trait `Condenser`.

---

```
::contentReference[oaicite:0]{index=0}
```
