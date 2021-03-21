// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::double::Double;
use std::f64;
use std::num::FpCategory;

impl Double {
    /// Calculates the absolute value of the `Double`.
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

    /// Returns the largest integer value less than or equal to the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.99);
    /// let g = dd!(3.0);
    /// let h = dd!(-3.99);
    ///
    /// assert!(f.floor() == dd!(3));
    /// assert!(g.floor() == dd!(3));
    /// assert!(h.floor() == dd!(-4));
    /// # }
    /// ```
    #[inline]
    pub fn floor(self) -> Double {
        let hi = self.0.floor();

        if (hi - self.0).abs() < f64::EPSILON {
            Double::new(hi, self.1.floor())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the smallest integer value greater than or equal to the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.01);
    /// let g = dd!(4.0);
    /// let h = dd!(-3.01);
    ///
    /// assert!(f.ceil() == dd!(4));
    /// assert!(g.ceil() == dd!(4));
    /// assert!(h.ceil() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn ceil(self) -> Double {
        let hi = self.0.ceil();

        if (hi - self.0).abs() < f64::EPSILON {
            Double::new(hi, self.1.ceil())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the nearest integer value to the `Double`. Half-way cases are rounded away
    /// from `0.0`, per the behavior of `f64`'s `round` method.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(3.5);
    /// let h = dd!(-3.3);
    ///
    /// assert!(f.round() == dd!(3));
    /// assert!(g.round() == dd!(4));
    /// assert!(h.round() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn round(self) -> Double {
        let hi = self.0.round();

        if (hi - self.0).abs() < f64::EPSILON {
            let lo = self.1.round();
            let (a, b) = p::renorm2(hi, lo);
            Double(a, b)
        } else if ((hi - self.0).abs() - 0.5).abs() < f64::EPSILON && self.1 < 0.0 {
            Double(hi - 1.0, 0.0)
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the integer part of the `Double`.
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

    /// Returns the fractional part of the `Double`.
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

    /// Returns a number that represents the sign of the `Double`.
    ///
    /// * `1.0` if the number is positive, including `+0.0` and [`INFINITY`]
    /// * `-1.0` if the number is negative, including `-0.0` and [`NEG_INFINITY`]
    /// *  [`NAN`] if the number is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(dd!(3.5).signum() == Double::ONE);
    /// assert!(Double::NEG_INFINITY.signum() == Double::NEG_ONE);
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
            Double::NEG_ONE
        } else {
            Double::ONE
        }
    }

    /// Returns the floating point category of the `Double`.
    ///
    /// The possible return values are the members of [`FpCategory`], as follows:
    ///
    /// * `FpCategory::Zero` if the number is ±0;
    /// * `FpCategory::Infinite` if the number is ±∞;
    /// * `FpCategory::Nan` if the number is not a number;
    /// * `FpCategory::Subnormal` if the number is ±[`MIN_POSITIVE`] (numbers this small can
    ///     be represented, but they lose some accuracy);
    /// * `FpCategory::Normal` if the number is anything else.
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
    ///
    /// [`FpCategory`]: https://doc.rust-lang.org/std/num/enum.FpCategory.html
    /// [`MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    pub fn classify(self) -> FpCategory {
        self.0.classify()
    }

    /// Returns `true` if the `Double` is neither zero, infinite, subnormal, or `NaN`.
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

    /// Returns `true` if the `Double` is either positive or negative zero.
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

    /// Returns `true` if the `Double` is negative, including negative zero, negative
    /// infinity, and `NaN` with a negative sign bit.
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

    /// Returns `true` if the `Double` is positive, including positive zero, positive
    /// infinity and `NaN` with a positive sign bit.
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

