// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates log<sub>2</sub> of the number.
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
    fn calc() {
        assert_close!(
            dd!("3.321928094887362347870319429489390175864831393024580612054756396"),
            dd!(10).log2()
        );
        assert_close!(
            dd!("7.924812503605780907268694719739082543799072038462405302278763273"),
            dd!(243).log2()
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Double::ZERO, dd!(1).log2());
        assert_exact!(Double::NAN, dd!(0).log2());
        assert_exact!(Double::NAN, dd!(-1).log2());
        assert_close!(Double::ONE, dd!(2).log2());
    }
}
