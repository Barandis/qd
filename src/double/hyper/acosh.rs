// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).acosh();
    /// let expected = dd!("0.96242365011920689499551782684874");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn acosh(self) -> Double {
        if self < Double::ONE {
            Double::NAN
        } else {
            (self + (self.sqr() - Double::ONE).sqrt()).ln()
        }
    }
}
