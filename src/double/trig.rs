// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common as c;
use crate::double::Double;

impl Double {
    /// Simultaneously computes the sine and cosine of $x$, $\sin x$ and $\cos x$, where $x$
    /// is `self`. This is more efficient than calling the separate [`sin`] and [`cos`]
    /// functions if you need both numbers.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::PI / dd!(4);
    /// let (sin_x, cos_x) = x.sin_cos();
    ///
    /// let diff_sin = (sin_x - x.sin()).abs();
    /// let diff_cos = (cos_x - x.cos()).abs();
    ///
    /// assert!(diff_sin < dd!(1e-30));
    /// assert!(diff_cos < dd!(1e-30));
    /// ```
    ///
    /// [`sin`]: #method.sin
    /// [`cos`]: #method.cos
    #[allow(clippy::many_single_char_names)]
    pub fn sin_cos(self) -> (Double, Double) {
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

    /// Computes the sine of $x$, $\sin x$, where $x$ is `self`.
    ///
    /// The domain of this function is $(-\infin, \infin)$, and the range is $[-1, 1]$.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = (Double::PI / dd!(2)).sin();
    /// let expected = dd!(1);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn sin(self) -> Double {
        match self.pre_sin() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // We choose integers a and b so that
                //
                //      x = s + aπ/2 + bπ/16
                //
                // where |s| <= π/32. Using a precomputed table of sin (kπ/16) and cos
                // (kπ/16), we can compute sin x from sin s and cos s. This greatly
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

    /// Computes the cosine of $x$, $\cos x$, where $x$ is `self`.
    ///
    /// The domain of this function is $(-\infin, \infin)$, and the range is $[-1, 1]$.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = (Double::PI / dd!(2)).cos();
    /// let expected = dd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn cos(self) -> Double {
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

    /// Computes the tangent of $x$, $\tan x$, where $x$ is `self`.
    ///
    /// The domain and range of this function are both $(-\infin, \infin)$.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = (Double::PI / dd!(4)).tan();
    /// let expected = dd!(1);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    pub fn tan(self) -> Double {
        let (s, c) = self.sin_cos();
        s / c
    }

    /// Computes the 2-argument inverse tangent of $x$ and $y$, $\arctan(y, x)$, where $x$
    /// is the argument and $y$ is `self`.
    ///
    /// The single-argument [`atan`] function always returns values in either the first (0
    /// to $\pi$/2) or fourth (0 to $-\pi$/2) quadrants. However, first-quadrant results
    /// repeat themselves in the third quadrant, and fourth-quadrant results repeat
    /// themselves in the second. For example, $\tan \frac{\pi}{4}$ is 1, but so is $\tan
    /// \frac{3\pi}{4}$. Single-argument [`atan`] cannot distinguish between these two
    /// possibilities, so it always returns the one in the range $[-\frac{\pi}{2},
    /// \frac{\pi}{2}]$.
    ///
    /// The double-argument `atan2` can return either, depending on the arguments. It
    /// essentially returns the angle between the positive x-axis and the point $(x, y)$,
    /// where $y$ is `self` and $x$ is the argument. Therefore `dd!(1).atan2(dd!(1))`
    /// returns $\frac{\pi}{4}$ (first quadrant), but flipping both signs to
    /// `dd!(-1).atan2(dd!(-1))` gives the $-\frac{3\pi}{4}$ result (third quadrant).
    ///
    /// This function extends the range of the result to $[-\pi, \pi]$.
    ///
    /// Because this function deals with angles around the origin and Cartesian coordinates,
    /// it's very useful for converting between Cartesian and polar coordinates.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let pi = Double::PI;
    ///
    /// // -π/4 radians (45 degrees clockwise)
    /// let x1 = dd!(3);
    /// let y1 = dd!(-3);
    /// let expected1 = -Double::FRAC_PI_4;
    ///
    /// // 3π/4 radians (135 degrees counter-clockwise)
    /// let x2 = dd!(-3);
    /// let y2 = dd!(3);
    /// let expected2 = Double::FRAC_3_PI_4;
    ///
    /// let diff1 = (y1.atan2(x1) - expected1).abs();
    /// let diff2 = (y2.atan2(x2) - expected2).abs();
    ///
    /// assert!(diff1 < dd!(1e-30));
    /// assert!(diff2 < dd!(1e-30));
    /// ```
    ///
    /// [`atan`]: #method.atan
    pub fn atan2(self, other: Double) -> Double {
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
                // The iteration is given by
                //      z' = z + (y - sin z) / cos z   (for the first equation)
                //      z' = z - (x - cos z) / sin z   (for the second equation)
                //
                // Here, x and y are normalized so that x² + y² = 1. If |x| > |y|, the first
                // iteration is used since the denominator is larger. Otherwise the second
                // is used.
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

    /// Computes the inverse sine of $x$, $\sin^{-1} x$, where $x$ is `self`.
    ///
    /// The domain of this function is $[-1, 1]$ while the range is $[-\frac{\pi}{2},
    /// \frac{\pi}{2}]$. Arguments outside of this domain will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = dd!(1).asin();
    /// let expected = Double::PI / dd!(2);  // π/2
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn asin(self) -> Double {
        match self.pre_asin() {
            Some(r) => r,
            None => self.atan2((Double::ONE - self.sqr()).sqrt()),
        }
    }

    /// Computes the inverse cosine of $x$, $\cos^{-1} x$, where $x$ is `self`.
    ///
    /// The domain of this function is $[-1, 1]$ and the range is $[0, \pi]$. Arguments
    /// outside of the domain will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(1).acos();
    /// let expected = dd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acos(self) -> Double {
        match self.pre_acos() {
            Some(r) => r,
            None => (Double::ONE - self.sqr()).sqrt().atan2(self),
        }
    }

