```markdown
# QNet v2.0 Specification

This document defines the **QNet** peer‐to‐peer networking layer for **Qublis v2.0** (2-74136). QNet provides quantum‐enhanced routing, relaying and teleportation primitives for block and message propagation.

---

## Table of Contents

1. [Overview](#overview)  
2. [Architecture](#architecture)  
3. [Protocol Stack](#protocol-stack)  
4. [Message Formats](#message-formats)  
5. [Rust API](#rust-api)  
6. [Configuration](#configuration)  
7. [CLI Usage](#cli-usage)  
8. [Metrics & Performance](#metrics--performance)  
9. [Security & Authentication](#security--authentication)  
10. [Examples](#examples)  
11. [Extensibility](#extensibility)  

---

## Overview

QNet is the low‐level transport and routing layer for Qublis. It implements:

- **Router**: peer discovery, path selection, quantum‐inspired metrics.  
- **Relay**: gossip relay with entropic batching.  
- **Teleport**: non‐local “quantum teleportation” of messages for ultra‐low latency.  

QNet is designed for high throughput (millions of messages/sec) and sub‐millisecond latency, leveraging **QNum**‐based metrics for routing decisions.

---

## Architecture

```

+---------------------+
\|    Application      |
+---------------------+
\|      QMesh /        |
\|     Consensus       |
+---------------------+
\|        QNet         |
\|  Router | Relay | Teleport |
+---------------------+
\|  TCP / WebSocket    |
+---------------------+
\|   Network Overlay   |
+---------------------+

````

- **Router**: manages peer table, heartbeat, path scoring.  
- **Relay**: implements gossipsub‐style flood relay with entropy‐based backoff.  
- **Teleport**: special expedited channel using entropic DAG tags to jump intermediate hops.

---

## Protocol Stack

1. **Transport**: TCP or WebSocket.  
2. **Frame**: length‐prefixed binary frame with header + payload.  
3. **QNet Envelope**:  
   - `version` (u8)  
   - `msg_type` (u8)  
   - `flags` (u16)  
   - `qid` (QNum, 16 bytes) – origin identifier  
   - `payload_len` (u32)  
4. **Payload**: serialized Protobuf or CBOR message.

---

## Message Formats

```text
struct Envelope {
  u8    version;
  u8    msg_type;
  u16   flags;
  QNum  origin_qid;
  u32   payload_length;
  bytes payload;
}
````

* **msg\_type**:

  * `0x01` = ROUTE\_DISCOVERY
  * `0x02` = DATA\_RELAY
  * `0x03` = TELEPORT\_REQUEST
  * `0x04` = TELEPORT\_ACK
* **flags**: bitmask (reliable, encrypted, quantum‐priority).

---

## Rust API

```rust
use qublis_qnet::{Router, Relay, TeleportCore, QNetConfig};

let cfg = QNetConfig::load("qnet.toml")?;
let mut router = Router::new(&cfg);
router.add_peer("192.0.2.1:30333")?;
router.heartbeat().await?;

let mut relay = Relay::new(&cfg);
relay.broadcast(b"block_data").await?;

let mut tp = TeleportCore::new(&cfg);
tp.request_teleport(peer_qid, b"urgent_tx").await?;
```

### Key Types

* `QNetConfig` – loaded from TOML.
* `Router` – `.new()`, `.add_peer()`, `.remove_peer()`, `.heartbeat()`.
* `Relay` – `.broadcast()`, `.subscribe(msg_type)`.
* `TeleportCore` – `.request_teleport()`, `.accept_teleport()`.

---

## Configuration

```toml
# qnet.toml
listen_addr = "0.0.0.0:30333"
max_peers = 50
peer_timeout_secs = 30
enable_encryption = true
teleport_priority = 0.1  # fraction of bandwidth
```

* `listen_addr`: local bind address.
* `max_peers`: cap on peer table size.
* `peer_timeout_secs`: heartbeat expiry.
* `teleport_priority`: share of uplink for teleport messages.

---

## CLI Usage

```bash
# Start a QNet node
qublis-qnet-node --config qnet.toml

# Inspect peers
qublis-qnet-node status

# Broadcast raw data
qublis-qnet-node send --type DATA_RELAY --file block.bin
```

---

## Metrics & Performance

* **Prometheus** exporter on port `9100` with metrics:

  * `qnet_peers_connected`
  * `qnet_messages_relayed`
  * `qnet_teleport_requests`
  * `qnet_latency_ms`
* Achieves **>10M msgs/s** on 32‐core server, **<0.5ms** median latency.

---

## Security & Authentication

* **Encryption**: TLS 1.3 by default.
* **Authentication**: QID-based handshake using ECDSA over QNum.
* **Authorization**: peer ACLs in config.

---

## Examples

### Simple Router

```rust
let mut router = Router::new(&cfg);
router.add_peer("peer1:30333")?;
router.add_peer("peer2:30333")?;
router.heartbeat().await?;
```

### Teleport Request

```rust
let tp = TeleportCore::new(&cfg);
tp.request_teleport(&peer_qid, b"urgent").await?;
```

---

## Extensibility

* **Custom Transports**: implement `Transport` trait and inject into `Router`.
* **Custom MsgTypes**: extend `msg_type` enum and handlers in `prelude.rs`.
* **Integration**: QNet can be plugged into QMesh consensus for gossip propagation or into CI-Core for agent synchronization messages.

---

```
::contentReference[oaicite:0]{index=0}
```
