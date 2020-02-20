// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the number raised to an integral power.
    ///
    /// This function correctly handles the special inputs defined in IEEE 754.
    /// In particular:
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
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(3);
    /// assert!(x.powi(3) == dd!(27));
    /// # }
    /// ```
    pub fn powi(self, n: i32) -> Double {
        if n == 0 {
            Double::ONE
        } else if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
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
        } else if self.is_infinite() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Double::INFINITY
                } else {
                    Double::ZERO
                }
            } else if n > 0 {
                Double::NEG_INFINITY
            } else {
                Double::NEG_ZERO
            }
        } else {
            let mut r = self;
            let mut s = Double::ONE;
            let mut i = n.abs();

            if i > 1 {
                while i > 0 {
                    if i % 2 == 1 {
                        s *= r;
                    }
                    i /= 2;
                    if i > 0 {
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
            dd!("-6.2092132305915517444784571346965e-6"),
            dd!(-11).powi(-5)
        );
        assert_close!(
            dd!("97.409091034002437236440332688705"),
            Double::PI.powi(4)
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.powi(3));
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.powi(3));
        assert_exact!(Double::ZERO, Double::ZERO.powi(4));
        assert_exact!(Double::ZERO, Double::NEG_ZERO.powi(4));
        assert_exact!(Double::INFINITY, Double::ZERO.powi(-1));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ZERO.powi(-1));
        assert_exact!(Double::INFINITY, Double::ZERO.powi(-2));
        assert_exact!(Double::INFINITY, Double::NEG_ZERO.powi(-2));
    }

    #[test]
    fn zero_exponent() {
        assert_exact!(Double::ONE, Double::ZERO.powi(0));
        assert_exact!(Double::ONE, Double::ONE.powi(0));
        assert_exact!(Double::ONE, dd!(2317).powi(0));
        assert_exact!(Double::ONE, Double::INFINITY.powi(0));
        assert_exact!(Double::ONE, Double::NEG_INFINITY.powi(0));
        assert_exact!(Double::ONE, Double::NAN.powi(0));
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.powi(2));
        assert_exact!(Double::INFINITY, Double::INFINITY.powi(3));
        assert_exact!(Double::ZERO, Double::INFINITY.powi(-2));
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.powi(2));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.powi(3));
        assert_exact!(Double::NEG_ZERO, Double::NEG_INFINITY.powi(-3));
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.powi(2));
    }
}
