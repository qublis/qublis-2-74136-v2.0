//! Quantum Digit (Qid) implementation for Qublis v2.0
//!
//! A `Qid` is a quantum “digit” in superposition over the basis |0⟩…|9⟩,
//! with methods for normalization, measurement (collapse), and entropy.

use num_complex::Complex;
use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;

/// A single quantum digit with amplitude vector over 10 basis states.
#[derive(Clone, Debug)]
pub struct Qid {
    /// Complex amplitudes α₀…α₉, normalized so that ∑|αᵢ|² = 1.
    pub amps: [Complex<f64>; 10],
}

impl Qid {
    /// Create a new `Qid` from raw amplitudes, normalizing them.
    pub fn new(mut amps: [Complex<f64>; 10]) -> Self {
        let mut q = Qid { amps };
        q.normalize();
        q
    }

    /// Construct a classical `Qid` collapsed to digit `d` (0 ≤ d < 10).
    pub fn from_classical(d: u8) -> Self {
        let mut amps = [Complex::new(0.0, 0.0); 10];
        if (d as usize) < 10 {
            amps[d as usize] = Complex::new(1.0, 0.0);
        }
        Qid { amps }
    }

    /// The “zero” digit |0⟩.
    pub fn zero() -> Self {
        Qid::from_classical(0)
    }

    /// A random `Qid` with Haar‐like distribution: random amplitudes then normalized.
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut amps = [Complex::new(0.0, 0.0); 10];
        for amp in amps.iter_mut() {
            *amp = Complex::new(rng.gen(), rng.gen());
        }
        Qid::new(amps)
    }

    /// Normalize the amplitude vector so that ∑|αᵢ|² = 1.
    pub fn normalize(&mut self) {
        let norm_sq: f64 = self.amps.iter().map(|c| c.norm_sqr()).sum();
        if norm_sq > 0.0 {
            let inv_norm = 1.0 / norm_sq.sqrt();
            for amp in &mut self.amps {
                *amp *= inv_norm;
            }
        }
    }

    /// Measure (collapse) this `Qid` to a classical digit:
    /// - Samples index `i` with probability |αᵢ|².  
    /// - Collapses `self.amps` so that only that basis has amplitude 1.
    pub fn measure(&mut self) -> u8 {
        // Build probability distribution
        let probs: Vec<f64> = self.amps.iter().map(|c| c.norm_sqr()).collect();
        let dist = WeightedIndex::new(&probs)
            .expect("Probability weights must sum to > 0");
        let mut rng = rand::thread_rng();
        let idx = dist.sample(&mut rng);

        // Collapse
        for (i, amp) in self.amps.iter_mut().enumerate() {
            *amp = if i == idx {
                Complex::new(1.0, 0.0)
            } else {
                Complex::new(0.0, 0.0)
            };
        }
        idx as u8
    }

    /// Compute the Shannon entropy of the probability distribution:
    /// `−∑ pᵢ ln(pᵢ)`, where `pᵢ = |αᵢ|²`.
    pub fn entropy(&self) -> f64 {
        self.amps.iter().fold(0.0, |acc, c| {
            let p = c.norm_sqr();
            if p > 0.0 {
                acc - p * p.ln()
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn classical_qid_measures_correctly() {
        let mut q = Qid::from_classical(7);
        assert_eq!(q.measure(), 7);
        // After collapse, measure again must be 7
        assert_eq!(q.measure(), 7);
    }

    #[test]
    fn random_qid_entropy_positive() {
        let q = Qid::random();
        let e = q.entropy();
        assert!(e >= 0.0);
    }

    #[test]
    fn normalize_preserves_state() {
        let mut amps = [Complex::new(1.0, 0.0); 10];
        let mut q = Qid { amps };
        q.normalize();
        let norm_sq: f64 = q.amps.iter().map(|c| c.norm_sqr()).sum();
        assert!((norm_sq - 1.0).abs() < 1e-12);
    }

    #[test]
    fn measure_superposition_yields_valid() {
        // Superposition of |2⟩ and |5⟩ with equal amplitudes
        let mut amps = [Complex::new(0.0, 0.0); 10];
        amps[2] = Complex::new(1.0 / 2f64.sqrt(), 0.0);
        amps[5] = Complex::new(1.0 / 2f64.sqrt(), 0.0);
        let mut q = Qid::new(amps);
        let d = q.measure();
        assert!(d == 2 || d == 5);
    }
}