    /// Computes the inverse tangent of $x$, $\tan^{-1} x$, where $x$ is `self`.
    ///
    /// This is the single-argument version of this function ([`atan2`] is the two-argument
    /// inverse tangent). Though inverse tangents have multiple answers, this function will
    /// always return the one with the lowest absolute value. `x.atan()` is the same as
    /// `x.atan2(Double::ONE)`.
    ///
    /// The domain of this function is $(-\infin, \infin)$ and the range is
    /// $[-\frac{\pi}{2}, \frac{\pi}{2}]$.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = dd!(1).atan();
    /// let expected = Double::PI / dd!(4);  // π/4
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    ///
    /// [`atan2`]: #method.atan2
    pub fn atan(self) -> Double {
        self.atan2(Double::ONE)
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
    fn pre_sin_cos(&self) -> Option<(Double, Double)> {
        if self.is_zero() {
            Some((Double::ZERO, Double::ONE))
        } else if !self.is_finite() {
            Some((Double::NAN, Double::NAN))
        } else {
            None
        }
    }

    #[inline]
    fn pre_sin(&self) -> Option<Double> {
        if self.is_zero() {
            Some(*self)
        } else if !self.is_finite() {
            Some(Double::NAN)
        } else {
            None
        }
    }

    #[inline]
    fn pre_cos(&self) -> Option<Double> {
        if self.is_zero() {
            Some(Double::ONE)
        } else if !self.is_finite() {
            Some(Double::NAN)
        } else {
            None
        }
    }

    #[inline]
    fn pre_atan2(&self, other: &Double) -> Option<Double> {
        if other.is_zero() {
            if self.is_zero() {
                Some(Double::NAN)
            } else if self.is_sign_positive() {
                Some(Double::FRAC_PI_2)
            } else {
                Some(-Double::FRAC_PI_2)
            }
        } else if self.is_zero() {
            if other.is_sign_positive() {
                Some(Double::ZERO)
            } else {
                Some(Double::PI)
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Some(Double::NAN)
            } else if self.is_sign_positive() {
                Some(Double::FRAC_PI_2)
            } else {
                Some(-Double::FRAC_PI_2)
            }
        } else if other.is_infinite() {
            Some(Double::ZERO)
        } else if self.is_nan() || other.is_nan() {
            Some(Double::NAN)
        } else if *self == *other {
            if self.is_sign_positive() {
                Some(Double::FRAC_PI_4)
            } else {
                Some(-Double::FRAC_3_PI_4)
            }
        } else if *self == -*other {
            if self.is_sign_positive() {
                Some(Double::FRAC_3_PI_4)
            } else {
                Some(-Double::FRAC_PI_4)
            }
        } else {
            None
        }
    }

    #[inline]
    fn pre_asin(&self) -> Option<Double> {
        if self.abs() > Double::ONE {
            Some(Double::NAN)
        } else if *self == Double::ONE {
            Some(Double::FRAC_PI_2)
        } else if *self == Double::NEG_ONE {
            Some(-Double::FRAC_PI_2)
        } else {
            None
        }
    }

    #[inline]
    fn pre_acos(&self) -> Option<Double> {
        if self.abs() > Double::ONE {
            Some(Double::NAN)
        } else if *self == Double::ONE {
            Some(Double::ZERO)
        } else if *self == Double::NEG_ONE {
            Some(Double::PI)
        } else {
            None
        }
    }
}

// Compute sin a using the Taylor series. This assumes that |a| <= π/32.
#[allow(clippy::many_single_char_names)]
fn sin_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ZERO
    } else {
        let threshold = c::mul_pwr2(a.abs() * Double::EPSILON, 0.5);
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

// Compute cos a using the Taylor series. This assumes that |a| <= π/32.
#[allow(clippy::many_single_char_names)]
fn cos_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ONE
    } else {
        let threshold = c::mul_pwr2(Double::EPSILON, 0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Double::ONE + c::mul_pwr2(r, 0.5);
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
    let z = (a / Double::TAU).round();
    let r = a - z * Double::TAU;

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

    // sin tests
    test_all_near!(
        sin_zero:
            Double::ZERO,
            Double::ZERO.sin();
        sin_pi_2:
            Double::ONE,
            Double::FRAC_PI_2.sin();
        sin_pi:
            Double::ZERO,
            Double::PI.sin();
        sin_3_pi_2:
            Double::NEG_ONE,
            Double::FRAC_3_PI_2.sin();
    );
    test_all_near!(
        sin_one:
            dd!("0.84147098480789650665250232163029915"),
            Double::ONE.sin();
        sin_pi_6:
            dd!("0.5"),
            Double::FRAC_PI_6.sin();
        sin_e:
            dd!("0.41078129050290869547600949201836151"),
            Double::E.sin();
        sin_5_pi_4:
            dd!("-0.70710678118654752440084436210484928"),
            Double::FRAC_5_PI_4.sin();
        sin_2e:
            dd!("-0.74904646822291702360901060145877393"),
            (Double::E + Double::E).sin();
        sin_7_pi_3:
            dd!("0.86602540378443864676372317075293693"),
            (Double::TAU + Double::FRAC_PI_3).sin();
        sin_neg_one:
            dd!("-0.84147098480789650665250232163029915"),
            Double::NEG_ONE.sin();
        sin_neg_pi_6:
            dd!("-0.5"),
            (-Double::FRAC_PI_6).sin();
        sin_neg_e:
            dd!("-0.41078129050290869547600949201836151"),
            (-Double::E).sin();
        sin_neg_5_pi_4:
            dd!("0.70710678118654752440084436210484928"),
            (-Double::FRAC_5_PI_4).sin();
        sin_neg_2e:
            dd!("0.74904646822291702360901060145877393"),
            (-Double::E - Double::E).sin();
        sin_neg_7_pi_3:
            dd!("-0.86602540378443864676372317075293693"),
            (-Double::TAU - Double::FRAC_PI_3).sin();
    );
    test_all_prec!(
        sin_150:
            dd!("-0.71487642962916463143638609739662987"),
            dd!(150).sin(),
            29;
        sin_neg_140:
            dd!("-0.98023965944031151566962646061837217"),
            dd!(-140).sin(),
            29;
    );
    test_all_exact!(
        sin_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.sin();
        sin_inf:
            Double::NAN,
            Double::INFINITY.sin();
        sin_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.sin();
        sin_nan:
            Double::NAN,
            Double::NAN.sin();
    );

    // cos tests
    test_all_near!(
        cos_zero:
            Double::ONE,
            Double::ZERO.cos();
        cos_pi_2:
            Double::ZERO,
            Double::FRAC_PI_2.cos();
        cos_pi:
            Double::NEG_ONE,
            Double::PI.cos();
        cos_3_pi_2:
            Double::ZERO,
            Double::FRAC_3_PI_2.cos();
    );
    test_all_near!(
        cos_one:
            dd!("0.54030230586813971740093660744297627"),
            Double::ONE.cos();
        cos_pi_6:
            dd!("0.86602540378443864676372317075293616"),
            Double::FRAC_PI_6.cos();
        cos_e:
            dd!("-0.91173391478696509789371731780543172"),
            Double::E.cos();
        cos_5_pi_4:
            dd!("-0.70710678118654752440084436210484851"),
            Double::FRAC_5_PI_4.cos();
        cos_2e:
            dd!("0.66251746274552986877475631529504789"),
            (Double::E + Double::E).cos();
        cos_7_pi_3:
            dd!("0.5"),
            (Double::TAU + Double::FRAC_PI_3).cos();
        cos_neg_one:
            dd!("0.54030230586813971740093660744297627"),
            Double::NEG_ONE.cos();
        cos_neg_pi_6:
            dd!("0.86602540378443864676372317075293616"),
            (-Double::FRAC_PI_6).cos();
        cos_neg_e:
            dd!("-0.91173391478696509789371731780543172"),
            (-Double::E).cos();
        cos_neg_5_pi_4:
            dd!("-0.70710678118654752440084436210484851"),
            (-Double::FRAC_5_PI_4).cos();
        cos_neg_2e:
            dd!("0.66251746274552986877475631529504789"),
            (-Double::E - Double::E).cos();
        cos_neg_7_pi_3:
            dd!("0.5"),
            (-Double::TAU - Double::FRAC_PI_3).cos();
    );
    test_all_prec!(
        cos_150:
            dd!("0.69925080647837513141645161882552848"),
            dd!(150).cos(),
            29;
        cos_neg_145:
            dd!("0.88386337370850022845621852749526465"),
            dd!(-145).cos(),
            29;
    );
    test_all_exact!(
        cos_neg_zero:
            Double::ONE,
            Double::NEG_ZERO.cos();
        cos_inf:
            Double::NAN,
            Double::INFINITY.cos();
        cos_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.cos();
        cos_nan:
            Double::NAN,
            Double::NAN.cos();
    );

    // sin_cos tests
    test_all_near!(
        sin_cos_zero_sin:
            Double::ZERO.sin(),
            Double::ZERO.sin_cos().0;
        sin_cos_zero_cos:
            Double::ZERO.cos(),
            Double::ZERO.sin_cos().1;
        sin_cos_pi_2_sin:
            Double::FRAC_PI_2.sin(),
            Double::FRAC_PI_2.sin_cos().0;
        sin_cos_pi_2_cos:
            Double::FRAC_PI_2.cos(),
            Double::FRAC_PI_2.sin_cos().1;
        sin_cos_pi_sin:
            Double::PI.sin(),
            Double::PI.sin_cos().0;
        sin_cos_pi_cos:
            Double::PI.cos(),
            Double::PI.sin_cos().1;
        sin_cos_3_pi_2_sin:
            Double::FRAC_3_PI_2.sin(),
            Double::FRAC_3_PI_2.sin_cos().0;
        sin_cos_3_pi_2_cos:
            Double::FRAC_3_PI_2.cos(),
            Double::FRAC_3_PI_2.sin_cos().1;
    );
    test_all_near!(
        sin_cos_one_sin:
            Double::ONE.sin(),
            Double::ONE.sin_cos().0;
        sin_cos_one_cos:
            Double::ONE.cos(),
            Double::ONE.sin_cos().1;
        sin_cos_pi_6_sin:
            Double::FRAC_PI_6.sin(),
            Double::FRAC_PI_6.sin_cos().0;
        sin_cos_pi_6_cos:
            Double::FRAC_PI_6.cos(),
            Double::FRAC_PI_6.sin_cos().1;
        sin_cos_e_sin:
            Double::E.sin(),
            Double::E.sin_cos().0;
        sin_cos_e_cos:
            Double::E.cos(),
            Double::E.sin_cos().1;
        sin_cos_5_pi_4_sin:
            Double::FRAC_5_PI_4.sin(),
            Double::FRAC_5_PI_4.sin_cos().0;
        sin_cos_5_pi_4_cos:
            Double::FRAC_5_PI_4.cos(),
            Double::FRAC_5_PI_4.sin_cos().1;
        sin_cos_2e_sin:
            (Double::E + Double::E).sin(),
            (Double::E + Double::E).sin_cos().0;
        sin_cos_2e_cos:
            (Double::E + Double::E).cos(),
            (Double::E + Double::E).sin_cos().1;
        sin_cos_7_pi_3_sin:
            (Double::TAU + Double::FRAC_PI_3).sin(),
            (Double::TAU + Double::FRAC_PI_3).sin_cos().0;
        sin_cos_7_pi_3_cos:
            (Double::TAU + Double::FRAC_PI_3).cos(),
            (Double::TAU + Double::FRAC_PI_3).sin_cos().1;
        sin_cos_neg_one_sin:
            Double::NEG_ONE.sin(),
            Double::NEG_ONE.sin_cos().0;
        sin_cos_neg_one_cos:
            Double::NEG_ONE.cos(),
            Double::NEG_ONE.sin_cos().1;
        sin_cos_neg_pi_6_sin:
            (-Double::FRAC_PI_6).sin(),
            (-Double::FRAC_PI_6).sin_cos().0;
        sin_cos_neg_pi_6_cos:
            (-Double::FRAC_PI_6).cos(),
            (-Double::FRAC_PI_6).sin_cos().1;
        sin_cos_neg_e_sin:
            (-Double::E).sin(),
            (-Double::E).sin_cos().0;
        sin_cos_neg_e_cos:
            (-Double::E).cos(),
            (-Double::E).sin_cos().1;
        sin_cos_neg_5_pi_4_sin:
            (-Double::FRAC_5_PI_4).sin(),
            (-Double::FRAC_5_PI_4).sin_cos().0;
        sin_cos_neg_5_pi_4_cos:
            (-Double::FRAC_5_PI_4).cos(),
            (-Double::FRAC_5_PI_4).sin_cos().1;
        sin_cos_neg_2e_sin:
            (-Double::E - Double::E).sin(),
            (-Double::E - Double::E).sin_cos().0;
        sin_cos_neg_2e_cos:
            (-Double::E - Double::E).cos(),
            (-Double::E - Double::E).sin_cos().1;
        sin_cos_neg_7_pi_3_sin:
            (-Double::TAU - Double::FRAC_PI_3).sin(),
            (-Double::TAU - Double::FRAC_PI_3).sin_cos().0;
        sin_cos_neg_7_pi_3_cos:
            (-Double::TAU - Double::FRAC_PI_3).cos(),
            (-Double::TAU - Double::FRAC_PI_3).sin_cos().1;
        sin_cos_150_sin:
            dd!(150).sin(),
            dd!(150).sin_cos().0;
        sin_cos_150_cos:
            dd!(150).cos(),
            dd!(150).sin_cos().1;
        sin_cos_neg_145_sin:
            dd!(-145).sin(),
            dd!(-145).sin_cos().0;
        sin_cos_neg_145_cos:
            dd!(-145).cos(),
            dd!(-145).sin_cos().1;
    );
    test_all_exact!(
        sin_cos_neg_zero_sin:
            Double::NEG_ZERO.sin(),
            Double::NEG_ZERO.sin_cos().0;
        sin_cos_neg_zero_cos:
            Double::NEG_ZERO.cos(),
            Double::NEG_ZERO.sin_cos().1;
        sin_cos_inf_sin:
            Double::INFINITY.sin(),
            Double::INFINITY.sin_cos().0;
        sin_cos_inf_cos:
            Double::INFINITY.cos(),
            Double::INFINITY.sin_cos().1;
        sin_cos_neg_inf_sin:
            Double::NEG_INFINITY.sin(),
            Double::NEG_INFINITY.sin_cos().0;
        sin_cos_neg_inf_cos:
            Double::NEG_INFINITY.cos(),
            Double::NEG_INFINITY.sin_cos().1;
        sin_cos_nan_sin:
            Double::NAN.sin(),
            Double::NAN.sin_cos().0;
        sin_cos_nan_cos:
            Double::NAN.cos(),
            Double::NAN.sin_cos().1;
    );

    // tan tests
    test_all_near!(
        tan_zero:
            Double::ZERO,
            Double::ZERO.tan();
        tan_pi:
            Double::ZERO,
            Double::PI.tan();
    );
    test_all_near!(
        tan_one:
            dd!("1.55740772465490223050697480745836"),
            Double::ONE.tan();
        tan_pi_6:
            dd!("0.57735026918962576450914878050195693"),
            Double::FRAC_PI_6.tan();
        tan_e:
            dd!("-0.4505495340698074957106341777012804"),
            Double::E.tan();
        tan_2e:
            dd!("-1.1306063769531499529943348786199917"),
            (Double::E + Double::E).tan();
        tan_7_pi_3:
            dd!("1.7320508075688772935274463415058754"),
            (Double::TAU + Double::FRAC_PI_3).tan();
        tan_neg_one:
            dd!("-1.55740772465490223050697480745836"),
            Double::NEG_ONE.tan();
        tan_neg_pi_6:
            dd!("-0.57735026918962576450914878050195693"),
            (-Double::FRAC_PI_6).tan();
        tan_neg_e:
            dd!("0.4505495340698074957106341777012804"),
            (-Double::E).tan();
        tan_neg_2e:
            dd!("1.1306063769531499529943348786199917"),
            (-Double::E - Double::E).tan();
        tan_neg_7_pi_3:
            dd!("-1.7320508075688772935274463415058754"),
            (-Double::TAU - Double::FRAC_PI_3).tan();
    );
    test_all_prec!(
        tan_pi_4:
            Double::ONE,
            Double::FRAC_PI_4.tan(),
            30;
        tan_3_pi_4:
            Double::NEG_ONE,
            Double::FRAC_3_PI_4.tan(),
            30;
        tan_5_pi_4:
            dd!("1.0"),
            Double::FRAC_5_PI_4.tan(),
            30;
        tan_neg_5_pi_4:
            dd!("-1.0"),
            (-Double::FRAC_5_PI_4).tan(),
            30;
        tan_150:
            dd!("-1.0223462354365875649863661852619368"),
            dd!(150).tan(),
            29;
        tan_neg_130:
            dd!("-2.5323384274693234710763369451550171"),
            dd!(-130).tan(),
            29;
    );
    test_all_exact!(
        tan_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.tan();
        tan_inf:
            Double::NAN,
            Double::INFINITY.tan();
        tan_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.tan();
        tan_nan:
            Double::NAN,
            Double::NAN.tan();
    );

    // atan2 test
    test_all_near!(
        atan2_pos_pos:
            dd!("0.35251342177761899747085992272395371"),
            Double::ONE.atan2(Double::E);
        atan2_pos_neg:
            dd!("2.8334235824738083026756437752454511"),
            Double::ONE.atan2(-Double::PI);
        atan2_neg_pos:
            dd!("-0.3081690711159849357869996080340532"),
            Double::NEG_ONE.atan2(Double::PI);
        atan2_neg_neg:
            dd!("-2.7890792318121742409917834605555499"),
            Double::NEG_ONE.atan2(-Double::E);
        atan2_pi_6:
            dd!("0.48234790710102497548087851189637084"),
            Double::FRAC_PI_6.atan2(Double::ONE);
        atan2_e:
            dd!("1.2182829050172776217604617689157985"),
            Double::E.atan2(Double::ONE);
        atan2_neg_e:
            dd!("-1.2182829050172776217604617689157985"),
            (-Double::E).atan2(Double::ONE);
        atan2_neg_pi_6:
            dd!("-0.48234790710102497548087851189637084"),
            (-Double::FRAC_PI_6).atan2(Double::ONE);
    );
    test_all_near!(
        atan2_ones_pos_pos:
            Double::FRAC_PI_4,
            Double::ONE.atan2(Double::ONE);
        atan2_ones_pos_neg:
            Double::FRAC_3_PI_4,
            Double::ONE.atan2(Double::NEG_ONE);
        atan2_ones_neg_pos:
            -Double::FRAC_PI_4,
            Double::NEG_ONE.atan2(Double::ONE);
        atan2_ones_neg_neg:
            -Double::FRAC_3_PI_4,
            Double::NEG_ONE.atan2(Double::NEG_ONE);

        atan2_zero_one:
            Double::ZERO,
            Double::ZERO.atan2(Double::ONE);
        atan2_zero_neg_one:
            Double::PI,
            Double::ZERO.atan2(Double::NEG_ONE);
        atan2_one_zero:
            Double::FRAC_PI_2,
            Double::ONE.atan2(Double::ZERO);
        atan2_neg_one_zero:
            -Double::FRAC_PI_2,
            Double::NEG_ONE.atan2(Double::ZERO);

        atan2_inf_one:
            Double::FRAC_PI_2,
            Double::INFINITY.atan2(Double::ONE);
        atan2_neg_inf_one:
            -Double::FRAC_PI_2,
            Double::NEG_INFINITY.atan2(Double::ONE);
    );
    test_all_exact!(
        atan2_zero_zero:
            Double::NAN,
            Double::ZERO.atan2(Double::ZERO);
        atan2_inf_inf:
            Double::NAN,
            Double::INFINITY.atan2(Double::INFINITY);
        atan2_one_inf:
            Double::ZERO,
            Double::ONE.atan2(Double::INFINITY);
        atan2_nan_one:
            Double::NAN,
            Double::NAN.atan2(Double::ONE);
        atan2_one_nan:
            Double::NAN,
            Double::ONE.atan2(Double::NAN);
        atan2_nan_nan:
            Double::NAN,
            Double::NAN.atan2(Double::NAN);
    );

    // asin tests
    test_all_near!(
        asin_one:
            Double::FRAC_PI_2,
            Double::ONE.asin();
        asin_neg_one:
            -Double::FRAC_PI_2,
            Double::NEG_ONE.asin();
        asin_half:
            dd!("0.52359877559829887307710723054658354"),
            dd!(0.5).asin();
        asin_neg_half:
            dd!("-0.52359877559829887307710723054658354"),
            dd!(-0.5).asin();
        asin_pi_4:
            dd!("0.90333911076651284735893593015790267"),
            Double::FRAC_PI_4.asin();
        asin_neg_pi_4:
            dd!("-0.90333911076651284735893593015790267"),
            (-Double::FRAC_PI_4).asin();
    );
    test_all_exact!(
        asin_zero:
            Double::ZERO,
            Double::ZERO.asin();
        asin_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.asin();
        asin_inf:
            Double::NAN,
            Double::INFINITY.asin();
        asin_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.asin();
        asin_pi:
            Double::NAN,
            Double::PI.asin();
        asin_neg_pi:
            Double::NAN,
            (-Double::PI).asin();
        asin_nan:
            Double::NAN,
            Double::NAN.asin();
    );

    // acos tests
    test_all_near!(
        acos_zero:
            Double::FRAC_PI_2,
            Double::ZERO.acos();
        acos_neg_zero:
            Double::FRAC_PI_2,
            Double::NEG_ZERO.acos();
        acos_neg_one:
            Double::PI,
            Double::NEG_ONE.acos();
        acos_half:
            dd!("1.0471975511965977461542144610931671"),
            dd!(0.5).acos();
        acos_neg_half:
            dd!("2.0943951023931954923084289221863342"),
            dd!(-0.5).acos();
        acos_pi_4:
            dd!("0.66745721602838377187238576148184873"),
            Double::FRAC_PI_4.acos();
        acos_neg_pi_4:
            dd!("2.4741354375614094665902576217976541"),
            (-Double::FRAC_PI_4).acos();
    );
    test_all_exact!(
        acos_one:
            Double::ZERO,
            Double::ONE.acos();
        acos_inf:
            Double::NAN,
            Double::INFINITY.acos();
        acos_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.acos();
        acos_pi:
            Double::NAN,
            Double::PI.acos();
        acos_neg_pi:
            Double::NAN,
            (-Double::PI).acos();
        acos_nan:
            Double::NAN,
            Double::NAN.acos();
    );

    // atan tests
    test_all_near!(
        atan_pi:
            dd!("1.2626272556789116834443220836056982"),
            Double::PI.atan();
        atan_e:
            dd!("1.2182829050172776217604617689157985"),
            Double::E.atan();
        atan_neg_pi:
            dd!("-1.2626272556789116834443220836056982"),
            (-Double::PI).atan();
        atan_neg_e:
            dd!("-1.2182829050172776217604617689157985"),
            (-Double::E).atan();
        atan_2_pi:
            dd!("1.4129651365067377590637129498569318"),
            Double::TAU.atan();
        atan_pi_2:
            dd!("1.0038848218538872141484239449171319"),
            Double::FRAC_PI_2.atan();
        atan_sqrt_2:
            dd!("0.9553166181245092781638571025157583"),
            Double::SQRT_2.atan();
        atan_1_sqrt_2:
            dd!("0.6154797086703873410674645891239931"),
            Double::FRAC_1_SQRT_2.atan();
        atan_150:
            dd!("1.5641297588910283900821777041381461"),
            dd!(150).atan();
        atan_neg_140:
            dd!("-1.5636535911254832167367110323350709"),
            dd!(-140).atan();
    );
    test_all_exact!(
        atan_zero:
            Double::ZERO,
            Double::ZERO.atan();
        atan_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.atan();
        atan_inf:
            Double::FRAC_PI_2,
            Double::INFINITY.atan();
        atan_neg_inf:
            -Double::FRAC_PI_2,
            Double::NEG_INFINITY.atan();
        atan_nan:
            Double::NAN,
            Double::NAN.atan();
    );
}
