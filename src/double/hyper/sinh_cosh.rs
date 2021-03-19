// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common;
use crate::double::Double;

impl Double {
    /// Simultaneously computes the hyperbolic sine and cosine (sinh and cosh) of the
    /// `Double`.
    /// 
    /// The domain of this function is (-∞, ∞), and the range is (-∞, ∞) for the first
    /// component of the answer (the hyperbolic sine) and [1, ∞) for the second (the
    /// hyperbolic cosine).
    ///
    /// This method is more efficient to run than [`sinh`] and [`cosh`] individually and is
    /// useful when both numbers are needed.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let (sin_h, cos_h) = dd!(1).sinh_cosh();
    /// let esin = dd!("1.1752011936438014568823818505956");
    /// let ecos = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff1 = (sin_h - esin).abs();
    /// let diff2 = (cos_h - ecos).abs();
    ///
    /// assert!(diff1 < dd!(1e-30));
    /// assert!(diff2 < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`sinh`]: #method.sinh
    /// [`cosh`]: #method.cosh
    pub fn sinh_cosh(self) -> (Double, Double) {
        if self.is_nan() {
            (Double::NAN, Double::NAN)
        } else if self.is_zero() {
            (Double::ZERO, Double::ONE)
        } else if self.abs().0 <= 0.05 {
            let s = self.sinh();
            let c = (Double::ONE + s.sqr()).sqrt();
            (s, c)
        } else {
            let a = self.exp();
            let inv_a = a.recip();
            let s = common::mul_pwr2(a - inv_a, 0.5);
            let c = common::mul_pwr2(a + inv_a, 0.5);
            (s, c)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sinh_cosh() {
        let (sinh_pi, cosh_pi) = Double::PI.sinh_cosh();
        assert_close!(dd!("11.548739357257748377977334315388"), sinh_pi);
        assert_close!(dd!("11.591953275521520627751752052560"), cosh_pi);

        let (sinh_e, cosh_e) = Double::E.sinh_cosh();
        assert_close!(dd!("7.5441371028169758263418200425165"), sinh_e);
        assert_close!(dd!("7.6101251386622883634186102301134"), cosh_e);
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sinh_cosh().0);
        assert_exact!(Double::ONE, Double::ZERO.sinh_cosh().1);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sinh_cosh().0);
        assert_exact!(Double::INFINITY, Double::INFINITY.sinh_cosh().1);

        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.sinh_cosh().0);
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.sinh_cosh().1);
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.sinh_cosh().0);
        assert_exact!(Double::NAN, Double::NAN.sinh_cosh().1);
    }
}
