// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the hyperbolic tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).tanh();
    /// let expected = dd!("0.76159415595576488811945828260479");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn tanh(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.abs().as_float() > 0.05 {
            let a = self.exp();
            let inv_a = a.recip();
            (a - inv_a) / (a + inv_a)
        } else {
            let (s, c) = self.sinh_cosh();
            s / c
        }
    }
}
