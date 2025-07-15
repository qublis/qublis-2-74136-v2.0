```markdown
# QNet vs. QNetX

This document compares the **QNet** and **QNetX** networking layers in **Qublis v2.0** (2-74136), highlighting their architectures, features, performance characteristics, and appropriate use-cases.

---

## Table of Contents

1. [Overview](#overview)  
2. [Feature Comparison](#feature-comparison)  
3. [Architecture Diagrams](#architecture-diagrams)  
4. [API Differences](#api-differences)  
5. [Performance & Scalability](#performance--scalability)  
6. [When to Use Which](#when-to-use-which)  
7. [Configuration Examples](#configuration-examples)  
8. [Migration Path](#migration-path)  

---

## Overview

- **QNet**: The base P2P transport and routing layer. Implements peer discovery, reliable gossip relay, and “quantum-inspired” scoring of paths. Designed for general block and transaction propagation.

- **QNetX**: An extended overlay built on top of QNet. Adds:
  - **Entropic DAG overlays** (QuantumMeshOverlay)  
  - **Zero-Propagation** rapid convergence protocol  
  - **State Condenser** for compact state fingerprints  
  - **Anomaly Filter** for resilience under adversarial conditions  

QNetX is optimized for ultra-high throughput (tens of millions of messages/sec), multi-dimensional routing, self-healing, and state-of-the-art mesh consistency.

---

## Feature Comparison

| Capability            | QNet               | QNetX                             |
|-----------------------|--------------------|-----------------------------------|
| Peer Discovery        | ✅                 | ✅ (inherits QNet)                |
| Gossip Relay          | ✅                 | ✅ (entropic batching)            |
| Teleportation         | ✅                 | ✅ (prioritized within overlay)   |
| DAG Overlay           | ❌                 | ✅ (QuantumMeshOverlay)           |
| Rapid Convergence     | ❌                 | ✅ (ZeroPropagation)              |
| State Summarization   | ❌                 | ✅ (StateCondenser)               |
| Anomaly Detection     | ❌                 | ✅ (AnomalyFilter)                |
| Multi-Dimensional     | ❌                 | ✅ (cross-dimension routing)      |
| Performance Target    | ~10M msgs/sec      | >50M msgs/sec                     |
| Latency (median)      | ~0.5 ms            | <0.1 ms                           |
| Complexity            | Low                | High                              |
| Suitable For          | Basic block/tx P2P | Enterprise-scale, adversarial env |

---

## Architecture Diagrams

### QNet

```

+--------------------+
\|   Application      |
+--------------------+
\|       QNet         |
\|  Router | Relay    |
\|  TeleportCore      |
+--------------------+
\|    TCP/WebSocket   |
+--------------------+

```

### QNetX

```

+-------------------------+
\|      Application        |
+-------------------------+
\|   QNetX Extended Overlay|
\| ┌─────────────────────┐ |
\| │ QuantumMeshOverlay  │ |
\| │ ZeroPropagation     │ |
\| │ StateCondenser      │ |
\| │ AnomalyFilter       │ |
\| └─────────────────────┘ |
+-------------------------+
\|         QNet            |
+-------------------------+
\|      Transport Layer    |
+-------------------------+

````

---

## API Differences

### QNet

```rust
use qublis_qnet::{Router, Relay, TeleportCore, QNetConfig};

let cfg = QNetConfig::load("qnet.toml")?;
let mut router = Router::new(&cfg);
router.add_peer("peer:30333")?;
router.heartbeat().await?;

let mut relay = Relay::new(&cfg);
relay.broadcast(b"data").await?;

let mut tp = TeleportCore::new(&cfg);
tp.request_teleport(peer_qid, b"urgent").await?;
````

### QNetX

```rust
use qublis_qnetx::{
    QuantumMeshOverlay, ZeroPropagation,
    StateCondenser, AnomalyFilter, QNetXConfig
};

let cfg = QNetXConfig::load("qnetx.toml")?;
let mut mesh = QuantumMeshOverlay::new(&cfg);
mesh.propagate().await?;

let mut zprop = ZeroPropagation::new(&cfg);
zprop.broadcast_zero().await?;

let condenser = StateCondenser::new(&cfg);
let fingerprint = condenser.condense(&mesh).await?;

let mut filter = AnomalyFilter::new(&cfg);
filter.inspect(&envelope).await?;
```

---

## Performance & Scalability

| Metric             | QNet                  | QNetX                        |
| ------------------ | --------------------- | ---------------------------- |
| Max Throughput     | \~10 M msgs/sec       | >50 M msgs/sec               |
| Median Latency     | \~0.5 ms              | <0.1 ms                      |
| Peer Table Size    | Up to 1000            | Up to 10 000                 |
| Convergence Time   | N/A                   | O(log N) zero convergence    |
| Resource Footprint | Low (single‐threaded) | High (multi‐threaded, async) |

---

## When to Use Which

* **Choose QNet** if:

  * You need a lightweight P2P layer for standard block/tx propagation.
  * Latency and throughput requirements are moderate (<10 M msg/sec).
  * Simplicity and minimal resource usage are priorities.

* **Choose QNetX** if:

  * You require ultra-high throughput (>50 M msg/sec).
  * Rapid network-wide state convergence is critical (zero-propagation).
  * You operate in adversarial or highly dynamic environments and need anomaly filtering.
  * You want entropic DAG overlays for advanced consensus experimentation.

---

## Configuration Examples

### qnet.toml

```toml
listen_addr          = "0.0.0.0:30333"
max_peers            = 100
peer_timeout_secs    = 30
enable_encryption    = true
teleport_priority    = 0.05
```

### qnetx.toml

```toml
listen_addr           = "0.0.0.0:30334"
mesh_gossip_interval  = 50      # ms
heartbeat_interval_ms = 20
condense_batch_size   = 500
anomaly_threshold     = 0.001
enable_zero_propagate = true
enable_mesh_snapshots = true
```

---

## Migration Path

1. **Start with QNet** for initial deployment and testing.
2. **Benchmark** your workload under QNet to identify bottlenecks.
3. **Evaluate QNetX** in a staging environment, tuning `qnetx.toml` parameters.
4. **Gradually roll out** QNetX to production, monitoring Prometheus metrics (`qnetx_mesh_nodes`, `qnetx_zero_heartbeats`, etc.).
5. **Decommission QNet** nodes once QNetX is stable and performance gains are realized.

---

*End of qnet\_vs\_qnetx.md*\`\`\`
