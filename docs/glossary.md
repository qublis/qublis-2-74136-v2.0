# Qublis v2.0 Glossary

A concise reference of key terms, acronyms, and concepts used throughout **Qublis v2.0** (2-74136).

---

### A

- **ACL**  
  *Access Control List.* A list of permissions attached to an identity or resource.

---

### C

- **CI-Core**  
  The “Conscious Intelligence Core” suite of modules (`morphic_ai`, `moral_regulator`, `collective_sync`) that embed AI-driven decision-making into the network.

- **Consensus**  
  The process by which validators agree on the order and content of blocks (in Qublis, implemented via the Entropic DAG in QMesh).

---

### D

- **DAG**  
  *Directed Acyclic Graph.* Data structure used by QMesh to represent blocks and their parent relationships without a strict chain.

- **Decay**  
  In cognitive entropy, the gradual reduction in uncertainty over time as blocks are confirmed.

---

### E

- **Entanglement**  
  A quantum-inspired linkage between two QNums or QIDs, causing their state or behavior to become correlated.

- **Entropic DAG**  
  The QMesh consensus structure where each block has an associated entropy score, guiding tip selection and finality.

- **Ethics Lattice**  
  A graph of entangled ethical principles managed by QLink, collapsed into enforcement weights.

---

### F

- **Fork Rate**  
  The fraction of blocks not referenced by subsequent tips; monitored by NeuroFlux to adjust consensus parameters.

---

### G

- **Gossip**  
  A peer-to-peer message propagation protocol used by QNet and QNetX to relay blocks and state updates.

---

### I

- **I/O**  
  *Input/Output.* Refers to reading/writing from disk, network, or other external systems.

---

### L

- **Latency**  
  The time delay between sending and receiving a message or block; modeled by `LatencyWave`.

- **Ledger**  
  The record of all finalized blocks and transactions; in Qublis, abstracted by the Entropic DAG.

---

### M

- **Mesh**  
  The overlay network topology used by QNetX (QuantumMeshOverlay) for high-throughput entropic DAG propagation.

- **MorphicAI**  
  Module within CI-Core that implements the classical 4D reinforcement-learning agent.

- **Metrics**  
  Quantitative measures (counters, gauges) exported by each crate for monitoring (e.g., TPS, latency, blockchain events).

---

### N

- **NeuroFlux**  
  The quantum-enhanced, AI-driven optimization process for tuning network and consensus parameters in real time.

- **NFT**  
  *Non-Fungible Token.* Not directly used in core Qublis, but may be expressed via QNums in smart contracts.

- **Node**  
  A running instance of the Qublis software (validator or full node), participating in consensus and networking.

---

### P

- **Peer**  
  Another node in the P2P network to which a node connects for message exchange.

- **PolicyUpdate**  
  A QLink mutation record that captures dynamic changes to ethical or operational parameters.

---

### Q

- **QBLang**  
  The domain-specific smart-contract language for writing on-chain logic, with first-class quantum types and self-modification.

- **QID**  
  *Quantum ID.* A fixed-length QNum used as a unique on-chain identifier for users, validators, or contracts.

- **QLink**  
  The on-chain module managing QIDs, ethics lattice, consent records, and policy mutations.

- **QMesh**  
  The entropic DAG consensus engine replacing linear blockchains with high-throughput, low-latency DAGs.

- **QNet**  
  The base peer-to-peer networking layer providing routing, relaying, and teleportation primitives.

- **QNetX**  
  The extended overlay on QNet, adding entropic DAG overlays, zero-propagation, state condensation, and anomaly filtering.

- **QNum**  
  A quantum-inspired numeric type representing superposed “quantum digits” (Qids) and supporting entanglement and measurement.

---

### R

- **RetroChain Tracker**  
  Component of QMesh that integrates late-arriving blocks into the DAG, enabling time-symmetric reorganization.

- **Reward**  
  The numerical feedback signal used by NeuroFlux to learn optimal parameter adjustments.

---

### S

- **Self-Modifying Contracts**  
  QBLang feature allowing contracts to deploy or update other contracts at runtime via `eval`.

- **Sim (Simulation Suite)**  
  Collection of offline tools (`TpsSimulator`, `LatencyWave`, `DimensionViewer`, `NeuroFluxSimulator`, `NetworkSimulator`, `ReportGenerator`) used to model network performance.

- **StateCondenser**  
  QNetX component that summarizes large DAGs into compact QNum fingerprints for lightweight synchronization.

---

### T

- **TPS**  
  *Transactions Per Second.* A measure of throughput; modeled and optimized by NeuroFlux and the Tesseract TPS model.

- **Teleportation**  
  A QNet primitive for expedited, non-local message forwarding leveraging quantum-inspired routing.

- **Tip Selection**  
  In QMesh, the algorithm for choosing which DAG tips to reference when producing a new block, guided by entropy.

---

### U

- **Uptime**  
  The fraction of time a validator or node remains online and responsive; used in NeuroFlux state.

---

### V

- **Validator**  
  A node authorized to produce blocks and participate in consensus; its weight may be adjusted by NeuroFlux.

---

### W

- **Weights**  
  Numerical coefficients (e.g., in ethics lattice or reward function) used to bias decisions or enforce policies.

---

### Z

- **Zero-Propagation**  
  QNetX protocol for rapid network-wide convergence by broadcasting a “zero QNum” heartbeat.

---

*End of glossary.md*```
::contentReference[oaicite:0]{index=0}
