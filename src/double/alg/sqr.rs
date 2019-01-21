// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, two_sqr};
use crate::double::Double;

impl Double {
    /// Calculates the square of the number.
    ///
    /// This method takes advantage of optimizations in multiplication that are available when the
    /// two numbers being multiplied are the same, so it is more efficient than bare multiplication.
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
        let (p, e) = two_sqr(self.0);
        Double::from(quick_two_sum(
            p,
            e + 2.0 * self.0 * self.1 + self.1 * self.1,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_exact!(dd!(121), dd!(-11).sqr());
        assert_close!(dd!("9.869604401089358618834490999876"), Double::PI.sqr());
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, Double::NAN.sqr());
        assert_exact!(Double::ZERO, dd!(0).sqr());
    }
}
