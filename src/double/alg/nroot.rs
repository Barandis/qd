// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the *n*th root of the `Double`.
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
            // Strategy: the square root method is specialized for square roots, but the
            // traditional way of finding roots is using Newton's iteration for the function
            //
            //      f(x) = x^(-n) - a
            //
            // to find its root a^(-1/n). The iteration is therefore
            //
            //      x' = x + x * (1 - a * x^n) / n
            //
            // This converges quadratically, which is pretty fast. We can then find a^(1/n)
            // by taking the reciprocal.

            let r = self.abs();
            // a^(-1/n) = exp(-ln(a) / n)
            let mut x: Double = (-(r.0.ln()) / n as f64).exp().into();

            x += x * (Double::ONE - r * x.powi(n)) / Double(n.into(), 0.0);
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
    fn nroot() {
        assert_close!(Double::PI, Double::PI.nroot(1));
        assert_close!(
            dd!("1.77245385090551602729816748334115"),
            Double::PI.nroot(2)
        );
        assert_close!(
            dd!("1.28402541668774148407342056806244"),
            Double::E.nroot(4)
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.nroot(4));
        assert_exact!(Double::ZERO, Double::NEG_ZERO.nroot(4));
        assert_exact!(Double::ZERO, Double::ZERO.nroot(5));
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.nroot(4));
        assert_exact!(Double::INFINITY, Double::ZERO.nroot(-2));
        assert_exact!(Double::INFINITY, Double::NEG_ZERO.nroot(-2));
        assert_exact!(Double::INFINITY, Double::ZERO.nroot(-3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ZERO.nroot(-3));
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.nroot(4));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.nroot(4));
        assert_exact!(Double::INFINITY, Double::INFINITY.nroot(3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.nroot(3));
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, dd!(2).nroot(-2));
        assert_exact!(Double::NAN, Double::NAN.nroot(3));
    }
}
