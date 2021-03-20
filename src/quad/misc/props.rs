// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::num::FpCategory;

impl Quad {
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
    use std::num::FpCategory::*;

    #[test]
    fn classify() {
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

