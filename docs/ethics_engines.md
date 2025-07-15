````markdown
# Ethics Engines v2.0

This document specifies the **Ethics Engines** in **Qublis v2.0** (2-74136), covering the core on-chain and off-chain modules that define, evaluate, and enforce ethical principles and user consent within the Qublis ecosystem.

---

## Table of Contents

1. [Overview](#overview)  
2. [Core Modules](#core-modules)  
   - [Ethics Lattice](#ethics-lattice)  
   - [Moral Regulator](#moral-regulator)  
   - [Conscious Consent Engine](#conscious-consent-engine)  
   - [Mutation Engine](#mutation-engine)  
3. [Data Structures & Formats](#data-structures---formats)  
4. [Rust API Usage](#rust-api-usage)  
5. [Configuration (TOML)](#configuration-toml)  
6. [CLI Usage](#cli-usage)  
7. [Metrics & Monitoring](#metrics---monitoring)  
8. [Examples](#examples)  
9. [Extensibility](#extensibility)  

---

## Overview

The Ethics Engines provide a unified framework to:

- **Define** ethical principles as entangled quantum numbers (QNums).  
- **Evaluate** agent actions against a lattice of principles.  
- **Record** probabilistic user consent on terms and policies.  
- **Mutate** policies dynamically via on-chain updates.

They ensure that all AI outputs, smart-contract decisions, and protocol parameters conform to community-defined moral constraints.

---

## Core Modules

### Ethics Lattice

- **Responsibilities**  
  - Store a set of named principles, each with an initial QNum state.  
  - Entangle pairs of principles to capture inter-dependencies.  
  - Collapse the lattice to a set of enforcement weights (0–9).

- **Key Methods**  
  ```rust
  EthicsLattice::new(&config)
      .add_principle(name: String, initial: QNum)
      .entangle_principles(a: &str, b: &str)
      .evaluate() -> HashMap<String, u8>
````

---

### Moral Regulator

* **Responsibilities**

  * Take a proposed `MotorOutput` (or other action) and vet it against the current lattice.
  * Veto (return `Err`) if any principle collapses to weight 0.
  * Emit Prometheus-style metrics on allowed/violated actions.

* **Key Methods**

  ```rust
  MoralRegulator::new(&config)
      .add_principle(...)
      .entangle_principles(...)
      .enforce(output: MotorOutput) -> Result<MotorOutput, CiCoreError>
  ```

---

### Conscious Consent Engine

* **Responsibilities**

  * Present registration or terms requests to users (off-chain or via UI).
  * Capture consent as a `QNum`-encoded record, with probabilistic grant/deny.
  * Store immutable consent logs for audit and on-chain anchoring.

* **Key Methods**

  ```rust
  ConsciousConsent::new(&config)
      .request_consent(qid: &QNum, terms: &str, timestamp: u64) -> ConsentRecord
      .revoke_consent(qid: &QNum) -> bool
  ```

---

### Mutation Engine

* **Responsibilities**

  * Allow dynamic updates to principles, consent rules, or ethical weights via entangled policy records.
  * Apply batched policy updates on‐chain in a single transaction.
  * Ensure new policies are entangled with existing lattice nodes to preserve consistency.

* **Key Methods**

  ```rust
  MutationEngine::new(&config)
      .record_update(qid: &QNum, policy: PolicyUpdate)
      .apply_updates(lattice: &mut EthicsLattice)
  ```

---

## Data Structures & Formats

```rust
/// Single ethical principle entry
struct Principle {
    name: String,
    state: QNum,
}

/// Entanglement between two principles
struct Entanglement {
    a: String,
    b: String,
}

/// User consent record
struct ConsentRecord {
    qid: QNum,
    terms: String,
    granted: bool,
    timestamp: u64,
}

/// Policy update
struct PolicyUpdate {
    id: String,
    parameters: QNum,
    timestamp: u64,
}
```

* **Off-chain config**: TOML
* **On-chain payloads**: CBOR or JSON

---

## Rust API Usage

```rust
use qublis_qlink::{
    EthicsLattice, MoralRegulator,
    ConsciousConsent, MutationEngine,
};
use qublis_qnum::QNum;

// Load config
let cfg = QLinkConfig::load("qlink.toml")?;

// Build the lattice
let mut lattice = EthicsLattice::new(&cfg);
lattice.add_principle("fairness".into(), QNum::from_digits(&[5]))?;
lattice.add_principle("safety".into(), QNum::from_digits(&[7]))?;
lattice.entangle_principles("fairness", "safety")?;

// Enforce an AI action
let mut regulator = MoralRegulator::new(&cfg);
let output = MotorOutput { signals: vec![1,2,3] };
let result = regulator.enforce(output);

// Record consent
let mut consent = ConsciousConsent::new(&cfg);
let rec = consent.request_consent(&user_qid, "Terms v2", now_ts);

// Mutate policies
let mut mutator = MutationEngine::new(&cfg);
mutator.record_update(&admin_qid, PolicyUpdate {
    id: "update_timeout".into(),
    parameters: QNum::from_digits(&[2]),
    timestamp: now_ts,
});
```

---

## Configuration (TOML)

```toml
# qlink.toml

# Ethics Lattice
principles = [
  { name = "fairness", state = [5] },
  { name = "safety",    state = [7] }
]
entanglements = [
  { a = "fairness", b = "safety" }
]

# Moral Regulator
enable_metrics = true

# Consent Engine
consent_probability = 0.8

# Mutation Engine
policy_store = "onchain"
```

---

## CLI Usage

```bash
# Add a principle
qublis-qlink-cli principle add --name fairness --initial 5

# Entangle two principles
qublis-qlink-cli principle entangle --a fairness --b safety

# Enforce an action (for testing)
qublis-qlink-cli regulator enforce --input action.json

# Request user consent
qublis-qlink-cli consent request --qid 12345678 --terms "Privacy v2"

# Apply a policy update
qublis-qlink-cli policy update --id timeout --params [2] --timestamp 1700000000
```

---

## Metrics & Monitoring

All modules emit Prometheus metrics on port **9400**:

* `qlink_principles_added_total`
* `qlink_principles_entangled_total`
* `ci_core_actions_allowed_total`
* `ci_core_actions_violated_total`
* `qlink_consents_requested_total`
* `qlink_consents_granted_total`
* `qlink_policy_updates_total`

---

## Examples

1. **Vetoing an AI action**

   ```bash
   qublis-qlink-cli regulator enforce --input bad_action.json
   # returns veto if any principle weight == 0
   ```

2. **Recording consent off-chain**

   ```bash
   qublis-qlink-cli consent request --qid 87654321 --terms "DataSharing v3"
   ```

---

## Extensibility

* **New Principle Types**: implement custom QNum initialization logic.
* **Custom Consent Flows**: extend `ConsciousConsent` with multi-party consent rules.
* **Policy Mutation Hooks**: subscribe off-chain services to `PolicyUpdate` events via gRPC.
* **Integration**: plug the MoralRegulator into the consensus loop (`consensus_neuroflux.rs`) for on-chain enforcement.

---

```
::contentReference[oaicite:0]{index=0}
```
