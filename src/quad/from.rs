// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::double::Double;
use crate::quad::Quad;
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

fn from_float(n: f64) -> Quad {
    if n == 0.0 {
        if n.is_sign_negative() {
            Quad::NEG_ZERO
        } else {
            Quad::ZERO
        }
    } else if n.is_nan() {
        Quad::NAN
    } else if n.is_infinite() {
        if n.is_sign_negative() {
            Quad::NEG_INFINITY
        } else {
            Quad::INFINITY
        }
    } else if is_dyadic(n) {
        Quad(n, 0.0, 0.0, 0.0)
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
        // parsed into a Quad.
        n.to_string().parse().unwrap()
    }
}

#[inline]
fn split_u64(a: u64) -> (u32, u32) {
    let x = (a >> 32) as u32;
    let y = a as u32;
    (x, y)
}

#[inline]
#[allow(clippy::many_single_char_names)]
fn split_u128(a: u128) -> (u32, u32, u32, u32) {
    let w = (a >> 96) as u32;
    let x = (a >> 64) as u32;
    let y = (a >> 32) as u32;
    let z = a as u32;
    (w, x, y, z)
}

fn from_u64(a: u64) -> Quad {
    let (x, y) = split_u64(a);
    let (a, b, c, d) = core::renorm4(x as f64 * 2f64.powi(32), y as f64, 0.0, 0.0);
    Quad(a, b, c, d)
}

fn from_i64(a: i64) -> Quad {
    let sign = a.signum();
    let a = a.abs() as u64;
    let (x, y) = split_u64(a);
    let (a, b, c, d) = core::renorm4(x as f64 * 2f64.powi(32), y as f64, 0.0, 0.0);
    if sign == -1 {
        Quad(-a, -b, -c, -d)
    } else {
        Quad(a, b, c, d)
    }
}

#[allow(clippy::many_single_char_names)]
fn from_u128(a: u128) -> Quad {
    let (w, x, y, z) = split_u128(a);
    let (a, b, c, d) = core::renorm4(
        w as f64 * 2f64.powi(96),
        x as f64 * 2f64.powi(64),
        y as f64 * 2f64.powi(32),
        z as f64,
    );
    Quad(a, b, c, d)
}

#[allow(clippy::many_single_char_names)]
fn from_i128(a: i128) -> Quad {
    let sign = a.signum();
    let a = a.abs() as u128;
    let (w, x, y, z) = split_u128(a);
    let (a, b, c, d) = core::renorm4(
        w as f64 * 2f64.powi(96),
        x as f64 * 2f64.powi(64),
        y as f64 * 2f64.powi(32),
        z as f64,
    );
    if sign == -1 {
        Quad(-a, -b, -c, -d)
    } else {
        Quad(a, b, c, d)
    }
}

macro_rules! from_int_impl {
    ($($t:ident)*) => ($(
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                Quad(a.into(), 0.0, 0.0, 0.0)
            }
        }
    )*);
}

macro_rules! from_float_impl {
    ($($t:ident)*) => ($(
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                from_float(a.into())
            }
        }
    )*);
}

macro_rules! from_long_int_impl {
    ($($t:ident $f:ident)*) => ($(
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                $f(a)
            }
        }
    )*);
}

from_int_impl! { i8 u8 i16 u16 i32 u32 }
from_long_int_impl! { i64 from_i64 u64 from_u64 i128 from_i128 u128 from_u128 }
from_float_impl! { f32 f64 }

impl From<Double> for Quad {
    fn from(a: Double) -> Quad {
        a.to_string().parse().unwrap()
    }
}

impl From<&str> for Quad {
    /// Converts a string representation of a number into a `Quad`.
    ///
    /// `parse` from [`FromStr`] is a safer way to make this conversion, as it returns a
    /// type (`Result`) that allows for error checking. This function returns `NaN` in the
    /// case of a parse error, which is indistinguishable from a legitimately-returned
    /// `NaN`. Take care when using this function.
    ///
    /// [`FromStr`]: #impl-FromStr
    fn from(s: &str) -> Quad {
        s.parse().unwrap_or(Quad::NAN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conv_from_u64() {
        let a = 0x0_123_456_789_abc_defu64;
        let q = qd!(a);
        assert_eq!(format!("{}", a), format!("{}", q));
    }

    #[test]
    fn conv_from_i64() {
        let a = -0x0_123_456_789_abc_defi64;
        let q = qd!(a);
        assert_eq!(format!("{}", a), format!("{}", q));
    }

    #[test]
    fn conv_from_and_to_i128() {
        let a = -0x0_123_456_789_abc_defi128;
        let q = qd!(a);
        let x = q.as_int();
        assert_eq!(a, x);
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
        assert_exact!(qd!(1.0), Quad(1.0, 0.0, 0.0, 0.0));
        assert_exact!(qd!(1.203125), Quad(1.203125, 0.0, 0.0, 0.0));
        assert_exact!(
            qd!(1.0005645751953125),
            Quad(1.0005645751953125, 0.0, 0.0, 0.0)
        );
        assert_ne!(qd!(1.1).1, 0.0);
        assert_exact!(qd!(0), Quad::ZERO);
        assert_exact!(qd!(-0.0), Quad::NEG_ZERO);
        assert_exact!(qd!(std::f64::INFINITY), Quad::INFINITY);
        assert_exact!(qd!(std::f64::NEG_INFINITY), Quad::NEG_INFINITY);
        assert_exact!(qd!(std::f64::NAN), Quad::NAN);
    }
}
