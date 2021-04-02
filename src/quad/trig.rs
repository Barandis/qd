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

// Computes both the sine and cosine of a using the Taylor series. This is a bit quicker
// than calling the two functions above separately, since if you have one of them you can
// calculate the other more efficiently.
fn sincos_taylor(a: Quad) -> (Quad, Quad) {
    if a.is_zero() {
        (Quad::ZERO, Quad::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Quad::ONE - sin_a.sqr()).sqrt())
    }
}

// Helper function to reduce the input to a value whose sin/cos can be calculated via Taylor
// series. It firsts reduces modulo 2π, then π/2, then π/1024. Aside from returning the
// reduced value (`t`), it also returns the group within the next higher modulo in which the
// value fell (`j` and `k`, this is the quadrant for `j`).
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

    // sin tests
    test_all_near!(
        sin_zero:
            Quad::ZERO,
            Quad::ZERO.sin();
        sin_pi_2:
            Quad::ONE,
            Quad::FRAC_PI_2.sin();
        sin_pi:
            Quad::ZERO,
            Quad::PI.sin();
        sin_3_pi_2:
            Quad::NEG_ONE,
            Quad::FRAC_3_PI_2.sin();
    );
    test_all_near!(
        sin_one:
            qd!("0.84147098480789650665250232163029899962256306079837106567275170999192"),
            Quad::ONE.sin();
        sin_pi_6:
            qd!("0.5"),
            Quad::FRAC_PI_6.sin();
        sin_e:
            qd!("0.41078129050290869547600949201836059188830697039341534530457165880643"),
            Quad::E.sin();
        sin_5_pi_4:
            qd!("-0.70710678118654752440084436210484903928483593768847403658833986899397"),
            Quad::FRAC_5_PI_4.sin();
        sin_2e:
            qd!("-0.74904646822291702360901060145877281237145151015215871652540204145216"),
            (Quad::E + Quad::E).sin();
        sin_7_pi_3:
            qd!("0.8660254037844386467637231707529361834714026269051903140279034897264"),
            (Quad::TAU + Quad::FRAC_PI_3).sin();
        sin_neg_one:
            qd!("-0.84147098480789650665250232163029899962256306079837106567275170999192"),
            Quad::NEG_ONE.sin();
        sin_neg_pi_6:
            qd!("-0.5"),
            (-Quad::FRAC_PI_6).sin();
        sin_neg_e:
            qd!("-0.41078129050290869547600949201836059188830697039341534530457165880643"),
            (-Quad::E).sin();
        sin_neg_5_pi_4:
            qd!("0.70710678118654752440084436210484903928483593768847403658833986899397"),
            (-Quad::FRAC_5_PI_4).sin();
        sin_neg_2e:
            qd!("0.74904646822291702360901060145877281237145151015215871652540204145216"),
            (-Quad::E - Quad::E).sin();
        sin_neg_7_pi_3:
            qd!("-0.8660254037844386467637231707529361834714026269051903140279034897264"),
            (-Quad::TAU - Quad::FRAC_PI_3).sin();
        sin_150:
            qd!("-0.71487642962916463143638609739662998937292172507126621479610892999516"),
            qd!(150).sin();
        sin_neg_140:
            qd!("-0.98023965944031151566962646061837215778826865408679490002662721963484"),
            qd!(-140).sin();
    );
    test_all_exact!(
        sin_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.sin();
        sin_inf:
            Quad::NAN,
            Quad::INFINITY.sin();
        sin_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.sin();
        sin_nan:
            Quad::NAN,
            Quad::NAN.sin();
    );

    // cos tests
    test_all_near!(
        cos_zero:
            Quad::ONE,
            Quad::ZERO.cos();
        cos_pi_2:
            Quad::ZERO,
            Quad::FRAC_PI_2.cos();
        cos_pi:
            Quad::NEG_ONE,
            Quad::PI.cos();
        cos_3_pi_2:
            Quad::ZERO,
            Quad::FRAC_3_PI_2.cos();
    );
    test_all_near!(
        cos_one:
            qd!("0.54030230586813971740093660744297660373231042061792222767009725538107"),
            Quad::ONE.cos();
        cos_pi_6:
            qd!("0.8660254037844386467637231707529361834714026269051903140279034897264"),
            Quad::FRAC_PI_6.cos();
        cos_e:
            qd!("-0.911733914786965097893717317805431845250413429215695401335640464733"),
            Quad::E.cos();
        cos_5_pi_4:
            qd!("-0.70710678118654752440084436210484903928483593768847403658833986899694"),
            Quad::FRAC_5_PI_4.cos();
        cos_2e:
            qd!("0.66251746274552986877475631529504922930839426893614457876918234325527"),
            (Quad::E + Quad::E).cos();
        cos_7_pi_3:
            qd!("0.5"),
            (Quad::TAU + Quad::FRAC_PI_3).cos();
        cos_neg_one:
            qd!("0.54030230586813971740093660744297660373231042061792222767009725538107"),
            Quad::NEG_ONE.cos();
        cos_neg_pi_6:
            qd!("0.8660254037844386467637231707529361834714026269051903140279034897264"),
            (-Quad::FRAC_PI_6).cos();
        cos_neg_e:
            qd!("-0.911733914786965097893717317805431845250413429215695401335640464733"),
            (-Quad::E).cos();
        cos_neg_5_pi_4:
            qd!("-0.70710678118654752440084436210484903928483593768847403658833986899694"),
            (-Quad::FRAC_5_PI_4).cos();
        cos_neg_2e:
            qd!("0.66251746274552986877475631529504922930839426893614457876918234325527"),
            (-Quad::E - Quad::E).cos();
        cos_neg_7_pi_3:
            qd!("0.5"),
            (-Quad::TAU - Quad::FRAC_PI_3).cos();
    );
    test_all_prec!(
        cos_150:
            qd!("0.69925080647837513141645161882552838949168176160550728024440292367885"),
            qd!(150).cos(),
            60;
        cos_neg_145:
            qd!("0.88386337370850022845621852749526436111574645997871066688312136066014"),
            qd!(-145).cos(),
            60;
    );
    test_all_exact!(
        cos_neg_zero:
            Quad::ONE,
            Quad::NEG_ZERO.cos();
        cos_inf:
            Quad::NAN,
            Quad::INFINITY.cos();
        cos_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.cos();
        cos_nan:
            Quad::NAN,
            Quad::NAN.cos();
    );

    // sin_cos tests
    test_all_near!(
        sin_cos_zero_sin:
            Quad::ZERO.sin(),
            Quad::ZERO.sin_cos().0;
        sin_cos_zero_cos:
            Quad::ZERO.cos(),
            Quad::ZERO.sin_cos().1;
        sin_cos_pi_2_sin:
            Quad::FRAC_PI_2.sin(),
            Quad::FRAC_PI_2.sin_cos().0;
        sin_cos_pi_2_cos:
            Quad::FRAC_PI_2.cos(),
            Quad::FRAC_PI_2.sin_cos().1;
        sin_cos_pi_sin:
            Quad::PI.sin(),
            Quad::PI.sin_cos().0;
        sin_cos_pi_cos:
            Quad::PI.cos(),
            Quad::PI.sin_cos().1;
        sin_cos_3_pi_2_sin:
            Quad::FRAC_3_PI_2.sin(),
            Quad::FRAC_3_PI_2.sin_cos().0;
        sin_cos_3_pi_2_cos:
            Quad::FRAC_3_PI_2.cos(),
            Quad::FRAC_3_PI_2.sin_cos().1;
    );
    test_all_near!(
        sin_cos_one_sin:
            Quad::ONE.sin(),
            Quad::ONE.sin_cos().0;
        sin_cos_one_cos:
            Quad::ONE.cos(),
            Quad::ONE.sin_cos().1;
        sin_cos_pi_6_sin:
            Quad::FRAC_PI_6.sin(),
            Quad::FRAC_PI_6.sin_cos().0;
        sin_cos_pi_6_cos:
            Quad::FRAC_PI_6.cos(),
            Quad::FRAC_PI_6.sin_cos().1;
        sin_cos_e_sin:
            Quad::E.sin(),
            Quad::E.sin_cos().0;
        sin_cos_e_cos:
            Quad::E.cos(),
            Quad::E.sin_cos().1;
        sin_cos_5_pi_4_sin:
            Quad::FRAC_5_PI_4.sin(),
            Quad::FRAC_5_PI_4.sin_cos().0;
        sin_cos_5_pi_4_cos:
            Quad::FRAC_5_PI_4.cos(),
            Quad::FRAC_5_PI_4.sin_cos().1;
        sin_cos_2e_sin:
            (Quad::E + Quad::E).sin(),
            (Quad::E + Quad::E).sin_cos().0;
        sin_cos_2e_cos:
            (Quad::E + Quad::E).cos(),
            (Quad::E + Quad::E).sin_cos().1;
        sin_cos_7_pi_3_sin:
            (Quad::TAU + Quad::FRAC_PI_3).sin(),
            (Quad::TAU + Quad::FRAC_PI_3).sin_cos().0;
        sin_cos_7_pi_3_cos:
            (Quad::TAU + Quad::FRAC_PI_3).cos(),
            (Quad::TAU + Quad::FRAC_PI_3).sin_cos().1;
        sin_cos_neg_one_sin:
            Quad::NEG_ONE.sin(),
            Quad::NEG_ONE.sin_cos().0;
        sin_cos_neg_one_cos:
            Quad::NEG_ONE.cos(),
            Quad::NEG_ONE.sin_cos().1;
        sin_cos_neg_pi_6_sin:
            (-Quad::FRAC_PI_6).sin(),
            (-Quad::FRAC_PI_6).sin_cos().0;
        sin_cos_neg_pi_6_cos:
            (-Quad::FRAC_PI_6).cos(),
            (-Quad::FRAC_PI_6).sin_cos().1;
        sin_cos_neg_e_sin:
            (-Quad::E).sin(),
            (-Quad::E).sin_cos().0;
        sin_cos_neg_e_cos:
            (-Quad::E).cos(),
            (-Quad::E).sin_cos().1;
        sin_cos_neg_5_pi_4_sin:
            (-Quad::FRAC_5_PI_4).sin(),
            (-Quad::FRAC_5_PI_4).sin_cos().0;
        sin_cos_neg_5_pi_4_cos:
            (-Quad::FRAC_5_PI_4).cos(),
            (-Quad::FRAC_5_PI_4).sin_cos().1;
        sin_cos_neg_2e_sin:
            (-Quad::E - Quad::E).sin(),
            (-Quad::E - Quad::E).sin_cos().0;
        sin_cos_neg_2e_cos:
            (-Quad::E - Quad::E).cos(),
            (-Quad::E - Quad::E).sin_cos().1;
        sin_cos_neg_7_pi_3_sin:
            (-Quad::TAU - Quad::FRAC_PI_3).sin(),
            (-Quad::TAU - Quad::FRAC_PI_3).sin_cos().0;
        sin_cos_neg_7_pi_3_cos:
            (-Quad::TAU - Quad::FRAC_PI_3).cos(),
            (-Quad::TAU - Quad::FRAC_PI_3).sin_cos().1;
        sin_cos_150_sin:
            qd!(150).sin(),
            qd!(150).sin_cos().0;
        sin_cos_150_cos:
            qd!(150).cos(),
            qd!(150).sin_cos().1;
        sin_cos_neg_145_sin:
            qd!(-145).sin(),
            qd!(-145).sin_cos().0;
        sin_cos_neg_145_cos:
            qd!(-145).cos(),
            qd!(-145).sin_cos().1;
    );
    test_all_exact!(
        sin_cos_neg_zero_sin:
            Quad::NEG_ZERO.sin(),
            Quad::NEG_ZERO.sin_cos().0;
        sin_cos_neg_zero_cos:
            Quad::NEG_ZERO.cos(),
            Quad::NEG_ZERO.sin_cos().1;
        sin_cos_inf_sin:
            Quad::INFINITY.sin(),
            Quad::INFINITY.sin_cos().0;
        sin_cos_inf_cos:
            Quad::INFINITY.cos(),
            Quad::INFINITY.sin_cos().1;
        sin_cos_neg_inf_sin:
            Quad::NEG_INFINITY.sin(),
            Quad::NEG_INFINITY.sin_cos().0;
        sin_cos_neg_inf_cos:
            Quad::NEG_INFINITY.cos(),
            Quad::NEG_INFINITY.sin_cos().1;
        sin_cos_nan_sin:
            Quad::NAN.sin(),
            Quad::NAN.sin_cos().0;
        sin_cos_nan_cos:
            Quad::NAN.cos(),
            Quad::NAN.sin_cos().1;
    );

    // tan tests
    test_all_near!(
        tan_zero:
            Quad::ZERO,
            Quad::ZERO.tan();
        tan_pi_4:
            Quad::ONE,
            Quad::FRAC_PI_4.tan();
        tan_3_pi_4:
            Quad::NEG_ONE,
            Quad::FRAC_3_PI_4.tan();
        tan_pi:
            Quad::ZERO,
            Quad::PI.tan();
    );
    test_all_near!(
        tan_one:
            qd!("1.5574077246549022305069748074583601730872507723815200383839466056984"),
            Quad::ONE.tan();
        tan_pi_6:
            qd!("0.57735026918962576450914878050195745564760175127012687601860232648328"),
            Quad::FRAC_PI_6.tan();
        tan_e:
            qd!("-0.45054953406980749571063417770127929443957091173203671001233561163239"),
            Quad::E.tan();
        tan_5_pi_4:
            qd!("1.0"),
            Quad::FRAC_5_PI_4.tan();
        tan_2e:
            qd!("-1.1306063769531499529943348786199875289786690773262622824786562271995"),
            (Quad::E + Quad::E).tan();
        tan_7_pi_3:
            qd!("1.7320508075688772935274463415058723669428052538103806280558069794564"),
            (Quad::TAU + Quad::FRAC_PI_3).tan();
        tan_neg_one:
            qd!("-1.5574077246549022305069748074583601730872507723815200383839466056984"),
            Quad::NEG_ONE.tan();
        tan_neg_pi_6:
            qd!("-0.57735026918962576450914878050195745564760175127012687601860232648328"),
            (-Quad::FRAC_PI_6).tan();
        tan_neg_e:
            qd!("0.45054953406980749571063417770127929443957091173203671001233561163239"),
            (-Quad::E).tan();
        tan_neg_5_pi_4:
            qd!("-1.0"),
            (-Quad::FRAC_5_PI_4).tan();
        tan_neg_2e:
            qd!("1.1306063769531499529943348786199875289786690773262622824786562271995"),
            (-Quad::E - Quad::E).tan();
        tan_neg_7_pi_3:
            qd!("-1.7320508075688772935274463415058723669428052538103806280558069794564"),
            (-Quad::TAU - Quad::FRAC_PI_3).tan();
        tan_150:
            qd!("-1.0223462354365875649863661852619364491718160692421925115947880505178"),
            qd!(150).tan();
        tan_neg_130:
            qd!("-2.532338427469323471076336945155016099803788824677643099814349339576"),
            qd!(-130).tan();
    );
    test_all_exact!(
        tan_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.tan();
        tan_inf:
            Quad::NAN,
            Quad::INFINITY.tan();
        tan_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.tan();
        tan_nan:
            Quad::NAN,
            Quad::NAN.tan();
    );

    // atan2 test
    test_all_near!(
        atan2_pos_pos:
            qd!("0.35251342177761899747085992272395350035945275021939640543781203320579"),
            Quad::ONE.atan2(Quad::E);
        atan2_pos_neg:
            qd!("2.8334235824738083026756437752454497856075323701119365074612822484066"),
            Quad::ONE.atan2(-Quad::PI);
        atan2_neg_pos:
            qd!("-0.3081690711159849357869996080340530985896370292631693135136623439018"),
            Quad::NEG_ONE.atan2(Quad::PI);
        atan2_neg_neg:
            qd!("-2.7890792318121742409917834605555493838377166491557094155371325591014"),
            Quad::NEG_ONE.atan2(-Quad::E);
        atan2_pi_6:
            qd!("0.48234790710102497548087851189637102255315375602186930768577892528851"),
            Quad::FRAC_PI_6.atan2(Quad::ONE);
        atan2_e:
            qd!("1.218282905017277621760461768915797941739131949468156505049660262948"),
            Quad::E.atan2(Quad::ONE);
        atan2_neg_e:
            qd!("-1.218282905017277621760461768915797941739131949468156505049660262948"),
            (-Quad::E).atan2(Quad::ONE);
        atan2_neg_pi_6:
            qd!("-0.48234790710102497548087851189637102255315375602186930768577892528851"),
            (-Quad::FRAC_PI_6).atan2(Quad::ONE);
    );
    test_all_near!(
        atan2_ones_pos_pos:
            Quad::FRAC_PI_4,
            Quad::ONE.atan2(Quad::ONE);
        atan2_ones_pos_neg:
            Quad::FRAC_3_PI_4,
            Quad::ONE.atan2(Quad::NEG_ONE);
        atan2_ones_neg_pos:
            -Quad::FRAC_PI_4,
            Quad::NEG_ONE.atan2(Quad::ONE);
        atan2_ones_neg_neg:
            -Quad::FRAC_3_PI_4,
            Quad::NEG_ONE.atan2(Quad::NEG_ONE);

        atan2_zero_one:
            Quad::ZERO,
            Quad::ZERO.atan2(Quad::ONE);
        atan2_zero_neg_one:
            Quad::PI,
            Quad::ZERO.atan2(Quad::NEG_ONE);
        atan2_one_zero:
            Quad::FRAC_PI_2,
            Quad::ONE.atan2(Quad::ZERO);
        atan2_neg_one_zero:
            -Quad::FRAC_PI_2,
            Quad::NEG_ONE.atan2(Quad::ZERO);

        atan2_inf_one:
            Quad::FRAC_PI_2,
            Quad::INFINITY.atan2(Quad::ONE);
        atan2_neg_inf_one:
            -Quad::FRAC_PI_2,
            Quad::NEG_INFINITY.atan2(Quad::ONE);
    );
    test_all_exact!(
        atan2_zero_zero:
            Quad::NAN,
            Quad::ZERO.atan2(Quad::ZERO);
        atan2_inf_inf:
            Quad::NAN,
            Quad::INFINITY.atan2(Quad::INFINITY);
        atan2_one_inf:
            Quad::ZERO,
            Quad::ONE.atan2(Quad::INFINITY);
        atan2_nan_one:
            Quad::NAN,
            Quad::NAN.atan2(Quad::ONE);
        atan2_one_nan:
            Quad::NAN,
            Quad::ONE.atan2(Quad::NAN);
        atan2_nan_nan:
            Quad::NAN,
            Quad::NAN.atan2(Quad::NAN);
    );

    // asin tests
    test_all_near!(
        asin_one:
            Quad::FRAC_PI_2,
            Quad::ONE.asin();
        asin_neg_one:
            -Quad::FRAC_PI_2,
            Quad::NEG_ONE.asin();
        asin_half:
            qd!("0.52359877559829887307710723054658381403286156656251763682915743205154"),
            qd!(0.5).asin();
        asin_neg_half:
            qd!("-0.52359877559829887307710723054658381403286156656251763682915743205154"),
            qd!(-0.5).asin();
        asin_pi_4:
            qd!("0.90333911076651284735893593015790303136770970711460887617465049079307"),
            Quad::FRAC_PI_4.asin();
        asin_neg_pi_4:
            qd!("-0.90333911076651284735893593015790303136770970711460887617465049079307"),
            (-Quad::FRAC_PI_4).asin();
    );
    test_all_exact!(
        asin_zero:
            Quad::ZERO,
            Quad::ZERO.asin();
        asin_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.asin();
        asin_inf:
            Quad::NAN,
            Quad::INFINITY.asin();
        asin_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.asin();
        asin_pi:
            Quad::NAN,
            Quad::PI.asin();
        asin_neg_pi:
            Quad::NAN,
            (-Quad::PI).asin();
        asin_nan:
            Quad::NAN,
            Quad::NAN.asin();
    );

    // acos tests
    test_all_near!(
        acos_zero:
            Quad::FRAC_PI_2,
            Quad::ZERO.acos();
        acos_neg_zero:
            Quad::FRAC_PI_2,
            Quad::NEG_ZERO.acos();
        acos_neg_one:
            Quad::PI,
            Quad::NEG_ONE.acos();
        acos_half:
            qd!("1.0471975511965977461542144610931676280657231331250352736583148641031"),
            qd!(0.5).acos();
        acos_neg_half:
            qd!("2.0943951023931954923084289221863352561314462662500705473166297282062"),
            qd!(-0.5).acos();
        acos_pi_4:
            qd!("0.66745721602838377187238576148184841073087499257294403431282180536097"),
            Quad::FRAC_PI_4.acos();
        acos_neg_pi_4:
            qd!("2.4741354375614094665902576217976544734662944068021617866621227869471"),
            (-Quad::FRAC_PI_4).acos();
    );
    test_all_exact!(
        acos_one:
            Quad::ZERO,
            Quad::ONE.acos();
        acos_inf:
            Quad::NAN,
            Quad::INFINITY.acos();
        acos_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.acos();
        acos_pi:
            Quad::NAN,
            Quad::PI.acos();
        acos_neg_pi:
            Quad::NAN,
            (-Quad::PI).acos();
        acos_nan:
            Quad::NAN,
            Quad::NAN.acos();
    );

    // atan tests
    test_all_near!(
        atan_pi:
            qd!("1.2626272556789116834443220836056983435089476704243835969738099522519"),
            Quad::PI.atan();
        atan_e:
            qd!("1.218282905017277621760461768915797941739131949468156505049660262948"),
            Quad::E.atan();
        atan_neg_pi:
            qd!("-1.2626272556789116834443220836056983435089476704243835969738099522519"),
            (-Quad::PI).atan();
        atan_neg_e:
            qd!("-1.218282905017277621760461768915797941739131949468156505049660262948"),
            (-Quad::E).atan();
        atan_2_pi:
            qd!("1.4129651365067377590637129498569325184935134590885018500719143289403"),
            Quad::TAU.atan();
        atan_pi_2:
            qd!("1.0038848218538872141484239449171322882921044605948705747297128241082"),
            Quad::FRAC_PI_2.atan();
        atan_sqrt_2:
            qd!("0.9553166181245092781638571025157577542434146950100054909596981293215"),
            Quad::SQRT_2.atan();
        atan_1_sqrt_2:
            qd!("0.61547970867038734106746458912399368785517000467754741952777416683254"),
            Quad::FRAC_1_SQRT_2.atan();
        atan_150:
            qd!("1.5641297588910283900821777041381460114763644584636508267920449540683"),
            qd!(150).atan();
        atan_neg_140:
            qd!("-1.5636535911254832167367110323350712639130068527674200722161387211928"),
            qd!(-140).atan();
    );
    test_all_exact!(
        atan_zero:
            Quad::ZERO,
            Quad::ZERO.atan();
        atan_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.atan();
        atan_inf:
            Quad::FRAC_PI_2,
            Quad::INFINITY.atan();
        atan_neg_inf:
            -Quad::FRAC_PI_2,
            Quad::NEG_INFINITY.atan();
        atan_nan:
            Quad::NAN,
            Quad::NAN.atan();
    );
}
