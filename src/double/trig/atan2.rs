// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the 2-argument arctangent of the number (`y`) and `other` (`x`) in radians.
    ///
    /// The second argument allows the avoidance of ambiguities in the single-argument
    /// [`atan`] function, notably allowing the determination of quadrant.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let pi = Double::PI;
    ///
    /// // -π/4 radians (45 degrees clockwise)
    /// let x1 = dd!(3);
    /// let y1 = dd!(-3);
    /// let expected1 = -pi / dd!(4);
    ///
    /// // 3π/4 radians (135 degrees counter-clockwise)
    /// let x2 = dd!(-3);
    /// let y2 = dd!(3);
    /// let expected2 = Double::from_div(3.0, 4.0) * pi;
    ///
    /// let diff1 = (y1.atan2(x1) - expected1).abs();
    /// let diff2 = (y2.atan2(x2) - expected2).abs();
    ///
    /// assert!(diff1 < dd!(1e-30));
    /// assert!(diff2 < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`atan`]: #method.atan
    pub fn atan2(self, other: Double) -> Double {
        // Strategy:
        //
        // Use Newton's iteration to solve one of the following equations
        //
        //      sin z = y / r
        //      cos z = x / r
        //
        // where r = √(x² + y²).
        //
        // The iteration is given by 
        //      z' = z + (y - sin z) / cos z   (for the first equation) 
        //      z' = z - (x - cos z) / sin z   (for the second equation)
        //
        // Here, x and y are normalized so that x² + y² = 1. If |x| > |y|, the first
        // iteration is used since the denominator is larger. Otherwise the second is used.

        if other.is_zero() {
            if self.is_zero() {
                Double::NAN
            } else if self.is_sign_positive() {
                Double::FRAC_PI_2
            } else {
                -Double::FRAC_PI_2
            }
        } else if self.is_zero() {
            if other.is_sign_positive() {
                Double::ZERO
            } else {
                Double::PI
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Double::NAN
            } else if self.is_sign_positive() {
                Double::FRAC_PI_2
            } else {
                -Double::FRAC_PI_2
            }
        } else if other.is_infinite() {
            Double::ZERO
        } else if self.is_nan() || other.is_nan() {
            Double::NAN
        } else if self == other {
            if self.is_sign_positive() {
                Double::FRAC_PI_4
            } else {
                -Double::FRAC_3_PI_4
            }
        } else if self == -other {
            if self.is_sign_positive() {
                Double::FRAC_3_PI_4
            } else {
                -Double::FRAC_PI_4
            }
        } else {
            let r = (self.sqr() + other.sqr()).sqrt();
            let x = other / r;
            let y = self / r;

            // Compute f64 approximation to atan
            let mut z = Double::from(self.0.atan2(other.0));
            let (sin_z, cos_z) = z.sin_cos();

            if x.0.abs() > y.0.abs() {
                // Use first iteration above
                z += (y - sin_z) / cos_z;
            } else {
                // Use second iteration above
                z -= (x - cos_z) / sin_z;
            }
            z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atan2() {
        assert_close!(
            dd!("0.46364760900080611621425623146121"),
            Double::ONE.atan2(dd!(2))
        );
        assert_close!(
            dd!("2.6779450445889871222483871518183"),
            Double::ONE.atan2(dd!(-2))
        );
        assert_close!(
            dd!("-0.46364760900080611621425623146121"),
            -Double::ONE.atan2(dd!(2))
        );
        assert_close!(
            dd!("-2.6779450445889871222483871518183"),
            -Double::ONE.atan2(dd!(-2))
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::ZERO.atan2(Double::ZERO));
        assert_exact!(Double::ZERO, Double::ZERO.atan2(Double::ONE));
        assert_close!(Double::PI, Double::ZERO.atan2(-Double::ONE));
        assert_close!(Double::FRAC_PI_2, Double::ONE.atan2(Double::ZERO));
        assert_close!(-Double::FRAC_PI_2, -Double::ONE.atan2(Double::ZERO));
    }

    #[test]
    fn one() {
        assert_close!(Double::FRAC_PI_4, Double::ONE.atan2(Double::ONE));
        assert_close!(-Double::FRAC_3_PI_4, -Double::ONE.atan2(-Double::ONE));
        assert_close!(Double::FRAC_3_PI_4, Double::ONE.atan2(-Double::ONE));
        assert_close!(-Double::FRAC_PI_4, -Double::ONE.atan2(Double::ONE));
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.atan2(Double::INFINITY));
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan2(Double::ONE));
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan2(Double::ONE));
        assert_exact!(Double::ZERO, Double::ONE.atan2(Double::INFINITY));
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.atan2(Double::ONE));
        assert_exact!(Double::NAN, Double::ONE.atan2(Double::NAN));
        assert_exact!(Double::NAN, Double::NAN.atan2(Double::NAN));
    }
}
