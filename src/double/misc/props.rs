// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::num::FpCategory;

impl Double {
    /// Returns the floating point category of the double-double. If only one property is
    /// being tested, it's generally faster to use the specific predicate rather than this
    /// function.
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
        self.0.is_nan() || self.1.is_nan()
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
        self.0.is_infinite() || self.1.is_infinite()
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
        self.0.is_finite() && self.1.is_finite()
    }
}
