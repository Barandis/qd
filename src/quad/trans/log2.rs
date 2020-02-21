// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates log<sub>2</sub> of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(10).log2();
    /// let expected = qd!("3.321928094887362347870319429489390175864831393024580612054756396");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn log2(self) -> Quad {
        self.ln() / Quad::LN_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log2() {
        assert_close!(
            qd!("3.321928094887362347870319429489390175864831393024580612054756396"),
            qd!(10).log2()
        );
        assert_close!(
            qd!("7.924812503605780907268694719739082543799072038462405302278763273"),
            qd!(243).log2()
        );
        assert_exact!(Quad::ZERO, Quad::ONE.log2());
        assert_close!(Quad::ONE, qd!(2).log2());
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.log2());
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.log2());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.log2());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.log2());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.log2());
    }

    #[test]
    fn negative() {
        assert_exact!(Quad::NAN, qd!(-1).log2());
    }
}
