````markdown
# Quantum Number System (QNS) Specification  
**Version:** 2.0–2-74136  
**Module:** `qublis-qnum`  

---

## 1. Overview

The Quantum Number System (QNS) is the foundation for representing numbers, symbolic logic, and decision-making in Qublis v2.0. Instead of classical scalars, QNS uses _quantum digits_ (`Qid`) and _quantum numbers_ (`QNum`), which live in superposition, undergo unitary transformations (quantum gates), can become entangled, and collapse to classical values upon measurement.

- **Qid**: a single “quantum digit” with amplitude vector over basis states \|0⟩…\|9⟩ (or arbitrary symbolic bases).  
- **QNum**: a tensor product of multiple `Qid`s, representing multi-digit numbers or composite logic states.  
- **Gates**: Unitary operations such as `qadd`, `qmul`, custom entangling transforms.  
- **Measurement**: Collapse operators that extract classical results with correct Born-rule probabilities.  

---

## 2. Installation

Add `qublis-qnum` to your workspace `Cargo.toml`:

```toml
[workspace.dependencies]
qublis-qnum = { path = "../qnum", version = "2.0.0" }
````

Then in your crate:

```rust
use qublis_qnum::{Qid, QNum, qadd, qmul, entangle, measure};
```

---

## 3. Core Types

### 3.1 Qid (Quantum Digit)

A `Qid` holds a normalized complex amplitude vector of length 10 by default:

```rust
/// A single quantum digit over basis |0⟩…|9⟩.
pub struct Qid {
    /// amplitudes α₀…α₉, must satisfy ∑|αᵢ|² = 1.0
    pub amps: [Complex<f64>; 10],
}
```

#### Constructors

* `Qid::zero() -> Qid`
  All amplitude on |0⟩.
* `Qid::from_classical(d: u8) -> Qid`
  Unit amplitude at basis |d⟩.
* `Qid::random() -> Qid`
  Random Haar-distributed state over 10 levels.

#### Methods

* `fn normalize(&mut self)`
  Enforce ∑|αᵢ|² = 1.
* `fn measure(&mut self) -> u8`
  Collapse the Qid to a classical digit, mutating `self.amps` to the measured basis.
* `fn entropy(&self) -> f64`
  Shannon entropy of the probability distribution |αᵢ|².

### 3.2 QNum (Quantum Number)

A `QNum` is a sequence of `Qid`s (e.g. decimal places), MSB first:

```rust
pub struct QNum(pub Vec<Qid>);
```

#### Constructors

* `QNum::from_digits(digits: &[u8]) -> QNum`
  Embeds each classical digit as a separate `Qid`.
* `QNum::zero(len: usize) -> QNum`
  A multi-digit zero (`len` copies of `Qid::zero()`).
* `QNum::superposed(states: Vec<(Vec<u8>, Complex<f64>)>) -> QNum`
  Build a superposition of integer values, each with amplitude.

#### Methods

* `fn measure(&mut self) -> Vec<u8>`
  Measure every digit Qid, yielding a classical digit vector.
* `fn entropy(&self) -> f64`
  Joint entropy (sum of individual digit entropies).

---

## 4. Quantum Gates

### 4.1 Addition: `qadd`

```rust
/// Unitarly add two QNums (place-value addition with carries).
/// Returns a new QNum in superposition of all possible sums.
pub fn qadd(a: &QNum, b: &QNum) -> QNum
```

Internally applies a reversible adder circuit in amplitude space, preserving unitarity.

### 4.2 Multiplication: `qmul`

```rust
/// Unitarly multiply two QNums, returning their product as a QNum.
/// Uses entangling carries and sum registers.
pub fn qmul(a: &QNum, b: &QNum) -> QNum
```

Supports full tensor entanglement for multi-digit multiplication.

---

## 5. Entanglement & Fusion

### 5.1 Entangle Two QNums

```rust
/// Entangle two QNums in a Bell-like state.
/// After applying, collapse of one will affect the other.
pub fn entangle(x: &mut QNum, y: &mut QNum)
```

Use for linking identity states, validator weights, or symbolic behaviors.

---

## 6. Measurement & Collapse

### 6.1 Partial & Full Measurement

```rust
/// Collapse a single digit in-place, returning its classical value.
pub fn measure_qid(q: &mut Qid) -> u8;

/// Collapse an entire QNum to classical digits.
pub fn measure(q: &mut QNum) -> Vec<u8>;
```

Measurement obeys the Born rule: probability of outcome `i` is |αᵢ|².

---

## 7. Examples

### 7.1 Creating & Measuring a Qid

```rust
use qublis_qnum::Qid;
use num_complex::Complex;

// create a superposed Qid: 50% |2⟩, 50% |7⟩
let mut q = Qid::new([
    Complex::new(0.0,0.0), // |0⟩
    /*…*/,
    Complex::new(1.0/√2.0, 0.0), // |2⟩
    /*…*/,
    Complex::new(1.0/√2.0, 0.0), // |7⟩
    /* fill zeros else */
]);

assert!((q.entropy() - 1.0).abs() < 1e-6);
let digit = q.measure(); // either 2 or 7, each 50%
```

### 7.2 Adding Two QNums

```rust
use qublis_qnum::{QNum, qadd};

let a = QNum::from_digits(&[1, 3]);  // classical 13
let b = QNum::from_digits(&[0, 9]);  // classical  9
let mut sum = qadd(&a, &b);          // superposition of {22}
let classical = sum.measure();       // always [2,2]
```

### 7.3 Entangled Identity

```rust
use qublis_qnum::{QNum, entangle};

let mut id1 = QNum::from_digits(&[4, 2]);
let mut id2 = QNum::from_digits(&[7, 9]);
entangle(&mut id1, &mut id2);

// measuring id1 collapses id2 consistently
let c1 = id1.measure();
let c2 = id2.measure();
assert!( (c1 == vec![4,2] && c2 == vec![7,9]) || (c1 == vec![7,9] && c2 == vec![4,2]) );
```

---

## 8. Performance & Scaling

* **Amortized cost**: Gate operations are \\(O(n·d·log d)) for `n` digits and `d` basis size (default 10).
* **Parallelism**: Internally uses SIMD for FFT-like transforms on amplitude vectors.
* **Entanglement depth**: Controlled via Rust features `--features entangle`, enabling multi-QNum routines.

---

## 9. Next Steps

* Integrate `qublis-qnum` with **QNet** to produce superposed routing decisions.
* Use QNS in **NeuroFlux** to encode validator lottery in amplitude space.
* Extend basis beyond decimal: support symbolic logic basis |AND⟩, |OR⟩, |XOR⟩ for CI-Core.

For full API, see the inline rustdoc in `qnum/src/`.

---

**Authors:** Nic NicNac Neil & Qublis Core Team
**© 2-74136 Qublis v2.0**

```
::contentReference[oaicite:0]{index=0}
```
