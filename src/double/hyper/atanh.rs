// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common;
use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic tangent (tanh<sup>-1</sup>) of the `Double`.
    /// 
    /// The domain of the function is (-1, 1) and the range is (-∞, ∞). Any argument whose
    /// absolute value is greater than or equal to 1 will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(0.5).atanh();
    /// let expected = dd!("0.54930614433405484569762261846126");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    /// 
    /// [`NAN`]: #associatedconstant.NAN
    pub fn atanh(self) -> Double {
        if self.abs() >= Double::ONE {
            Double::NAN
        } else {
            common::mul_pwr2(((Double::ONE + self) / (Double::ONE - self)).ln(), 0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atanh() {
        assert_close!(
            dd!("0.3297653149566991076178634175552186"),
            Double::PI.recip().atanh()
        );
        assert_close!(
            dd!("0.3859684164526523625353195700175927"),
            Double::E.recip().atanh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.atanh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.atanh());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.atanh());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.atanh());
    }
}
