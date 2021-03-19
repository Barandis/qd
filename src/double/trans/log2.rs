// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates log<sub>2</sub> of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log2();
    /// let expected = dd!("3.32192809488736234787031942948939");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log2(self) -> Double {
        self.ln() / Double::LN_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log2() {
        assert_close!(dd!("3.32192809488736234787031942948939"), dd!(10).log2());
        assert_close!(dd!("7.92481250360578090726869471973908"), dd!(243).log2());
        assert_exact!(Double::ZERO, dd!(1).log2());
        assert_close!(Double::ONE, dd!(2).log2());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::ZERO.log2());
        assert_exact!(Double::NAN, Double::NEG_ZERO.log2());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log2());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log2());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.log2());
    }

    #[test]
    fn negative() {
        assert_exact!(Double::NAN, dd!(-1).log2());
    }
}
