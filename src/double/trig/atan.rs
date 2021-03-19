// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the inverse tangent (tan<sup>-1</sup>) of the `Double`. The domain of this
    /// function is [-∞, ∞] and the range is [-π/2, π/2].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).atan();
    /// let expected = Double::PI / dd!(4);  // π/4
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn atan(self) -> Double {
        self.atan2(Double::ONE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atan() {
        assert_close!(dd!("0.98279372324732906798571061101467"), dd!(1.5).atan());
        assert_close!(Double::FRAC_PI_4, dd!(1).atan());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, dd!(0).atan());
    }

    #[test]
    fn infinity() {
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan());
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.atan());
    }
}
