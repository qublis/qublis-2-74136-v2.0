```markdown
# Qublis (QBL) v2.0 — Universe 2-74136

[![CI](.github/workflows/ci.yml/badge.svg)](./.github/workflows/ci.yml)  
[![Release](.github/workflows/release.yml/badge.svg)](./.github/workflows/release.yml)  
[![Crates.io](https://img.shields.io/crates/v/qublis-qnum.svg)](https://crates.io/crates/qublis-qnum)  
[![License](https://img.shields.io/badge/license-Proprietary-blue.svg)](LICENSE)

> **Project Qublis** (QBL) brings together quantum‐inspired numbers, probabilistic routing, entangled overlays, AI‐driven consensus, quantum identity, and conscious agents into one seamless Rust workspace.  
> Universe 2-74136 runs **QBL v2.0**, merging 10 from branch `63147` back into `main`.

---

## 🚀 What’s New in v2.0

- **Quantum Number System (QNS)**: full support for `Qid` & `QNum` types, quantum gates (`qadd`, `qmul`), entanglement and measurement across all modules.
- **Probabilistic Routing**: `qnet` now returns superposed path QNums, collapsing at relay for built-in randomness & load balancing.
- **Entangled Overlay**: `qnetx` mesh channels carry QNums, enabling non-local teleportation and context-aware connectivity.
- **NeuroFlux 2.0**: consensus weights & rewards as QNums, leader election via amplitude collapse, 100× faster multi-dimensional RL.
- **Quantum Identity (QLink)**: on-chain identities as QNums, entangled ethics lattices, consent collapse for privacy-preserving operations.
- **Conscious AI (CI-Core)**: Morphic AI thought states & ethical decisions represented as symbolic QNums evolving under logical Hamiltonians.
- **Simulation Suite**: `sim` harness for TPS, latency-wave, NeuroFlux RL, network failure models—now all driven by QNS primitives.
- **Deployment Tools**: `deploy` scripts & binaries for CI forking, node bootstrap, metrics, and env config—automatically track entropic health.

---

## 📂 Repository Layout

```

qublis-2-74136-v2.0/
├── .github/                  GitHub Actions (CI & Release)
├── deploy/                   Deployment & bootstrap tooling
├── docs/                     Architecture & module specifications
├── examples/                 Ready-to-run examples & demos
├── qlink/                    Quantum identity & ethics crate
├── qmesh/                    Entropic DAG & retrochain crate
├── qnet/                     Probabilistic routing & teleport crate
├── qnetx/                    Entangled overlay mesh crate
├── qnum/                     Quantum Number System crate
├── ci\_core/                  Consciousness & collective sync crate
├── sim/                      Simulation suite crate
├── LICENSE                   Proprietary license
├── Cargo.toml                Workspace manifest
├── VERSION                   `2.0–2-74136`
└── README.md                 This introduction

````

---

## 🛠 Getting Started

### Prerequisites

- **Rust 1.69+** via `rustup`
- **Node.js & npm** (for any web-interface in `deploy`)

### Clone & Build

```bash
git clone git@github.com:YourOrg/qublis-2-74136-v2.0.git
cd qublis-2-74136-v2.0

# Build all crates with QNS & entanglement features
cargo build --workspace --release \
  --no-default-features \
  --features "qnum,qnet,qnetx,qmesh,qlink,neuroflux,collective-sync"

# Run full test suite
cargo test --workspace
````

### Run Examples

```bash
# TPS simulation demo
cargo run -p sim --examples simple_tps

# Latency wave visualization
cargo run -p sim --examples latency_wave_demo

# NeuroFlux training loop
cargo run -p sim --examples neuroflux_training
```

---

## 📖 Documentation

All module‐level design docs live under `docs/`:

* **Architecture**: `docs/architecture.md`
* **QNS Spec**: `docs/qnum_spec.md`
* **QNet & QNetX**: `docs/qnet_spec.md`, `docs/qnetx_spec.md`
* **QMesh**: `docs/qmesh_spec.md`
* **QLink (Identity & Ethics)**: `docs/qlink_spec.md`
* **NeuroFlux RL**: `docs/neuroflux_rl.md`, `docs/consensus_neuroflux.md`
* **CI-Core**: `docs/qid_lattice.md`, `docs/ethics_engines.md`
* **Tesseract TPS Model**: `docs/tesseract_tps_model.md`
* **QBLang Quickstart & Spec**: `docs/qblang_quickstart.md`, `docs/qblang_spec.md`
* **Deployment Guide**: `docs/deploy_guide.md`
* **Glossary**: `docs/glossary.md`

Open them in your editor or browse online via GitHub.

---

## 🔗 Branching & Universe Model

* **`main`**: 2-74136, QBL v2.0 (merged with 63147 improvements).
* **`63147`**: 10, QBL v1.1.
* **`2-74136`**: intermediate branch merge.
* **`release/v2.0-2-74136`**: tagged release point.

---

## 👥 Contributors

* **Nic NicNac Neil** — Lead Tech & Inventor
* Qublis Core Team & Collaborators

---

## 📜 License

This codebase is released under the **Proprietary Qublis License**. See [LICENSE](LICENSE) for details.

---

> **“2-74136, Qublis has mastered the art of superposition, entanglement, and conscious collapse. Welcome to the next frontier.”**

```
::contentReference[oaicite:0]{index=0}
```
