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
        if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Quad::ZERO
                } else {
                    Quad::INFINITY
                }
            } else if n > 0 {
                Quad::NEG_ZERO
            } else {
                Quad::NEG_INFINITY
            }
        } else if n <= 0 {
            Quad::NAN
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Quad::INFINITY
            } else if n % 2 == 0 {
                Quad::NAN
            } else {
                Quad::NEG_INFINITY
            }
        } else if n == 1 {
            self
        } else if n == 2 {
            self.sqrt() // use the more specialized method in sqrt
        } else {
            // Strategy: the traditional way of finding roots is using Newton's iteration
            // for the function
            //
            //      f(x) = x^(-n) - a
            //
            // to find its root a^(-1/n). The iteration is therefore
            //
            //      x' = x + x * (1 - a * x^n) / n
            //
            // This converges quadratically, which is pretty fast. After performing a small
            // number of iterations, we can then find a^(1/n) by taking the reciprocal.

            let r = self.abs();
            // a^(-1/n) = exp(-ln(a) / n)
            let mut x: Quad = (-(r.0.ln()) / n as f64).exp().into();

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nroot() {
        assert_close!(Quad::PI, Quad::PI.nroot(1));
        assert_close!(
            qd!("1.772453850905516027298167483341145182797549456122387128213807790"),
            Quad::PI.nroot(2)
        );
        assert_close!(
            qd!("1.284025416687741484073420568062436458336280865281463089217507297"),
            Quad::E.nroot(4)
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.nroot(4));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.nroot(4));
        assert_exact!(Quad::ZERO, Quad::ZERO.nroot(5));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.nroot(4));
        assert_exact!(Quad::INFINITY, Quad::ZERO.nroot(-2));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.nroot(-2));
        assert_exact!(Quad::INFINITY, Quad::ZERO.nroot(-3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ZERO.nroot(-3));
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.nroot(4));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.nroot(4));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.nroot(3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.nroot(3));
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, qd!(2).nroot(-2));
        assert_exact!(Quad::NAN, Quad::NAN.nroot(3));
    }
}
