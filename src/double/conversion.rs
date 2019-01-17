// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

// #region From implementations

fn from_float(n: f64) -> Double {
    if n == 0.0 {
        return if n.is_sign_negative() {
            Double::NEG_ZERO
        } else {
            Double::ZERO
        };
    }
    if n.is_nan() {
        return Double::NAN;
    }
    if n.is_infinite() {
        return if n.is_sign_negative() {
            Double::NEG_INFINITY
        } else {
            Double::INFINITY
        };
    }
    if n.floor() == n {
        return Double(n, 0.0);
    }

    // TODO: This needs investigation. The idea of converting the number back and forth from being
    // a string seems terribly inefficient, but that shouldn't be declared to be so until
    // benchmarking is done.

    // `unwrap` is safe because `n.to_string` will never return a string that can't be parsed into
    // a Double
    n.to_string().parse().unwrap()
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<($t, $t)> for Double {
            fn from((a, b): ($t, $t)) -> Double {
                Double(a.into(), b.into())
            }
        }

        impl From<$t> for Double {
            fn from(a: $t) -> Double {
                from_float(a.into())
            }
        }
    )*);
}

from_impl! { i8 i16 i32 u8 u16 u32 f32 f64 }

impl From<&str> for Double {
    /// Converts a string representation of a number into a `Double`.
    ///
    /// `parse` from [`FromStr`] is a safer way to make this conversion, as it returns a type
    /// (`Result`) that allows for error checking. This function returns `NaN` in the case of a
    /// parse error, which is indistinguishable from a legitimately-returned `NaN`. Take care when
    /// using this function.
    ///
    /// [`FromStr`]: #impl-FromStr
    fn from(s: &str) -> Double {
        s.parse().unwrap_or(Double::NAN)
    }
}

// #endregion

// #region Miscellaneous conversions

impl Double {
    /// Converts the number into an `f64`.
    #[inline]
    pub fn to_float(self) -> f64 {
        self.0
    }

    /// Converts the number into an `i32`.
    #[inline]
    pub fn to_int(self) -> i32 {
        self.0 as i32
    }
}

// #endregion
