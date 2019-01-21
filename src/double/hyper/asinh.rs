// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).asinh();
    /// let expected = dd!("1.1947632172871093041119308285191");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn asinh(self) -> Double {
        (self + (self.sqr() + Double::ONE).sqrt()).ln()
    }
}
