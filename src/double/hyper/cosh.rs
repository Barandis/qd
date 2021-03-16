// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common;
use crate::double::Double;

impl Double {
    /// Computes the hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).cosh();
    /// let expected = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn cosh(self) -> Double {
        if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ONE
        } else {
            let a = self.exp();
            common::mul_pwr2(a + a.recip(), 0.5)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosh() {
        assert_close!(dd!("11.591953275521520627751752052560"), Double::PI.cosh());
        assert_close!(dd!("7.6101251386622883634186102301134"), Double::E.cosh());
    }

    #[test]
    fn one() {
        assert_exact!(Double::ONE, Double::ZERO.cosh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.cosh());
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.cosh());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.cosh());
    }
}
