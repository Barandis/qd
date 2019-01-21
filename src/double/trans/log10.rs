// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates log<sub>10</sub> of the number.
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
    fn calc() {
        assert_close!(
            dd!("1.623249290397900463220983056572244529451891141976769812643928055"),
            dd!(42).log10()
        );
        assert_close!(
            dd!("2.385606273598312186475139516275576546000644320953479324149328202"),
            dd!(243).log10()
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Double::ZERO, dd!(1).log10());
        assert_exact!(Double::NAN, dd!(0).log10());
        assert_exact!(Double::NAN, dd!(-1).log10());
        assert_close!(Double::ONE, dd!(10).log10());
    }
}
