// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::utils as u;
use crate::quad::Quad;
use std::f64;
use std::num::FpCategory;

impl Quad {
    /// Calculates the absolute value of $x$, $|x|$, where $x$ is `self`. The absolute value
    /// of $x$ is simply the same value as $x$, but with the opposite sign if $x$ is
    /// negative.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// assert!(qd!(3).abs() == qd!(3));
    /// assert!(qd!(-3).abs() == qd!(3));
    /// ```
    #[inline]
    pub fn abs(self) -> Quad {
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
    /// # use qd::qd;
    /// let f = qd!(3.99);
    /// let g = qd!(3.0);
    ///
    /// assert!(f.floor() == qd!(3));
    /// assert!(g.floor() == qd!(3));
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
            let (a, b, c, d) = u::renorm4(a, b, c, d);
            Quad(a, b, c, d)
        } else {
            Quad(a, b, c, d)
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
    /// # use qd::qd;
    /// let f = qd!(3.01);
    /// let g = qd!(4.0);
    ///
    /// assert!(f.ceil() == qd!(4));
    /// assert!(g.ceil() == qd!(4));
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
            let (a, b, c, d) = u::renorm4(a, b, c, d);
            Quad(a, b, c, d)
        } else {
            Quad(a, b, c, d)
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
    /// # use qd::qd;
    /// let f = qd!(3.3);
    /// let g = qd!(-3.3);
    ///
    /// assert!(f.round() == qd!(3));
    /// assert!(g.round() == qd!(-3));
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
                    let (a, b, c, d) = u::renorm4(a, b, c, d);
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

    /// Returns the integer part of `self`. This integer part will be of the same sign as
    /// the original number.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let f = qd!(3.3);
    /// let g = qd!(-3.7);
    ///
    /// assert!(f.trunc() == qd!(3));
    /// assert!(g.trunc() == qd!(-3));
    /// ```
    #[inline]
    pub fn trunc(self) -> Quad {
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
    /// # use qd::qd;
    /// let f = qd!(3.3);
    /// let g = qd!(-3.7);
    ///
    /// let fdiff = (f.fract() - qd!(0.3)).abs();
    /// let gdiff = (g.fract() - qd!(-0.7)).abs();
    ///
    /// assert!(fdiff < qd!(1e-60));
    /// assert!(gdiff < qd!(1e-60));
    /// ```
    #[inline]
    pub fn fract(self) -> Quad {
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
    /// # use qd::{qd, Quad};
    /// assert!(qd!(3.5).signum() == Quad::ONE);
    /// assert!(Quad::NEG_INFINITY.signum() == Quad::NEG_ONE);
    /// assert!(Quad::NAN.signum().is_nan());
    /// ```
    ///
    /// [`ONE`]: #associatedconstant.ONE
    /// [`NEG_ONE`]: #associatedconstant.NEG_ONE
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
    /// A `Quad` can also register as `FpCategory::Subnormal` if it has a small enough
    /// negative exponent that one of the other components of the number is a subnormal
    /// number itself. This will typically happen around `1e-260` or so.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// use std::num::FpCategory;
    ///
    /// let num = qd!(12.4);
    /// let inf = Quad::INFINITY;
    ///
    /// assert!(num.classify() == FpCategory::Normal);
    /// assert!(inf.classify() == FpCategory::Infinite);
    /// ```
    ///
    /// [`FpCategory`]: https://doc.rust-lang.org/std/num/enum.FpCategory.html
    /// [`MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    pub fn classify(self) -> FpCategory {
        if self.0.classify() == FpCategory::Subnormal
            || self.1.classify() == FpCategory::Subnormal
            || self.2.classify() == FpCategory::Subnormal
            || self.3.classify() == FpCategory::Subnormal
        {
            // The other categories can be determined from only the first component, but a
            // number is subnormal if *any* component is subnormal.
            FpCategory::Subnormal
        } else {
            self.0.classify()
        }
    }

    /// Returns `true` if `self` is neither zero, infinite, subnormal, or `NaN`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
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
    /// ```
    #[inline]
    pub fn is_normal(self) -> bool {
        self.classify() == FpCategory::Normal
    }

    /// Returns `true` if `self` is either positive or negative zero.
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

    /// Returns `true` if `self` is negative, including negative zero, negative infinity,
    /// and `NaN` with a negative sign bit.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// assert!(Quad::NEG_ZERO.is_sign_negative());
    /// assert!(Quad::NEG_INFINITY.is_sign_negative());
    /// assert!(qd!(-7.0).is_sign_negative());
    /// assert!(!Quad::ZERO.is_sign_negative());
    /// assert!(!qd!(7.0).is_sign_negative());
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
    /// # use qd::{qd, Quad};
    /// assert!(Quad::ZERO.is_sign_positive());
    /// assert!(Quad::INFINITY.is_sign_positive());
    /// assert!(qd!(7.0).is_sign_positive());
    /// assert!(!Quad::NEG_ZERO.is_sign_positive());
    /// assert!(!qd!(-7.0).is_sign_positive());
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
    /// # use qd::{qd, Quad};
    /// assert!(Quad::NAN.is_nan());
    /// assert!(!qd!(7.0).is_nan());
    /// ```
    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns `true` if `self` is positive or negative infinity.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// assert!(Quad::INFINITY.is_infinite());
    /// assert!(Quad::NEG_INFINITY.is_infinite());
    /// assert!(!Quad::NAN.is_infinite());
    /// assert!(!qd!(7.0).is_infinite());
    /// ```
    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    /// Returns `true` if `self` is neither infinite nor `NaN`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// assert!(!Quad::INFINITY.is_finite());
    /// assert!(!Quad::NEG_INFINITY.is_finite());
    /// assert!(!Quad::NAN.is_finite());
    /// assert!(qd!(7.0).is_finite());
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
    /// # use qd::{qd, Quad};
    /// assert!(!Quad::PI.is_subnormal());
    /// assert!(qd!(1e-308).is_subnormal());
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
            Quad::PI,
            Quad::PI.abs();
        abs_neg:
            Quad::PI,
            (-Quad::PI).abs();
        abs_zero:
            Quad::ZERO,
            Quad::ZERO.abs();
        abs_neg_zero:
            Quad::ZERO,
            Quad::NEG_ZERO.abs();
        abs_inf:
            Quad::INFINITY,
            Quad::INFINITY.abs();
        abs_neg_inf:
            Quad::INFINITY,
            Quad::NEG_INFINITY.abs();
        abs_nan:
            Quad::NAN,
            Quad::NAN.abs();
    );

    // floor tests
    test_all_exact!(
        floor_pi:
            qd!(3),
            Quad::PI.floor();
        floor_e:
            qd!(2),
            Quad::E.floor();
        floor_neg_pi:
            qd!(-4),
            (-Quad::PI).floor();
        floor_neg_e:
            qd!(-3),
            (-Quad::E).floor();
        floor_int:
            qd!(2),
            qd!(2).floor();

        floor_zero:
            Quad::ZERO,
            Quad::ZERO.floor();
        floor_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.floor();
        floor_inf:
            Quad::INFINITY,
            Quad::INFINITY.floor();
        floor_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.floor();
        floor_nan:
            Quad::NAN,
            Quad::NAN.floor();
    );

    // ceil tests
    test_all_exact!(
        ceil_pi:
            qd!(4),
            Quad::PI.ceil();
        ceil_e:
            qd!(3),
            Quad::E.ceil();
        ceil_neg_pi:
            qd!(-3),
            (-Quad::PI).ceil();
        ceil_neg_e:
            qd!(-2),
            (-Quad::E).ceil();
        ceil_int:
            qd!(2),
            qd!(2).ceil();

        ceil_zero:
            Quad::ZERO,
            Quad::ZERO.ceil();
        ceil_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.ceil();
        ceil_inf:
            Quad::INFINITY,
            Quad::INFINITY.ceil();
        ceil_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.ceil();
        ceil_nan:
            Quad::NAN,
            Quad::NAN.ceil();
    );

    // round tests
    test_all_exact!(
        round_pi:
            qd!(3),
            Quad::PI.round();
        round_e:
            qd!(3),
            Quad::E.round();
        round_neg_pi:
            qd!(-3),
            (-Quad::PI).round();
        round_neg_e:
            qd!(-3),
            (-Quad::E).round();
        round_int:
            qd!(2),
            qd!(2).round();
        round_half:
            qd!(3),
            qd!(2.5).round();
        round_neg_half:
            qd!(-4),
            qd!(-3.5).round();

        round_zero:
            Quad::ZERO,
            Quad::ZERO.round();
        round_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.round();
        round_inf:
            Quad::INFINITY,
            Quad::INFINITY.round();
        round_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.round();
        round_nan:
            Quad::NAN,
            Quad::NAN.round();
    );

    // trunc tests
    test_all_exact!(
        trunc_pi:
            qd!(3),
            Quad::PI.trunc();
        trunc_e:
            qd!(2),
            Quad::E.trunc();
        trunc_neg_pi:
            qd!(-3),
            (-Quad::PI).trunc();
        trunc_neg_e:
            qd!(-2),
            (-Quad::E).trunc();
        trunc_int:
            qd!(2),
            qd!(2).trunc();
        trunc_half:
            qd!(2),
            qd!(2.5).trunc();
        trunc_neg_half:
            qd!(-3),
            qd!(-3.5).trunc();

        trunc_zero:
            Quad::ZERO,
            Quad::ZERO.trunc();
        trunc_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.trunc();
        trunc_inf:
            Quad::INFINITY,
            Quad::INFINITY.trunc();
        trunc_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.trunc();
        trunc_nan:
            Quad::NAN,
            Quad::NAN.trunc();
    );

    // fract tests
    test_all_near!(
        fract_pi:
            Quad::PI - qd!(3),
            Quad::PI.fract();
        fract_e:
            Quad::E - qd!(2),
            Quad::E.fract();
        fract_neg_pi:
            -Quad::PI + qd!(3),
            (-Quad::PI).fract();
        fract_neg_e:
            -Quad::E + qd!(2),
            (-Quad::E).fract();
    );
    test_all_exact!(
        fract_int:
            Quad::ZERO,
            qd!(2).fract();
        fract_half:
            qd!(0.5),
            qd!(2.5).fract();
        fract_neg_half:
            qd!(-0.5),
            qd!(-3.5).fract();

        fract_zero:
            Quad::ZERO,
            Quad::ZERO.fract();
        fract_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.fract();
        fract_inf:
            Quad::NAN,
            Quad::INFINITY.fract();
        fract_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.fract();
        fract_nan:
            Quad::NAN,
            Quad::NAN.fract();
    );

    // signum tests
    test_all_exact!(
        signum_pi:
            Quad::ONE,
            Quad::PI.signum();
        signum_e:
            Quad::ONE,
            Quad::E.signum();
        signum_neg_pi:
            Quad::NEG_ONE,
            (-Quad::PI).signum();
        signum_neg_e:
            Quad::NEG_ONE,
            (-Quad::E).signum();
        signum_int:
            Quad::ONE,
            qd!(2).signum();
        signum_half:
            Quad::ONE,
            qd!(2.5).signum();
        signum_neg_half:
            Quad::NEG_ONE,
            qd!(-3.5).signum();

        signum_zero:
            Quad::ONE,
            Quad::ZERO.signum();
        signum_neg_zero:
            Quad::NEG_ONE,
            Quad::NEG_ZERO.signum();
        signum_inf:
            Quad::ONE,
            Quad::INFINITY.signum();
        signum_neg_inf:
            Quad::NEG_ONE,
            Quad::NEG_INFINITY.signum();
        signum_nan:
            Quad::NAN,
            Quad::NAN.signum();
    );

    // classify tests
    test_all_eq!(
        classify_pi:
            Quad::PI.classify(),
            Normal;
        classify_zero:
            Quad::ZERO.classify(),
            Zero;
        classify_neg_zero:
            Quad::NEG_ZERO.classify(),
            Zero;
        classify_inf:
            Quad::INFINITY.classify(),
            Infinite;
        classify_neg_inf:
            Quad::NEG_INFINITY.classify(),
            Infinite;
        classify_nan:
            Quad::NAN.classify(),
            Nan;
        classify_sub:
            qd!(1e-308).classify(),
            Subnormal;
    );

    // is_normal tests
    test_all_assert!(
        is_normal_pi:
            Quad::PI.is_normal();
        is_normal_neg_pi:
            (-Quad::PI).is_normal();
        is_normal_zero:
            !Quad::ZERO.is_normal();
        is_normal_neg_zero:
            !Quad::NEG_ZERO.is_normal();
        is_normal_inf:
            !Quad::INFINITY.is_normal();
        is_normal_neg_inf:
            !Quad::NEG_INFINITY.is_normal();
        is_normal_nan:
            !Quad::NAN.is_normal();
        is_normal_sub:
            !qd!(1e-308).is_normal();
    );

    // is_zero tests
    test_all_assert!(
        is_zero_pi:
            !Quad::PI.is_zero();
        is_zero_neg_pi:
            !(-Quad::PI).is_zero();
        is_zero_zero:
            Quad::ZERO.is_zero();
        is_zero_neg_zero:
            Quad::NEG_ZERO.is_zero();
        is_zero_inf:
            !Quad::INFINITY.is_zero();
        is_zero_neg_inf:
            !Quad::NEG_INFINITY.is_zero();
        is_zero_nan:
            !Quad::NAN.is_zero();
        is_zero_sub:
            !qd!(1e-308).is_zero();
    );

    // is_sign_negative tests
    test_all_assert!(
        is_sign_negative_pi:
            !Quad::PI.is_sign_negative();
        is_sign_negative_neg_pi:
            (-Quad::PI).is_sign_negative();
        is_sign_negative_zero:
            !Quad::ZERO.is_sign_negative();
        is_sign_negative_neg_zero:
            Quad::NEG_ZERO.is_sign_negative();
        is_sign_negative_inf:
            !Quad::INFINITY.is_sign_negative();
        is_sign_negative_neg_inf:
            Quad::NEG_INFINITY.is_sign_negative();
        is_sign_negative_nan:
            !Quad::NAN.is_sign_negative();
        is_sign_negative_sub:
            !qd!(1e-308).is_sign_negative();
    );

    // is_sign_positive tests
    test_all_assert!(
        is_sign_positive_pi:
            Quad::PI.is_sign_positive();
        is_sign_positive_neg_pi:
            !(-Quad::PI).is_sign_positive();
        is_sign_positive_zero:
            Quad::ZERO.is_sign_positive();
        is_sign_positive_neg_zero:
            !Quad::NEG_ZERO.is_sign_positive();
        is_sign_positive_inf:
            Quad::INFINITY.is_sign_positive();
        is_sign_positive_neg_inf:
            !Quad::NEG_INFINITY.is_sign_positive();
        is_sign_positive_nan:
            Quad::NAN.is_sign_positive();
        is_sign_positive_sub:
            qd!(1e-308).is_sign_positive();
    );

    // is_nan tests
    test_all_assert!(
        is_nan_pi:
            !Quad::PI.is_nan();
        is_nan_neg_pi:
            !(-Quad::PI).is_nan();
        is_nan_zero:
            !Quad::ZERO.is_nan();
        is_nan_neg_zero:
            !Quad::NEG_ZERO.is_nan();
        is_nan_inf:
            !Quad::INFINITY.is_nan();
        is_nan_neg_inf:
            !Quad::NEG_INFINITY.is_nan();
        is_nan_nan:
            Quad::NAN.is_nan();
        is_nan_sub:
            !qd!(1e-308).is_nan();
    );

    // is_infinite tests
    test_all_assert!(
        is_infinite_pi:
            !Quad::PI.is_infinite();
        is_infinite_neg_pi:
            !(-Quad::PI).is_infinite();
        is_infinite_zero:
            !Quad::ZERO.is_infinite();
        is_infinite_neg_zero:
            !Quad::NEG_ZERO.is_infinite();
        is_infinite_inf:
            Quad::INFINITY.is_infinite();
        is_infinite_neg_inf:
            Quad::NEG_INFINITY.is_infinite();
        is_infinite_nan:
            !Quad::NAN.is_infinite();
        is_infinite_sub:
            !qd!(1e-308).is_infinite();
    );

    // is_finite tests
    test_all_assert!(
        is_finite_pi:
            Quad::PI.is_finite();
        is_finite_neg_pi:
            (-Quad::PI).is_finite();
        is_finite_zero:
            Quad::ZERO.is_finite();
        is_finite_neg_zero:
            Quad::NEG_ZERO.is_finite();
        is_finite_inf:
            !Quad::INFINITY.is_finite();
        is_finite_neg_inf:
            !Quad::NEG_INFINITY.is_finite();
        is_finite_nan:
            !Quad::NAN.is_finite();
        is_finite_sub:
            qd!(1e-308).is_finite();
    );

    // is_subnormal tests
    test_all_assert!(
        is_subnormal_pi:
            !Quad::PI.is_subnormal();
        is_subnormal_neg_pi:
            !(-Quad::PI).is_subnormal();
        is_subnormal_zero:
            !Quad::ZERO.is_subnormal();
        is_subnormal_neg_zero:
            !Quad::NEG_ZERO.is_subnormal();
        is_subnormal_inf:
            !Quad::INFINITY.is_subnormal();
        is_subnormal_neg_inf:
            !Quad::NEG_INFINITY.is_subnormal();
        is_subnormal_nan:
            !Quad::NAN.is_subnormal();
        is_subnormal_sub:
            qd!(1e-308).is_subnormal();
    );
}
