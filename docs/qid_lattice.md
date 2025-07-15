````markdown
# QID Lattice v2.0 Specification

This document defines the **Quantum Identity (QID) Lattice** for **Qublis v2.0** (2-74136). The QID Lattice provides a structured, entangled hierarchy of on-chain identities, enabling fine-grained permissioning, group membership, and inter-identity relationships.

---

## Table of Contents

1. [Overview](#overview)  
2. [Core Concepts](#core-concepts)  
   - [Quantum Identity (QID)](#quantum-identity-qid)  
   - [Lattice Structure](#lattice-structure)  
   - [Entanglement Relations](#entanglement-relations)  
3. [Lattice Construction](#lattice-construction)  
4. [Data Structures & Formats](#data-structures--formats)  
5. [Rust API](#rust-api)  
6. [Configuration](#configuration)  
7. [CLI Usage](#cli-usage)  
8. [Examples](#examples)  
9. [Extensibility](#extensibility)  

---

## Overview

The QID Lattice organizes quantum identities (QNums) into a **partial order** where each node represents a registered identity or group of identities. Edges encode **entanglement-based** inclusion: a child QID is entangled with its parent QID, inheriting rights and context.

Use cases:

- **Role hierarchies** (e.g. User → Moderator → Admin)  
- **Group membership** (e.g. Department teams, consortiums)  
- **Delegated authority** via entanglement weights  

---

## Core Concepts

### Quantum Identity (QID)

A **QID** is a `QNum` representing a unique on-chain identity. It is derived via:

```rust
let qid: QNum = QidLayer::generate_qid(seed);
````

### Lattice Structure

A **lattice** is a directed acyclic graph (DAG) with join (least upper bound) and meet (greatest lower bound) operations defined for any two QIDs:

* **Join** (`⊔`): smallest QID that entangles both operands.
* **Meet** (`⊓`): largest QID common to both operands’ ancestry.

### Entanglement Relations

Each parent-child edge carries an **entanglement weight** `w ∈ (0,1]`, indicating the strength of inheritance:

* `w = 1.0`: full inheritance.
* `w < 1.0`: partial or conditional inheritance.

---

## Lattice Construction

1. **Register identities** as lattice nodes.
2. **Define entanglement** between QIDs:

   ```rust
   lattice.entangle(qid_parent, qid_child, weight);
   ```
3. **Compute join/meet** on demand:

   ```rust
   let j = lattice.join(&a, &b)?;
   let m = lattice.meet(&a, &b)?;
   ```

---

## Data Structures & Formats

```rust
/// Single entanglement relation
struct Entanglement {
    parent: QNum,
    child: QNum,
    weight: f64,      // 0 < weight ≤ 1
}

/// The QID lattice
struct QidLattice {
    nodes: HashSet<QNum>,
    edges: Vec<Entanglement>,
}
```

Serialization (off-chain): TOML or JSON
Serialization (on-chain): CBOR

Example TOML snippet:

```toml
[[entanglement]]
parent = "12345678"
child  = "87654321"
weight = 0.75
```

---

## Rust API

```rust
use qublis_qlink::QidLattice;
use qublis_qnum::QNum;

// Load from file
let mut lattice = QidLattice::load("qid_lattice.toml")?;

// Register a new identity
lattice.add_node(new_qid.clone())?;

// Entangle child under parent
lattice.entangle(&parent_qid, &child_qid, 0.8)?;

// Compute join & meet
let lub = lattice.join(&a_qid, &b_qid)?;
let glb = lattice.meet(&a_qid, &b_qid)?;
```

Primary methods:

* `add_node(&mut self, qid: QNum)`
* `entangle(&mut self, parent: &QNum, child: &QNum, weight: f64)`
* `join(&self, a: &QNum, b: &QNum) -> Result<QNum, LatticeError>`
* `meet(&self, a: &QNum, b: &QNum) -> Result<QNum, LatticeError>`
* `save(&self, path: &str) -> Result<(), LatticeError>`

---

## Configuration

```toml
# qid_lattice.toml
nodes = [ "12345678", "87654321", "11223344" ]

[[entanglement]]
parent = "12345678"
child  = "87654321"
weight = 1.0

[[entanglement]]
parent = "12345678"
child  = "11223344"
weight = 0.5
```

---

## CLI Usage

```bash
# Initialize a new lattice
qublis-qid-lattice init --file qid_lattice.toml

# Add a node
qublis-qid-lattice add-node --qid 33445566

# Entangle two nodes
qublis-qid-lattice entangle \
  --parent 12345678 \
  --child 33445566 \
  --weight 0.9

# Compute join/meet
qublis-qid-lattice join --a 87654321 --b 33445566
qublis-qid-lattice meet --a 87654321 --b 11223344
```

---

## Examples

1. **Role Hierarchy**

   ```toml
   [[entanglement]]
   parent = "ROLE_USER"
   child  = "ROLE_MODERATOR"
   weight = 1.0

   [[entanglement]]
   parent = "ROLE_MODERATOR"
   child  = "ROLE_ADMIN"
   weight = 1.0
   ```

2. **Group Membership**

   ```toml
   [[entanglement]]
   parent = "GROUP_DEV"
   child  = "USER_ALICE"
   weight = 0.8
   ```

---

## Extensibility

* **Custom Lattice Metrics**: implement new `Similarity` or `Distance` traits.
* **Dynamic Updates**: load additional entanglements at runtime via RPC.
* **Integration**: tie into **MoralRegulator** to enforce permissions based on lattice position.
* **Visualization**: export lattice to Graphviz for audit and inspection.

---

```
::contentReference[oaicite:0]{index=0}
```
