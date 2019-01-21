// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the `n`th root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).nroot(4);
    /// let expected = dd!("1.1892071150027210667174999705605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn nroot(self, n: i32) -> Double {
        if n <= 0 {
            return Double::NAN;
        }
        if n % 2 == 0 && self.is_sign_negative() {
            return Double::NAN;
        }
        if n == 1 {
            return self;
        }
        if n == 2 {
            return self.sqrt(); // use the more specialized method in sqrt
        }
        if self.is_zero() {
            return Double::ZERO;
        }

        // Strategy: the square root method is specialized for square roots, but the traditional
        // way of finding roots is using Newton's iteration for the function
        //
        //      f(x) = x^(-n) - a
        //
        // to find its root a^(-1/n). The iteration is therefore
        //
        //      x' = x + x * (1 - a * x^n) / n
        //
        // This converges quadratically, which is pretty fast. We can then find a^(1/n) by taking
        // the reciprocal.

        let r = self.abs();
        let mut x: Double = (-(r.0.ln()) / n as f64).exp().into(); // a^(-1/n) = exp(-ln(a) / n)

        x += x * (Double::ONE - r * x.powi(n)) / Double::from(n);
        if self.is_sign_negative() {
            x = -x;
        }
        x.recip()
    }
}
