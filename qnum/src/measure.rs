//! Measurement and collapse operators for QNS (`qublis-qnum`)
//!
//! Provides functions to measure (collapse) a single `Qid` or an entire `QNum`
//! into classical values according to the Born rule.

use crate::qid::Qid;
use crate::qnum::QNum;

/// Measure (collapse) a single quantum digit (`Qid`) in place,
/// returning the classical digit (0…9).
///
/// After calling this, `qid.amps` will be reset so that only the
/// measured basis has amplitude 1.
pub fn measure_qid(qid: &mut Qid) -> u8 {
    qid.measure()
}

/// Measure (collapse) an entire quantum number (`QNum`) in place,
/// returning a `Vec<u8>` of classical digits (MSB first).
///
/// Each digit’s superposition is collapsed independently.
pub fn measure(qnum: &mut QNum) -> Vec<u8> {
    qnum.0.iter_mut()
        .map(|qid| qid.measure())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn test_measure_qid_classical() {
        let mut q = Qid::from_classical(7);
        assert_eq!(measure_qid(&mut q), 7);
        // After collapse, measuring again must yield the same
        assert_eq!(measure_qid(&mut q), 7);
    }

    #[test]
    fn test_measure_qid_superposed() {
        // 50/50 superposition of |2⟩ and |5⟩
        let mut amps = [Complex::new(0.0,0.0); 10];
        amps[2] = Complex::new(1.0 / 2f64.sqrt(), 0.0);
        amps[5] = Complex::new(1.0 / 2f64.sqrt(), 0.0);
        let mut q = Qid::new(amps);
        let d = measure_qid(&mut q);
        assert!(d == 2 || d == 5);
    }

    #[test]
    fn test_measure_qnum_classical() {
        let mut qnum = QNum::from_digits(&[3, 1, 4]);
        assert_eq!(measure(&mut qnum), vec![3, 1, 4]);
    }

    #[test]
    fn test_measure_qnum_superposed() {
        let states = vec![
            (vec![1, 2], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
            (vec![9, 8], Complex::new(1.0 / 2f64.sqrt(), 0.0)),
        ];
        let mut qnum = QNum::from_superposed(states);
        let result = measure(&mut qnum);
        assert!(
            result == vec![1, 2] || result == vec![9, 8],
            "Collapsed to unexpected digits: {:?}",
            result
        );
    }
}
