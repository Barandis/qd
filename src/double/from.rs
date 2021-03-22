// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::utils as u;
use crate::double::Double;
use std::f64;

#[inline]
fn split_u64(a: u64) -> (u32, u32) {
    let x = (a >> 32) as u32;
    let y = a as u32;
    (x, y)
}

fn from_u64(a: u64) -> Double {
    let (x, y) = split_u64(a);
    let (a, b) = u::renorm2(x as f64 * 2f64.powi(32), y as f64);
    Double(a, b)
}

fn from_i64(a: i64) -> Double {
    let sign = a.signum();
    // The first part prevents a.abs() from failing with overflow because the absolute
    // value of i64::MIN is i64::MAX + 1
    let a = if a == i64::MIN {
        i64::MAX as u64 + 1
    } else {
        a.abs() as u64
    };
    let (x, y) = split_u64(a);
    let (a, b) = u::renorm2(x as f64 * 2f64.powi(32), y as f64);
    if sign == -1 {
        Double(-a, -b)
    } else {
        Double(a, b)
    }
}

macro_rules! from_int_impl {
    ($(
        $(#[$m:meta])*
        $t:ty
    )*) => ($(
        $(#[$m])*
        impl From<$t> for Double {
            #[inline]
            fn from(a: $t) -> Double {
                Double(a.into(), 0.0)
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
        $t:ty
    )*) => ($(
        $(#[$m])*
        impl From<$t> for Double {
            fn from(a: $t) -> Double {
                if a == 0.0 {
                    if a.is_sign_negative() {
                        Double::NEG_ZERO
                    } else {
                        Double::ZERO
                    }
                } else if a.is_nan() {
                    Double::NAN
                } else if a.is_infinite() {
                    if a.is_sign_negative() {
                        Double::NEG_INFINITY
                    } else {
                        Double::INFINITY
                    }
                } else if u::is_dyadic(a as f64) {
                    Double(a.into(), 0.0)
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
                    // that can't be parsed into a Double.
                    a.to_string().parse().unwrap()
                }
            }
        }
    )*);
}

from_int_impl! {
    /// Generates a `Double` from an `i8`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = -128i8;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "-128");
    /// # }
    /// ```
    i8
    /// Generates a `Double` from a `u8`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = 255u8;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "255");
    /// # }
    /// ```
    u8
    /// Generates a `Double` from an `i16`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = -32768i16;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "-32768");
    /// # }
    /// ```
    i16
    /// Generates a `Double` from a `u8`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = 65535u16;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "65535");
    /// # }
    /// ```
    u16
    /// Generates a `Double` from an `i32`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = -2_147_483_648i32;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "-2147483648");
    /// # }
    /// ```
    i32
    /// Generates a `Double` from a `u32`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = 4_294_967_295u32;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "4294967295");
    /// # }
    /// ```
    u32
}

from_float_impl! {
    /// Generates a `Double` from an `f32`.
    ///
    /// This function *does* account for floating point rounding error. Even though the
    /// first component of a `Double` is enough to fit an `f32`, if that `f32` is not
    /// exactly representable in binary, then the second component of the `Double` will
    /// account for the rounding error.
    ///
    /// Note that in order to do this, the `f32` needs to be parsed digit by digit. While
    /// the parser does work quite fast with integers or any `f32` that is represented
    /// perfectly in binary (any number that can be represented as a fraction with a power
    /// of 2 in the denominator), it's not a particularly fast operation otherwise.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x = 0.9921875f32;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "0.9921875");
    ///
    /// // Xot exactly representable in binary
    /// let x = 0.9921876f32;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "0.9921876");
    /// # }
    /// ```
    f32
    /// Generates a `Double` from an `f64`.
    ///
    /// This function *does* account for floating point rounding error. Even though the
    /// first component of a `Double` is enough to fit an `f64`, if that `f64` is not
    /// exactly representable in binary, then the second component of the `Double` will
    /// account for the rounding error.
    ///
    /// Note that in order to do this, the `f64` needs to be parsed digit by digit. While
    /// the parser does work quite fast with integers or any `f64` that is represented
    /// perfectly in binary (any number that can be represented as a fraction with a power
    /// of 2 in the denominator), it's not a particularly fast operation otherwise.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x = 0.999969482421875f64;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "0.999969482421875");
    ///
    /// // Not exactly representable in binary
    /// let x = 0.999969482421876f64;
    /// let a = Double::from(x);
    /// assert!(a.to_string() == "0.999969482421876");
    /// # }
    /// ```
    f64
}

/// Generates a `Double` from a `u64`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qd;
/// # use qd::Double;
/// # fn main() {
/// let x = 18_446_744_073_709_551_615u64;
/// let a = Double::from(x);
/// assert!(a.to_string() == "18446744073709551615");
/// # }
/// ```
impl From<u64> for Double {
    fn from(a: u64) -> Double {
        from_u64(a)
    }
}

/// Generates a `Double` from an `i64`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qd;
/// # use qd::Double;
/// # fn main() {
/// let x = -9_223_372_036_854_775_808i64;
/// let a = Double::from(x);
/// assert!(a.to_string() == "-9223372036854775808");
/// # }
/// ```
impl From<i64> for Double {
    fn from(a: i64) -> Double {
        from_i64(a)
    }
}

impl From<&str> for Double {
    /// Parses a string to create a `Double`.
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
    /// # use qd::Double;
    /// # fn main() {
    /// let expected = (dd!(3).powi(15) - dd!(1)) / dd!(3).powi(15);
    /// let x = Double::from("0.9999999303082806237436760862691");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    /// 
    /// [`NAN`]: #associatedconstant.NAN
    /// [`from_str`]: #method.from_str
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
    /// has the capability of losing range (for example, no other type could be used to
    /// represent `dd!(1e308)`). Casts can be made from the `f64` provided by this function
    /// to other numeric types as needed.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let a = Double::PI;
    /// let x = f64::from(a);
    /// 
    /// let diff = (x - std::f64::consts::PI).abs();
    /// assert!(diff < 1e-15);
    /// # }
    #[inline]
    fn from(a: Double) -> f64 {
        a.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_f32() {
        assert_exact!(dd!(1.0f32), Double(1.0, 0.0));
        assert_exact!(dd!(1.203125f32), Double(1.203125, 0.0));
        assert_ne!(dd!(1.1f32).1, 0.0);
        assert_exact!(dd!(0f32), Double::ZERO);
        assert_exact!(dd!(-0.0f32), Double::NEG_ZERO);
        assert_exact!(dd!(std::f32::INFINITY), Double::INFINITY);
        assert_exact!(dd!(std::f32::NEG_INFINITY), Double::NEG_INFINITY);
        assert_exact!(dd!(std::f32::NAN), Double::NAN);
    }

    #[test]
    fn from_f64() {
        assert_exact!(dd!(1.0), Double(1.0, 0.0));
        assert_exact!(dd!(1.203125), Double(1.203125, 0.0));
        assert_exact!(dd!(1.0005645751953125), Double(1.0005645751953125, 0.0));
        assert_ne!(dd!(1.1).1, 0.0);
        assert_exact!(dd!(0), Double::ZERO);
        assert_exact!(dd!(-0.0), Double::NEG_ZERO);
        assert_exact!(dd!(std::f64::INFINITY), Double::INFINITY);
        assert_exact!(dd!(std::f64::NEG_INFINITY), Double::NEG_INFINITY);
        assert_exact!(dd!(std::f64::NAN), Double::NAN);
    }

    #[test]
    fn from_int() {
        assert_eq!(i8::MIN.to_string(), dd!(i8::MIN).to_string());
        assert_eq!(u8::MAX.to_string(), dd!(u8::MAX).to_string());
        assert_eq!(i16::MIN.to_string(), dd!(i16::MIN).to_string());
        assert_eq!(u16::MAX.to_string(), dd!(u16::MAX).to_string());
        assert_eq!(i32::MIN.to_string(), dd!(i32::MIN).to_string());
        assert_eq!(u32::MAX.to_string(), dd!(u32::MAX).to_string());
        assert_eq!(i64::MIN.to_string(), dd!(i64::MIN).to_string());
        assert_eq!(u64::MAX.to_string(), dd!(u64::MAX).to_string());
    }
}
