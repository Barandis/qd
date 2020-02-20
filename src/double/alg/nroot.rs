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
        if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Double::ZERO
                } else {
                    Double::INFINITY
                }
            } else if n > 0 {
                Double::NEG_ZERO
            } else {
                Double::NEG_INFINITY
            }
        } else if n <= 0 {
            Double::NAN
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Double::INFINITY
            } else if n % 2 == 0 {
                Double::NAN
            } else {
                Double::NEG_INFINITY
            }
        } else if n == 1 {
            self
        } else if n == 2 {
            self.sqrt() // use the more specialized method in sqrt
        } else {
            // Strategy: the square root method is specialized for square roots,
            // but the traditional way of finding roots is using Newton's
            // iteration for the function
            //
            //      f(x) = x^(-n) - a
            //
            // to find its root a^(-1/n). The iteration is therefore
            //
            //      x' = x + x * (1 - a * x^n) / n
            //
            // This converges quadratically, which is pretty fast. We can then
            // find a^(1/n) by taking the reciprocal.

            let r = self.abs();
            // a^(-1/n) = exp(-ln(a) / n)
            let mut x: Double = (-(r.0.ln()) / n as f64).exp().into();

            x += x * (Double::ONE - r * x.powi(n)) / Double::from(n);
            if self.is_sign_negative() {
                x = -x;
            }
            x.recip()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(Double::PI, Double::PI.nroot(1));
        assert_close!(
            dd!("1.772453850905516027298167483341145182797549456122387128213807790"),
            Double::PI.nroot(2)
        );
        assert_close!(
            dd!("1.284025416687741484073420568062436458336280865281463089217507297"),
            Double::E.nroot(4)
        );
    }

    #[test]
    fn special() {
        assert_exact!(Double::INFINITY, dd!(0.0).nroot(-2));
        assert_exact!(Double::INFINITY, dd!(-0.0).nroot(-2));
        assert_exact!(Double::INFINITY, dd!(0.0).nroot(-3));
        assert_exact!(Double::NEG_INFINITY, dd!(-0.0).nroot(-3));
        assert_exact!(Double::ZERO, dd!(0.0).nroot(4));
        assert_exact!(Double::ZERO, dd!(-0.0).nroot(4));
        assert_exact!(Double::ZERO, dd!(0.0).nroot(5));
        assert_exact!(Double::NEG_ZERO, dd!(-0.0).nroot(4));
        assert_exact!(Double::NAN, dd!(2).nroot(-2));
        assert_exact!(Double::INFINITY, Double::INFINITY.nroot(4));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.nroot(4));
        assert_exact!(Double::INFINITY, Double::INFINITY.nroot(3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.nroot(3));
        assert_exact!(Double::NAN, Double::NAN.nroot(3));
    }
}
