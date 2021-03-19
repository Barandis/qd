// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates log<sub>10</sub> of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E.log10();
    /// let expected = dd!("0.434294481903251827651128918916605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log10(self) -> Double {
        self.ln() / Double::LN_10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log10() {
        assert_close!(dd!("1.62324929039790046322098305657224"), dd!(42).log10());
        assert_close!(dd!("2.38560627359831218647513951627558"), dd!(243).log10());
        assert_exact!(Double::ZERO, dd!(1).log10());
        assert_close!(Double::ONE, dd!(10).log10());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::ZERO.log10());
        assert_exact!(Double::NAN, Double::NEG_ZERO.log10());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log10());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log10());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.log10());
    }

    #[test]
    fn negative() {
        assert_exact!(Double::NAN, dd!(-1).log10());
    }
}
