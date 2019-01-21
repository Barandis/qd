// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Computes the hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).cosh();
    /// let expected = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn cosh(self) -> Double {
        if self.is_zero() {
            Double::ONE
        } else {
            let a = self.exp();
            mul_pwr2(a + a.recip(), 0.5)
        }
    }
}
