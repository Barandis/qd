// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::mul_pwr2;
use crate::quad::Quad;

impl Quad {
    /// Computes the hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).cosh();
    /// let expected = qd!("1.543080634815243778477905620757061682601529112365863704737402215");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn cosh(self) -> Quad {
        if self.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
            Quad::ONE
        } else {
            let a = self.exp();
            mul_pwr2(a + a.recip(), 0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cosh() {
        assert_close!(
            qd!("11.59195327552152062775175205256013769577091717620542253821288305"),
            Quad::PI.cosh()
        );
        assert_close!(
            qd!("7.610125138662288363418610230113379165233562792554468102771609974"),
            Quad::E.cosh()
        );
    }

    #[test]
    fn one() {
        assert_exact!(Quad::ONE, Quad::ZERO.cosh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.cosh());
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.cosh());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.cosh());
    }
}
