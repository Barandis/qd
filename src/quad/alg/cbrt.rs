// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the cube root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).cbrt();
    /// let expected = qd!("1.259921049894873164767210607278228350570251464701507980081975112");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn cbrt(self) -> Quad {
        self.nroot(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbrt() {
        assert_close!(
            qd!("1.464591887561523263020142527263790391738596855627937174357255937"),
            Quad::PI.cbrt()
        );
        assert_close!(
            qd!("-1.395612425086089528628125319602586837597906515199406982617516706"),
            (-Quad::E).cbrt()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.cbrt());
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.cbrt());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.cbrt());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.cbrt());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.cbrt());
    }
}
