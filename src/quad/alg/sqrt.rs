// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::mul_pwr2;
use crate::quad::Quad;

impl Quad {
    /// Calculates the square root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).sqrt();
    /// let expected = qd!("1.414213562373095048801688724209698078569671875376948073176679738");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sqrt(self) -> Quad {
        if self.is_zero() {
            Quad::ZERO
        } else if self.is_sign_negative() {
            Quad::NAN
        } else {
            // Strategy: use a Newton iteration.
            //
            // Perform the following Newton iteration
            //
            //      x' = x + (1 - ax²) * x / 2
            //
            // which converges to 1/√a, starting with a double-precision approximation of 1/√a.
            // Newton's iteration more or less doubles the precision with each pass, so performing
            // it three times should be enough.

            let mut r = Quad::ONE / Quad::from(self.0.sqrt());
            let h = mul_pwr2(self, 0.5);
            let k = Quad(0.5, 0.0, 0.0, 0.0);

            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;

            r *= self;
            r
        }
    }
}
