// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the arctangent of the number. The return value is in the range [-π/2, π/2].
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
    fn calc() {
        assert_close!(dd!("0.98279372324732906798571061101467"), dd!(1.5).atan());
    }

    #[test]
    fn edge() {
        assert_exact!(Double::ZERO, dd!(0).atan());
        assert_close!(Double::FRAC_PI_4, dd!(1).atan());
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan());
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan());
        assert_exact!(Double::NAN, Double::NAN.atan());
    }
}
