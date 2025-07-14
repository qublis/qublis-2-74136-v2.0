//! Quantum Number System (QNS) — `qublis-qnum`  
//! ==========================================  
//!  
//! This crate provides the core types and operations for the Quantum Number System  
//! used throughout Qublis v2.0. A **Qid** is a quantum digit (amplitudes over a discrete basis),  
//! and a **QNum** is a multi-digit quantum number. You can combine QNums with unitary gates  
//! (`qadd`, `qmul`), entangle multiple QNums, and measure/collapse them to classical values.  
//!  
//! For a full specification and usage examples, see [`docs/qnum_spec.md`].  

#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// Quantum Digit primitives.
pub mod qid;
/// Place-value Quantum Numbers.
pub mod qnum;
/// Unitary arithmetic gates (addition, multiplication).
pub mod gates;
/// Entanglement utilities for linking QNums.
pub mod entangle;
/// Measurement and collapse operators.
pub mod measure;

pub use qid::Qid;
pub use qnum::QNum;
pub use gates::{qadd, qmul};
pub use entangle::entangle;
pub use measure::{measure, measure_qid};

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    /// Basic sanity check: a Qid created from a classical 3 measures to 3.
    #[test]
    fn qid_classical_measure() {
        let mut q = Qid::from_classical(3);
        let d = q.measure();
        assert_eq!(d, 3);
    }

    /// QAdd of two classical QNums yields the expected classical sum.
    #[test]
    fn qadd_classical_sum() {
        let a = QNum::from_digits(&[1, 2]); // 12
        let b = QNum::from_digits(&[0, 7]); //  7
        let mut sum = qadd(&a, &b);
        assert_eq!(sum.measure(), vec![1, 9]); // 19
    }

    /// Entangling two QNums yields correlated collapse.
    #[test]
    fn entangle_correlates() {
        let mut x = QNum::from_digits(&[4]);
        let mut y = QNum::from_digits(&[7]);
        entangle(&mut x, &mut y);
        let dx = x.measure();
        let dy = y.measure();
        // After entanglement, either (4,7) or (7,4)
        let valid = dx == vec![4] && dy == vec![7] || dx == vec![7] && dy == vec![4];
        assert!(valid, "Entanglement did not correlate as expected");
    }

    /// Measurement of a superposed Qid yields a valid basis element.
    #[test]
    fn qid_measure_valid() {
        // equal superposition of |2⟩ and |5⟩
        let amps = {
            let mut a = [Complex::new(0.0,0.0); 10];
            a[2] = Complex::new(1.0/2f64.sqrt(), 0.0);
            a[5] = Complex::new(1.0/2f64.sqrt(), 0.0);
            a
        };
        let mut q = Qid::new(amps);
        let d = q.measure();
        assert!((d == 2) || (d == 5));
    }
}
