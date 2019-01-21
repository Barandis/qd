// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(0.5).atanh();
    /// let expected = dd!("0.54930614433405484569762261846126");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn atanh(self) -> Double {
        if self.abs() >= Double::ONE {
            Double::NAN
        } else {
            mul_pwr2(((Double::ONE + self) / (Double::ONE - self)).ln(), 0.5)
        }
    }
}
