// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the inverse tangent (tan<sup>-1</sup>) of the `Quad`. The domain of this
    /// function is [-∞, ∞] and the range is [-π/2, π/2].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).atan();
    /// let expected = Quad::PI / qd!(4);  // π/4
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn atan(self) -> Quad {
        self.atan2(Quad::ONE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atan() {
        assert_close!(
            qd!("0.9827937232473290679857106110146660144968774536316285567614250883"),
            qd!(1.5).atan()
        );
        assert_close!(Quad::FRAC_PI_4, qd!(1).atan());
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.atan());
    }

    #[test]
    fn infinity() {
        assert_close!(Quad::FRAC_PI_2, Quad::INFINITY.atan());
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_INFINITY.atan());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.atan());
    }
}
