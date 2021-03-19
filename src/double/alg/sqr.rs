// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::double::Double;

impl Double {
    /// Calculates the square of the `Double`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available
    /// when the two numbers being multiplied are the same, so it is more efficient than
    /// bare multiplication.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(3);
    /// assert!(x.sqr() == x * x); // The left side is faster though
    /// # }
    /// ```
    #[inline]
    pub fn sqr(self) -> Double {
        if self.is_infinite() {
            Double::INFINITY
        } else {
            let (p, e) = core::two_sqr(self.0);
            let (a, b) = core::renorm2(p, e + 2.0 * self.0 * self.1 + self.1 * self.1);
            Double(a, b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqr() {
        assert_exact!(dd!(121), dd!(-11).sqr());
        assert_close!(dd!("9.869604401089358618834490999876"), Double::PI.sqr());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sqr());
        assert_exact!(Double::ZERO, Double::NEG_ZERO.sqr());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sqr());
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.sqr());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.sqr());
    }
}
