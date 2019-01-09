// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

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
}

// #endregion

// #region Number properties

impl Double {
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }

    #[inline]
    pub fn is_one(&self) -> bool {
        self.0 == 1.0 && self.1 == 0.0
    }

    #[inline]
    pub fn is_sign_negative(&self) -> bool {
        self.0.is_sign_negative()
    }

    #[inline]
    pub fn is_sign_positive(&self) -> bool {
        self.0.is_sign_positive()
    }

    #[inline]
    pub fn is_nan(&self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }

    #[inline]
    pub fn is_infinite(&self) -> bool {
        self.0.is_infinite() || self.1.is_infinite()
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite() && self.1.is_finite()
    }
}

// #endregion
