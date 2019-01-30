// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::quick_two_sum;
use crate::double::Double;
use std::num::FpCategory;

// #region Miscellaneous mathematical operations

impl Double {
    /// Calculates the absolute value of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(dd!(3).abs() == dd!(3));
    /// assert!(dd!(-3).abs() == dd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn abs(self) -> Double {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    /// Returns the largest integer less than or equal to the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.99);
    /// let g = dd!(3.0);
    ///
    /// assert!(f.floor() == dd!(3));
    /// assert!(g.floor() == dd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn floor(self) -> Double {
        let hi = self.0.floor();

        if hi == self.0 {
            Double::norm(hi, self.1.floor())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the smallest integer greater than or equal to the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.01);
    /// let g = dd!(4.0);
    ///
    /// assert!(f.ceil() == dd!(4));
    /// assert!(g.ceil() == dd!(4));
    /// # }
    /// ```
    #[inline]
    pub fn ceil(self) -> Double {
        let hi = self.0.ceil();

        if hi == self.0 {
            Double::norm(hi, self.1.ceil())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the nearest integer to the number. Half-way cases are rounded away from `0.0`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(-3.3);
    ///
    /// assert!(f.round() == dd!(3));
    /// assert!(g.round() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn round(self) -> Double {
        let hi = self.0.round();

        if hi == self.0 {
            let lo = self.1.round();
            Double::from(quick_two_sum(hi, lo))
        } else {
            if (hi - self.0).abs() == 0.5 && self.1 < 0.0 {
                Double(hi - 1.0, 0.0)
            } else {
                Double(hi, 0.0)
            }
        }
    }

    /// Returns the integer part of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(-3.7);
    ///
    /// assert!(f.trunc() == dd!(3));
    /// assert!(g.trunc() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn trunc(self) -> Double {
        if self.0 >= 0.0 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    /// Returns the fractional part of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(-3.7);
    ///
    /// let fdiff = (f.fract() - dd!(0.3)).abs();
    /// let gdiff = (g.fract() - dd!(-0.7)).abs();
    ///
    /// assert!(fdiff < dd!(1e-30));
    /// assert!(gdiff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn fract(self) -> Double {
        self - self.trunc()
    }

    /// Returns a number that represents the sign of the number.
    ///
    /// * `1.0` if the number is positive, `+0.0`, or [`INFINITY`]
    /// * `-1.0` if the number is negative, `-0.0`, or [`NEG_INFINITY`]
    /// *  [`NAN`] if the number is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(dd!(3.5).signum() == Double::ONE);
    /// assert!(Double::NEG_INFINITY.signum() == -Double::ONE);
    /// assert!(Double::NAN.signum().is_nan());
    /// # }
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    /// [`NEG_INFINITY`]: #associatedconstant.NEG_INFINITY
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    pub fn signum(self) -> Double {
        if self.is_nan() {
            Double::NAN
        } else if self.is_sign_negative() {
            -Double::ONE
        } else {
            Double::ONE
        }
    }
}

// #endregion

// #region Number properties

impl Double {
    /// Returns the floating point category of the number. If only one property is being tested,
    /// it's generally faster to use the specific predicate rather than this function.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// use std::num::FpCategory;
    ///
    /// let num = dd!(12.4);
    /// let inf = Double::INFINITY;
    ///
    /// assert!(num.classify() == FpCategory::Normal);
    /// assert!(inf.classify() == FpCategory::Infinite);
    /// # }
    /// ```
    pub fn classify(self) -> FpCategory {
        use std::num::FpCategory::*;

        let c0 = self.0.classify();
        let c1 = self.1.classify();

        if c0 == Zero && c1 == Zero {
            Zero
        } else if c0 == Subnormal || c1 == Subnormal {
            Subnormal
        } else if c0 == Infinite || c1 == Infinite {
            Infinite
        } else if c0 == Nan || c1 == Nan {
            Nan
        } else {
            Normal
        }
    }

    /// Returns `true` if the number is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let min = Double::MIN_POSITIVE;
    /// let max = Double::MAX;
    /// let lower = dd!(1e-308);
    /// let zero = Double::ZERO;
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!Double::NAN.is_normal());
    /// assert!(!Double::INFINITY.is_normal());
    /// // Values between `0` and `MIN_POSITIVE` are subnormal.
    /// assert!(!lower.is_normal());
    /// # }
    /// ```
    #[inline]
    pub fn is_normal(self) -> bool {
        self.classify() == FpCategory::Normal
    }

    /// Returns `true` if the number is either positive or negative zero.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// assert!(Double::ZERO.is_zero());
    /// assert!(Double::NEG_ZERO.is_zero());
    /// assert!(!Double::PI.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns `true` if the number is negative, including negative zero and infinity and `NaN`
    /// with a negative sign bit.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(Double::NEG_ZERO.is_sign_negative());
    /// assert!(Double::NEG_INFINITY.is_sign_negative());
    /// assert!(dd!(-7.0).is_sign_negative());
    /// assert!(!Double::ZERO.is_sign_negative());
    /// assert!(!dd!(7.0).is_sign_negative());
    /// # }
    /// ```
    #[inline]
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    /// Returns `true` if the number is positive, including positive zero and infinity and `NaN`
    /// with a positive sign bit.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(Double::ZERO.is_sign_positive());
    /// assert!(Double::INFINITY.is_sign_positive());
    /// assert!(dd!(7.0).is_sign_positive());
    /// assert!(!Double::NEG_ZERO.is_sign_positive());
    /// assert!(!dd!(-7.0).is_sign_positive());
    /// # }
    /// ```
    #[inline]
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    /// Returns `true` if this value is `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(Double::NAN.is_nan());
    /// assert!(!dd!(7.0).is_nan());
    /// # }
    /// ```
    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }

    /// Returns `true` if this number is positive or negative infinity.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(Double::INFINITY.is_infinite());
    /// assert!(Double::NEG_INFINITY.is_infinite());
    /// assert!(!Double::NAN.is_infinite());
    /// assert!(!dd!(7.0).is_infinite());
    /// # }
    /// ```
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite() || self.1.is_infinite()
    }

    /// Returns `true` if this number is neither infinite nor `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(!Double::INFINITY.is_finite());
    /// assert!(!Double::NEG_INFINITY.is_finite());
    /// assert!(!Double::NAN.is_finite());
    /// assert!(dd!(7.0).is_finite());
    /// # }
    /// ```
    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite() && self.1.is_finite()
    }
}

// #endregion

// #region Conversions

impl Double {
    /// Converts the number into an `f64`.
    #[inline]
    pub fn as_float(self) -> f64 {
        self.0
    }

    /// Converts the number into an `i64`.
    ///
    /// While it is possible for a `Double` to be created from a `u64`, whether or not the original
    /// is signed is not recorded (since `Double` itself is signed). The return value of this
    /// function can be cast to u64 if necessary.
    #[inline]
    pub fn as_int(self) -> i64 {
        self.0 as i64 + self.1 as i64
    }

    #[inline]
    pub fn as_tuple(self) -> (f64, f64) {
        (self.0, self.1)
    }

    #[inline]
    pub fn assign(&mut self, (a, b): (f64, f64)) {
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn misc_fract() {
        assert_close!(0.3, dd!(3.3).fract());
        assert_close!(-0.7, dd!(-3.7).fract());
    }

    // Used to quickly get raw Double definitions for the odd number here and there
    // Run with `cargo test print_dd -- --nocapture`
    // #[test]
    // fn print_dd() {
    //     let x = dd!("4.9303806576313237838233035330174e-32");
    //     println!("Double({:e}, {:e})", x.0, x.1);
    // }
}
