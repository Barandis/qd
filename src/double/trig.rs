// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::{mul_pwr2, INV_FACTS};
use crate::double::Double;

const SINES: [Double; 4] = [
    Double(1.9509032201612828e-1, -7.991079068461734e-18),
    Double(3.826834323650898e-1, -1.005077269646159e-17),
    Double(5.555702330196022e-1, 4.7094109405616756e-17),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];

const COSINES: [Double; 4] = [
    Double(9.807852804032304e-1, 1.8546939997824996e-17),
    Double(9.238795325112867e-1, 1.764504708433667e-17),
    Double(8.314696123025452e-1, 1.4073856984728008e-18),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];

// Compute sin a using the Taylor series. This assumes that |a| <= π/32.
fn sin_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ZERO
    } else {
        let threshold = mul_pwr2(a.abs() * Double::EPSILON, 0.5);
        let x = -a.sqr();
        let mut s = a;
        let mut r = a;
        let mut i = 0;

        loop {
            r *= x;
            let t = r * INV_FACTS[i];
            s += t;
            i += 2;
            if i >= INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Compute cos a using the Taylor series. This assumes that |a| <= π/32.
fn cos_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ONE
    } else {
        let threshold = mul_pwr2(Double::EPSILON, 0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Double::ONE + mul_pwr2(r, 0.5);
        let mut i = 1;

        loop {
            r *= x;
            let t = r * INV_FACTS[i];
            s += t;
            i += 2;
            if i >= INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Computes both the sine and cosine of a using the Taylor series. This is a bit quicker than
// calling the two functions above separately, since if you have one of them you can calculate the
// other more efficiently.
fn sincos_taylor(a: Double) -> (Double, Double) {
    if a.is_zero() {
        (Double::ZERO, Double::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Double::ONE - sin_a.sqr()).sqrt())
    }
}

// Helper function to reduce the input to a value whose sin/cos can be calculated via Taylor series.
// It firsts reduces modulo 2π, then π/2, then π/16. Aside from returning the reduced value (`t`),
// it also returns the group within the next higher modulo in which the value fell (`j` and `k`,
// this is the quadrant for `j`).
#[inline]
fn reduce(a: Double) -> (i32, i32, Double) {
    // reduce modulo 2π
    let z = (a / Double::MUL_2_PI).round();
    let r = a - z * Double::MUL_2_PI;

    // reduce modulo π/2
    let mut q = (r.0 / Double::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - Double::from(q) * Double::FRAC_PI_2;
    let j = q as i32;

    // reduce modulo π/16
    q = (t.0 / Double::FRAC_PI_16.0 + 0.5).floor();
    t -= Double::from(q) * Double::FRAC_PI_16;
    let k = q as i32;

    (j, k, t)
}

impl Double {
    /// Computes the sine of the number.
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
    pub fn sin(self) -> Double {
        // Strategy:
        //
        // We choose integers a and b so that
        //
        //      x = s + aπ/2 + bπ/16
        //
        // where |s| <= π/32. Using a precomputed table of sin (kπ/16) and cos (kπ/16), we can
        // compute sin x from sin s and cos s. This greatly increases the convergence of the Taylor
        // series for sine and cosine.
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
                let u = COSINES[abs_k - 1];
                let v = SINES[abs_k - 1];
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

    /// Computes the cosine of the number.
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
                let u = COSINES[abs_k - 1];
                let v = SINES[abs_k - 1];
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

    /// Simultaneously computes the sine and the cosine of the number. This is more efficient if you
    /// need both numbers. Returns `(sin(x), cos(x))`.
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
                let u = COSINES[abs_k - 1];
                let v = SINES[abs_k - 1];
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

    /// Computes the tangent of the number.
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

    /// Computes the 2-argument arctangent of the number (`y`) and `other` (`x`) in radians.
    ///
    /// The second argument allows the avoidance of ambiguities in the single-argument [`atan`]
    /// function, notably allowing the determination of quadrant.
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
        //      z' = z + (y - sin z) / cos z            (for the first equation)
        //      z' = z - (x - cos z) / sin z            (for the second equation)
        //
        // Here, x and y are normalized so that x² + y² = 1. If |x| > |y|, the first iteration is
        // used since the denominator is larger. Otherwise the second is used.

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
            } else {
                if self.is_sign_positive() {
                    Double::FRAC_PI_2
                } else {
                    -Double::FRAC_PI_2
                }
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

    /// Computes the arctangent of the number. The return value is in the range [-π/2, π/2].
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

    /// Computes the arcsine of the number. The return value is in the range [-π/2, π/2] for any
    /// number in the range [-1, 1]. Otherwise the return value is `NaN`.
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
    pub fn asin(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::FRAC_PI_2
        } else if self == -Double::ONE {
            -Double::FRAC_PI_2
        } else {
            self.atan2((Double::ONE - self.sqr()).sqrt())
        }
    }

    /// Computes the arccosine of the number. The return value is in the range [0, π] for any number
    /// in the range [-1, 1]. Otherwise the return value is `NaN`.
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
    pub fn acos(self) -> Double {
        if self.abs() > Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::ZERO
        } else if self == -Double::ONE {
            Double::PI
        } else {
            (Double::ONE - self.sqr()).sqrt().atan2(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_trig_sine() {
        assert_close!(dd!("0.84147098480789650665250232163030"), dd!(1).sin());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / dd!(4)).sin()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());
        assert_exact!(Double::ZERO, Double::ZERO.sin());
        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin());
        assert_exact!(Double::NAN, Double::INFINITY.sin());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin());
        assert_exact!(Double::NAN, Double::NAN.sin());
    }

    #[test]
    fn double_trig_cosine() {
        assert_close!(dd!("0.54030230586813971740093660744298"), dd!(1).cos());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / dd!(4)).cos()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_3.cos());
        assert_exact!(Double::ONE, Double::ZERO.cos());
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.cos());
        assert_exact!(Double::NAN, Double::INFINITY.cos());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.cos());
        assert_exact!(Double::NAN, Double::NAN.cos());
    }

    #[test]
    fn double_trig_tangent() {
        assert_close!(dd!("1.5574077246549022305069748074584"), dd!(1).tan());
        assert_close!(dd!(1), Double::FRAC_PI_4.tan());
        assert_exact!(Double::ZERO, Double::ZERO.tan());
        assert!(Double::FRAC_PI_2.tan().is_infinite());
        assert_exact!(Double::NAN, Double::INFINITY.tan());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.tan());
        assert_exact!(Double::NAN, Double::NAN.tan());
    }

    #[test]
    fn double_trig_sin_cos() {
        let (s, c) = dd!(1).sin_cos();
        assert_close!(dd!("0.84147098480789650665250232163030"), s);
        assert_close!(dd!("0.54030230586813971740093660744298"), c);
        let (s, c) = dd!(Double::PI / dd!(4)).sin_cos();
        assert_close!(dd!("0.70710678118654752440084436210485"), s);
        assert_close!(dd!("0.70710678118654752440084436210485"), c);
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());

        assert_exact!(Double::ZERO, Double::ZERO.sin_cos().0);
        assert_exact!(Double::ONE, Double::ZERO.sin_cos().1);

        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin_cos().0);
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.sin_cos().1);

        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().1);

        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().1);

        assert_exact!(Double::NAN, Double::NAN.sin_cos().0);
        assert_exact!(Double::NAN, Double::NAN.sin_cos().1);
    }

    #[test]
    fn double_trig_atan2() {
        assert_exact!(Double::NAN, dd!(0).atan2(dd!(0)));
        assert_exact!(Double::ZERO, dd!(0).atan2(dd!(1)));
        assert_close!(Double::PI, dd!(0).atan2(dd!(-1)));
        assert_close!(Double::FRAC_PI_2, dd!(1).atan2(dd!(0)));
        assert_close!(-Double::FRAC_PI_2, dd!(-1).atan2(dd!(0)));
        assert_close!(Double::FRAC_PI_4, dd!(1).atan2(dd!(1)));
        assert_close!(-Double::FRAC_3_PI_4, dd!(-1).atan2(dd!(-1)));
        assert_close!(Double::FRAC_3_PI_4, dd!(1).atan2(dd!(-1)));
        assert_close!(-Double::FRAC_PI_4, dd!(-1).atan2(dd!(1)));
        assert_exact!(Double::NAN, Double::INFINITY.atan2(Double::INFINITY));
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan2(dd!(1)));
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan2(dd!(1)));
        assert_exact!(Double::ZERO, dd!(1).atan2(Double::INFINITY));
        assert_close!(
            dd!("0.46364760900080611621425623146121"),
            dd!(1).atan2(dd!(2))
        );
        assert_close!(
            dd!("2.6779450445889871222483871518183"),
            dd!(1).atan2(dd!(-2))
        );
        assert_close!(
            dd!("-0.46364760900080611621425623146121"),
            dd!(-1).atan2(dd!(2))
        );
        assert_close!(
            dd!("-2.6779450445889871222483871518183"),
            dd!(-1).atan2(dd!(-2))
        );
    }

    #[test]
    fn double_trig_atan() {
        assert_exact!(Double::ZERO, dd!(0).atan());
        assert_close!(Double::FRAC_PI_4, dd!(1).atan());
        assert_close!(Double::FRAC_PI_2, Double::INFINITY.atan());
        assert_close!(-Double::FRAC_PI_2, Double::NEG_INFINITY.atan());
        assert_exact!(Double::NAN, Double::NAN.atan());
        assert_close!(dd!("0.98279372324732906798571061101467"), dd!(1.5).atan());
    }

    #[test]
    fn double_trig_asin() {
        assert_exact!(Double::NAN, dd!(1.5).asin());
        assert_exact!(Double::NAN, dd!(-1.5).asin());
        assert_close!(Double::FRAC_PI_2, dd!(1).asin());
        assert_close!(-Double::FRAC_PI_2, dd!(-1).asin());
        assert_close!(dd!("0.52359877559829887307710723054658"), dd!(0.5).asin());
    }

    #[test]
    fn double_trig_acos() {
        assert_exact!(Double::NAN, dd!(1.5).acos());
        assert_exact!(Double::NAN, dd!(-1.5).acos());
        assert_exact!(Double::ZERO, dd!(1).acos());
        assert_close!(Double::PI, dd!(-1).acos());
        assert_close!(dd!("1.0471975511965977461542144610932"), dd!(0.5).acos());
    }
}
