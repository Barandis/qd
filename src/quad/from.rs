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
    // The first part prevents a.abs() from failing with overflow because the absolute
    // value of i64::MIN is i64::MAX + 1
    let a = if a == i64::MIN {
        i64::MAX as u64 + 1
    } else {
        a.abs() as u64
    };
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
    // The first part prevents a.abs() from failing with overflow because the absolute
    // value of i128::MIN is i128::MAX + 1
    let a = if a == i128::MIN {
        i128::MAX as u128 + 1
    } else {
        a.abs() as u128
    };
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
    ($(
        $(#[$m:meta])*
        $t:ident
    )*) => ($(
        $(#[$m])*
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                Quad(a.into(), 0.0, 0.0, 0.0)
            }
        }
    )*);
}

macro_rules! from_long_int_impl {
    ($(
        $(#[$m:meta])*
        $t:ident
        $f:ident
    )*) => ($(
        $(#[$m])*
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                $f(a)
            }
        }
    )*);
}

// The Rust conversion from f32 to f64 is a bit-for-bit translation. It does not attempt to
// account for floating point rounding error, so the parsed f64 is different from the
// "equivalent" parsed f32. So rather than having a helper function that takes an f64, we
// put the entire function into this macro so that `a.to_string().parse().unwrap()` calls
// the f32 parser if an f32 is being converted.
//
// is_dyadic is still fine to convert for, because a dyadic f32 will convert accurately
// into an f64 (and still return true) while a non-dyadic f32 may not convert accurately,
// but it'll still be non-dyadic after the conversion.
macro_rules! from_float_impl {
    ($(
        $(#[$m:meta])*
        $t:ident
    )*) => ($(
        $(#[$m])*
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                if a == 0.0 {
                    if a.is_sign_negative() {
                        Quad::NEG_ZERO
                    } else {
                        Quad::ZERO
                    }
                } else if a.is_nan() {
                    Quad::NAN
                } else if a.is_infinite() {
                    if a.is_sign_negative() {
                        Quad::NEG_INFINITY
                    } else {
                        Quad::INFINITY
                    }
                } else if is_dyadic(a as f64) {
                    Quad(a.into(), 0.0, 0.0, 0.0)
                } else {
                    // Yes, this converts an f32/f64 to a string and then parses it. After a
                    // lot of study, doing it decimal-digit-by-decimal-digit seems to be the
                    // only way to do this accurately, because doing it as a whole f64
                    // causes floating-point error to cancel itself out. And parsing from a
                    // string is the most reasonable way to do it digit-by-digit.
                    //
                    // I'm concerned about the effect on the speed of math functions that
                    // use it, like `log` and `exp` and `powf`, but the answer seems to be
                    // to optimize the parsing.
                    //
                    // `unwrap` is safe because `n.to_string` will never return a string
                    // that can't be parsed into a Quad.
                    a.to_string().parse().unwrap()
                }
            }
        }
    )*);
}

from_int_impl! {
    /// Generates a `Quad` from an `i8`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = i8::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-128");
    /// # }
    /// ```
    i8
    /// Generates a `Quad` from a `u8`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = u8::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "255");
    /// # }
    /// ```
    u8
    /// Generates a `Quad` from an `i16`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = i16::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-32768");
    /// # }
    /// ```
    i16
    /// Generates a `Quad` from a `u8`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = u16::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "65535");
    /// # }
    /// ```
    u16
    /// Generates a `Quad` from an `i32`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = i32::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-2147483648");
    /// # }
    /// ```
    i32
    /// Generates a `Quad` from a `u32`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = u32::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "4294967295");
    /// # }
    /// ```
    u32
}

from_long_int_impl! {
    /// Generates a `Quad` from an `i64`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = i64::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-9223372036854775808");
    /// # }
    /// ```
    i64 from_i64
    /// Generates a `Quad` from a `u64`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = u64::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "18446744073709551615");
    /// # }
    /// ```
    u64 from_u64
    /// Generates a `Quad` from an `i128`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = i128::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-170141183460469231731687303715884105728");
    /// # }
    /// ```
    i128 from_i128
    /// Generates a `Quad` from a `u128`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = u128::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "340282366920938463463374607431768211455");
    /// # }
    /// ```
    u128 from_u128
}

