// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the `n`th root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).nroot(4);
    /// let expected = qd!("1.189207115002721066717499970560475915292972092463817413019002225");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn nroot(self, n: i32) -> Quad {
        if n <= 0 {
            return Quad::NAN;
        }
        if n % 2 == 0 && self.is_sign_negative() {
            return Quad::NAN;
        }
        if n == 1 {
            return self;
        }
        if n == 2 {
            return self.sqrt(); // use the more specialized method in sqrt
        }
        if self.is_zero() {
            return Quad::ZERO;
        }

        // Strategy: the traditional way of finding roots is using Newton's iteration for the
        // function
        //
        //      f(x) = x^(-n) - a
        //
        // to find its root a^(-1/n). The iteration is therefore
        //
        //      x' = x + x * (1 - a * x^n) / n
        //
        // This converges quadratically, which is pretty fast. After performing a small number of
        // iterations, we can then find a^(1/n) by taking the reciprocal.

        let r = self.abs();
        let mut x: Quad = (-(r.0.ln()) / n as f64).exp().into(); // a^(-1/n) = exp(-ln(a) / n)

        let qd_n = Quad::from(n);
        x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
        x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
        x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
        if self.0 < 0.0 {
            x = -x
        }
        x.recip()
    }
}