    /// Returns `true` if the `Double` is `NaN`.
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
        self.0.is_nan()
    }

    /// Returns `true` if the `Double` is positive or negative infinity.
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
        self.0.is_infinite()
    }

    /// Returns `true` if the `Double` is neither infinite nor `NaN`.
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
        self.0.is_finite()
    }

    /// Returns `true` if the `Double` has an absolute value of less than [`MIN_POSITIVE`].
    ///
    /// Numbers this small can be represented by floating point numbers, but they are not as
    /// accurate. This inaccuracy is inherent in the IEEE-754 format for 64-bit numbers;
    /// making a double-double out of an inaccurate number means the double-double is also
    /// going to be inaccurate.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(!Double::PI.is_subnormal());
    /// assert!(dd!(1e-308).is_subnormal());
    /// # }
    /// ```
    ///
    /// [`MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    #[inline]
    pub fn is_subnormal(self) -> bool {
        self.classify() == FpCategory::Subnormal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs() {
        assert_exact!((-Double::PI).abs(), Double::PI);
        assert_exact!(Double::PI.abs(), Double::PI);
    }

    #[test]
    fn abs_zero() {
        assert_exact!(Double::ZERO.abs(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.abs(), Double::ZERO);
    }

    #[test]
    fn abs_inf() {
        assert_exact!(Double::INFINITY.abs(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.abs(), Double::INFINITY);
    }

    #[test]
    fn abs_nan() {
        assert!(Double::NAN.abs().is_nan());
    }

    #[test]
    fn floor() {
        assert_exact!(Double::PI.floor(), dd!(3));
        assert_exact!(Double::E.floor(), dd!(2));
        assert_exact!((-Double::PI).floor(), dd!(-4));
        assert_exact!((-Double::E).floor(), dd!(-3));
        assert_exact!(dd!(2).floor(), dd!(2));
    }

    #[test]
    fn floor_zero() {
        assert_exact!(Double::ZERO.floor(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.floor(), Double::NEG_ZERO);
    }

    #[test]
    fn floor_inf() {
        assert_exact!(Double::INFINITY.floor(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.floor(), Double::NEG_INFINITY);
    }

    #[test]
    fn floor_nan() {
        assert!(Double::NAN.floor().is_nan());
    }

    #[test]
    fn ceil() {
        assert_exact!(Double::PI.ceil(), dd!(4));
        assert_exact!(Double::E.ceil(), dd!(3));
        assert_exact!((-Double::PI).ceil(), dd!(-3));
        assert_exact!((-Double::E).ceil(), dd!(-2));
        assert_exact!(dd!(2).ceil(), dd!(2));
    }

    #[test]
    fn ceil_zero() {
        assert_exact!(Double::ZERO.ceil(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.ceil(), Double::NEG_ZERO);
    }

    #[test]
    fn ceil_inf() {
        assert_exact!(Double::INFINITY.ceil(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.ceil(), Double::NEG_INFINITY);
    }

    #[test]
    fn ceil_nan() {
        assert!(Double::NAN.ceil().is_nan());
    }

    #[test]
    fn round() {
        assert_exact!(Double::PI.round(), dd!(3));
        assert_exact!(Double::E.round(), dd!(3));
        assert_exact!((-Double::PI).round(), dd!(-3));
        assert_exact!((-Double::E).round(), dd!(-3));
        assert_exact!(dd!(2).round(), dd!(2));
        assert_exact!(dd!(2.5).round(), dd!(3));
        assert_exact!(dd!(-3.5).round(), dd!(-4));
    }

    #[test]
    fn round_zero() {
        assert_exact!(Double::ZERO.round(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.round(), Double::NEG_ZERO);
    }

    #[test]
    fn round_infi() {
        assert_exact!(Double::INFINITY.round(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.round(), Double::NEG_INFINITY);
    }

    #[test]
    fn round_nan() {
        assert!(Double::NAN.round().is_nan());
    }

    #[test]
    fn trunc() {
        assert_exact!(Double::PI.trunc(), dd!(3));
        assert_exact!(Double::E.trunc(), dd!(2));
        assert_exact!((-Double::PI).trunc(), dd!(-3));
        assert_exact!((-Double::E).trunc(), dd!(-2));
        assert_exact!(dd!(2).trunc(), dd!(2));
        assert_exact!(dd!(2.5).trunc(), dd!(2));
        assert_exact!(dd!(-3.5).trunc(), dd!(-3));
    }

    #[test]
    fn trunc_zero() {
        assert_exact!(Double::ZERO.trunc(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.trunc(), Double::NEG_ZERO);
    }

    #[test]
    fn trunc_inf() {
        assert_exact!(Double::INFINITY.trunc(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.trunc(), Double::NEG_INFINITY);
    }

    #[test]
    fn trunc_nan() {
        assert!(Double::NAN.trunc().is_nan());
    }

    #[test]
    fn fract() {
        assert_close!(Double::PI.fract(), Double::PI - dd!(3));
        assert_close!(Double::E.fract(), Double::E - dd!(2));
        assert_close!((-Double::PI).fract(), -Double::PI + dd!(3));
        assert_close!((-Double::E).fract(), -Double::E + dd!(2));
        assert_exact!(dd!(2).fract(), Double::ZERO);
        assert_exact!(dd!(2.5).fract(), dd!(0.5));
        assert_exact!(dd!(-3.5).fract(), dd!(-0.5));
    }

    #[test]
    fn fract_zero() {
        assert_exact!(Double::ZERO.fract(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.fract(), Double::NEG_ZERO);
    }

    #[test]
    fn fract_inf() {
        assert_exact!(Double::INFINITY.fract(), Double::NAN);
        assert_exact!(Double::NEG_INFINITY.fract(), Double::NAN);
    }

    #[test]
    fn fract_nan() {
        assert!(Double::NAN.fract().is_nan());
    }

    #[test]
    fn signum() {
        assert_exact!(Double::PI.signum(), Double::ONE);
        assert_exact!(Double::E.signum(), Double::ONE);
        assert_exact!((-Double::PI).signum(), Double::NEG_ONE);
        assert_exact!((-Double::E).signum(), Double::NEG_ONE);
        assert_exact!(dd!(2).signum(), Double::ONE);
        assert_exact!(dd!(2.5).signum(), Double::ONE);
        assert_exact!(dd!(-3.5).signum(), Double::NEG_ONE);
    }

    #[test]
    fn signum_zero() {
        assert_exact!(Double::ZERO.signum(), Double::ONE);
        assert_exact!(Double::NEG_ZERO.signum(), Double::NEG_ONE);
    }

    #[test]
    fn signum_inf() {
        assert_exact!(Double::INFINITY.signum(), Double::ONE);
        assert_exact!(Double::NEG_INFINITY.signum(), Double::NEG_ONE);
    }

    #[test]
    fn signum_nan() {
        assert!(Double::NAN.signum().is_nan());
    }

    #[test]
    fn classify() {
        use std::num::FpCategory::*;
        
        assert_eq!(Double::PI.classify(), Normal);
        assert_eq!(Double::ZERO.classify(), Zero);
        assert_eq!(Double::NEG_ZERO.classify(), Zero);
        assert_eq!(Double::INFINITY.classify(), Infinite);
        assert_eq!(Double::NEG_INFINITY.classify(), Infinite);
        assert_eq!(Double::NAN.classify(), Nan);
        assert_eq!(dd!(1e-308).classify(), Subnormal);
    }

    #[test]
    fn is_normal() {
        assert!(Double::PI.is_normal());
        assert!((-Double::PI).is_normal());
        assert!(!Double::ZERO.is_normal());
        assert!(!Double::NEG_ZERO.is_normal());
        assert!(!Double::INFINITY.is_normal());
        assert!(!Double::NEG_INFINITY.is_normal());
        assert!(!Double::NAN.is_normal());
        assert!(!dd!(1e-308).is_normal());
    }

    #[test]
    fn is_zero() {
        assert!(!Double::PI.is_zero());
        assert!(!(-Double::PI).is_zero());
        assert!(Double::ZERO.is_zero());
        assert!(Double::NEG_ZERO.is_zero());
        assert!(!Double::INFINITY.is_zero());
        assert!(!Double::NEG_INFINITY.is_zero());
        assert!(!Double::NAN.is_zero());
        assert!(!dd!(1e-308).is_zero());
    }

    #[test]
    fn is_sign_negative() {
        assert!(!Double::PI.is_sign_negative());
        assert!((-Double::PI).is_sign_negative());
        assert!(!Double::ZERO.is_sign_negative());
        assert!(Double::NEG_ZERO.is_sign_negative());
        assert!(!Double::INFINITY.is_sign_negative());
        assert!(Double::NEG_INFINITY.is_sign_negative());
        assert!(!Double::NAN.is_sign_negative());
        assert!(!dd!(1e-308).is_sign_negative());
    }

    #[test]
    fn is_sign_positive() {
        assert!(Double::PI.is_sign_positive());
        assert!(!(-Double::PI).is_sign_positive());
        assert!(Double::ZERO.is_sign_positive());
        assert!(!Double::NEG_ZERO.is_sign_positive());
        assert!(Double::INFINITY.is_sign_positive());
        assert!(!Double::NEG_INFINITY.is_sign_positive());
        assert!(Double::NAN.is_sign_positive());
        assert!(dd!(1e-308).is_sign_positive());
    }

    #[test]
    fn is_nan() {
        assert!(!Double::PI.is_nan());
        assert!(!(-Double::PI).is_nan());
        assert!(!Double::ZERO.is_nan());
        assert!(!Double::NEG_ZERO.is_nan());
        assert!(!Double::INFINITY.is_nan());
        assert!(!Double::NEG_INFINITY.is_nan());
        assert!(Double::NAN.is_nan());
        assert!(!dd!(1e-308).is_nan());
    }

    #[test]
    fn is_infinite() {
        assert!(!Double::PI.is_infinite());
        assert!(!(-Double::PI).is_infinite());
        assert!(!Double::ZERO.is_infinite());
        assert!(!Double::NEG_ZERO.is_infinite());
        assert!(Double::INFINITY.is_infinite());
        assert!(Double::NEG_INFINITY.is_infinite());
        assert!(!Double::NAN.is_infinite());
        assert!(!dd!(1e-308).is_infinite());
    }

    #[test]
    fn is_finite() {
        assert!(Double::PI.is_finite());
        assert!((-Double::PI).is_finite());
        assert!(Double::ZERO.is_finite());
        assert!(Double::NEG_ZERO.is_finite());
        assert!(!Double::INFINITY.is_finite());
        assert!(!Double::NEG_INFINITY.is_finite());
        assert!(!Double::NAN.is_finite());
        assert!(dd!(1e-308).is_finite());
    }

    #[test]
    fn is_subnormal() {
        assert!(!Double::PI.is_subnormal());
        assert!(!(-Double::PI).is_subnormal());
        assert!(!Double::ZERO.is_subnormal());
        assert!(!Double::NEG_ZERO.is_subnormal());
        assert!(!Double::INFINITY.is_subnormal());
        assert!(!Double::NEG_INFINITY.is_subnormal());
        assert!(!Double::NAN.is_subnormal());
        assert!(dd!(1e-308).is_subnormal());
    }
}
