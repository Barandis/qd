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
    let threshold = 0.5 * a.to_float().abs() * Double::EPSILON;

    if a.is_zero() {
        return Double::ZERO;
    }

    let mut i = 0;
    let x = -a.sqr();
    let mut s = a;
    let mut r = a;

    loop {
        r *= x;
        let t = r * INV_FACTS[i];
        s += t;
        i += 2;
        if i >= INV_FACTS.len() || t.to_float().abs() <= threshold {
            break;
        }
    }

    s
}

// Compute cos a using the Taylor series. This assumes that |a| <= π/32.
fn cos_taylor(a: Double) -> Double {
    let threshold = 0.5 * Double::EPSILON;

    if a.is_zero() {
        return Double::ONE;
    }

    let mut i = 1;
    let x = -a.sqr();
    let mut r = x;
    let mut s = 1.0 + mul_pwr2(r, 0.5);

    loop {
        r *= x;
        let t = r * INV_FACTS[i];
        s += t;
        i += 2;
        if i >= INV_FACTS.len() || t.to_float().abs() <= threshold {
            break;
        }
    }

    s
}

// Computes both the sine and cosine of a using the Taylor series. This is a bit quicker than
// calling the two functions above separately, since if you have one of them you can calculate the
// other more efficiently.
fn sincos_taylor(a: Double) -> (Double, Double) {
    if a.is_zero() {
        (Double::ZERO, Double::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (1.0 - sin_a.sqr()).sqrt())
    }
}

