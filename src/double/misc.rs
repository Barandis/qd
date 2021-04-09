// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::utils as u;
use crate::double::Double;
use std::f64;
use std::num::FpCategory;

impl Double {
    /// Calculates the absolute value of $x$, $|x|$, where $x$ is `self`. The absolute value
    /// of $x$ is simply the same value as $x$, but with the opposite sign if $x$ is
    /// negative.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// assert!(dd!(3).abs() == dd!(3));
    /// assert!(dd!(-3).abs() == dd!(3));
    /// ```
    #[inline]
    pub fn abs(self) -> Double {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    /// Calculates the floor of $x$, $\lfloor{x}\rfloor$, where $x$ is `self`.\
    ///
    /// The floor of $x$ is the largest integer value less than or equal to $x$. This means
    /// that the floor of a negative number will have an absolute value greater than that of
    /// the number itself.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let f = dd!(3.99);
    /// let g = dd!(3.0);
    /// let h = dd!(-3.99);
    ///
    /// assert!(f.floor() == dd!(3));
    /// assert!(g.floor() == dd!(3));
    /// assert!(h.floor() == dd!(-4));
    /// ```
    #[inline]
    pub fn floor(self) -> Double {
        let hi = self.0.floor();

        if (hi - self.0).abs() < f64::EPSILON {
            let (a, b) = u::renorm2(hi, self.1.floor());
            Double(a, b)
        } else {
            Double(hi, 0.0)
        }
    }

    /// Calculates the ceiling of $x$, $\lceil{x}\rceil$, where $x$ is `self`.
    ///
    /// The ceiling of $x$ is the smallest integer value greater than or equal to $x$. This
    /// means that the ceiling of a negative number will have an absolute value the same as
    /// (not greater than) that of the number itself.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let f = dd!(3.01);
    /// let g = dd!(4.0);
    /// let h = dd!(-3.01);
    ///
    /// assert!(f.ceil() == dd!(4));
    /// assert!(g.ceil() == dd!(4));
    /// assert!(h.ceil() == dd!(-3));
    /// ```
    #[inline]
    pub fn ceil(self) -> Double {
        let hi = self.0.ceil();

        if (hi - self.0).abs() < f64::EPSILON {
            let (a, b) = u::renorm2(hi, self.1.ceil());
            Double(a, b)
        } else {
            Double(hi, 0.0)
        }
    }

    /// Calculates the rounded value of $x$, where $x$ is `self`.
    ///
    /// The rounded value is the nearest integer to $x$. Halfway cases (i.e., numbers with a
    /// fractional portion of `0.5`) are rounded away from `0`, per the behavior of `f64`'s
    /// `round` function.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let f = dd!(3.3);
    /// let g = dd!(3.5);
    /// let h = dd!(-3.3);
    ///
    /// assert!(f.round() == dd!(3));
    /// assert!(g.round() == dd!(4));
    /// assert!(h.round() == dd!(-3));
    /// ```
    #[inline]
    pub fn round(self) -> Double {
        let hi = self.0.round();

        if (hi - self.0).abs() < f64::EPSILON {
            let lo = self.1.round();
            let (a, b) = u::renorm2(hi, lo);
            Double(a, b)
        } else if ((hi - self.0).abs() - 0.5).abs() < f64::EPSILON && self.1 < 0.0 {
            Double(hi - 1.0, 0.0)
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the integer part of `self`. This integer part will be of the same sign as
    /// the original number.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let f = dd!(3.3);
    /// let g = dd!(-3.7);
    ///
    /// assert!(f.trunc() == dd!(3));
    /// assert!(g.trunc() == dd!(-3));
    /// ```
    #[inline]
    pub fn trunc(self) -> Double {
        if self.0 >= 0.0 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    /// Returns the fractional part of the `self`. This fractional part will be of the same
    /// sign as the original number.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let f = dd!(3.3);
    /// let g = dd!(-3.7);
    ///
    /// let fdiff = (f.fract() - dd!(0.3)).abs();
    /// let gdiff = (g.fract() - dd!(-0.7)).abs();
    ///
    /// assert!(fdiff < dd!(1e-30));
    /// assert!(gdiff < dd!(1e-30));
    /// ```
    #[inline]
    pub fn fract(self) -> Double {
        self - self.trunc()
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// * [`ONE`] if `self` is positive, including `+0.0` and [`INFINITY`]
    /// * [`NEG_ONE`] if `self` is negative, including `-0.0` and [`NEG_INFINITY`]
    /// * [`NAN`] if `self` is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(dd!(3.5).signum() == Double::ONE);
    /// assert!(Double::NEG_INFINITY.signum() == Double::NEG_ONE);
    /// assert!(Double::NAN.signum().is_nan());
    /// ```
    ///
    /// [`ONE`]: #associatedconstant.ONE
    /// [`NEG_ONE`]: #associatedconstant.NEG_ONE
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

    /// Returns `self`'s floating point category.
    ///
    /// The possible return values are the members of [`FpCategory`], as follows:
    ///
    /// * `FpCategory::Zero` if the number is $\pm0$;
    /// * `FpCategory::Infinite` if the number is $\pm\infin$;
    /// * `FpCategory::Nan` if the number is not a number;
    /// * `FpCategory::Subnormal` if the number is $\pm$[`MIN_POSITIVE`] (numbers this small
    ///     can be represented, but they lose some accuracy);
    /// * `FpCategory::Normal` if the number is anything else.
    ///
    /// A `Double` can also register as `FpCategory::Subnormal` if it has a small enough
    /// negative exponent that the second component of the number is a subnormal number
    /// itself. This will typically happen around `1e-292` or so.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// use std::num::FpCategory;
    ///
    /// let num = dd!(12.4);
    /// let inf = Double::INFINITY;
    ///
    /// assert!(num.classify() == FpCategory::Normal);
    /// assert!(inf.classify() == FpCategory::Infinite);
    /// ```
    ///
    /// [`FpCategory`]: https://doc.rust-lang.org/std/num/enum.FpCategory.html
    /// [`MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    pub fn classify(self) -> FpCategory {
        if self.0.classify() == FpCategory::Subnormal || self.1.classify() == FpCategory::Subnormal
        {
            // The other categories can be determined from only the first component, but a
            // number is subnormal if *either* component is subnormal.
            FpCategory::Subnormal
        } else {
            self.0.classify()
        }
    }

    /// Returns `true` if `self` is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
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
    /// ```
    #[inline]
    pub fn is_normal(self) -> bool {
        self.classify() == FpCategory::Normal
    }

    /// Returns `true` if `self` is either positive or negative zero.
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

    /// Returns `true` if `self` is negative, including negative zero, negative infinity,
    /// and `NaN` with a negative sign bit.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(Double::NEG_ZERO.is_sign_negative());
    /// assert!(Double::NEG_INFINITY.is_sign_negative());
    /// assert!(dd!(-7.0).is_sign_negative());
    /// assert!(!Double::ZERO.is_sign_negative());
    /// assert!(!dd!(7.0).is_sign_negative());
    /// ```
    #[inline]
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    /// Returns `true` if `self` is positive, including positive zero, positive infinity and
    /// `NaN` with a positive sign bit.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(Double::ZERO.is_sign_positive());
    /// assert!(Double::INFINITY.is_sign_positive());
    /// assert!(dd!(7.0).is_sign_positive());
    /// assert!(!Double::NEG_ZERO.is_sign_positive());
    /// assert!(!dd!(-7.0).is_sign_positive());
    /// ```
    #[inline]
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    /// Returns `true` if `self` is `NaN`.
    ///
    /// This is the proper way to test for `NaN` because it cannot be done with an equality
    /// test (since `NaN` is not equal to itself).
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(Double::NAN.is_nan());
    /// assert!(!dd!(7.0).is_nan());
    /// ```
    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns `true` if `self` is positive or negative infinity.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(Double::INFINITY.is_infinite());
    /// assert!(Double::NEG_INFINITY.is_infinite());
    /// assert!(!Double::NAN.is_infinite());
    /// assert!(!dd!(7.0).is_infinite());
    /// ```
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Returns `true` if `self` is neither infinite nor `NaN`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(!Double::INFINITY.is_finite());
    /// assert!(!Double::NEG_INFINITY.is_finite());
    /// assert!(!Double::NAN.is_finite());
    /// assert!(dd!(7.0).is_finite());
    /// ```
    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Returns `true` if `self` has an absolute value of less than [`MIN_POSITIVE`].
    ///
    /// Numbers this small can be represented by floating point numbers, but they are not as
    /// accurate. This inaccuracy is inherent in the IEEE-754 format for 64-bit numbers;
    /// making a double-double out of an inaccurate number means the double-double is also
    /// going to be inaccurate.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(!Double::PI.is_subnormal());
    /// assert!(dd!(1e-308).is_subnormal());
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
    use std::num::FpCategory::*;

    // abs tests
    test_all_exact!(
        abs_pos:
            Double::PI,
            Double::PI.abs();
        abs_neg:
            Double::PI,
            (-Double::PI).abs();
        abs_zero:
            Double::ZERO,
            Double::ZERO.abs();
        abs_neg_zero:
            Double::ZERO,
            Double::NEG_ZERO.abs();
        abs_inf:
            Double::INFINITY,
            Double::INFINITY.abs();
        abs_neg_inf:
            Double::INFINITY,
            Double::NEG_INFINITY.abs();
        abs_nan:
            Double::NAN,
            Double::NAN.abs();
    );

    // floor tests
    test_all_exact!(
        floor_pi:
            dd!(3),
            Double::PI.floor();
        floor_e:
            dd!(2),
            Double::E.floor();
        floor_neg_pi:
            dd!(-4),
            (-Double::PI).floor();
        floor_neg_e:
            dd!(-3),
            (-Double::E).floor();
        floor_int:
            dd!(2),
            dd!(2).floor();

        floor_zero:
            Double::ZERO,
            Double::ZERO.floor();
        floor_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.floor();
        floor_inf:
            Double::INFINITY,
            Double::INFINITY.floor();
        floor_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.floor();
        floor_nan:
            Double::NAN,
            Double::NAN.floor();
    );

    // ceil tests
    test_all_exact!(
        ceil_pi:
            dd!(4),
            Double::PI.ceil();
        ceil_e:
            dd!(3),
            Double::E.ceil();
        ceil_neg_pi:
            dd!(-3),
            (-Double::PI).ceil();
        ceil_neg_e:
            dd!(-2),
            (-Double::E).ceil();
        ceil_int:
            dd!(2),
            dd!(2).ceil();

        ceil_zero:
            Double::ZERO,
            Double::ZERO.ceil();
        ceil_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.ceil();
        ceil_inf:
            Double::INFINITY,
            Double::INFINITY.ceil();
        ceil_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.ceil();
        ceil_nan:
            Double::NAN,
            Double::NAN.ceil();
    );

    // round tests
    test_all_exact!(
        round_pi:
            dd!(3),
            Double::PI.round();
        round_e:
            dd!(3),
            Double::E.round();
        round_neg_pi:
            dd!(-3),
            (-Double::PI).round();
        round_neg_e:
            dd!(-3),
            (-Double::E).round();
        round_int:
            dd!(2),
            dd!(2).round();
        round_half:
            dd!(3),
            dd!(2.5).round();
        round_neg_half:
            dd!(-4),
            dd!(-3.5).round();

        round_zero:
            Double::ZERO,
            Double::ZERO.round();
        round_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.round();
        round_inf:
            Double::INFINITY,
            Double::INFINITY.round();
        round_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.round();
        round_nan:
            Double::NAN,
            Double::NAN.round();
    );

    // trunc tests
    test_all_exact!(
        trunc_pi:
            dd!(3),
            Double::PI.trunc();
        trunc_e:
            dd!(2),
            Double::E.trunc();
        trunc_neg_pi:
            dd!(-3),
            (-Double::PI).trunc();
        trunc_neg_e:
            dd!(-2),
            (-Double::E).trunc();
        trunc_int:
            dd!(2),
            dd!(2).trunc();
        trunc_half:
            dd!(2),
            dd!(2.5).trunc();
        trunc_neg_half:
            dd!(-3),
            dd!(-3.5).trunc();

        trunc_zero:
            Double::ZERO,
            Double::ZERO.trunc();
        trunc_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.trunc();
        trunc_inf:
            Double::INFINITY,
            Double::INFINITY.trunc();
        trunc_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.trunc();
        trunc_nan:
            Double::NAN,
            Double::NAN.trunc();
    );

    // fract tests
    test_all_near!(
        fract_pi:
            Double::PI - dd!(3),
            Double::PI.fract();
        fract_e:
            Double::E - dd!(2),
            Double::E.fract();
        fract_neg_pi:
            -Double::PI + dd!(3),
            (-Double::PI).fract();
        fract_neg_e:
            -Double::E + dd!(2),
            (-Double::E).fract();
    );
    test_all_exact!(
        fract_int:
            Double::ZERO,
            dd!(2).fract();
        fract_half:
            dd!(0.5),
            dd!(2.5).fract();
        fract_neg_half:
            dd!(-0.5),
            dd!(-3.5).fract();

        fract_zero:
            Double::ZERO,
            Double::ZERO.fract();
        fract_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.fract();
        fract_inf:
            Double::NAN,
            Double::INFINITY.fract();
        fract_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.fract();
        fract_nan:
            Double::NAN,
            Double::NAN.fract();
    );

    // signum tests
    test_all_exact!(
        signum_pi:
            Double::ONE,
            Double::PI.signum();
        signum_e:
            Double::ONE,
            Double::E.signum();
        signum_neg_pi:
            Double::NEG_ONE,
            (-Double::PI).signum();
        signum_neg_e:
            Double::NEG_ONE,
            (-Double::E).signum();
        signum_int:
            Double::ONE,
            dd!(2).signum();
        signum_half:
            Double::ONE,
            dd!(2.5).signum();
        signum_neg_half:
            Double::NEG_ONE,
            dd!(-3.5).signum();

        signum_zero:
            Double::ONE,
            Double::ZERO.signum();
        signum_neg_zero:
            Double::NEG_ONE,
            Double::NEG_ZERO.signum();
        signum_inf:
            Double::ONE,
            Double::INFINITY.signum();
        signum_neg_inf:
            Double::NEG_ONE,
            Double::NEG_INFINITY.signum();
        signum_nan:
            Double::NAN,
            Double::NAN.signum();
    );

    // classify tests
    test_all_eq!(
        classify_pi:
            Double::PI.classify(),
            Normal;
        classify_zero:
            Double::ZERO.classify(),
            Zero;
        classify_neg_zero:
            Double::NEG_ZERO.classify(),
            Zero;
        classify_inf:
            Double::INFINITY.classify(),
            Infinite;
        classify_neg_inf:
            Double::NEG_INFINITY.classify(),
            Infinite;
        classify_nan:
            Double::NAN.classify(),
            Nan;
        classify_sub:
            dd!(1e-308).classify(),
            Subnormal;
    );

    // is_normal tests
    test_all_assert!(
        is_normal_pi:
            Double::PI.is_normal();
        is_normal_neg_pi:
            (-Double::PI).is_normal();
        is_normal_zero:
            !Double::ZERO.is_normal();
        is_normal_neg_zero:
            !Double::NEG_ZERO.is_normal();
        is_normal_inf:
            !Double::INFINITY.is_normal();
        is_normal_neg_inf:
            !Double::NEG_INFINITY.is_normal();
        is_normal_nan:
            !Double::NAN.is_normal();
        is_normal_sub:
            !dd!(1e-308).is_normal();
    );

    // is_zero tests
    test_all_assert!(
        is_zero_pi:
            !Double::PI.is_zero();
        is_zero_neg_pi:
            !(-Double::PI).is_zero();
        is_zero_zero:
            Double::ZERO.is_zero();
        is_zero_neg_zero:
            Double::NEG_ZERO.is_zero();
        is_zero_inf:
            !Double::INFINITY.is_zero();
        is_zero_neg_inf:
            !Double::NEG_INFINITY.is_zero();
        is_zero_nan:
            !Double::NAN.is_zero();
        is_zero_sub:
            !dd!(1e-308).is_zero();
    );

    // is_sign_negative tests
    test_all_assert!(
        is_sign_negative_pi:
            !Double::PI.is_sign_negative();
        is_sign_negative_neg_pi:
            (-Double::PI).is_sign_negative();
        is_sign_negative_zero:
            !Double::ZERO.is_sign_negative();
        is_sign_negative_neg_zero:
            Double::NEG_ZERO.is_sign_negative();
        is_sign_negative_inf:
            !Double::INFINITY.is_sign_negative();
        is_sign_negative_neg_inf:
            Double::NEG_INFINITY.is_sign_negative();
        is_sign_negative_nan:
            !Double::NAN.is_sign_negative();
        is_sign_negative_sub:
            !dd!(1e-308).is_sign_negative();
    );

    // is_sign_positive tests
    test_all_assert!(
        is_sign_positive_pi:
            Double::PI.is_sign_positive();
        is_sign_positive_neg_pi:
            !(-Double::PI).is_sign_positive();
        is_sign_positive_zero:
            Double::ZERO.is_sign_positive();
        is_sign_positive_neg_zero:
            !Double::NEG_ZERO.is_sign_positive();
        is_sign_positive_inf:
            Double::INFINITY.is_sign_positive();
        is_sign_positive_neg_inf:
            !Double::NEG_INFINITY.is_sign_positive();
        is_sign_positive_nan:
            Double::NAN.is_sign_positive();
        is_sign_positive_sub:
            dd!(1e-308).is_sign_positive();
    );

    // is_nan tests
    test_all_assert!(
        is_nan_pi:
            !Double::PI.is_nan();
        is_nan_neg_pi:
            !(-Double::PI).is_nan();
        is_nan_zero:
            !Double::ZERO.is_nan();
        is_nan_neg_zero:
            !Double::NEG_ZERO.is_nan();
        is_nan_inf:
            !Double::INFINITY.is_nan();
        is_nan_neg_inf:
            !Double::NEG_INFINITY.is_nan();
        is_nan_nan:
            Double::NAN.is_nan();
        is_nan_sub:
            !dd!(1e-308).is_nan();
    );

    // is_infinite tests
    test_all_assert!(
        is_infinite_pi:
            !Double::PI.is_infinite();
        is_infinite_neg_pi:
            !(-Double::PI).is_infinite();
        is_infinite_zero:
            !Double::ZERO.is_infinite();
        is_infinite_neg_zero:
            !Double::NEG_ZERO.is_infinite();
        is_infinite_inf:
            Double::INFINITY.is_infinite();
        is_infinite_neg_inf:
            Double::NEG_INFINITY.is_infinite();
        is_infinite_nan:
            !Double::NAN.is_infinite();
        is_infinite_sub:
            !dd!(1e-308).is_infinite();
    );

    // is_finite tests
    test_all_assert!(
        is_finite_pi:
            Double::PI.is_finite();
        is_finite_neg_pi:
            (-Double::PI).is_finite();
        is_finite_zero:
            Double::ZERO.is_finite();
        is_finite_neg_zero:
            Double::NEG_ZERO.is_finite();
        is_finite_inf:
            !Double::INFINITY.is_finite();
        is_finite_neg_inf:
            !Double::NEG_INFINITY.is_finite();
        is_finite_nan:
            !Double::NAN.is_finite();
        is_finite_sub:
            dd!(1e-308).is_finite();
    );

    // is_subnormal tests
    test_all_assert!(
        is_subnormal_pi:
            !Double::PI.is_subnormal();
        is_subnormal_neg_pi:
            !(-Double::PI).is_subnormal();
        is_subnormal_zero:
            !Double::ZERO.is_subnormal();
        is_subnormal_neg_zero:
            !Double::NEG_ZERO.is_subnormal();
        is_subnormal_inf:
            !Double::INFINITY.is_subnormal();
        is_subnormal_neg_inf:
            !Double::NEG_INFINITY.is_subnormal();
        is_subnormal_nan:
            !Double::NAN.is_subnormal();
        is_subnormal_sub:
            dd!(1e-308).is_subnormal();
    );
}
