// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::DoubleDouble;

// #region Miscellaneous mathematical operations

impl DoubleDouble {
    #[inline]
    pub fn abs(self) -> DoubleDouble {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    #[inline]
    pub fn floor(self) -> DoubleDouble {
        let hi = self.0.floor();

        if hi == self.0 {
            DoubleDouble::norm(hi, self.1.floor())
        } else {
            DoubleDouble(hi, 0.0)
        }
    }

    #[inline]
    pub fn ceil(self) -> DoubleDouble {
        let hi = self.0.ceil();

        if hi == self.0 {
            DoubleDouble::norm(hi, self.1.ceil())
        } else {
            DoubleDouble(hi, 0.0)
        }
    }
}

// #endregion

// #region Number properties

impl DoubleDouble {
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