#[inline]
fn reduce(a: Double) -> (i32, i32, Double) {
    // approximately reduce modulo 2π
    let z = (a / Double::MUL_2_PI).round();
    let r = a - z * Double::MUL_2_PI;

    // approx. reduce modulo π/2 and then modulo π/16
    let mut q = (r.0 / Double::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - q * Double::FRAC_PI_2;
    let j = q as i32;
    q = (t.0 / Double::FRAC_PI_16.0 + 0.5).floor();
    t -= q * Double::FRAC_PI_16;
    let k = q as i32;

    (j, k, t)
}

impl Double {
    /// Computes the sine of `self`, which is assumed to be in radians.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let x = (Double::PI / 2.0).sin();
    ///
    /// // Answer should be 1
    /// let diff = (x - 1.0).abs();
    /// assert!(diff < 1e-20);
    /// ```
    pub fn sin(self) -> Double {
        // Strategy:
        //
        // We choose integers a and b so that
        //
        //      x = s + aπ/2 + bπ/16
        //
        // and |s| <= π/32. Using the fact that
        //
        //      sin π/16 = 0.5 * √(2 - √(2 + √2))
        //
        // we can comput sin x from sin s and cos s. This greatly increases the convergence of the
        // Taylor series for sine and cosine.
        if self.is_zero() {
            return Double::ZERO;
        }

        let (j, k, t) = reduce(self);
        let abs_k = k.abs() as usize;

        if j < -2 || j > 2 {
            // Cannot reduce modulo π/2
            return Double::NAN;
        }
        if abs_k > 4 {
            // Cannot reduce modulo π/16
            return Double::NAN;
        }

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

    /// Computes the cosine of `self`, which is assumed to be in radians.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let x = (Double::PI / 2.0).cos();
    ///
    /// // Answer should be 0
    /// let diff = (x - 0.0).abs();
    /// assert!(diff < 1e-20);
    /// ```
    pub fn cos(self) -> Double {
        if self.is_zero() {
            return Double::ONE;
        }

        let (j, k, t) = reduce(self);
        let abs_k = k.abs() as usize;

        if j < -2 || j > 2 {
            // Cannot reduce modulo π/2
            return Double::NAN;
        }
        if abs_k > 4 {
            // Cannot reduce modulo π/16
            return Double::NAN;
        }

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

    /// Simultaneously computes the sine and the cosine of `self`. This is more efficient if you
    /// need both numbers. Returns `(sin(self), cos(self))`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let x = Double::PI / 4.0;
    /// let (sin_x, cos_x) = x.sin_cos();
    ///
    /// let diff_sin = (sin_x - x.sin()).abs();
    /// let diff_cos = (cos_x - x.cos()).abs();
    ///
    /// assert!(diff_sin < 1e-20);
    /// assert!(diff_cos < 1e-20);
    /// ```
    pub fn sin_cos(self) -> (Double, Double) {
        if self.is_zero() {
            return (Double::ZERO, Double::ONE);
        }

        let (j, k, t) = reduce(self);
        let abs_k = k.abs() as usize;

        // I honestly don't know if either of these error conditions happen. Will look into it more.
        if j < -2 || j > 2 {
            // Cannot reduce modulo π/2
            return (Double::NAN, Double::NAN);
        }
        if abs_k > 4 {
            // Cannot reduce modulo π/16
            return (Double::NAN, Double::NAN);
        }

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

    /// Computes the tangent of `self`, which is assumed to be in radians.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let x = (Double::PI / 4.0).tan();
    ///
    /// // Answer should be 1
    /// let diff = (x - 1.0).abs();
    /// assert!(diff < 1e-20);
    /// ```
    pub fn tan(self) -> Double {
        let (s, c) = self.sin_cos();
        s / c
    }

    /// Computes the 2-argument arctangent of `self` (`y`) and `other` (`x`) in radians.
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
    ///
    /// // 3π/4 radians (135 degrees counter-clockwise)
    /// let x2 = dd!(-3);
    /// let y2 = dd!(3);
    ///
    /// let diff1 = (y1.atan2(x1) - (-pi / 4.0)).abs();
    /// let diff2 = (y2.atan2(x2) - 3.0 * pi / 4.0).abs();
    ///
    /// assert!(diff1 < 1e-20);
    /// assert!(diff2 < 1e-20);
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
            let mut z = Double::from(self.to_float().atan2(other.to_float()));
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

    /// Computes the arctangent of `self`. The return value is in the range [-π/2, π/2].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// let x = dd!(1).atan();
    ///
    /// // Answer should be π/4
    /// let diff = (x - Double::PI / 4.0).abs();
    /// assert!(diff < 1e-20);
    /// ```
    pub fn atan(self) -> Double {
        self.atan2(Double::ONE)
    }

    /// Computes the arcsine of `self`. The return value is in the range [-π/2, π/2] for any `self`
    /// value in the range [-1, 1]. Otherwise the return value is `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// let x = dd!(1).asin();
    ///
    /// // Answer should be π/2
    /// let diff = (x - Double::PI / 2.0).abs();
    /// assert!(diff < 1e-20);
    /// ```
    pub fn asin(self) -> Double {
        if self.abs() > 1.0 {
            Double::NAN
        } else if self == 1.0 {
            Double::FRAC_PI_2
        } else if self == -1.0 {
            -Double::FRAC_PI_2
        } else {
            self.atan2((1.0 - self.sqr()).sqrt())
        }
    }

    /// Computes the arccosine of `self`. The return value is in the range [0, π] for any `self`
    /// value in the range [-1, 1]. Otherwise the return value is `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// let x = dd!(1).acos();
    ///
    /// // Answer should be 0
    /// let diff = (x - 0.0).abs();
    /// assert!(diff < 1e-20);
    /// ```
    pub fn acos(self) -> Double {
        if self.abs() > 1.0 {
            Double::NAN
        } else if self == 1.0 {
            Double::ZERO
        } else if self == -1.0 {
            Double::PI
        } else {
            (1.0 - self.sqr()).sqrt().atan2(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 30);
        };
    }

    fn print_impl(a: Double) -> String {
        format!("    Double({:e}, {:e}),", a.0, a.1)
    }

    #[test]
    fn print() {
        let values = ["2.3561944901923449288469825374596"];
        for s in values.iter() {
            println!("{}", print_impl(dd!(*s)));
        }
    }

    #[test]
    fn trig_sine() {
        assert_close!(dd!("0.84147098480789650665250232163030"), dd!(1).sin());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / 4.0).sin()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());
        assert_exact!(Double::ZERO, Double::ZERO.sin());
        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin());
    }

    #[test]
    fn trig_cosine() {
        assert_close!(dd!("0.54030230586813971740093660744298"), dd!(1).cos());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / 4.0).cos()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_3.cos());
        assert_exact!(Double::ONE, Double::ZERO.cos());
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.cos());
    }

    #[test]
    fn trig_tangent() {
        assert_close!(dd!("1.5574077246549022305069748074584"), dd!(1).tan());
        assert_close!(dd!(1), Double::FRAC_PI_4.tan());
        assert_exact!(Double::ZERO, Double::ZERO.tan());
        assert!(Double::FRAC_PI_2.tan().is_infinite());
    }
}