from_float_impl! {
    /// Generates a `Quad` from an `f32`.
    ///
    /// This function *does* account for floating point rounding error. Even though the
    /// first component of a `Quad` is enough to fit an `f32`, if that `f32` is not
    /// exactly representable in binary, then the second component of the `Quad` will
    /// account for the rounding error.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x = 0.9921875f32;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.9921875");
    ///
    /// // Xot exactly representable in binary
    /// let x = 0.9921876f32;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.9921876");
    /// # }
    /// ```
    f32
    /// Generates a `Quad` from an `f64`.
    ///
    /// This function *does* account for floating point rounding error. Even though the
    /// first component of a `Quad` is enough to fit an `f64`, if that `f64` is not
    /// exactly representable in binary, then the second component of the `Quad` will
    /// account for the rounding error.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x = 0.999969482421875f64;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.999969482421875");
    ///
    /// // Not exactly representable in binary
    /// let x = 0.999969482421876f64;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.999969482421876");
    /// # }
    /// ```
    f64
}

impl From<Double> for Quad {
    /// Generates a `Quad` from a `Double`.
    /// 
    /// The new `Quad`'s third and fourth components will be used to account for
    /// floating-point rounding error at the end of the `Double`, but it will of course
    /// otherwise only have the precision of the `Double` used to make it.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let expected = Quad::from("0.9999999303082806237436760862691");
    /// let a = (dd!(3).powi(15) - dd!(1)) / dd!(3).powi(15);
    /// let x = Quad::from(a);
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    fn from(a: Double) -> Quad {
        a.to_string().parse().unwrap()
    }
}

impl From<&str> for Quad {
    /// Parses a string to create a `Quad`.
    ///
    /// The parser works pretty similarly to parsers for `f32` and `f64`. It will fail if
    /// characters are present that are not digits, decimal points, signs, or exponent
    /// markers. It will also fail if there are multiples of these or if they're in the
    /// wrong places; two decimal points or a negative sign after the number will both be
    /// rejected, for instance.
    ///
    /// Failure will return [`NAN`]. This can be an issue because parsing the string `"nan"`
    /// *also* produces [`NAN`]. For this reason it's suggested to use [`from_str`] (or its
    /// associated `parse` function) instead of this function if there is any chance that
    /// the parsed string will be legitimately [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
    /// let x = Quad::from("0.9999999303082806237436760862691492808476631704421807180156648865");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    /// [`from_str`]: #method.from_str
    fn from(s: &str) -> Quad {
        s.parse().unwrap_or(Quad::NAN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn from_f32() {
        assert_exact!(qd!(1.0f32), Quad(1.0, 0.0, 0.0, 0.0));
        assert_exact!(qd!(1.203125f32), Quad(1.203125, 0.0, 0.0, 0.0));
        assert_ne!(qd!(1.1f32).1, 0.0);
        assert_exact!(qd!(0f32), Quad::ZERO);
        assert_exact!(qd!(-0.0f32), Quad::NEG_ZERO);
        assert_exact!(qd!(std::f32::INFINITY), Quad::INFINITY);
        assert_exact!(qd!(std::f32::NEG_INFINITY), Quad::NEG_INFINITY);
        assert_exact!(qd!(std::f32::NAN), Quad::NAN);
    }

    #[test]
    fn from_f64() {
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

    #[test]
    fn from_int() {
        assert_eq!(i8::MIN.to_string(), qd!(i8::MIN).to_string());
        assert_eq!(u8::MAX.to_string(), qd!(u8::MAX).to_string());
        assert_eq!(i16::MIN.to_string(), qd!(i16::MIN).to_string());
        assert_eq!(u16::MAX.to_string(), qd!(u16::MAX).to_string());
        assert_eq!(i32::MIN.to_string(), qd!(i32::MIN).to_string());
        assert_eq!(u32::MAX.to_string(), qd!(u32::MAX).to_string());
        assert_eq!(i64::MIN.to_string(), qd!(i64::MIN).to_string());
        assert_eq!(u64::MAX.to_string(), qd!(u64::MAX).to_string());
        assert_eq!(i128::MIN.to_string(), qd!(i128::MIN).to_string());
        assert_eq!(u128::MAX.to_string(), qd!(u128::MAX).to_string());
    }
}
