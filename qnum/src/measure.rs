// qublis-2-74136-v2.0/qnum/src/measure.rs

//! Quantum measurement utilities for Qid and QNum.
//!
//! Exposes two functions:
//! - `measure_qid(&Qid) -> usize`
//! - `measure(&[Qid]) -> Vec<usize>`

use crate::qid::Qid;

/// Measure (collapse) a single `Qid` into one of its basis digits 0–9.
pub fn measure_qid(qid: &Qid) -> usize {
    // `Qid::measure(&self)` returns a usize in 0..10
    qid.measure()
}

/// Measure (collapse) a slice of `Qid`s, returning each outcome.
pub fn measure(qids: &[Qid]) -> Vec<usize> {
    qids.iter()
        .map(|qid| qid.measure())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qid::Qid;

    #[test]
    fn measure_qid_definite() {
        let q = Qid::definite(4);
        assert_eq!(measure_qid(&q), 4);
    }

    #[test]
    fn measure_slice_definite() {
        let qs = vec![Qid::definite(1), Qid::definite(7), Qid::definite(0)];
        assert_eq!(measure(&qs), vec![1, 7, 0]);
    }
}
