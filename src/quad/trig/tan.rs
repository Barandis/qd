// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).tan();
    /// let expected = qd!("1.557407724654902230506974807458360173087250772381520038383946606");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn tan(self) -> Quad {
        let (s, c) = self.sin_cos();
        s / c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_trig_tangent() {
        assert_close!(
            qd!("1.557407724654902230506974807458360173087250772381520038383946606"),
            qd!(1).tan()
        );
        assert_close!(qd!(1), Quad::FRAC_PI_4.tan());
        assert_exact!(Quad::ZERO, Quad::ZERO.tan());
        assert!(Quad::FRAC_PI_2.tan().is_infinite());
        assert_exact!(Quad::NAN, Quad::INFINITY.tan());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.tan());
        assert_exact!(Quad::NAN, Quad::NAN.tan());
    }
}
