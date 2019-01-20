// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the arctangent of the number. The return value is in the range [-π/2, π/2].
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
    fn quad_trig_atan() {
        assert_exact!(Quad::ZERO, qd!(0).atan());
        assert_close!(Quad::FRAC_PI_4, qd!(1).atan());
        assert_close!(Quad::FRAC_PI_2, Quad::INFINITY.atan());
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_INFINITY.atan());
        assert_exact!(Quad::NAN, Quad::NAN.atan());
        assert_close!(
            qd!("0.9827937232473290679857106110146660144968774536316285567614250883"),
            qd!(1.5).atan()
        );
    }
}
