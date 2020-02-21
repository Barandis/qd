// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::mul_pwr2;
use crate::quad::Quad;

impl Quad {
    /// Simultaneously computes the hyperbolic sine and cosine of the number.
    ///
    /// This method is more efficient to run than [`sinh`] and [`cosh`]
    /// individually and is useful when both numbers are needed.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let (sin_h, cos_h) = qd!(1).sinh_cosh();
    /// let esin = qd!("1.175201193643801456882381850595600815155717981334095870229565413");
    /// let ecos = qd!("1.543080634815243778477905620757061682601529112365863704737402215");
    ///
    /// let diff1 = (sin_h - esin).abs();
    /// let diff2 = (cos_h - ecos).abs();
    ///
    /// assert!(diff1 < qd!(1e-60));
    /// assert!(diff2 < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [`sinh`]: #method.sinh [`cosh`]: #method.cosh
    pub fn sinh_cosh(self) -> (Quad, Quad) {
        if self.is_nan() {
            (Quad::NAN, Quad::NAN)
        } else if self.is_zero() {
            (Quad::ZERO, Quad::ONE)
        } else if self.abs().as_float() <= 0.05 {
            let s = self.sinh();
            let c = (Quad::ONE + s.sqr()).sqrt();
            (s, c)
        } else {
            let a = self.exp();
            let inv_a = a.recip();
            let s = mul_pwr2(a - inv_a, 0.5);
            let c = mul_pwr2(a + inv_a, 0.5);
            (s, c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sinh_cosh() {
        let (sinh_pi, cosh_pi) = Quad::PI.sinh_cosh();
        assert_close!(
            qd!("11.54873935725774837797733431538840968449518906639478945523216336"),
            sinh_pi
        );
        assert_close!(
            qd!("11.59195327552152062775175205256013769577091717620542253821288305"),
            cosh_pi
        );

        let (sinh_e, cosh_e) = Quad::E.sinh_cosh();
        assert_close!(
            qd!("7.544137102816975826341820042516532740294985744301671666369136432"),
            sinh_e
        );
        assert_close!(
            qd!("7.610125138662288363418610230113379165233562792554468102771609974"),
            cosh_e
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sinh_cosh().0);
        assert_exact!(Quad::ONE, Quad::ZERO.sinh_cosh().1);
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sinh_cosh().0);
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sinh_cosh().1);

        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.sinh_cosh().0);
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.sinh_cosh().1);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sinh_cosh().0);
        assert_exact!(Quad::NAN, Quad::NAN.sinh_cosh().1);
    }
}
