// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod tables;

use self::tables::{COSINES, SINES};
use crate::quad::common::{mul_pwr2, INV_FACTS};
use crate::quad::Quad;

const FRAC_PI_1024: Quad = Quad(
    3.0679615757712823e-3,
    1.195944139792337e-19,
    -2.924579892303066e-36,
    1.0863810750618759e-52,
);

// Compute sin a using the Taylor series. This assumes that |a| <= π/2048.
fn sin_taylor(a: Quad) -> Quad {
    if a.is_zero() {
        Quad::ZERO
    } else {
        let threshold = mul_pwr2(Quad::EPSILON * a.abs(), 0.5);
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

// Compute cos a using the Taylor series. This assumes that |a| <= π/2048.
fn cos_taylor(a: Quad) -> Quad {
    if a.is_zero() {
        Quad::ONE
    } else {
        let threshold = mul_pwr2(Quad::EPSILON, 0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Quad::ONE + mul_pwr2(r, 0.5);
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
fn sincos_taylor(a: Quad) -> (Quad, Quad) {
    if a.is_zero() {
        (Quad::ZERO, Quad::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Quad::ONE - sin_a.sqr()).sqrt())
    }
}

#[inline]
fn reduce(a: Quad) -> (i32, i32, Quad) {
    // approximately reduce modulo 2π
    let z = (a / Quad::MUL_2_PI).round();
    let r = a - z * Quad::MUL_2_PI;

    // approx. reduce modulo π/2 and then modulo π/1024
    let mut q = (r.0 / Quad::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - Quad::from(q) * Quad::FRAC_PI_2;
    let j = q as i32;
    q = (t.0 / FRAC_PI_1024.0 + 0.5).floor();
    t -= Quad::from(q) * FRAC_PI_1024;
    let k = q as i32;

    (j, k, t)
}

impl Quad {
    /// Computes the sine of the number.
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
    pub fn sin(self) -> Quad {
        // Strategy:
        //
        // We choose integers a and b so that
        //
        //      x = s + aπ/2 + bπ/1024
        //
        // where |s| <= π/2048. Using a precomputed table of sin (kπ/1024) and cos (kπ/1024), we can
        // compute sin x from sin s and cos s. This greatly increases the convergence of the Taylor
        // series for sine and cosine.
        if self.is_zero() {
            return Quad::ZERO;
        }

        let (j, k, t) = reduce(self);
        let abs_k = k.abs() as usize;

        if j < -2 || j > 2 {
            // Cannot reduce modulo π/2
            return Quad::NAN;
        }
        if abs_k > 256 {
            // Cannot reduce modulo π/1024
            return Quad::NAN;
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

    /// Computes the cosine of the number.
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
    pub fn cos(self) -> Quad {
        if self.is_zero() {
            return Quad::ONE;
        }

        let (j, k, t) = reduce(self);
        let abs_k = k.abs() as usize;

        if j < -2 || j > 2 {
            // Cannot reduce modulo π/2
            return Quad::NAN;
        }
        if abs_k > 256 {
            // Cannot reduce modulo π/1024
            return Quad::NAN;
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

    /// Simultaneously computes the sine and the cosine of the number. This is more efficient if you
    /// need both numbers. Returns `(sin(x), cos(x))`.
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
    pub fn sin_cos(self) -> (Quad, Quad) {
        if self.is_zero() {
            return (Quad::ZERO, Quad::ONE);
        }

        let (j, k, t) = reduce(self);
        let abs_k = k.abs() as usize;

        // I honestly don't know if either of these error conditions happen. Will look into it more.
        if j < -2 || j > 2 {
            // Cannot reduce modulo π/2
            return (Quad::NAN, Quad::NAN);
        }
        if abs_k > 256 {
            // Cannot reduce modulo π/16
            return (Quad::NAN, Quad::NAN);
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

    /// Computes the tangent of the number.
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

    /// Computes the 2-argument arctangent of the number (`y`) and `other` (`x`) in radians.
    ///
    /// The second argument allows the avoidance of ambiguities in the single-argument [`atan`]
    /// function, notably allowing the determination of quadrant.
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
        // The iteration is given by
        //      z' = z + (y - sin z) / cos z            (for the first equation)
        //      z' = z - (x - cos z) / sin z            (for the second equation)
        //
        // Here, x and y are normalized so that x² + y² = 1. If |x| > |y|, the first iteration is
        // used since the denominator is larger. Otherwise the second is used.

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

    /// Computes the arctangent of the number. The return value is in the range [-π/2, π/2].
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

    /// Computes the arcsine of the number. The return value is in the range [-π/2, π/2] for any
    /// number in the range [-1, 1]. Otherwise the return value is `NaN`.
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
    pub fn asin(self) -> Quad {
        if self.abs() > Quad::ONE {
            Quad::NAN
        } else if self == Quad::ONE {
            Quad::FRAC_PI_2
        } else if self == -Quad::ONE {
            -Quad::FRAC_PI_2
        } else {
            self.atan2((Quad::ONE - self.sqr()).sqrt())
        }
    }

    /// Computes the arccosine of the number. The return value is in the range [0, π] for any number
    /// in the range [-1, 1]. Otherwise the return value is `NaN`.
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
    pub fn acos(self) -> Quad {
        if self.abs() > Quad::ONE {
            Quad::NAN
        } else if self == Quad::ONE {
            Quad::ZERO
        } else if self == -Quad::ONE {
            Quad::PI
        } else {
            (Quad::ONE - self.sqr()).sqrt().atan2(self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 60);
        };
    }

    #[test]
    fn quad_trig_sine() {
        assert_close!(
            qd!("0.8414709848078965066525023216302989996225630607983710656727517100"),
            qd!(1).sin()
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            (Quad::PI / qd!(4)).sin()
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_6.sin());
        assert_exact!(Quad::ZERO, Quad::ZERO.sin());
        assert_exact!(Quad::ONE, Quad::FRAC_PI_2.sin());
    }

    #[test]
    fn quad_trig_cosine() {
        assert_close!(
            qd!("0.5403023058681397174009366074429766037323104206179222276700972554"),
            qd!(1).cos()
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            (Quad::PI / qd!(4)).cos()
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_3.cos());
        assert_exact!(Quad::ONE, Quad::ZERO.cos());
        assert_exact!(Quad::ZERO, Quad::FRAC_PI_2.cos());
    }
}
