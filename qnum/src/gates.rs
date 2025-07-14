//! Unitary Arithmetic Gates for QNS (`qublis-qnum`)
//!
//! Provides quantum addition (`qadd`) and multiplication (`qmul`) operations
//! on `QNum` values, implemented as reversible, amplitude‐preserving transforms
//! via enumeration of basis states and linear combination.
//!
//! Note: These implementations are illustrative; in a real quantum runtime
//! you would implement dedicated reversible circuits rather than classical
//! enumeration.

use crate::{qnum::QNum, qid::Qid};
use num_complex::Complex;
use std::cmp;
use std::collections::HashMap;

/// Quantum addition: unitary superposition of all possible sums of `a + b`.
pub fn qadd(a: &QNum, b: &QNum) -> QNum {
    // Determine output length: one extra digit for possible final carry
    let out_len = cmp::max(a.len(), b.len()) + 1;

    // Enumerate all classical states of a and b with their amplitudes
    let a_states = enumerate_states(a, out_len);
    let b_states = enumerate_states(b, out_len);

    // Accumulate amplitude for each sum result
    let mut sums: HashMap<Vec<u8>, Complex<f64>> = HashMap::new();

    for (adigits, a_amp) in &a_states {
        for (bdigits, b_amp) in &b_states {
            // Classical addition with carry
            let mut result = vec![0u8; out_len];
            let mut carry = 0;
            for i in (0..out_len).rev() {
                let ai = adigits[i] as usize;
                let bi = bdigits[i] as usize;
                let s = ai + bi + carry;
                result[i] = (s % 10) as u8;
                carry = s / 10;
            }
            // If desired, drop leading zero of carry at index 0:
            // but we keep fixed length for QNum::from_superposed.

            // Combined amplitude
            let amp = *a_amp * *b_amp;
            *sums.entry(result).or_insert(Complex::new(0.0, 0.0)) += amp;
        }
    }

    // Build the resulting QNum superposition
    let states: Vec<(Vec<u8>, Complex<f64>)> =
        sums.into_iter().collect();
    QNum::from_superposed(states)
}

/// Quantum multiplication: unitary superposition of all possible products `a * b`.
pub fn qmul(a: &QNum, b: &QNum) -> QNum {
    // Output length = sum of input lengths
    let out_len = a.len() + b.len();

    let a_states = enumerate_states(a, out_len);
    let b_states = enumerate_states(b, out_len);

    let mut prods: HashMap<Vec<u8>, Complex<f64>> = HashMap::new();

    for (adigits, a_amp) in &a_states {
        for (bdigits, b_amp) in &b_states {
            // Classical multiplication
            let mut result = vec![0u8; out_len];
            // Convert to usize for intermediate math
            let mut tmp = vec![0usize; out_len];
            for (i, &ad) in adigits.iter().enumerate() {
                for (j, &bd) in bdigits.iter().enumerate() {
                    // positions i and j from MSB; convert to indices from LSB:
                    let pos = out_len - 1 - ( (a.len() - 1 - i) + (b.len() - 1 - j) );
                    tmp[pos] += (ad as usize) * (bd as usize);
                }
            }
            // Handle carries
            let mut carry = 0;
            for k in (0..out_len).rev() {
                let sum = tmp[k] + carry;
                result[k] = (sum % 10) as u8;
                carry = sum / 10;
            }
            // ignore overflow beyond out_len

            let amp = *a_amp * *b_amp;
            *prods.entry(result).or_insert(Complex::new(0.0, 0.0)) += amp;
        }
    }

    let states: Vec<(Vec<u8>, Complex<f64>)> =
        prods.into_iter().collect();
    QNum::from_superposed(states)
}

// === Internal Helpers ===

/// Enumerate all classical basis states of a `QNum` padded/truncated to `out_len`,
/// returning pairs of (digit‐vector of length `out_len`, amplitude).
fn enumerate_states(q: &QNum, out_len: usize) -> Vec<(Vec<u8>, Complex<f64>)> {
    // Start with a single empty prefix and amplitude 1
    let mut states: Vec<(Vec<u8>, Complex<f64>)> = vec![(Vec::new(), Complex::new(1.0, 0.0))];

    // For each digit Qid in q (MSB → LSB)
    for qid in &q.0 {
        let mut next = Vec::new();
        for (prefix, amp) in &states {
            for (digit, alpha) in qid.amps.iter().enumerate() {
                let p = alpha.norm_sqr();
                if p > 0.0 {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push(digit as u8);
                    next.push((new_prefix, *amp * *alpha));
                }
            }
        }
        states = next;
    }

    // Now pad each state vector on the left (MSB) with zeros to match out_len
    states
        .into_iter()
        .map(|(mut digits, amp)| {
            if digits.len() < out_len {
                let mut pad = vec![0u8; out_len - digits.len()];
                pad.append(&mut digits);
                (pad, amp)
            } else if digits.len() > out_len {
                // Truncate most-significant digits if too long
                let start = digits.len() - out_len;
                (digits[start..].to_vec(), amp)
            } else {
                (digits, amp)
            }
        })
        .collect()
}
