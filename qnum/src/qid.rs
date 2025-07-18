//! A single quantum digit (Qid) holding 10 complex amplitudes.
//! We wrap each `f64` in `OrderedFloat` so that `Complex<OrderedFloat<f64>>`
//! can derive `Hash` and `Eq`.

use num_complex::Complex;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

/// A single “digit” in the Quantum Number System: a superposition
/// over the values 0–9, each with a complex amplitude.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qid {
    /// Each amplitude’s real and imaginary parts are wrapped in `OrderedFloat<f64>`.
    pub amps: [Complex<OrderedFloat<f64>>; 10],
}

impl Qid {
    /// Construct a Qid from raw `f64` amplitudes (array length = 10).
    /// Panics if the caller provides an array of a different size.
    pub fn from_f64(amps: [Complex<f64>; 10]) -> Self {
        // Map each `Complex<f64>` into `Complex<OrderedFloat<f64>>`
        let wrapped = amps.map(|c| Complex {
            re: OrderedFloat(c.re),
            im: OrderedFloat(c.im),
        });
        Qid { amps: wrapped }
    }

    /// Create a “definite” (classical) Qid that collapses to digit `i` with probability 1.
    pub fn definite(i: usize) -> Self {
        assert!(i < 10, "digit out of range");
        let zero = Complex {
            re: OrderedFloat(0.0),
            im: OrderedFloat(0.0),
        };
        let one = Complex {
            re: OrderedFloat(1.0),
            im: OrderedFloat(0.0),
        };
        // Fill all entries with zero, then set index `i` to one.
        let mut amps = [zero; 10];
        amps[i] = one;
        Qid { amps }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn from_f64_and_definite_hash_and_eq() {
        // two Qids constructed the same way should be equal & hash identically
        let raw = [
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 1.0, im: 0.0 },
            Complex { re: 0.5, im: -0.5 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ];
        let q1 = Qid::from_f64(raw);
        let q2 = Qid::from_f64(raw);
        assert_eq!(q1, q2);

        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        q1.hash(&mut h1);
        q2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn definite_creates_one_hot() {
        for i in 0..10 {
            let q = Qid::definite(i);
            for (j, amp) in q.amps.iter().enumerate() {
                let norm = (amp.re.into_inner().powi(2) + amp.im.into_inner().powi(2)).sqrt();
                if j == i {
                    assert!((norm - 1.0).abs() < 1e-12);
                } else {
                    assert!((norm - 0.0).abs() < 1e-12);
                }
            }
        }
    }
}
