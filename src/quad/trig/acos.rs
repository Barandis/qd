// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the arccosine of the number. The return value is in the range [0, π] for any number
    /// in the range [-1, 1]. Otherwise the return value is `NaN`.
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
    pub fn acos(self) -> Quad {
        if self.abs() > Quad::ONE {
            Quad::NAN
        } else if self == Quad::ONE {
            Quad::ZERO
        } else if self == -Quad::ONE {
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
    fn quad_trig_acos() {
        assert_exact!(Quad::NAN, qd!(1.5).acos());
        assert_exact!(Quad::NAN, qd!(-1.5).acos());
        assert_exact!(Quad::ZERO, qd!(1).acos());
        assert_close!(Quad::PI, qd!(-1).acos());
        assert_close!(
            qd!("1.047197551196597746154214461093167628065723133125035273658314864"),
            qd!(0.5).acos()
        );
    }
}
