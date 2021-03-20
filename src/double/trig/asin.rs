// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the inverse sine (sin<sup>-1</sup>) of the `Double`. The domain of this
    /// function is [-1, 1] while the range is [-π/2, π/2]. Arguments outside of this domain
    /// will result in [`NAN`].
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
    /// 
    /// [`NAN`]: #associatedconstant.NAN
    pub fn asin(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::FRAC_PI_2
        } else if self == Double::NEG_ONE {
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
