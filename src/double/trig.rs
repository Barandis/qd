// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::{tables, Double};

impl Double {
    /// Simultaneously computes the sine (sin) and the cosine (cos) of the `Double`. This is
    /// more efficient than calling the separate [`sin`] and [`cos`] functions if you need
    /// both numbers.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI / dd!(4);
    /// let (sin_x, cos_x) = x.sin_cos();
    ///
    /// let diff_sin = (sin_x - x.sin()).abs();
    /// let diff_cos = (cos_x - x.cos()).abs();
    ///
    /// assert!(diff_sin < dd!(1e-30));
    /// assert!(diff_cos < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`sin`]: #method.sin
    /// [`cos`]: #method.cos
    #[allow(clippy::many_single_char_names)]
    pub fn sin_cos(self) -> (Double, Double) {
        if self.is_zero() {
            (Double::ZERO, Double::ONE)
        } else if !self.is_finite() {
            (Double::NAN, Double::NAN)
        } else {
            let (j, k, t) = reduce(self);
            let abs_k = k.abs() as usize;

            let (sin_t, cos_t) = sincos_taylor(t);

            let (s, c) = if k == 0 {
                (sin_t, cos_t)
            } else {
                let u = tables::COSINES[abs_k - 1];
                let v = tables::SINES[abs_k - 1];
                if k > 0 {
                    (u * sin_t + v * cos_t, u * cos_t - v * sin_t)
                } else {
                    (u * sin_t - v * cos_t, u * cos_t + v * sin_t)
                }
            };

            match j {
                0 => (s, c),
                1 => (c, -s),
                -1 => (-c, s),
                _ => (-s, -c),
            }
        }
    }

    /// Computes the sine (sin) of the `Double`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is [-1, 1].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = (Double::PI / dd!(2)).sin();
    /// let expected = dd!(1);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn sin(self) -> Double {
        // Strategy:
        //
        // We choose integers a and b so that
        //
        //      x = s + aπ/2 + bπ/16
        //
        // where |s| <= π/32. Using a precomputed table of sin (kπ/16) and cos (kπ/16), we
        // can compute sin x from sin s and cos s. This greatly increases the convergence of
        // the Taylor series for sine and cosine.
        if self.is_zero() {
            Double::ZERO
        } else if !self.is_finite() {
            Double::NAN
        } else {
            let (j, k, t) = reduce(self);
            let abs_k = k.abs() as usize;

            if k == 0 {
                match j {
                    0 => sin_taylor(t),
                    1 => cos_taylor(t),
                    -1 => -cos_taylor(t),
                    _ => -sin_taylor(t),
                }
            } else {
                let u = tables::COSINES[abs_k - 1];
                let v = tables::SINES[abs_k - 1];
                let (sin_t, cos_t) = sincos_taylor(t);

                if k > 0 {
                    match j {
                        0 => u * sin_t + v * cos_t,
                        1 => u * cos_t - v * sin_t,
                        -1 => -u * cos_t + v * sin_t,
                        _ => -u * sin_t - v * cos_t,
                    }
                } else {
                    match j {
                        0 => u * sin_t - v * cos_t,
                        1 => u * cos_t + v * sin_t,
                        -1 => -u * cos_t - v * sin_t,
                        _ => -u * sin_t + v * cos_t,
                    }
                }
            }
        }
    }

    /// Computes the cosine (cos) of the `Double`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is [-1, 1].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = (Double::PI / dd!(2)).cos();
    /// let expected = dd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn cos(self) -> Double {
        if self.is_zero() {
            Double::ONE
        } else if !self.is_finite() {
            Double::NAN
        } else {
            let (j, k, t) = reduce(self);
            let abs_k = k.abs() as usize;

            if k == 0 {
                match j {
                    0 => cos_taylor(t),
                    1 => -sin_taylor(t),
                    -1 => sin_taylor(t),
                    _ => -cos_taylor(t),
                }
            } else {
                let u = tables::COSINES[abs_k - 1];
                let v = tables::SINES[abs_k - 1];
                let (sin_t, cos_t) = sincos_taylor(t);

                if k > 0 {
                    match j {
                        0 => u * cos_t - v * sin_t,
                        1 => -u * sin_t - v * cos_t,
                        -1 => u * sin_t + v * cos_t,
                        _ => -u * cos_t + v * sin_t,
                    }
                } else {
                    match j {
                        0 => u * cos_t + v * sin_t,
                        1 => v * cos_t - u * sin_t,
                        -1 => u * sin_t - v * cos_t,
                        _ => -u * cos_t - v * sin_t,
                    }
                }
            }
        }
    }

    /// Computes the tangent (tan) of the `Double`.
    ///
    /// The domain and range of this function are both (-∞, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = (Double::PI / dd!(4)).tan();
    /// let expected = dd!(1);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn tan(self) -> Double {
        let (s, c) = self.sin_cos();
        s / c
    }

    /// Computes the 2-argument inverse tangent (tan<sup>-1</sup>) of this `Double` and
    /// another `Double`.
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
    /// where *y* is the `Double` that `atan2` is called on and *x* is the argument.
    /// Therefore `Double::ONE.atan2(Double::ONE)` is π/4 (first quadrant), but flipping
    /// both signs to `(Double::NEG_ONE).atan2(Double::NEG_ONE)` gives the -3π/4 result
    /// (third quadrant).
    ///
    /// This function extends the range of the result to [-π, π].
    ///
    /// Because this function deals with angles around the origin and Cartesian coordinates,
    /// it's very useful for converting between Cartesian and polar coordinates.
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

    /// Computes the inverse sine (sin<sup>-1</sup>) of the `Double`. The domain of this
    /// function is [-1, 1] while the range is [-π/2, π/2]. Arguments outside of this domain
    /// will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).asin();
    /// let expected = Double::PI / dd!(2);  // π/2
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn asin(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::FRAC_PI_2
        } else if self == Double::NEG_ONE {
            -Double::FRAC_PI_2
        } else {
            self.atan2((Double::ONE - self.sqr()).sqrt())
        }
    }

    /// Computes the inverse cosine (cos<sup>-1</sup>) of the `Double`. The domain of this
    /// function is [-1, 1] and the range is [0, π]. Arguments outside of the domain will
    /// result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).acos();
    /// let expected = dd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acos(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::ZERO
        } else if self == Double::NEG_ONE {
            Double::PI
        } else {
            (Double::ONE - self.sqr()).sqrt().atan2(self)
        }
    }

    /// Computes the inverse tangent (tan<sup>-1</sup>) of the `Double`. The domain of this
    /// function is [-∞, ∞] and the range is [-π/2, π/2].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).atan();
    /// let expected = Double::PI / dd!(4);  // π/4
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn atan(self) -> Double {
        self.atan2(Double::ONE)
    }
}

