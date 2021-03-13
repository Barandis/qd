// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).asinh();
    /// let expected = dd!("1.1947632172871093041119308285191");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn asinh(self) -> Double {
        if self.is_infinite() {
            if self.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else {
            (self + (self.sqr() + Double::ONE).sqrt()).ln()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asinh() {
        assert_close!(dd!("1.8622957433108482198883613251826"), Double::PI.asinh());
        assert_close!(dd!("1.7253825588523150939450979704049"), Double::E.asinh());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, dd!(0.0).asinh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.asinh());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.asinh());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.asinh());
    }
}
