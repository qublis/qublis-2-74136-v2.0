```markdown
# QMesh v2.0 Specification

This document defines the **QMesh** entropic DAG consensus engine for **Qublis v2.0** (2-74136). QMesh replaces classical blockchain chains with a multi-dimensional, entropy-driven DAG to achieve high throughput, instant finality, and quantum-inspired resilience.

---

## Table of Contents

1. [Overview](#overview)  
2. [Architecture](#architecture)  
3. [Core Components](#core-components)  
   - [Entropic DAG](#entropic-dag)  
   - [Cognitive Entropy](#cognitive-entropy)  
   - [RetroChain Tracker](#retrochains-tracker)  
   - [Causal Reflector](#causal-reflector)  
4. [Consensus Protocol](#consensus-protocol)  
5. [Data Structures & Message Formats](#data-structures--message-formats)  
6. [Rust API](#rust-api)  
7. [Configuration](#configuration)  
8. [CLI Usage](#cli-usage)  
9. [Metrics & Monitoring](#metrics--monitoring)  
10. [Security & Attack Resistance](#security--attack-resistance)  
11. [Extensibility](#extensibility)  

---

## Overview

QMesh implements a **Directed Acyclic Graph** of “blocks” (or events), each annotated with an **entropy score** derived from its parent references. Instead of a strict linear chain, QMesh:

- Allows **parallel block production** by multiple validators.  
- Uses **entropy accumulation** to measure confidence in finality.  
- Integrates **retrocausal tracking** for time-symmetric reorganization.  

This yields sub-millisecond confirmation and effectively unlimited TPS.

---

## Architecture

```

┌──────────────────────────┐
│   Application / Runtime  │
└──────────────────────────┘
│
▼
┌──────────────────────────┐
│       Consensus         │
│  ┌────────────────────┐ │
│  │   Entropic DAG     │ │
│  │ Cognitive Entropy  │ │
│  │ RetroChain Tracker │ │
│  │ Causal Reflector   │ │
│  └────────────────────┘ │
└──────────────────────────┘
│
▼
Storage & Networking

````

- **Entropic DAG**: Core DAG of blocks and reference edges.  
- **Cognitive Entropy**: Computes a global “uncertainty” metric.  
- **RetroChain Tracker**: Supports re-entry of late blocks and time-symmetric reorg.  
- **Causal Reflector**: Reflects and validates causal dependencies across dimensions.

---

## Core Components

### Entropic DAG

- **Block**: `struct Block { id: QNum, parents: Vec<QNum>, payload: Bytes, entropy: f64 }`  
- **DAG**: A set of blocks with parent references; no cycles.  
- **Insertion**: New block references one or more tips; entropy updated.

### Cognitive Entropy

- Measures overall DAG uncertainty:  
  \[
    H = -\sum_{b \in \text{tips}} p_b \log p_b
  \]  
- Helps choose which tips to extend and when to finalize.

### RetroChain Tracker

- Allows **late-arriving blocks** (out-of-order) to be integrated.  
- Maintains a secondary “retro” DAG for reorgs across time-symmetric branches.

### Causal Reflector

- Ensures **causal consistency** across infinite-dimensional branches.  
- Reflects state updates along all ancestor paths using quantum gate analogues.

---

## Consensus Protocol

1. **Block Proposal**  
   - Validator assembles payload, computes entropy weight, and broadcasts.  
2. **Tip Selection**  
   - Select top-k tips by highest joint entropy + lowest age.  
3. **Attachment**  
   - New block references selected tips.  
4. **Finality**  
   - When a block’s cumulative entropy exceeds threshold \(H_{\text{final}}\), it is final.  
5. **Retro Reorg**  
   - If late block increases entropy of an alternate path above current, trigger **RetroChain Tracker**.

---

## Data Structures & Message Formats

### Block

```rust
struct Block {
    qid: QNum,                  // Unique block identifier
    parents: Vec<QNum>,         // Parent block QNums
    payload: Vec<u8>,           // Application data (transactions, etc.)
    entropy: f64,               // Entropy score
    timestamp: u64,             // Unix timestamp
}
````

### Gossip Message

```text
enum QMeshMsgType : u8 {
  NEW_BLOCK      = 0x20,
  ENTROPY_UPDATE = 0x21,
  RETRO_UPDATE   = 0x22,
}

struct QMeshEnvelope {
  u8    version;
  u8    msg_type;
  QNum  origin_qid;
  u32   payload_length;
  bytes payload; // serialized Block or entropy vector
}
```

---

## Rust API

```rust
use qublis_qmesh::{EntropicDag, QMeshConfig, CognitiveEntropy, RetroChainTracker, CausalReflector};

let cfg = QMeshConfig::load("qmesh.toml")?;
let mut dag = EntropicDag::new(&cfg);
let block = dag.create_block(parents, payload)?;
dag.insert_block(block)?;

let entropy = CognitiveEntropy::compute(&dag);
let finalized = dag.check_finality(&block.id);

let mut retro = RetroChainTracker::new(&cfg);
retro.process_late_block(block)?;

let mut reflector = CausalReflector::new(&cfg);
reflector.ensure_causal(&dag);
```

### Key Types

* `QMeshConfig` – consensus parameters (entropy thresholds, tip count).
* `EntropicDag` – DAG storage & insertion.
* `CognitiveEntropy` – global entropy calculator.
* `RetroChainTracker` – handles late blocks and reorg.
* `CausalReflector` – enforces causal consistency hooks.

---

## Configuration

```toml
# qmesh.toml
max_tips = 8
entropy_finality = 10.0
retro_window_secs = 30
causal_reflect = true
snapshot_interval_ms = 100
```

* `max_tips`: number of tips to attach to.
* `entropy_finality`: entropy threshold for finality.
* `retro_window_secs`: time window for retro blocks.
* `causal_reflect`: enable causal reflection.
* `snapshot_interval_ms`: periodic DAG snapshot frequency.

---

## CLI Usage

```bash
# Start a QMesh validator
qublis-qmesh-node --config qmesh.toml

# Inspect DAG tips
qublis-qmesh-node tips

# Force a retro update with a block file
qublis-qmesh-node retro --block block.bin
```

---

## Metrics & Monitoring

Prometheus exporter on port `9300` exposes:

* `qmesh_blocks_created`
* `qmesh_tips_count`
* `qmesh_entropy_current`
* `qmesh_finalized_blocks`
* `qmesh_retro_events`
* `qmesh_causal_reflections`

---

## Security & Attack Resistance

* **Sybil Resistance**: weighted by validator stakes encoded in QNums.
* **DAG Flood Protection**: entropy backoff to throttle malicious forks.
* **Retro Attack Mitigation**: limited retro window and quantum-proof finality.

---

## Extensibility

* **Custom Tip Selection**: implement `TipSelector` trait.
* **Alternate Entropy Metrics**: swap in new `EntropyCalculator`.
* **Plugin Storage**: use custom `DagStorage` backends (e.g., LevelDB, RocksDB).
* **Interoperability**: integrate with QNetX overlay for mesh-level gossip.

---

```
::contentReference[oaicite:0]{index=0}
```
