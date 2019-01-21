// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Simultaneously computes the hyperbolic sine and cosine of the number.
    ///
    /// This method is more efficient to run than [`sinh`] and [`cosh`] individually and is useful
    /// when both numbers are needed.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let (sin_h, cos_h) = dd!(1).sinh_cosh();
    /// let esin = dd!("1.1752011936438014568823818505956");
    /// let ecos = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff1 = (sin_h - esin).abs();
    /// let diff2 = (cos_h - ecos).abs();
    ///
    /// assert!(diff1 < dd!(1e-30));
    /// assert!(diff2 < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`sinh`]: #method.sinh
    /// [`cosh`]: #method.cosh
    pub fn sinh_cosh(self) -> (Double, Double) {
        if self.abs().as_float() <= 0.05 {
            let s = self.sinh();
            let c = (Double::ONE + s.sqr()).sqrt();
            (s, c)
        } else {
            let a = self.exp();
            let inv_a = a.recip();
            let s = mul_pwr2(a - inv_a, 0.5);
            let c = mul_pwr2(a + inv_a, 0.5);
            (s, c)
        }
    }
}
