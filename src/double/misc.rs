// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use crate::basic::quick_two_sum;

// #region Miscellaneous mathematical operations

impl Double {
    #[inline]
    pub fn abs(self) -> Double {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    #[inline]
    pub fn floor(self) -> Double {
        let hi = self.0.floor();

        if hi == self.0 {
            Double::norm(hi, self.1.floor())
        } else {
            Double(hi, 0.0)
        }
    }

    #[inline]
    pub fn ceil(self) -> Double {
        let hi = self.0.ceil();

        if hi == self.0 {
            Double::norm(hi, self.1.ceil())
        } else {
            Double(hi, 0.0)
        }
    }

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

    #[inline]
    pub fn trunc(self) -> Double {
        if self.0 >= 0.0 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    #[inline]
    pub fn fract(self) -> Double {
        self - self.trunc()
    }

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

    #[inline]
    pub fn is_normal(self) -> bool {
        self.classify() == FpCategory::Normal
    }

    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0.0
    }

    #[inline]
    pub fn is_one(self) -> bool {
        self.0 == 1.0 && self.1 == 0.0
    }

    #[inline]
    pub fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    #[inline]
    pub fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }

    #[inline]
    pub fn is_infinite(self) -> bool {
        self.0.is_infinite() || self.1.is_infinite()
    }

    #[inline]
    pub fn is_finite(self) -> bool {
        self.0.is_finite() && self.1.is_finite()
    }
}

// #endregion
