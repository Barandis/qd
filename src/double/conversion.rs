// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

// #region From implementations

macro_rules! from_impl {
    ($t:ty) => {
        impl From<($t, $t)> for Double {
            fn from((a, b): ($t, $t)) -> Double {
                Double(a.into(), b.into())
            }
        }

        impl From<$t> for Double {
            fn from(a: $t) -> Double {
                Double(a.into(), 0.0)
            }
        }
    };
}

from_impl!(f64);
from_impl!(f32);
from_impl!(i32);
from_impl!(u32);
from_impl!(i16);
from_impl!(u16);
from_impl!(i8);
from_impl!(u8);

// #endregion

// #region Miscellaneous conversions

impl Double {
    #[inline]
    pub fn to_float(self) -> f64 {
        self.0 + self.1
    }

    #[inline]
    pub fn to_int(self) -> i32 {
        self.0 as i32
    }
}

// #endregion
