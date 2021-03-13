// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the number raised to an integral power.
    ///
    /// This function correctly handles the special inputs defined in IEEE 754. In
    /// particular:
    ///
    /// * `x.powi(0)` is `1` for any `x` (including `0`, `NaN`, or infinity)
    /// * `x.powi(n)` is ±∞ for `x == ±0` and any odd negative `n`
    /// * `x.powi(n)` is +∞ for `x == ±0` and any even negative `n`
    /// * `x.powi(n)` is ±0 for `x == ±0` and any odd positive `n`
    /// * `x.powi(n)` is +0 for `x == ±0` and any even positive `n`
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.powi(3) == qd!(27));
    /// # }
    /// ```
    pub fn powi(self, n: i32) -> Quad {
        if n == 0 {
            Quad::ONE
        } else if self.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
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
        } else if self.is_infinite() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Quad::INFINITY
                } else {
                    Quad::ZERO
                }
            } else if n > 0 {
                Quad::NEG_INFINITY
            } else {
                Quad::NEG_ZERO
            }
        } else {
            let mut r = self;
            let mut s = Quad::ONE;
            let mut k = n.abs();

            if k > 1 {
                while k > 0 {
                    if k % 2 == 1 {
                        s *= r;
                    }
                    k /= 2;
                    if k > 0 {
                        r = r.sqr();
                    }
                }
            } else {
                s = r;
            }

            if n < 0 {
                s.recip()
            } else {
                s
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powi() {
        assert_close!(
            qd!("-6.209213230591551744478457134696462611222531992971170622970363425e-6"),
            qd!(-11).powi(-5)
        );
        assert_close!(
            qd!("97.40909103400243723644033268870511124972758567268542169146785939"),
            Quad::PI.powi(4)
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.powi(3));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.powi(3));
        assert_exact!(Quad::ZERO, Quad::ZERO.powi(4));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.powi(4));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powi(-1));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ZERO.powi(-1));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powi(-2));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.powi(-2));
    }

    #[test]
    fn zero_exponent() {
        assert_exact!(Quad::ONE, Quad::ZERO.powi(0));
        assert_exact!(Quad::ONE, Quad::ONE.powi(0));
        assert_exact!(Quad::ONE, qd!(2317).powi(0));
        assert_exact!(Quad::ONE, Quad::INFINITY.powi(0));
        assert_exact!(Quad::ONE, Quad::NEG_INFINITY.powi(0));
        assert_exact!(Quad::ONE, Quad::NAN.powi(0));
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.powi(2));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.powi(3));
        assert_exact!(Quad::ZERO, Quad::INFINITY.powi(-2));
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.powi(2));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.powi(3));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_INFINITY.powi(-3));
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.powi(2));
    }
}
