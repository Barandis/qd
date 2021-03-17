// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic;
use crate::double::Double;
use std::f64;

// Determines whether a number is exact (true) or has floating-point error (false)
fn is_dyadic(n: f64) -> bool {
    let f = n.fract();
    if f == 0.0 {
        true
    } else {
        let len = f.to_string().len() - 2; // ignore the leading "0."
        let base = 2f64.powi(-(len as i32));
        f % base == 0.0
    }
}

fn from_float(n: f64) -> Double {
    if n == 0.0 {
        if n.is_sign_negative() {
            Double::NEG_ZERO
        } else {
            Double::ZERO
        }
    } else if n.is_nan() {
        Double::NAN
    } else if n.is_infinite() {
        if n.is_sign_negative() {
            Double::NEG_INFINITY
        } else {
            Double::INFINITY
        }
    } else if is_dyadic(n) {
        Double(n, 0.0)
    } else {
        // Yes, this converts an f64 to a string and then parses it. After a lot of study,
        // doing it decimal-digit-by-decimal-digit seems to be the only way to do this
        // accurately, because doing it as a whole f64 causes floating-point error to cancel
        // itself out. And parsing from a string is the most reasonable way to do it
        // digit-by-digit.
        //
        // I'm concerned about the effect on the speed of math functions that use it, like
        // `log` and `exp` and `powf`, but the answer seems to be to optimize the parsing.
        //
        // `unwrap` is safe because `n.to_string` will never return a string that can't be
        // parsed into a Double.
        n.to_string().parse().unwrap()
    }
}

#[inline]
fn split_u64(a: u64) -> (u32, u32) {
    let x = (a >> 32) as u32;
    let y = a as u32;
    (x, y)
}

fn from_u64(a: u64) -> Double {
    let (x, y) = split_u64(a);
    Double::from(basic::renorm2(x as f64 * 2f64.powi(32), y as f64))
}

fn from_i64(a: i64) -> Double {
    let sign = a.signum();
    let a = a.abs() as u64;
    let (x, y) = split_u64(a);
    let d = Double::from(basic::renorm2(x as f64 * 2f64.powi(32), y as f64));
    if sign == -1 {
        -d
    } else {
        d
    }
}

macro_rules! from_int_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for Double {
            fn from(a: $t) -> Double {
                Double(a.into(), 0.0)
            }
        }
    )*);
}

macro_rules! from_float_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for Double {
            fn from(a: $t) -> Double {
                from_float(a.into())
            }
        }
    )*);
}

macro_rules! from_tuple_impl {
    ($($t:ty)*) => ($(
        impl From<($t, $t)> for Double {
            fn from((a, b): ($t, $t)) -> Double {
                Double(a.into(), b.into())
            }
        }
    )*);
}

from_int_impl! { i8 u8 i16 u16 i32 u32 }
from_float_impl! { f32 f64 }
from_tuple_impl! { i8 u8 i16 u16 i32 u32 f32 f64 }

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
    /// `parse` from [`FromStr`] is a safer way to make this conversion, as it returns a
    /// type (`Result`) that allows for error checking. This function returns `NaN` in the
    /// case of a parse error, which is indistinguishable from a legitimately-returned
    /// `NaN`. Take care when using this function.
    ///
    /// [`FromStr`]: #impl-FromStr
    fn from(s: &str) -> Double {
        s.parse().unwrap_or(Double::NAN)
    }
}

impl From<Double> for f64 {
    /// Converts a `Double` into an `f64`.
    /// 
    /// This will lose precision if the second component of the `Double` is not 0, but it
    /// will not lose range.
    /// 
    /// No other conversions from `Double` to numeric types are provided, as every other one
    /// has the capability of losing range. Casts can be made from the `f64` provided by
    /// this function to other numeric types as needed.
    #[inline]
    fn from(a: Double) -> f64 {
        a.0
    }
}

impl From<Double> for (f64, f64) {
    /// Converts a `Double` into a 2-tuple of `f64`s.
    /// 
    /// The components of the resulting tuple are simply the components of the `Double`.
    #[inline]
    fn from(a: Double) -> (f64, f64) {
        (a.0, a.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_from_u64() {
        let a = 0x0_123_456_789_abc_defu64;
        let d = dd!(a);
        assert_eq!(format!("{}", a), format!("{}", d));
    }

    #[test]
    fn conv_from_i64() {
        let a = -0x0_123_456_789_abc_defi64;
        let d = dd!(a);
        assert_eq!(format!("{}", a), format!("{}", d));
    }

    #[test]
    fn dyadic() {
        assert!(is_dyadic(1.0));
        assert!(is_dyadic(1.5));
        assert!(is_dyadic(1.75));
        assert!(is_dyadic(1.625));
        assert!(is_dyadic(1.8125));
        assert!(is_dyadic(1.40625));
        assert!(is_dyadic(1.203125));
        assert!(is_dyadic(1.1015625));
        assert!(is_dyadic(1.14453125));
        assert!(is_dyadic(1.0005645751953125));
        assert!(!is_dyadic(1.1));
    }

    #[test]
    fn conv_from_f64() {
        assert_exact!(dd!(1.0), Double(1.0, 0.0));
        assert_exact!(dd!(1.203125), Double(1.203125, 0.0));
        assert_exact!(dd!(1.0005645751953125), Double(1.0005645751953125, 0.0));
        assert_ne!(dd!(1.1).1, 0.0);
        assert_exact!(dd!(0), Double::ZERO);
        assert_exact!(dd!(-0.0), Double::NEG_ZERO);
        assert_exact!(dd!(std::f64::INFINITY), Double::INFINITY);
        assert_exact!(dd!(std::f64::NEG_INFINITY), Double::NEG_INFINITY);
        assert_exact!(dd!(std::f64::NAN), Double::NAN);
        println!("{:.32}", Double(1.1, 0.0));
        println!("{:?}", Double::from(1.1));
    }
}