// Compute sin a using the Taylor series. This assumes that |a| <= π/32.
#[allow(clippy::many_single_char_names)]
fn sin_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ZERO
    } else {
        let threshold = (a.abs() * Double::EPSILON).mul_pwr2(0.5);
        let x = -a.sqr();
        let mut s = a;
        let mut r = a;
        let mut i = 0;

        loop {
            r *= x;
            let t = r * tables::INV_FACTS[i];
            s += t;
            i += 2;
            if i >= tables::INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Compute cos a using the Taylor series. This assumes that |a| <= π/32.
#[allow(clippy::many_single_char_names)]
fn cos_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ONE
    } else {
        let threshold = Double::EPSILON.mul_pwr2(0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Double::ONE + r.mul_pwr2(0.5);
        let mut i = 1;

        loop {
            r *= x;
            let t = r * tables::INV_FACTS[i];
            s += t;
            i += 2;
            if i >= tables::INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Computes both the sine and cosine of a using the Taylor series. This is a bit quicker
// than calling the two functions above separately, since if you have one of them you can
// calculate the other more efficiently.
fn sincos_taylor(a: Double) -> (Double, Double) {
    if a.is_zero() {
        (Double::ZERO, Double::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Double::ONE - sin_a.sqr()).sqrt())
    }
}

// Helper function to reduce the input to a value whose sin/cos can be calculated via Taylor
// series. It firsts reduces modulo 2π, then π/2, then π/16. Aside from returning the
// reduced value (`t`), it also returns the group within the next higher modulo in which the
// value fell (`j` and `k`, this is the quadrant for `j`).
#[inline]
#[allow(clippy::many_single_char_names)]
fn reduce(a: Double) -> (i32, i32, Double) {
    // reduce modulo 2π
    let z = (a / Double::MUL_2_PI).round();
    let r = a - z * Double::MUL_2_PI;

    // reduce modulo π/2
    let mut q = (r.0 / Double::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - Double(q, 0.0) * Double::FRAC_PI_2;
    let j = q as i32;

    // reduce modulo π/16
    q = (t.0 / Double::FRAC_PI_16.0 + 0.5).floor();
    t -= Double(q, 0.0) * Double::FRAC_PI_16;
    let k = q as i32;

    (j, k, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_cos() {
        let (s, c) = dd!(1).sin_cos();
        assert_close!(dd!("0.84147098480789650665250232163030"), s);
        assert_close!(dd!("0.54030230586813971740093660744298"), c);

        let (s, c) = dd!(Double::PI / dd!(4)).sin_cos();
        assert_close!(dd!("0.70710678118654752440084436210485"), s);
        assert_close!(dd!("0.70710678118654752440084436210485"), c);
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());

        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin_cos().0);
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.sin_cos().1);
    }

    #[test]
    fn sin_cos_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sin_cos().0);
        assert_exact!(Double::ONE, Double::ZERO.sin_cos().1);
    }

    #[test]
    fn sin_cos_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().1);

        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().1);
    }

    #[test]
    fn sin_cos_nan() {
        assert_exact!(Double::NAN, Double::NAN.sin_cos().0);
        assert_exact!(Double::NAN, Double::NAN.sin_cos().1);
    }

    #[test]
    fn sin() {
        assert_close!(dd!("0.84147098480789650665250232163030"), dd!(1).sin());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / dd!(4)).sin()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());
        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin());
    }

    #[test]
    fn sin_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sin());
    }

    #[test]
    fn sin_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.sin());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin());
    }

    #[test]
    fn sin_nan() {
        assert_exact!(Double::NAN, Double::NAN.sin());
    }

    #[test]
    fn cos() {
        assert_close!(dd!("0.54030230586813971740093660744298"), dd!(1).cos());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / dd!(4)).cos()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_3.cos());
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.cos());
    }

    #[test]
    fn cos_zero() {
        assert_exact!(Double::ONE, Double::ZERO.cos());
        assert_exact!(Double::ONE, Double::NEG_ZERO.cos());
    }

    #[test]
    fn cos_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.cos());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.cos());
    }

    #[test]
    fn cos_nan() {
        assert_exact!(Double::NAN, Double::NAN.cos());
    }

    #[test]
    fn tan() {
        assert_close!(dd!("1.5574077246549022305069748074584"), dd!(1).tan());
        assert_close!(dd!(1), Double::FRAC_PI_4.tan());
        assert!(Double::FRAC_PI_2.tan().is_infinite());
    }

    #[test]
    fn tan_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.tan());
    }

    #[test]
    fn tan_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.tan());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.tan());
    }

    #[test]
    fn tan_nan() {
        assert_exact!(Double::NAN, Double::NAN.tan());
    }

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
            Double::NEG_ONE.atan2(dd!(2))
        );
        assert_close!(
            dd!("-2.6779450445889871222483871518183"),
            Double::NEG_ONE.atan2(dd!(-2))
        );
    }

    #[test]
    fn atan2_zero() {
        assert_exact!(Double::NAN, Double::ZERO.atan2(Double::ZERO));
        assert_exact!(Double::ZERO, Double::ZERO.atan2(Double::ONE));
        assert_close!(Double::PI, Double::ZERO.atan2(Double::NEG_ONE));
        assert_close!(Double::FRAC_PI_2, Double::ONE.atan2(Double::ZERO));
        assert_close!(-Double::FRAC_PI_2, Double::NEG_ONE.atan2(Double::ZERO));
    }

    #[test]
    fn atan2_one() {
        assert_close!(Double::FRAC_PI_4, Double::ONE.atan2(Double::ONE));
        assert_close!(-Double::FRAC_3_PI_4, Double::NEG_ONE.atan2(Double::NEG_ONE));
        assert_close!(Double::FRAC_3_PI_4, Double::ONE.atan2(Double::NEG_ONE));
        assert_close!(-Double::FRAC_PI_4, Double::NEG_ONE.atan2(Double::ONE));
    }

    #[test]
    fn atan2_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.atan2(Double::INFINITY));
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan2(Double::ONE));
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan2(Double::ONE));
        assert_exact!(Double::ZERO, Double::ONE.atan2(Double::INFINITY));
    }

    #[test]
    fn atan2_nan() {
        assert_exact!(Double::NAN, Double::NAN.atan2(Double::ONE));
        assert_exact!(Double::NAN, Double::ONE.atan2(Double::NAN));
        assert_exact!(Double::NAN, Double::NAN.atan2(Double::NAN));
    }

    #[test]
    fn asin() {
        assert_close!(dd!("0.52359877559829887307710723054658"), dd!(0.5).asin());
        assert_close!(Double::FRAC_PI_2, dd!(1).asin());
        assert_close!(-Double::FRAC_PI_2, dd!(-1).asin());
    }

    #[test]
    fn asin_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.asin());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.asin());
    }

    #[test]
    fn asin_nan() {
        assert_exact!(Double::NAN, Double::NAN.asin());
        assert_exact!(Double::NAN, dd!(1.5).asin());
        assert_exact!(Double::NAN, dd!(-1.5).asin());
    }

    #[test]
    fn acos() {
        assert_close!(dd!("1.0471975511965977461542144610932"), dd!(0.5).acos());
        assert_exact!(Double::ZERO, dd!(1).acos());
        assert_close!(Double::PI, dd!(-1).acos());
    }

    #[test]
    fn acos_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.acos());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.acos());
    }

    #[test]
    fn acos_nan() {
        assert_exact!(Double::NAN, Double::NAN.acos());
        assert_exact!(Double::NAN, dd!(1.5).acos());
        assert_exact!(Double::NAN, dd!(-1.5).acos());
    }

    #[test]
    fn atan() {
        assert_close!(dd!("0.98279372324732906798571061101467"), dd!(1.5).atan());
        assert_close!(Double::FRAC_PI_4, dd!(1).atan());
    }

    #[test]
    fn atan_zero() {
        assert_exact!(Double::ZERO, dd!(0).atan());
    }

    #[test]
    fn atan_inf() {
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan());
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan());
    }

    #[test]
    fn atan_nan() {
        assert_exact!(Double::NAN, Double::NAN.atan());
    }
}
