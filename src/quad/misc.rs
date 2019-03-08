// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use crate::common::basic::renorm4;
use std::num::FpCategory;

// #region Miscellaneous mathematical operations

impl Quad {
    /// Calculates the absolute value of the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
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

    /// Returns the largest integer less than or equal to the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {Absool
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

        if a == self.0 {
            b = self.1.floor();
            if b == self.1 {
                c = self.2.floor();
                if c == self.2 {
                    d = self.3.floor();
                }
            }
            Quad::from(renorm4(a, b, c, d))
        } else {
            Quad(a, b, c, d)
        }
    }

    /// Returns the smallest integer greater than or equal to the quad-double.
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

        if a == self.0 {
            b = self.1.ceil();
            if b == self.1 {
                c = self.2.ceil();
                if c == self.2 {
                    d = self.3.ceil();
                }
            }
            Quad::from(renorm4(a, b, c, d))
        } else {
            Quad(a, b, c, d)
        }
    }

    /// Returns the nearest integer to the quad-double. Half-way cases are rounded away from `0.0`.
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
        if a == self.0 {
            let b = self.1.round();
            if b == self.1 {
                let c = self.2.round();
                if c == self.2 {
                    let d = self.3.round();
                    Quad::from(renorm4(a, b, c, d))
                } else {
                    if (c - self.2).abs() == 0.5 && self.3 < 0.0 {
                        Quad(a, b, c - 1.0, 0.0)
                    } else {
                        Quad(a, b, c, 0.0)
                    }
                }
            } else {
                if (b - self.1).abs() == 0.5 && self.2 < 0.0 {
                    Quad(a, b - 1.0, 0.0, 0.0)
                } else {
                    Quad(a, b, 0.0, 0.0)
                }
            }
        } else {
            if (a - self.0).abs() == 0.5 && self.1 < 0.0 {
                Quad(a - 1.0, 0.0, 0.0, 0.0)
            } else {
                Quad(a, 0.0, 0.0, 0.0)
            }
        }
    }

    /// Returns the integer part of the quad-double.
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

    /// Returns the fractional part of the quad-double.
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

    /// Returns a number that represents the sign of the quad-double.
    ///
    /// * `1.0` if the quad-double is positive, `+0.0`, or [`INFINITY`]
    /// * `-1.0` if the quad-double is negative, `-0.0`, or [`NEG_INFINITY`]
    /// *  [`NAN`] if the quad-double is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(3.5).signum() == Quad::ONE);
    /// assert!(Quad::NEG_INFINITY.signum() == -Quad::ONE);
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
            -Quad::ONE
        } else {
            Quad::ONE
        }
    }
}

// #endregion

// #region Number properties

impl Quad {
    /// Returns the floating point category of the quad-double. If only one property is being
    /// tested, it's generally faster to use the specific predicate rather than this function.
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
        } else if c0 == Subnormal || c1 == Subnormal || c2 == Subnormal || c3 == Subnormal {
            Subnormal
        } else if c0 == Infinite || c1 == Infinite || c2 == Infinite || c3 == Infinite {
            Infinite
        } else if c0 == Nan || c1 == Nan || c2 == Nan || c3 == Nan {
            Nan
        } else {
            Normal
        }
    }

    /// Returns `true` if the quad-double is neither zero, infinite, subnormal, or `NaN`.
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

    /// Returns `true` if the quad-double is negative, including negative zero and infinity and
    /// `NaN` with a negative sign bit.
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

    /// Returns `true` if the quad-double is positive, including positive zero and infinity and
    /// `NaN` with a positive sign bit.
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
        self.0.is_infinite() || self.1.is_infinite() || self.2.is_infinite() || self.3.is_infinite()
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
        self.0.is_finite() && self.1.is_finite() && self.2.is_finite() && self.3.is_finite()
    }
}

// #endregion

// #region Conversions

impl Quad {
    /// Converts the quad-double into an `f64`.
    ///
    /// There *will* be accuracy loss if the quad-double was more accurate than an `f64` to begin
    /// with.
    #[inline]
    pub fn as_float(self) -> f64 {
        self.0
    }

    /// Converts the quad-double into an `i128`.
    ///
    /// While it is possible for a `Double` to be created from a `u128`, whether or not the original
    /// is signed is not recorded (since `Double` itself is signed). The return value of this
    /// function can be cast to `u128` if necessary.
    #[inline]
    pub fn as_int(self) -> i128 {
        self.0 as i128 + self.1 as i128 + self.2 as i128 + self.3 as i128
    }

    /// Converts the quad-double into a 2-tuple of `f64`s.
    ///
    /// The components of the returned tuples are the same numbers used to represent the quad-double
    /// internally.
    #[inline]
    pub fn as_tuple(self) -> (f64, f64, f64, f64) {
        (self.0, self.1, self.2, self.3)
    }

    /// Assigns the components of a tuple to the components of the quad-double.
    #[inline]
    pub fn assign(&mut self, (a, b, c, d): (f64, f64, f64, f64)) {
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

// #endregion
