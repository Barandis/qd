// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates log<sub>10</sub> of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E.log10();
    /// let expected = qd!("0.4342944819032518276511289189166050822943970058036665661144537832");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn log10(self) -> Quad {
        self.ln() / Quad::LN_10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(
            qd!("1.623249290397900463220983056572244529451891141976769812643928055"),
            qd!(42).log10()
        );
        assert_close!(
            qd!("2.385606273598312186475139516275576546000644320953479324149328202"),
            qd!(243).log10()
        );
        assert_exact!(Quad::ZERO, qd!(1).log10());
        assert_close!(Quad::ONE, qd!(10).log10());
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.log10());
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.log10());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.log10());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.log10());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.log10());
    }

    #[test]
    fn negative() {
        assert_exact!(Quad::NAN, qd!(-1).log10());
    }
}
