// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::quad::Quad;
use std::f64;
use std::num::FpCategory;

impl Quad {
    /// Calculates the absolute value of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(3).abs() == qd!(3));
    /// assert!(qd!(-3).abs() == qd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn abs(self) -> Quad {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    /// Returns the largest integer value less than or equal to the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.99);
    /// let g = qd!(3.0);
    ///
    /// assert!(f.floor() == qd!(3));
    /// assert!(g.floor() == qd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn floor(self) -> Quad {
        let a = self.0.floor();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;

        if (a - self.0).abs() < f64::EPSILON {
            b = self.1.floor();
            if (b - self.1).abs() < f64::EPSILON {
                c = self.2.floor();
                if (c - self.2).abs() < f64::EPSILON {
                    d = self.3.floor();
                }
            }
            let (a, b, c, d) = core::renorm4(a, b, c, d);
            Quad(a, b, c, d)
        } else {
            Quad(a, b, c, d)
        }
    }

    /// Returns the smallest integer value greater than or equal to the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.01);
    /// let g = qd!(4.0);
    ///
    /// assert!(f.ceil() == qd!(4));
    /// assert!(g.ceil() == qd!(4));
    /// # }
    /// ```
    #[inline]
    pub fn ceil(self) -> Quad {
        let a = self.0.ceil();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;

        if (a - self.0).abs() < f64::EPSILON {
            b = self.1.ceil();
            if (b - self.1).abs() < f64::EPSILON {
                c = self.2.ceil();
                if (c - self.2).abs() < f64::EPSILON {
                    d = self.3.ceil();
                }
            }
            let (a, b, c, d) = core::renorm4(a, b, c, d);
            Quad(a, b, c, d)
        } else {
            Quad(a, b, c, d)
        }
    }

    /// Returns the nearest integer value to the `Double`. Half-way cases are rounded away
    /// from `0.0`, per the behavior of `f64`'s `round` method.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.3);
    /// let g = qd!(-3.3);
    ///
    /// assert!(f.round() == qd!(3));
    /// assert!(g.round() == qd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn round(self) -> Quad {
        let a = self.0.round();
        if (a - self.0).abs() < f64::EPSILON {
            let b = self.1.round();
            if (b - self.1).abs() < f64::EPSILON {
                let c = self.2.round();
                if (c - self.2).abs() < f64::EPSILON {
                    let d = self.3.round();
                    let (a, b, c, d) = core::renorm4(a, b, c, d);
                    Quad(a, b, c, d)
                } else if ((c - self.2).abs() - 0.5).abs() < f64::EPSILON && self.3 < 0.0 {
                    Quad(a, b, c - 1.0, 0.0)
                } else {
                    Quad(a, b, c, 0.0)
                }
            } else if ((b - self.1).abs() - 0.5).abs() < f64::EPSILON && self.2 < 0.0 {
                Quad(a, b - 1.0, 0.0, 0.0)
            } else {
                Quad(a, b, 0.0, 0.0)
            }
        } else if ((a - self.0).abs() - 0.5).abs() < f64::EPSILON && self.1 < 0.0 {
            Quad(a - 1.0, 0.0, 0.0, 0.0)
        } else {
            Quad(a, 0.0, 0.0, 0.0)
        }
    }

    /// Returns the integer part of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.3);
    /// let g = qd!(-3.7);
    ///
    /// assert!(f.trunc() == qd!(3));
    /// assert!(g.trunc() == qd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn trunc(self) -> Quad {
        if self.0 >= 0.0 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    /// Returns the fractional part of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.3);
    /// let g = qd!(-3.7);
    ///
    /// let fdiff = (f.fract() - qd!(0.3)).abs();
    /// let gdiff = (g.fract() - qd!(-0.7)).abs();
    ///
    /// assert!(fdiff < qd!(1e-60));
    /// assert!(gdiff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn fract(self) -> Quad {
        self - self.trunc()
    }

    /// Returns a number that represents the sign of the `Quad`.
    ///
    /// * `1.0` if the number is positive, including `+0.0` and [`INFINITY`]
    /// * `-1.0` if the number is negative, including `-0.0` and [`NEG_INFINITY`]
    /// *  [`NAN`] if the number is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(3.5).signum() == Quad::ONE);
    /// assert!(Quad::NEG_INFINITY.signum() == Quad::NEG_ONE);
    /// assert!(Quad::NAN.signum().is_nan());
    /// # }
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    /// [`NEG_INFINITY`]: #associatedconstant.NEG_INFINITY
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    pub fn signum(self) -> Quad {
        if self.is_nan() {
            Quad::NAN
        } else if self.is_sign_negative() {
            Quad::NEG_ONE
        } else {
            Quad::ONE
        }
    }

    /// Returns the floating point category of the `Quad`.
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
    /// # use qd::Quad;
    /// # fn main() {
    /// use std::num::FpCategory;
    ///
    /// let num = qd!(12.4);
    /// let inf = Quad::INFINITY;
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

    /// Returns `true` if the `Quad` is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let min = Quad::MIN_POSITIVE;
    /// let max = Quad::MAX;
    /// let lower = qd!(1e-308);
    /// let zero = Quad::ZERO;
    ///
    /// assert!(min.is_normal());
    /// assert!(max.is_normal());
    ///
    /// assert!(!zero.is_normal());
    /// assert!(!Quad::NAN.is_normal());
    /// assert!(!Quad::INFINITY.is_normal());
    /// // Values between `0` and `MIN_POSITIVE` are subnormal.
    /// assert!(!lower.is_normal());
    /// # }
    /// ```
    #[inline]
    pub fn is_normal(self) -> bool {
        self.classify() == FpCategory::Normal
    }

    /// Returns `true` if the `Quad` is either positive or negative zero.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// assert!(Quad::ZERO.is_zero());
    /// assert!(Quad::NEG_ZERO.is_zero());
    /// assert!(!Quad::PI.is_zero());
    /// ```
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    /// Returns `true` if the `Quad` is negative, including negative zero, negative
    /// infinity, and `NaN` with a negative sign bit.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::NEG_ZERO.is_sign_negative());
    /// assert!(Quad::NEG_INFINITY.is_sign_negative());
    /// assert!(qd!(-7.0).is_sign_negative());
    /// assert!(!Quad::ZERO.is_sign_negative());
    /// assert!(!qd!(7.0).is_sign_negative());
    /// # }
    /// ```
    #[inline]
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    /// Returns `true` if the `Quad` is positive, including positive zero, positive infinity
    /// and `NaN` with a positive sign bit.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::ZERO.is_sign_positive());
    /// assert!(Quad::INFINITY.is_sign_positive());
    /// assert!(qd!(7.0).is_sign_positive());
    /// assert!(!Quad::NEG_ZERO.is_sign_positive());
    /// assert!(!qd!(-7.0).is_sign_positive());
    /// # }
    /// ```
    #[inline]
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    /// Returns `true` if the `Quad` is `NaN`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::NAN.is_nan());
    /// assert!(!qd!(7.0).is_nan());
    /// # }
    /// ```
    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns `true` if the `Quad` is positive or negative infinity.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::INFINITY.is_infinite());
    /// assert!(Quad::NEG_INFINITY.is_infinite());
    /// assert!(!Quad::NAN.is_infinite());
    /// assert!(!qd!(7.0).is_infinite());
    /// # }
    /// ```
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Returns `true` if the `Quad` is neither infinite nor `NaN`..
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(!Quad::INFINITY.is_finite());
    /// assert!(!Quad::NEG_INFINITY.is_finite());
    /// assert!(!Quad::NAN.is_finite());
    /// assert!(qd!(7.0).is_finite());
    /// # }
    /// ```
    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Returns `true` if the `Quad` has an absolute value of less than [`MIN_POSITIVE`]. 
    ///
    /// Numbers this small can be represented by floating point numbers, but they are not as
    /// accurate. This inaccuracy is inherent in the IEEE-754 format for 64-bit numbers;
    /// making a double-double out of an inaccurate number means the double-double is also
    /// going to be inaccurate.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(!Quad::PI.is_subnormal());
    /// assert!(qd!(1e-308).is_subnormal());
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
        assert_exact!((-Quad::PI).abs(), Quad::PI);
        assert_exact!(Quad::PI.abs(), Quad::PI);
    }

    #[test]
    fn abs_zero() {
        assert_exact!(Quad::ZERO.abs(), Quad::ZERO);
        assert_exact!(Quad::NEG_ZERO.abs(), Quad::ZERO);
    }

    #[test]
    fn abs_inf() {
        assert_exact!(Quad::INFINITY.abs(), Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY.abs(), Quad::INFINITY);
    }

    #[test]
    fn abs_nan() {
        assert!(Quad::NAN.abs().is_nan());
    }

    #[test]
    fn floor() {
        assert_exact!(Quad::PI.floor(), qd!(3));
        assert_exact!(Quad::E.floor(), qd!(2));
        assert_exact!((-Quad::PI).floor(), qd!(-4));
        assert_exact!((-Quad::E).floor(), qd!(-3));
        assert_exact!(qd!(2).floor(), qd!(2));
    }

    #[test]
    fn floor_zero() {
        assert_exact!(Quad::ZERO.floor(), Quad::ZERO);
        assert_exact!(Quad::NEG_ZERO.floor(), Quad::NEG_ZERO);
    }

    #[test]
    fn floor_inf() {
        assert_exact!(Quad::INFINITY.floor(), Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY.floor(), Quad::NEG_INFINITY);
    }

    #[test]
    fn floor_nan() {
        assert!(Quad::NAN.floor().is_nan());
    }

    #[test]
    fn ceil() {
        assert_exact!(Quad::PI.ceil(), qd!(4));
        assert_exact!(Quad::E.ceil(), qd!(3));
        assert_exact!((-Quad::PI).ceil(), qd!(-3));
        assert_exact!((-Quad::E).ceil(), qd!(-2));
        assert_exact!(qd!(2).ceil(), qd!(2));
    }

    #[test]
    fn ceil_zero() {
        assert_exact!(Quad::ZERO.ceil(), Quad::ZERO);
        assert_exact!(Quad::NEG_ZERO.ceil(), Quad::NEG_ZERO);
    }

    #[test]
    fn ceil_inf() {
        assert_exact!(Quad::INFINITY.ceil(), Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY.ceil(), Quad::NEG_INFINITY);
    }

    #[test]
    fn ceil_nan() {
        assert!(Quad::NAN.ceil().is_nan());
    }

    #[test]
    fn round() {
        assert_exact!(Quad::PI.round(), qd!(3));
        assert_exact!(Quad::E.round(), qd!(3));
        assert_exact!((-Quad::PI).round(), qd!(-3));
        assert_exact!((-Quad::E).round(), qd!(-3));
        assert_exact!(qd!(2).round(), qd!(2));
        assert_exact!(qd!(2.5).round(), qd!(3));
        assert_exact!(qd!(-3.5).round(), qd!(-4));
    }

    #[test]
    fn round_zero() {
        assert_exact!(Quad::ZERO.round(), Quad::ZERO);
        assert_exact!(Quad::NEG_ZERO.round(), Quad::NEG_ZERO);
    }

    #[test]
    fn round_inf() {
        assert_exact!(Quad::INFINITY.round(), Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY.round(), Quad::NEG_INFINITY);
    }

    #[test]
    fn round_nan() {
        assert!(Quad::NAN.round().is_nan());
    }

    #[test]
    fn trunc() {
        assert_exact!(Quad::PI.trunc(), qd!(3));
        assert_exact!(Quad::E.trunc(), qd!(2));
        assert_exact!((-Quad::PI).trunc(), qd!(-3));
        assert_exact!((-Quad::E).trunc(), qd!(-2));
        assert_exact!(qd!(2).trunc(), qd!(2));
        assert_exact!(qd!(2.5).trunc(), qd!(2));
        assert_exact!(qd!(-3.5).trunc(), qd!(-3));
    }

    #[test]
    fn trunc_zero() {
        assert_exact!(Quad::ZERO.trunc(), Quad::ZERO);
        assert_exact!(Quad::NEG_ZERO.trunc(), Quad::NEG_ZERO);
    }

    #[test]
    fn trunc_inf() {
        assert_exact!(Quad::INFINITY.trunc(), Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY.trunc(), Quad::NEG_INFINITY);
    }

    #[test]
    fn trunc_nan() {
        assert!(Quad::NAN.trunc().is_nan());
    }

    #[test]
    fn fract() {
        assert_close!(Quad::PI.fract(), Quad::PI - qd!(3));
        assert_close!(Quad::E.fract(), Quad::E - qd!(2));
        assert_close!((-Quad::PI).fract(), -Quad::PI + qd!(3));
        assert_close!((-Quad::E).fract(), -Quad::E + qd!(2));
        assert_exact!(qd!(2).fract(), Quad::ZERO);
        assert_exact!(qd!(2.5).fract(), qd!(0.5));
        assert_exact!(qd!(-3.5).fract(), qd!(-0.5));
    }

    #[test]
    fn fract_zero() {
        assert_exact!(Quad::ZERO.fract(), Quad::ZERO);
        assert_exact!(Quad::NEG_ZERO.fract(), Quad::NEG_ZERO);
    }

    #[test]
    fn fract_inf() {
        assert_exact!(Quad::INFINITY.fract(), Quad::NAN);
        assert_exact!(Quad::NEG_INFINITY.fract(), Quad::NAN);
    }

    #[test]
    fn fract_nan() {
        assert!(Quad::NAN.fract().is_nan());
    }

    #[test]
    fn signum() {
        assert_exact!(Quad::PI.signum(), Quad::ONE);
        assert_exact!(Quad::E.signum(), Quad::ONE);
        assert_exact!((-Quad::PI).signum(), Quad::NEG_ONE);
        assert_exact!((-Quad::E).signum(), Quad::NEG_ONE);
        assert_exact!(qd!(2).signum(), Quad::ONE);
        assert_exact!(qd!(2.5).signum(), Quad::ONE);
        assert_exact!(qd!(-3.5).signum(), Quad::NEG_ONE);
    }

    #[test]
    fn signum_zero() {
        assert_exact!(Quad::ZERO.signum(), Quad::ONE);
        assert_exact!(Quad::NEG_ZERO.signum(), Quad::NEG_ONE);
    }

    #[test]
    fn signum_inf() {
        assert_exact!(Quad::INFINITY.signum(), Quad::ONE);
        assert_exact!(Quad::NEG_INFINITY.signum(), Quad::NEG_ONE);
    }

    #[test]
    fn signum_nan() {
        assert!(Quad::NAN.signum().is_nan());
    }

    #[test]
    fn classify() {
        use std::num::FpCategory::*;
        
        assert_eq!(Quad::PI.classify(), Normal);
        assert_eq!(Quad::ZERO.classify(), Zero);
        assert_eq!(Quad::NEG_ZERO.classify(), Zero);
        assert_eq!(Quad::INFINITY.classify(), Infinite);
        assert_eq!(Quad::NEG_INFINITY.classify(), Infinite);
        assert_eq!(Quad::NAN.classify(), Nan);
        assert_eq!(qd!(1e-308).classify(), Subnormal);
    }

    #[test]
    fn is_normal() {
        assert!(Quad::PI.is_normal());
        assert!((-Quad::PI).is_normal());
        assert!(!Quad::ZERO.is_normal());
        assert!(!Quad::NEG_ZERO.is_normal());
        assert!(!Quad::INFINITY.is_normal());
        assert!(!Quad::NEG_INFINITY.is_normal());
        assert!(!Quad::NAN.is_normal());
        assert!(!qd!(1e-308).is_normal());
    }

    #[test]
    fn is_zero() {
        assert!(!Quad::PI.is_zero());
        assert!(!(-Quad::PI).is_zero());
        assert!(Quad::ZERO.is_zero());
        assert!(Quad::NEG_ZERO.is_zero());
        assert!(!Quad::INFINITY.is_zero());
        assert!(!Quad::NEG_INFINITY.is_zero());
        assert!(!Quad::NAN.is_zero());
        assert!(!qd!(1e-308).is_zero());
    }

    #[test]
    fn is_sign_negative() {
        assert!(!Quad::PI.is_sign_negative());
        assert!((-Quad::PI).is_sign_negative());
        assert!(!Quad::ZERO.is_sign_negative());
        assert!(Quad::NEG_ZERO.is_sign_negative());
        assert!(!Quad::INFINITY.is_sign_negative());
        assert!(Quad::NEG_INFINITY.is_sign_negative());
        assert!(!Quad::NAN.is_sign_negative());
        assert!(!qd!(1e-308).is_sign_negative());
    }

    #[test]
    fn is_sign_positive() {
        assert!(Quad::PI.is_sign_positive());
        assert!(!(-Quad::PI).is_sign_positive());
        assert!(Quad::ZERO.is_sign_positive());
        assert!(!Quad::NEG_ZERO.is_sign_positive());
        assert!(Quad::INFINITY.is_sign_positive());
        assert!(!Quad::NEG_INFINITY.is_sign_positive());
        assert!(Quad::NAN.is_sign_positive());
        assert!(qd!(1e-308).is_sign_positive());
    }

    #[test]
    fn is_nan() {
        assert!(!Quad::PI.is_nan());
        assert!(!(-Quad::PI).is_nan());
        assert!(!Quad::ZERO.is_nan());
        assert!(!Quad::NEG_ZERO.is_nan());
        assert!(!Quad::INFINITY.is_nan());
        assert!(!Quad::NEG_INFINITY.is_nan());
        assert!(Quad::NAN.is_nan());
        assert!(!qd!(1e-308).is_nan());
    }

    #[test]
    fn is_infinite() {
        assert!(!Quad::PI.is_infinite());
        assert!(!(-Quad::PI).is_infinite());
        assert!(!Quad::ZERO.is_infinite());
        assert!(!Quad::NEG_ZERO.is_infinite());
        assert!(Quad::INFINITY.is_infinite());
        assert!(Quad::NEG_INFINITY.is_infinite());
        assert!(!Quad::NAN.is_infinite());
        assert!(!qd!(1e-308).is_infinite());
    }

    #[test]
    fn is_finite() {
        assert!(Quad::PI.is_finite());
        assert!((-Quad::PI).is_finite());
        assert!(Quad::ZERO.is_finite());
        assert!(Quad::NEG_ZERO.is_finite());
        assert!(!Quad::INFINITY.is_finite());
        assert!(!Quad::NEG_INFINITY.is_finite());
        assert!(!Quad::NAN.is_finite());
        assert!(qd!(1e-308).is_finite());
    }

    #[test]
    fn is_subnormal() {
        assert!(!Quad::PI.is_subnormal());
        assert!(!(-Quad::PI).is_subnormal());
        assert!(!Quad::ZERO.is_subnormal());
        assert!(!Quad::NEG_ZERO.is_subnormal());
        assert!(!Quad::INFINITY.is_subnormal());
        assert!(!Quad::NEG_INFINITY.is_subnormal());
        assert!(!Quad::NAN.is_subnormal());
        assert!(qd!(1e-308).is_subnormal());
    }
}

