// qublis-2-74136-v2.0/qnum/src/qid.rs

//! A single quantum digit (Qid) holding 10 complex amplitudes.
//! Each f64 is wrapped in `OrderedFloat` so we can derive `Hash` + `Eq`.

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
        let wrapped = amps.map(|c| Complex {
            re: OrderedFloat(c.re),
            im: OrderedFloat(c.im),
        });
        Qid { amps: wrapped }
    }

    /// Create a “definite” (classical) Qid that collapses to digit `i`.
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
        let mut amps = [zero; 10];
        amps[i] = one;
        Qid { amps }
    }

    /// Normalize this Qid so that the sum of squared magnitudes of `amps` equals 1.
    ///
    /// If the total norm is zero, this is a no-op.
    pub fn normalize(&mut self) {
        // compute sum of squared magnitudes as f64
        let sum_sq: f64 = self
            .amps
            .iter()
            .map(|c| {
                let re = c.re.into_inner();
                let im = c.im.into_inner();
                re * re + im * im
            })
            .sum();

        if sum_sq <= 0.0 {
            // zero‐vector: cannot normalize
            return;
        }
        let norm = sum_sq.sqrt();

        // divide each amplitude by norm
        for c in &mut self.amps {
            let re = c.re.into_inner() / norm;
            let im = c.im.into_inner() / norm;
            c.re = OrderedFloat(re);
            c.im = OrderedFloat(im);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn normalize_scales_to_unit_norm() {
        // raw amplitudes not summing to 1
        let raw = [
            Complex { re: 2.0, im: 0.0 },
            Complex { re: 3.0, im: 4.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
            Complex { re: 0.0, im: 0.0 },
        ];
        let mut q = Qid::from_f64(raw);
        q.normalize();

        let sum_sq: f64 = q
            .amps
            .iter()
            .map(|c| {
                let re = c.re.into_inner();
                let im = c.im.into_inner();
                re * re + im * im
            })
            .sum();

        assert!((sum_sq - 1.0).abs() < 1e-12, "normalized Qid must have unit norm");
    }

    #[test]
    fn normalize_zero_does_nothing() {
        let mut q = Qid::definite(0);
        // zero all amplitudes
        for c in &mut q.amps {
            c.re = OrderedFloat(0.0);
            c.im = OrderedFloat(0.0);
        }
        // this is the zero‐vector; normalize() should not panic or divide by zero
        q.normalize();
        // still all zero
        assert!(q.amps.iter().all(|c| c.re.into_inner() == 0.0 && c.im.into_inner() == 0.0));
    }
}
