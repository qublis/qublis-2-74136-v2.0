//! Quantum measurement utilities for Qid and QNum.
//!
//! Provides functions to collapse Qid superpositions into classical outcomes.

use crate::qid::Qid;

/// Measure a slice of Qids, returning each collapse outcome 0..9.
///
/// Iterates over immutable references so that the `measure(&self)`
/// method can be called directly, regardless of whether you pass `&[Qid]` or `&mut [Qid]`.
pub fn measure_states(qids: &[Qid]) -> Vec<usize> {
    qids.iter()
        .map(|qid| qid.measure())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qid::Qid;

    #[test]
    fn measure_states_definite() {
        // Definite Qids collapse deterministically
        let data = [
            Qid::definite(2),
            Qid::definite(5),
            Qid::definite(9),
        ];
        let results = measure_states(&data);
        assert_eq!(results, vec![2, 5, 9]);
    }
}
