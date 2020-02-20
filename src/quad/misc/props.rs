// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::num::FpCategory;

impl Quad {
    /// Returns the floating point category of the quad-double. If only one
    /// property is being tested, it's generally faster to use the specific
    /// predicate rather than this function.
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
    pub fn classify(self) -> FpCategory {
        use std::num::FpCategory::*;

        let c0 = self.0.classify();
        let c1 = self.1.classify();
        let c2 = self.2.classify();
        let c3 = self.3.classify();

        if c0 == Zero && c1 == Zero && c2 == Zero && c3 == Zero {
            Zero
        } else if c0 == Subnormal
            || c1 == Subnormal
            || c2 == Subnormal
            || c3 == Subnormal
        {
            Subnormal
        } else if c0 == Infinite
            || c1 == Infinite
            || c2 == Infinite
            || c3 == Infinite
        {
            Infinite
        } else if c0 == Nan || c1 == Nan || c2 == Nan || c3 == Nan {
            Nan
        } else {
            Normal
        }
    }

    /// Returns `true` if the quad-double is neither zero, infinite, subnormal,
    /// or `NaN`.
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

    /// Returns `true` if the quad-double is either positive or negative zero.
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

    /// Returns `true` if the quad-double is negative, including negative zero
    /// and infinity and `NaN` with a negative sign bit.
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

    /// Returns `true` if the quad-double is positive, including positive zero
    /// and infinity and `NaN` with a positive sign bit.
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

    /// Returns `true` if the quad-double is `NaN`.
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
        self.0.is_nan() || self.1.is_nan()
    }

    /// Returns `true` if the quad-double is positive or negative infinity.
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
            || self.1.is_infinite()
            || self.2.is_infinite()
            || self.3.is_infinite()
    }

    /// Returns `true` if the quad-double is neither infinite nor `NaN`.
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
            && self.1.is_finite()
            && self.2.is_finite()
            && self.3.is_finite()
    }
}
