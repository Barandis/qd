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

    // TODO: This needs investigation. It seems incorrect to double-convert a value, once to a
    // string and then back again, but parsing a number as a string might be the most sensible way
    // to do it in this particular case.

    // `unwrap` is safe because `n.to_string` will never return a string that can't be parsed into
    // a Double
    n.to_string().parse().unwrap()
}

#[inline]
fn split_u64(a: u64) -> (u32, u32) {
    let x = (a >> 32) as u32;
    let y = a as u32;
    (x, y)
}

fn from_u64(a: u64) -> Double {
    let (x, y) = split_u64(a);
    Double::from_add(x as f64 * 2f64.powi(32), y as f64)
}

fn from_i64(a: i64) -> Double {
    let sign = a.signum();
    let a = a.abs() as u64;
    let (x, y) = split_u64(a);
    let d = Double::from_add(x as f64 * 2f64.powi(32), y as f64);
    if sign == -1 { -d } else { d }
}

macro_rules! from_float_impl {
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

macro_rules! from_int_impl {
    ($($t:ty)*) => ($(
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
    )*);
}

from_float_impl! { f32 f64 }
from_int_impl! { i8 u8 i16 u16 i32 u32 }

impl From<(u64, u64)> for Double {
    fn from((a, b): (u64, u64)) -> Double {
        from_u64(a) + from_u64(b)
    }
}

impl From<u64> for Double {
    fn from(a: u64) -> Double {
        from_u64(a)
    }
}

impl From<(i64, i64)> for Double {
    fn from((a, b): (i64, i64)) -> Double {
        from_i64(a) + from_i64(b)
    }
}

impl From<i64> for Double {
    fn from(a: i64) -> Double {
        from_i64(a)
    }
}

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
    pub fn as_float(self) -> f64 {
        self.0
    }

    /// Converts the number into an `i64`.
    ///
    /// While it is possible for a `Double` to be created from a `u64`, whether or not the original
    /// is signed is not recorded (since `Double` itself is signed). The return value of this
    /// function can be cast to u64 if necessary.
    #[inline]
    pub fn as_int(self) -> i64 {
        self.0 as i64 + self.1 as i64
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_from_u64() {
        let a = 0x0123456789abcdefu64;
        let d = dd!(a);
        assert_eq!(format!("{}", a), format!("{}", d));
    }

    #[test]
    fn conv_from_i64() {
        let a = -0x0123456789abcdefi64;
        let d = dd!(a);
        assert_eq!(format!("{}", a), format!("{}", d));
    }

    #[test]
    fn conv_from_and_to_i64() {
        let a = -0x0123456789abcdefi64;
        let d = dd!(a);
        let x = d.as_int();
        assert_eq!(a, x);
    }
}
