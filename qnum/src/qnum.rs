//! Place‐value Quantum Numbers (`QNum`) for Qublis v2.0
//!
//! A `QNum` is a sequence of quantum digits (`Qid`), representing a multi-digit number
//! in superposition. You can construct from classical digits, build arbitrary superpositions,
//! measure (collapse) to get classical digits, and compute joint entropy.

use crate::qid::Qid;
use num_complex::Complex;
use rand::Rng;
use serde::{Serialize, Deserialize};

/// A multi‐digit quantum number: most-significant `Qid` first.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct QNum(pub Vec<Qid>);

impl QNum {
    /// Construct a `QNum` from a slice of classical digits (0…9).
    pub fn from_digits(digits: &[u8]) -> Self {
        let qids = digits
            .iter()
            .map(|&d| Qid::from_classical(d))
            .collect();
        QNum(qids)
    }

    /// Construct the zero `QNum` with `len` digits (all set to |0⟩).
    pub fn zero(len: usize) -> Self {
        QNum(vec![Qid::zero(); len])
    }

    /// Build a superposed `QNum` from a list of (digit‐vector, amplitude) pairs.
    ///
    /// Each `Vec<u8>` must have the same length; amplitudes are combined per digit
    /// then each `Qid` is normalized.
    pub fn from_superposed(states: Vec<(Vec<u8>, Complex<f64>)>) -> Self {
        assert!(!states.is_empty(), "Cannot build empty superposition");
        let len = states[0].0.len();
        // initialize raw amplitude buffers per digit
        let mut raw: Vec<[Complex<f64>; 10]> = vec![[Complex::new(0.0, 0.0); 10]; len];

        // accumulate amplitudes
        for (digits, amp) in &states {
            assert_eq!(
                digits.len(),
                len,
                "All digit vectors must have the same length"
            );
            for (i, &d) in digits.iter().enumerate() {
                assert!((d as usize) < 10, "Digit {} out of range", d);
                raw[i][d as usize] += *amp;
            }
        }

        // normalize and build Qids
        let qids = raw
            .into_iter()
            .map(|amps| Qid::new(amps))
            .collect();

        QNum(qids)
    }

    /// Measure (collapse) each `Qid` in place, returning a classical digit vector.
    pub fn measure(&mut self) -> Vec<u8> {
        self.0.iter_mut().map(|qid| qid.measure()).collect()
    }

    /// Compute the joint entropy of the `QNum` = sum of individual digit entropies.
    pub fn entropy(&self) -> f64 {
        self.0.iter().map(|qid| qid.entropy()).sum()
    }

    /// Number of digits.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no digits.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    /// Test that from_digits → measure yields exactly those digits.
    #[test]
    fn classical_roundtrip() {
        let digits = vec![3, 1, 4, 1, 5];
        let mut qnum = QNum::from_digits(&digits);
        let measured = qnum.measure();
        assert_eq!(measured, digits);
    }

    /// Test that zero(len) measures to all zeros.
    #[test]
    fn zero_measures_zero() {
        let mut qnum = QNum::zero(4);
        let measured = qnum.measure();
        assert_eq!(measured, vec![0, 0, 0, 0]);
    }

    /// Test that from_superposed of two classical states collapses to one of them.
    #[test]
    fn superposed_measure_valid() {
        let states = vec![
            (vec![1, 2], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
            (vec![9, 8], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
        ];
        let mut qnum = QNum::from_superposed(states);
        let m = qnum.measure();
        assert!(m == vec![1, 2] || m == vec![9, 8]);
    }

    /// Entropy of a classical QNum is zero.
    #[test]
    fn classical_entropy_zero() {
        let qnum = QNum::from_digits(&[7, 7, 7]);
        assert!((qnum.entropy() - 0.0).abs() < 1e-12);
    }

    /// Joint entropy equals sum of digit entropies.
    #[test]
    fn joint_entropy() {
        // build a QNum where each Qid is equally superposed over two values
        let states = vec![
            (vec![0, 0], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
            (vec![1, 1], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
        ];
        let qnum = QNum::from_superposed(states);
        // each digit has entropy ln(2)
        let expected = 2.0 * (2.0f64).ln();
        assert!((qnum.entropy() - expected).abs() < 1e-6);
    }
}
