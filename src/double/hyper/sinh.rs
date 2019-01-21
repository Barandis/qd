// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Computes the hyperbolic sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).sinh();
    /// let expected = dd!("1.1752011936438014568823818505956");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn sinh(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.abs().as_float() > 0.05 {
            let a = self.exp();
            mul_pwr2(a - a.recip(), 0.5)
        } else {
            // The above formula is not accurate enough with very small numbers. Use a Taylor
            // series instead.
            let mut s = self;
            let mut t = self;
            let r = t.sqr();
            let mut m = 1.0;
            let threshold = (self * Double::EPSILON).abs();

            loop {
                m += 2.0;
                t *= r;
                t /= Double::from_mul(m - 1.0, m);
                s += t;
                if t.abs() <= threshold {
                    break;
                }
            }
            s
        }
    }
}
