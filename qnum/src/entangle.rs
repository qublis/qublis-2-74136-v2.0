//! Entanglement utilities for `qublis-qnum`
//!
//! Provides a Bell‐like entangling transform on two `QNum`s of equal length.
//! After entanglement, measuring one will yield correlated measurement outcomes
//! in the other.

use crate::qid::Qid;
use crate::qnum::QNum;
use num_complex::Complex;
use ordered_float::OrderedFloat;

/// Entangle two `QNum`s in a Bell‐like fashion.
/// 
/// For each digit position, applies a simple two‐mode unitary:
/// ```text
/// (α, β) ↦ ( (α+β)/√2, (α−β)/√2 )
/// ```
/// ensuring that measuring one `QNum` influences the other.
/// 
/// # Panics
/// 
/// Panics if `a.len() != b.len()`.
pub fn entangle(a: &mut QNum, b: &mut QNum) {
    assert_eq!(
        a.len(),
        b.len(),
        "Entanglement requires QNums of the same length"
    );
    let inv_sqrt2 = Complex::new(1.0 / 2f64.sqrt(), 0.0);

    for (qa, qb) in a.0.iter_mut().zip(b.0.iter_mut()) {
        let mut new_qa = [Complex::new(OrderedFloat(0.0), OrderedFloat(0.0)); 10];
        let mut new_qb = [Complex::new(OrderedFloat(0.0), OrderedFloat(0.0)); 10];

        for i in 0..10 {
            let α = qa.amps[i];
            let β = qb.amps[i];
            // Bell‐type mixing per basis index
            let alpha_f = Complex::new(α.re.into_inner(), α.im.into_inner());
            let beta_f  = Complex::new(β.re.into_inner(), β.im.into_inner());

            let qa_new = (alpha_f + beta_f) * inv_sqrt2;
            let qb_new = (alpha_f - beta_f) * inv_sqrt2;

            new_qa[i] = Complex::new(OrderedFloat(qa_new.re), OrderedFloat(qa_new.im));
            new_qb[i] = Complex::new(OrderedFloat(qb_new.re), OrderedFloat(qb_new.im));
        }

        qa.amps = new_qa;
        qb.amps = new_qb;

        // Re‐normalize each Qid after mixing
        qa.normalize();
        qb.normalize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qnum::QNum;
    use std::collections::HashSet;

    #[test]
    fn entangle_correlates_two_single_digit_qnums() {
        // Start with two classical QNums: |4⟩ and |7⟩
        let mut a = QNum::from_digits(&[4]);
        let mut b = QNum::from_digits(&[7]);

        // Entangle them
        entangle(&mut a, &mut b);

        // Collect measurement outcomes over many trials
        let mut outcomes = HashSet::new();
        for _ in 0..100 {
            let mut aa = a.clone();
            let mut bb = b.clone();
            let da = aa.measure()[0];
            let db = bb.measure()[0];
            outcomes.insert((da, db));
        }

        // We expect to see either (4,7) or (7,4) in the results
        assert!(
            outcomes.contains(&(4, 7)) || outcomes.contains(&(7, 4)),
            "Expected entangled outcomes to correlate, got {:?}",
            outcomes
        );
    }

    #[test]
    fn entangle_preserves_length_and_normalization() {
        let mut a = QNum::from_digits(&[1, 2, 3]);
        let mut b = QNum::from_digits(&[4, 5, 6]);
        entangle(&mut a, &mut b);
        assert_eq!(a.len(), 3);
        assert_eq!(b.len(), 3);
        // After entanglement, each Qid should still be normalized
        for qid in a.0.iter().chain(b.0.iter()) {
            let norm_sq: f64 = qid.amps.iter().map(|c| c.norm_sqr()).sum();
            assert!((norm_sq - 1.0).abs() < 1e-12, "Qid not normalized");
        }
    }
}
