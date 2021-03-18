// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::num::FpCategory;

impl Double {
    /// Returns the floating point category of the double-double.
    ///
    /// The possible return values are the members of `FpCategory`, as follows:
    ///
    /// * `FpCategory::Zero` if the number is ±0;
    /// * `FpCategory::Infinite` if the number is ±∞;
    /// * `FpCategory::Nan` if the number is not a number;
    /// * `FpCategory::Subnormal` if the number is ±[`Double::MIN_POSITIVE`] (numbers this
    ///     small can be represented, but they lose some accuracy);
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
    /// [`Double::MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
    pub fn classify(self) -> FpCategory {
        self.0.classify()
    }

    /// Returns `true` if the double-double is neither zero, infinite, subnormal, or `NaN`.
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

    /// Returns `true` if the double-double is either positive or negative zero.
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

    /// Returns `true` if the double-double is negative, including negative zero and
    /// infinity and `NaN` with a negative sign bit.
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

    /// Returns `true` if the double-double is positive, including positive zero and
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

    /// Returns `true` if the double-double is `NaN`.
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

    /// Returns `true` if the double-double is positive or negative infinity.
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

    /// Returns `true` if the double-double is neither infinite nor `NaN`.
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

    /// Returns `true` if the double-double has an absolute value of less than
    /// [`Double::MIN_POSITIVE`]. 
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
    /// [`Double::MIN_POSITIVE`]: #associatedconstant.MIN_POSITIVE
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
