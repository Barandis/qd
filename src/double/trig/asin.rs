// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the arcsine of the number. The return value is in the range [-π/2, π/2] for
    /// any number in the range [-1, 1]. Otherwise the return value is `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).asin();
    /// let expected = Double::PI / dd!(2);  // π/2
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn asin(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::FRAC_PI_2
        } else if self == -Double::ONE {
            -Double::FRAC_PI_2
        } else {
            self.atan2((Double::ONE - self.sqr()).sqrt())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asin() {
        assert_close!(dd!("0.52359877559829887307710723054658"), dd!(0.5).asin());
        assert_close!(Double::FRAC_PI_2, dd!(1).asin());
        assert_close!(-Double::FRAC_PI_2, dd!(-1).asin());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.asin());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.asin());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.asin());
    }

    #[test]
    fn out_of_range() {
        assert_exact!(Double::NAN, dd!(1.5).asin());
        assert_exact!(Double::NAN, dd!(-1.5).asin());
    }
}
