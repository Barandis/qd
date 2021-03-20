// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the reciprocal of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI.recip();
    /// let expected = qd!("0.3183098861837906715377675267450287240689192914809128974953346881");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Quad {
        Quad::ONE / self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(
            qd!("0.3183098861837906715377675267450287240689192914809128974953346881"),
            Quad::PI.recip()
        );
        assert_close!(
            qd!("0.3678794411714423215955237701614608674458111310317678345078368017"),
            Quad::E.recip()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::INFINITY, Quad::ZERO.recip());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ZERO.recip());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::ZERO, Quad::INFINITY.recip());
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_INFINITY.recip());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.recip());
    }
}
