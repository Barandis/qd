// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common as c;
use crate::quad::Quad;

impl Quad {
    /// Simultaneously computes the sine (sin) and the cosine (cos) of the `Quad`. This is
    /// more efficient than calling the separate [`sin`] and [`cos`] functions if you need
    /// both numbers.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI / qd!(4);
    /// let (sin_x, cos_x) = x.sin_cos();
    ///
    /// let diff_sin = (sin_x - x.sin()).abs();
    /// let diff_cos = (cos_x - x.cos()).abs();
    ///
    /// assert!(diff_sin < qd!(1e-60));
    /// assert!(diff_cos < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [`sin`]: #method.sin
    /// [`cos`]: #method.cos
    #[allow(clippy::many_single_char_names)]
    pub fn sin_cos(self) -> (Quad, Quad) {
        match self.pre_sin_cos() {
            Some(r) => r,
            None => {
                let (j, k, t) = reduce(self);
                let abs_k = k.abs() as usize;

                let (sin_t, cos_t) = sincos_taylor(t);

                let (s, c) = if k == 0 {
                    (sin_t, cos_t)
                } else {
                    let u = c::COSINES[abs_k - 1];
                    let v = c::SINES[abs_k - 1];
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
    }

    /// Computes the sine (sin) of the `Quad`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is [-1, 1].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).sin();
    /// let expected = qd!("0.8414709848078965066525023216302989996225630607983710656727517099884");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn sin(self) -> Quad {
        match self.pre_sin() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // We choose integers a and b so that
                //
                //      x = s + aπ/2 + bπ/1024
                //
                // where |s| <= π/2048. Using a precomputed table of sin (kπ/1024) and cos
                // (kπ/1024), we can compute sin x from sin s and cos s. This greatly
                // increases the convergence of the Taylor series for sine and cosine.
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
                    let u = c::COSINES[abs_k - 1];
                    let v = c::SINES[abs_k - 1];
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
    }

    /// Computes the cosine (cos) of the `Quad`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is [-1, 1].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).cos();
    /// let expected = qd!("0.5403023058681397174009366074429766037323104206179222276700972553787");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn cos(self) -> Quad {
        match self.pre_cos() {
            Some(r) => r,
            None => {
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
                    let u = c::COSINES[abs_k - 1];
                    let v = c::SINES[abs_k - 1];
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
    }

    /// Computes the tangent (tan) of the `Quad`.
    ///
    /// The domain and range of this function are both (-∞, ∞).
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
        match self.pre_atan2(&other) {
            Some(r) => r,
            None => {
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
                // Here, x and y are normalized so that x² + y² = 1. If |x| > |y|, the first
                // iteration is used since the denominator is larger. Otherwise the second
                // is used.
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

    /// Computes the inverse sine (sin<sup>-1</sup>) of the `Quad`. The domain of this
    /// function is [-1, 1] while the range is [-π/2, π/2]. Arguments outside of this domain
    /// will result in [`NAN`].
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
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn asin(self) -> Quad {
        match self.pre_asin() {
            Some(r) => r,
            None => self.atan2((Quad::ONE - self.sqr()).sqrt()),
        }
    }

    /// Computes the inverse cosine (cos<sup>-1</sup>) of the `Quad`. The domain of this
    /// function is [-1, 1] and the range is [0, π]. Arguments outside of the domain will
    /// result in [`NAN`].
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
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acos(self) -> Quad {
        match self.pre_acos() {
            Some(r) => r,
            None => (Quad::ONE - self.sqr()).sqrt().atan2(self),
        }
    }

    /// Computes the inverse tangent (tan<sup>-1</sup>) of the `Quad`. The domain of this
    /// function is [-∞, ∞] and the range is [-π/2, π/2].
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

    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_sin_cos(&self) -> Option<(Quad, Quad)> {
        if self.is_zero() {
            Some((Quad::ZERO, Quad::ONE))
        } else if !self.is_finite() {
            Some((Quad::NAN, Quad::NAN))
        } else {
            None
        }
    }

    #[inline]
    fn pre_sin(&self) -> Option<Quad> {
        if self.is_zero() {
            Some(Quad::ZERO)
        } else if !self.is_finite() {
            Some(Quad::NAN)
        } else {
            None
        }
    }

    #[inline]
    fn pre_cos(&self) -> Option<Quad> {
        if self.is_zero() {
            Some(Quad::ONE)
        } else if !self.is_finite() {
            Some(Quad::NAN)
        } else {
            None
        }
    }

    #[inline]
    fn pre_atan2(&self, other: &Quad) -> Option<Quad> {
        if other.is_zero() {
            if self.is_zero() {
                Some(Quad::NAN)
            } else if self.is_sign_positive() {
                Some(Quad::FRAC_PI_2)
            } else {
                Some(-Quad::FRAC_PI_2)
            }
        } else if self.is_zero() {
            if other.is_sign_positive() {
                Some(Quad::ZERO)
            } else {
                Some(Quad::PI)
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Some(Quad::NAN)
            } else if self.is_sign_positive() {
                Some(Quad::FRAC_PI_2)
            } else {
                Some(-Quad::FRAC_PI_2)
            }
        } else if other.is_infinite() {
            Some(Quad::ZERO)
        } else if self.is_nan() || other.is_nan() {
            Some(Quad::NAN)
        } else if *self == *other {
            if self.is_sign_positive() {
                Some(Quad::FRAC_PI_4)
            } else {
                Some(-Quad::FRAC_3_PI_4)
            }
        } else if *self == -*other {
            if self.is_sign_positive() {
                Some(Quad::FRAC_3_PI_4)
            } else {
                Some(-Quad::FRAC_PI_4)
            }
        } else {
            None
        }
    }

    #[inline]
    fn pre_asin(&self) -> Option<Quad> {
        if self.abs() > Quad::ONE {
            Some(Quad::NAN)
        } else if *self == Quad::ONE {
            Some(Quad::FRAC_PI_2)
        } else if *self == Quad::NEG_ONE {
            Some(-Quad::FRAC_PI_2)
        } else {
            None
        }
    }

    #[inline]
    fn pre_acos(&self) -> Option<Quad> {
        if self.abs() > Quad::ONE {
            Some(Quad::NAN)
        } else if *self == Quad::ONE {
            Some(Quad::ZERO)
        } else if *self == Quad::NEG_ONE {
            Some(Quad::PI)
        } else {
            None
        }
    }
}

const FRAC_PI_1024: Quad = Quad(
    3.067_961_575_771_282_3e-3,
    1.195_944_139_792_337e-19,
    -2.924_579_892_303_066e-36,
    1.086_381_075_061_875_9e-52,
);

// Compute sin a using the Taylor series. This assumes that |a| <= π/2048.
#[allow(clippy::many_single_char_names)]
fn sin_taylor(a: Quad) -> Quad {
    if a.is_zero() {
        Quad::ZERO
    } else {
        let threshold = c::mul_pwr2(Quad::EPSILON * a.abs(), 0.5);
        let x = -a.sqr();
        let mut s = a;
        let mut r = a;
        let mut i = 0;

        loop {
            r *= x;
            let t = r * c::INV_FACTS[i];
            s += t;
            i += 2;
            if i >= c::INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Compute cos a using the Taylor series. This assumes that |a| <= π/2048.
#[allow(clippy::many_single_char_names)]
fn cos_taylor(a: Quad) -> Quad {
    if a.is_zero() {
        Quad::ONE
    } else {
        let threshold = c::mul_pwr2(Quad::EPSILON, 0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Quad::ONE + c::mul_pwr2(r, 0.5);
        let mut i = 1;

        loop {
            r *= x;
            let t = r * c::INV_FACTS[i];
            s += t;
            i += 2;
            if i >= c::INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Computes both the sine and cosine of a using the Taylor series. This is a bit
// quicker than calling the two functions above separately, since if you have
// one of them you can calculate the other more efficiently.
fn sincos_taylor(a: Quad) -> (Quad, Quad) {
    if a.is_zero() {
        (Quad::ZERO, Quad::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Quad::ONE - sin_a.sqr()).sqrt())
    }
}

// Helper function to reduce the input to a value whose sin/cos can be
// calculated via Taylor series. It firsts reduces modulo 2π, then π/2, then
// π/1024. Aside from returning the reduced value (`t`), it also returns the
// group within the next higher modulo in which the value fell (`j` and `k`,
// this is the quadrant for `j`).
#[inline]
#[allow(clippy::many_single_char_names)]
fn reduce(a: Quad) -> (i32, i32, Quad) {
    // reduce modulo 2π
    let z = (a / Quad::TAU).round();
    let r = a - z * Quad::TAU;

    // reduce modulo π/2
    let mut q = (r.0 / Quad::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - Quad(q, 0.0, 0.0, 0.0) * Quad::FRAC_PI_2;
    let j = q as i32;

    // reduce modulo π/1024
    q = (t.0 / FRAC_PI_1024.0 + 0.5).floor();
    t -= Quad(q, 0.0, 0.0, 0.0) * FRAC_PI_1024;
    let k = q as i32;

    (j, k, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_cos() {
        let (s, c) = qd!(1).sin_cos();
        assert_close!(
            qd!("0.8414709848078965066525023216302989996225630607983710656727517100"),
            s
        );
        assert_close!(
            qd!("0.5403023058681397174009366074429766037323104206179222276700972554"),
            c
        );
        let (s, c) = qd!(Quad::PI / qd!(4)).sin_cos();
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            s
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            c
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_6.sin());

        assert_exact!(Quad::ONE, Quad::FRAC_PI_2.sin_cos().0);
        assert_exact!(Quad::ZERO, Quad::FRAC_PI_2.sin_cos().1);
    }

    #[test]
    fn sin_cos_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sin_cos().0);
        assert_exact!(Quad::ONE, Quad::ZERO.sin_cos().1);
    }

    #[test]
    fn sin_cos_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.sin_cos().0);
        assert_exact!(Quad::NAN, Quad::INFINITY.sin_cos().1);

        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sin_cos().0);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sin_cos().1);
    }

    #[test]
    fn sin_cos_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sin_cos().0);
        assert_exact!(Quad::NAN, Quad::NAN.sin_cos().1);
    }

    #[test]
    fn sin() {
        assert_close!(
            qd!("0.8414709848078965066525023216302989996225630607983710656727517100"),
            qd!(1).sin()
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            (Quad::PI / qd!(4)).sin()
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_6.sin());
        assert_exact!(Quad::ONE, Quad::FRAC_PI_2.sin());
    }

    #[test]
    fn sin_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sin());
    }

    #[test]
    fn sin_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.sin());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sin());
    }

    #[test]
    fn sin_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sin());
    }

    #[test]
    fn cos() {
        assert_close!(
            qd!("0.5403023058681397174009366074429766037323104206179222276700972554"),
            qd!(1).cos()
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            (Quad::PI / qd!(4)).cos()
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_3.cos());
        assert_exact!(Quad::ZERO, Quad::FRAC_PI_2.cos());
    }

    #[test]
    fn cos_zero() {
        assert_exact!(Quad::ONE, Quad::ZERO.cos());
        assert_exact!(Quad::ONE, Quad::NEG_ZERO.cos());
    }

    #[test]
    fn cos_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.cos());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.cos());
    }

    #[test]
    fn cos_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.cos());
    }

    #[test]
    fn tan() {
        assert_close!(
            qd!("1.557407724654902230506974807458360173087250772381520038383946606"),
            qd!(1).tan()
        );
        assert_close!(qd!(1), Quad::FRAC_PI_4.tan());
        assert!(Quad::FRAC_PI_2.tan().is_infinite());
    }

    #[test]
    fn tan_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.tan());
    }

    #[test]
    fn tan_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.tan());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.tan());
    }

    #[test]
    fn tan_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.tan());
    }

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
    fn atan2_zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.atan2(Quad::ZERO));
        assert_exact!(Quad::ZERO, Quad::ZERO.atan2(Quad::ONE));
        assert_close!(Quad::PI, Quad::ZERO.atan2(Quad::NEG_ONE));
        assert_close!(Quad::FRAC_PI_2, Quad::ONE.atan2(Quad::ZERO));
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_ONE.atan2(Quad::ZERO));
    }

    #[test]
    fn atan2_one() {
        assert_close!(Quad::FRAC_PI_4, Quad::ONE.atan2(Quad::ONE));
        assert_close!(-Quad::FRAC_3_PI_4, Quad::NEG_ONE.atan2(Quad::NEG_ONE));
        assert_close!(Quad::FRAC_3_PI_4, Quad::ONE.atan2(Quad::NEG_ONE));
        assert_close!(-Quad::FRAC_PI_4, Quad::NEG_ONE.atan2(Quad::ONE));
    }

    #[test]
    fn atan2_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.atan2(Quad::INFINITY));
        assert_close!(Quad::FRAC_PI_2, Quad::INFINITY.atan2(Quad::ONE));
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_INFINITY.atan2(Quad::ONE));
        assert_exact!(Quad::ZERO, Quad::ONE.atan2(Quad::INFINITY));
    }

    #[test]
    fn atan2_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.atan2(Quad::ONE));
        assert_exact!(Quad::NAN, Quad::ONE.atan2(Quad::NAN));
        assert_exact!(Quad::NAN, Quad::NAN.atan2(Quad::NAN));
    }

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
    fn asin_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.asin());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.asin());
    }

    #[test]
    fn asin_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.asin());
        assert_exact!(Quad::NAN, qd!(1.5).asin());
        assert_exact!(Quad::NAN, qd!(-1.5).asin());
    }

    #[test]
    fn acos() {
        assert_close!(
            qd!("1.047197551196597746154214461093167628065723133125035273658314864"),
            qd!(0.5).acos()
        );
        assert_exact!(Quad::ZERO, qd!(1).acos());
        assert_close!(Quad::PI, qd!(-1).acos());
    }

    #[test]
    fn acos_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.acos());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.acos());
    }

    #[test]
    fn acos_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.acos());
        assert_exact!(Quad::NAN, qd!(1.5).acos());
        assert_exact!(Quad::NAN, qd!(-1.5).acos());
    }

    #[test]
    fn atan() {
        assert_close!(
            qd!("0.9827937232473290679857106110146660144968774536316285567614250883"),
            qd!(1.5).atan()
        );
        assert_close!(Quad::FRAC_PI_4, qd!(1).atan());
    }

    #[test]
    fn atan_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.atan());
    }

    #[test]
    fn atan_inf() {
        assert_close!(Quad::FRAC_PI_2, Quad::INFINITY.atan());
        assert_close!(-Quad::FRAC_PI_2, Quad::NEG_INFINITY.atan());
    }

    #[test]
    fn atan_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.atan());
    }
}
