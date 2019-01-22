// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic tangent of the number.
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
    pub fn atanh(self) -> Double {
        if self.abs() >= Double::ONE {
            Double::NAN
        } else {
            mul_pwr2(((Double::ONE + self) / (Double::ONE - self)).ln(), 0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_close!(
            dd!("0.3297653149566991076178634175552186042701373911406924144029083548"),
            Double::PI.recip().atanh()
        );
        assert_close!(
            dd!("0.3859684164526523625353195700175926718961289961812712597770308403"),
            Double::E.recip().atanh()
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Double::ZERO, dd!(0.0).atanh());
        assert_exact!(Double::NAN, Double::NAN.atanh());
        assert_exact!(Double::NAN, Double::INFINITY.atanh());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.atanh());
    }
}
