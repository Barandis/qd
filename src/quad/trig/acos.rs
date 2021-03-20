// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the inverse cosine (cos<sup>-1</sup>) of the `Quad`. The domain of this
    /// function is [-1, 1] and the range is [0, π]. Arguments outside of the domain will
    /// result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).acos();
    /// let expected = qd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    /// 
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acos(self) -> Quad {
        if self.abs() > Quad::ONE {
            Quad::NAN
        } else if self == Quad::ONE {
            Quad::ZERO
        } else if self == Quad::NEG_ONE {
            Quad::PI
        } else {
            (Quad::ONE - self.sqr()).sqrt().atan2(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acos() {
        assert_close!(
            qd!("1.047197551196597746154214461093167628065723133125035273658314864"),
            qd!(0.5).acos()
        );
        assert_exact!(Quad::ZERO, qd!(1).acos());
        assert_close!(Quad::PI, qd!(-1).acos());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::NAN, Quad::INFINITY.acos());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.acos());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.acos());
    }

    #[test]
    fn out_of_range() {
        assert_exact!(Quad::NAN, qd!(1.5).acos());
        assert_exact!(Quad::NAN, qd!(-1.5).acos());
    }
}
