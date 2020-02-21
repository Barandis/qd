// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the arcsine of the number. The return value is in the range
    /// [-π/2, π/2] for any number in the range [-1, 1]. Otherwise the return
    /// value is `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).asin();
    /// let expected = Quad::PI / qd!(2);  // π/2
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn asin(self) -> Quad {
        if self.abs() > Quad::ONE {
            Quad::NAN
        } else if self == Quad::ONE {
            Quad::FRAC_PI_2
        } else if self == -Quad::ONE {
            -Quad::FRAC_PI_2
        } else {
            self.atan2((Quad::ONE - self.sqr()).sqrt())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asin() {
        assert_close!(
            qd!("0.5235987755982988730771072305465838140328615665625176368291574321"),
            qd!(0.5).asin()
        );
        assert_close!(Quad::FRAC_PI_2, qd!(1).asin());
        assert_close!(-Quad::FRAC_PI_2, qd!(-1).asin());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::NAN, Quad::INFINITY.asin());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.asin());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.asin());
    }

    #[test]
    fn out_of_range() {
        assert_exact!(Quad::NAN, qd!(1.5).asin());
        assert_exact!(Quad::NAN, qd!(-1.5).asin());
    }
}
