````markdown
# Qublis v2.0 Architecture

This document describes the overall architecture of **Qublis v2.0** (2-74136), including its core crates, data flows, and component interactions.

---

## Table of Contents

1. [High-Level Overview](#high-level-overview)  
2. [Core Crates & Responsibilities](#core-crates--responsibilities)  
   - [qublis-qnum](#qublis-qnum)  
   - [qublis-qnet](#qublis-qnet)  
   - [qublis-qnetx](#qublis-qnetx)  
   - [qublis-qmesh](#qublis-qmesh)  
   - [qublis-qlink](#qublis-qlink)  
   - [qublis-ci_core](#qublis-ci_core)  
   - [qublis-sim](#qublis-sim)  
   - [qublis-deploy](#qublis-deploy)  
3. [Data & Control Flows](#data--control-flows)  
4. [Consensus & NeuroFlux Optimization](#consensus--neuroflux-optimization)  
5. [Extensibility & QBLang Integration](#extensibility--qblang-integration)  
6. [Security & Ethics](#security--ethics)  
7. [Deployment Topology](#deployment-topology)  

---

## High-Level Overview

Qublis v2.0 is a modular, Rust-based blockchain & AI platform featuring:

- **Quantum-Number System (QNum)** as foundational data type.  
- **QNet / QNetX** for quantum-enhanced P2P networking.  
- **QMesh** for entropic DAG consensus and state propagation.  
- **QLink** for on-chain quantum identities, ethics, and consent.  
- **CI-Core** for conscious-AI subsystems (MorphicAI, MoralRegulator, CollectiveSync).  
- **Sim** suite for TPS, latency, NeuroFlux, and full network simulation.  
- **Deploy** tools for CI forking and node bootstrapping.

All crates live in a single workspace (`qublis-2-74136-v2.0/`) and interoperate via well-defined Rust API surfaces.

---

## Core Crates & Responsibilities

### qublis-qnum

- **Defines** `Qid` and `QNum` types: superposed “quantum digits” and multi-digit numbers.  
- **Implements** quantum gates (`entangle`, `qadd`, `qmul`), measurement, entropy.  
- **Foundation** for all higher-level quantum abstractions.

---

### qublis-qnet

- **Peer-to-peer layer**: routing, relaying, teleportation primitives (`router.rs`, `relay.rs`, `teleport_core.rs`).  
- **Transport**: multiplexed channels over TCP/WebSockets.  
- **Pluggable**: supports swapping in custom network backends.

---

### qublis-qnetx

- **Extended quantum network**: entropic DAG overlays, zero-propagation, state condensation, anomaly filtering.  
- **“X” features**: advanced routing, probabilistic teleportation, network coding.

---

### qublis-qmesh

- **Consensus engine**: entropic DAG (Directed Acyclic Graph) of blocks (`entropic_dag.rs`).  
- **Cognitive entropy**: measures global network uncertainty.  
- **Retro-chain tracking**: supports time-symmetry and retrocausality in block ordering.

---

### qublis-qlink

- **Quantum Identity (QID)**: deterministic on-chain identifiers (`qid_layer.rs`).  
- **Ethics lattice**: entangled ethical principles, collapsible to weights (`ethics_lattice.rs`).  
- **Conscious consent**: probabilistic user consent records (`conscious_consent.rs`).  
- **Mutation engine**: dynamic policy updates via entanglement (`mutation_engine.rs`).

---

### qublis-ci_core

- **MorphicAI**: quantum-enhanced neural substrate (`morphic_ai.rs`).  
- **MoralRegulator**: enforces ethics on AI outputs (`moral_regulator.rs`).  
- **CollectiveSync**: synchronizes distributed AI agents (`collective_sync.rs`).  

---

### qublis-sim

- **Simulators**:  
  - `TpsSimulator` (throughput),  
  - `LatencyWave`,  
  - `DimensionViewer`,  
  - `NeuroFluxSimulator`,  
  - `NetworkSimulator`,  
  - `ReportGenerator`.  
- **Configuration & reporting**: TOML config loader, JSON/CSV export.

---

### qublis-deploy

- **CI Fork Launcher**: clones & prepares test forks (`ci_fork_launcher.rs`).  
- **QNetX Node Bootstrap**: configures & launches network nodes (`qnetx_node_bootstrap.rs`).  
- **Deploy metrics** and CLI (`main.rs`, `config.rs`, `metrics.rs`).

---

## Data & Control Flows

```txt
[User CLI] → qublis-deploy → (ci_fork, bootstrap)
         ↓
      qublis-sim → simulate → ReportData
         ↓
      qublis-ci_core → MorphicAI / MoralRegulator / CollectiveSync
         ↓
      qublis-qlink → QID & consent & ethics
         ↓
      qublis-qmesh → entropic DAG consensus
         ↓
      qublis-qnet[x] → P2P transport & routing
         ↓
      qublis-qnum → quantum arithmetic primitives
````

1. **Configuration** loaded via `config.rs` in each crate.
2. **Sim** may feed performance metrics back into **CI-Core** training.
3. **CI-Core** decisions (e.g., consent, ethics) encoded on-chain via **QLink**.
4. **Consensus** finalizes blocks in **QMesh**.
5. **Network** propagates blocks via **QNet/X**.

---

## Consensus & NeuroFlux Optimization

* **Entropic DAG** (QMesh) organizes blocks without a strict chain, reducing forks.
* **NeuroFlux** (within Sim & runtime consensus) dynamically tunes parameters:

  * Block validation thresholds
  * Peer selection weights
  * Latency vs. throughput trade-offs
* **Feedback loop**: on-chain metrics → NeuroFlux RL agent → updated config parameters → redeployed via **Deploy**.

---

## Extensibility & QBLang Integration

* **QBLang** (domain-specific language) can be embedded in smart contracts (`.qblang` files) to add modules at runtime.
* **Plugin points**:

  * New quantum gates in `qublis-qnum`
  * Custom network overlays in `qublis-qnet`
  * Additional AI modules in `ci_core`
  * New consensus rules via NeuroFlux RL hooks in `qmesh`
* **Hot-swap**: services expose gRPC/HTTP endpoints for dynamic reconfiguration.

---

## Security & Ethics

* **Quantum identities** cryptographically bind user keys to QNums.
* **Ethics lattice** ensures on-chain actions respect entangled ethical constraints.
* **Consent records** are immutable, auditable quantum-collapsed decisions.
* **CI-Core moral regulator** vetoes disallowed AI actions before block inclusion.

---

## Deployment Topology

* **Validator Nodes** run `qublis-qnetx-node` (bootstrapped via `qublis-deploy`).
* **Simulation Nodes** run `qublis-sim` for offline performance tuning.
* **CI Builders** use `ci_fork_launcher` to validate new protocol versions in parallel.
* **Monitoring**: Prometheus exporters in each crate (`*_metrics`) feed into a centralized dashboard.

---

*End of architecture.md*
