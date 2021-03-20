// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the 2-argument inverse tangent (tan<sup>-1</sup>) of this `Quad` and
    /// another `Quad`.
    ///
    /// The single-argument [`atan`] function always returns values in either the first (0
    /// to π/2) or fourth (0 to -π/2) quadrants. However, first-quadrant results repeat
    /// themselves in the third quadrant, and fourth-quadrant results repeat themselves in
    /// the second. For example, the tangent of π/4 is 1, but so is the tangent of -3π/4.
    /// Single-argument [`atan`] cannot distinguish between these two possibilities, so it
    /// always returns the one in the range [-π/2, π/2].
    ///
    /// The double-argument `atan2` can return either, depending on the arguments. It
    /// essentially returns the angle between the positive x-axis and the point (x, y),
    /// where *y* is the `Quad` that `atan2` is called on and *x* is the argument. Therefore
    /// `Quad::ONE.atan2(Quad::ONE)` is π/4 (first quadrant), but flipping both signs to
    /// `(Quad::NEG_ONE).atan2(Quad::NEG_ONE)` gives the -3π/4 result (third quadrant).
    ///
    /// This function extends the range of the result to [-π, π].
    ///
    /// Because this function deals with angles around the origin and Cartesian coordinates,
    /// it's very useful for converting between Cartesian and polar coordinates.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let pi = Quad::PI;
    ///
    /// // -π/4 radians (45 degrees clockwise)
    /// let x1 = qd!(3);
    /// let y1 = qd!(-3);
    /// let expected1 = -pi / qd!(4);
    ///
    /// // 3π/4 radians (135 degrees counter-clockwise)
    /// let x2 = qd!(-3);
    /// let y2 = qd!(3);
    /// let expected2 = Quad::from(0.75) * pi;
    ///
    /// let diff1 = (y1.atan2(x1) - expected1).abs();
    /// let diff2 = (y2.atan2(x2) - expected2).abs();
    ///
    /// assert!(diff1 < qd!(1e-60));
    /// assert!(diff2 < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [`atan`]: #method.atan
    pub fn atan2(self, other: Quad) -> Quad {
        // Strategy:
        //
        // Use Newton's iteration to solve one of the following equations
        //
        //      sin z = y / r
        //      cos z = x / r
        //
        // where r = √(x² + y²).
        //
        // The iteration is given by z' = z + (y - sin z) / cos z
        //      (for the first equation) z' = z - (x - cos z) / sin z
        //      (for the second equation)
        //
        // Here, x and y are normalized so that x² + y² = 1. If |x| > |y|, the
        // first iteration is used since the denominator is larger. Otherwise
        // the second is used.

        if other.is_zero() {
            if self.is_zero() {
                Quad::NAN
            } else if self.is_sign_positive() {
                Quad::FRAC_PI_2
            } else {
                -Quad::FRAC_PI_2
            }
        } else if self.is_zero() {
            if other.is_sign_positive() {
                Quad::ZERO
            } else {
                Quad::PI
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Quad::NAN
            } else if self.is_sign_positive() {
                Quad::FRAC_PI_2
            } else {
                -Quad::FRAC_PI_2
            }
        } else if other.is_infinite() {
            Quad::ZERO
        } else if self.is_nan() || other.is_nan() {
            Quad::NAN
        } else if self == other {
            if self.is_sign_positive() {
                Quad::FRAC_PI_4
            } else {
                -Quad::FRAC_3_PI_4
            }
        } else if self == -other {
            if self.is_sign_positive() {
                Quad::FRAC_3_PI_4
            } else {
                -Quad::FRAC_PI_4
            }
        } else {
            let r = (self.sqr() + other.sqr()).sqrt();
            let x = other / r;
            let y = self / r;

            // Compute f64 approximation to atan
            let mut z = Quad::from(self.0.atan2(other.0));

            if x.0.abs() > y.0.abs() {
                // Use the first iteration above
                let (sin_z, cos_z) = z.sin_cos();
                z += (y - sin_z) / cos_z;
                let (sin_z, cos_z) = z.sin_cos();
                z += (y - sin_z) / cos_z;
                let (sin_z, cos_z) = z.sin_cos();
                z += (y - sin_z) / cos_z;
            } else {
                // Use the second iteration above
                let (sin_z, cos_z) = z.sin_cos();
                z -= (x - cos_z) / sin_z;
                let (sin_z, cos_z) = z.sin_cos();
                z -= (x - cos_z) / sin_z;
                let (sin_z, cos_z) = z.sin_cos();
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
            qd!("0.4636476090008061162142562314612144020285370542861202638109330887"),
            qd!(1).atan2(qd!(2))
        );
        assert_close!(
            qd!("2.677945044588987122248387151818288482168632345088985557164011504"),
            qd!(1).atan2(qd!(-2))
        );
        assert_close!(
            qd!("-0.4636476090008061162142562314612144020285370542861202638109330887"),
            qd!(-1).atan2(qd!(2))
        );
        assert_close!(
            qd!("-2.677945044588987122248387151818288482168632345088985557164011504"),
            qd!(-1).atan2(qd!(-2))
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.atan2(Quad::ZERO));
        assert_exact!(Quad::ZERO, Quad::ZERO.atan2(Quad::ONE));
        assert_close!(Quad::PI, Quad::ZERO.atan2(Quad::NEG_ONE));
        assert_close!(Quad::FRAC_PI_2, Quad::ONE.atan2(Quad::ZERO));
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_ONE.atan2(Quad::ZERO));
    }

    #[test]
    fn one() {
        assert_close!(Quad::FRAC_PI_4, Quad::ONE.atan2(Quad::ONE));
        assert_close!(-Quad::FRAC_3_PI_4, Quad::NEG_ONE.atan2(Quad::NEG_ONE));
        assert_close!(Quad::FRAC_3_PI_4, Quad::ONE.atan2(Quad::NEG_ONE));
        assert_close!(-Quad::FRAC_PI_4, Quad::NEG_ONE.atan2(Quad::ONE));
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::NAN, Quad::INFINITY.atan2(Quad::INFINITY));
        assert_close!(Quad::FRAC_PI_2, Quad::INFINITY.atan2(Quad::ONE));
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_INFINITY.atan2(Quad::ONE));
        assert_exact!(Quad::ZERO, Quad::ONE.atan2(Quad::INFINITY));
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.atan2(Quad::ONE));
        assert_exact!(Quad::NAN, Quad::ONE.atan2(Quad::NAN));
        assert_exact!(Quad::NAN, Quad::NAN.atan2(Quad::NAN));
    }
}
