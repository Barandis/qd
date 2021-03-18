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
    // The first part prevents a.abs() from failing with overflow because the absolute
    // value of i64::MIN is i64::MAX + 1
    let a = if a == i64::MIN {
        i64::MAX as u64 + 1
    } else {
        a.abs() as u64
    };
    let (x, y) = split_u64(a);
    let d = Double::from(basic::renorm2(x as f64 * 2f64.powi(32), y as f64));
    if sign == -1 {
        -d
    } else {
        d
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
                } else if is_dyadic(a as f64) {
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

macro_rules! from_tuple_impl {
    ($(
        $(#[$m:meta])*
        $t:ty
    )*) => ($(
        $(#[$m])*
        impl From<($t, $t)> for Double {
            #[inline]
            fn from((a, b): ($t, $t)) -> Double {
                let (a, b) = basic::two_sum(a.into(), b.into());
                Double(a, b)
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
    /// let x: i8 = -128;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "-128");
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
    /// let x: u8 = 255;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "255");
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
    /// let x: i16 = -32768;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "-32768");
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
    /// let x: u16 = 65535;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "65535");
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
    /// let x: i32 = -2_147_483_648;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "-2147483648");
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
    /// let x: u32 = 4_294_967_295;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "4294967295");
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
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x: f32 = 0.9921875;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "0.9921875");
    ///
    /// // Xot exactly representable in binary
    /// let x: f32 = 0.9921876;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "0.9921876");
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
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x = 0.999969482421875;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "0.999969482421875");
    ///
    /// // Not exactly representable in binary
    /// let x = 0.999969482421876;
    /// let d = Double::from(x);
    /// assert!(d.to_string() == "0.999969482421876");
    /// # }
    /// ```
    f64
}
from_tuple_impl! {
    /// Generates a `Double` from a 2-tuple of `i8`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be an `i8` itself.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x: i8 = -128;
    /// let d = Double::from((x, x));
    /// assert!(d.to_string() == "-256");
    /// # }
    /// ```
    i8
    /// Generates a `Double` from a 2-tuple of `u8`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be a `u8` itself.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x: u8 = 255;
    /// let d = Double::from((x, x));
    /// assert!(d.to_string() == "510");
    /// # }
    /// ```
    u8
    /// Generates a `Double` from a 2-tuple of `i16`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be an `i16` itself.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x: i16 = -32768;
    /// let d = Double::from((x, x));
    /// assert!(d.to_string() == "-65536");
    /// # }
    /// ```
    i16
    /// Generates a `Double` from a 2-tuple of `u16`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be a `u16` itself.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x: u16 = 32767;
    /// let d = Double::from((x, x));
    /// assert!(d.to_string() == "65534");
    /// # }
    /// ```
    u16
    /// Generates a `Double` from a 2-tuple of `i32`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be an `i32` itself.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x: i32 = -2_147_483_648;
    /// let d = Double::from((x, x));
    /// assert!(d.to_string() == "-4294967296");
    /// # }
    /// ```
    i32
    /// Generates a `Double` from a 2-tuple of `u32`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be a `u32` itself.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x: u32 = 4_294_967_295;
    /// let d = Double::from((x, x));
    /// assert!(d.to_string() == "8589934590");
    /// # }
    /// ```
    u32
    /// Generates a `Double` from a 2-tuple of `f32`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be an `f32` itself.
    /// 
    /// Generating a `Double` from a tuple is *not* intended to be accurate as a sum of the
    /// tuple components. While normalization happens, floating-point rounding error is
    /// *not* accounted for, so as a sum, the resulting `Double` will only be as accurate
    /// as a plain `f32`. Using a tuple to create a `Double` is intended for use when the
    /// tuple components are known to be exactly what should be used for the components
    /// internal to the `Double`.
    /// 
    /// To create a `Double` that *does* account for floating-point error, create `Double`s
    /// separately from each tuple component and then add them.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x: f32 = 0.9921875;
    /// let d = Double::from((x, x));
    /// // Accurate
    /// assert!(d.to_string() == "1.984375");
    ///
    /// // Not exactly representable in binary
    /// let x: f32 = 0.9921876;
    /// let d = Double::from((x, x));
    /// // Only accurate as a sum to f32 precision
    /// assert!(d.to_string() == "1.9843752384185791015625");
    /// 
    /// // Created as a sum of the tuple components
    /// let d = Double::from(x) + Double::from(x);
    /// // Accurate
    /// assert!(d.to_string() == "1.9843752");
    /// # }
    /// ```
    f32

    /// Generates a `Double` from a 2-tuple of `f32`s. The two components of the tuple are
    /// added together and the result is normalized. Note that the resulting sum does *not*
    /// have to be small enough to be an `f32` itself.
    /// 
    /// Generating a `Double` from a tuple is *not* intended to be accurate as a sum of the
    /// tuple components. While normalization happens, floating-point rounding error is
    /// *not* accounted for, so as a sum, the resulting `Double` will only be as accurate
    /// as a plain `f64`. Using a tuple to create a `Double` is intended for use when the
    /// tuple components are known to be exactly what should be used for the components
    /// internal to the `Double`.
    /// 
    /// To create a `Double` that *does* account for floating-point error, create `Double`s
    /// separately from each tuple component and then add them.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// // Exactly representable in binary
    /// let x = 0.999969482421875;
    /// let d = Double::from((x, x));
    /// // Accurate
    /// assert!(d.to_string() == "1.99993896484375");
    ///
    /// // Not exactly representable in binary
    /// let x = 0.999969482421876;
    /// let d = Double::from((x, x));
    /// // Only accurate as a sum to f64 precision
    /// assert!(d.to_string() == "1.999938964843751998401444325282");
    /// 
    /// // Created as a sum of the tuple components
    /// let d = Double::from(x) + Double::from(x);
    /// // Accurate
    /// assert!(d.to_string() == "1.999938964843752");
    /// # }
    /// ```
    f64
}

/// Generates a `Double` from a 2-tuple of `u64`s. The two components of the tuple are added
/// together and the result is normalized. Note that the resulting sum does *not* have to be
/// small enough to be a `u64` itself.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qd;
/// # use qd::Double;
/// # fn main() {
/// let x: u64 = 18_446_744_073_709_551_615;
/// let d = Double::from((x, x));
/// assert!(d.to_string() == "36893488147419103230");
/// # }
/// ```
impl From<(u64, u64)> for Double {
    fn from((a, b): (u64, u64)) -> Double {
        from_u64(a) + from_u64(b)
    }
}

/// Generates a `Double` from a `u64`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qd;
/// # use qd::Double;
/// # fn main() {
/// let x: u64 = 18_446_744_073_709_551_615;
/// let d = Double::from(x);
/// assert!(d.to_string() == "18446744073709551615");
/// # }
/// ```
impl From<u64> for Double {
    fn from(a: u64) -> Double {
        from_u64(a)
    }
}

/// Generates a `Double` from a 2-tuple of `i64`s. The two components of the tuple are
/// added together and the result is normalized.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qd;
/// # use qd::Double;
/// # fn main() {
/// let x: i64 = -9_223_372_036_854_775_808;
/// let d = Double::from((x, x));
/// assert!(d.to_string() == "-18446744073709551616");
/// # }
/// ```
impl From<(i64, i64)> for Double {
    fn from((a, b): (i64, i64)) -> Double {
        from_i64(a) + from_i64(b)
    }
}

/// Generates a `Double` from a `u64`.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate qd;
/// # use qd::Double;
/// # fn main() {
/// let x: i64 = -9_223_372_036_854_775_808;
/// let d = Double::from(x);
/// assert!(d.to_string() == "-9223372036854775808");
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
    /// Failure will return [`Double::NAN`]. This can be an issue because parsing the string
    /// `"nan"` *also* produces [`Double::NAN`]. For this reason it's suggested to use
    /// [`from_str`] (or its associated `parse` function) instead of this function if there
    /// is any chance that the parsed string will be legitimately [`Double::NAN`].
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
    /// [`Double::NAN`]: #associatedconstant.NAN
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
    /// let d = Double::PI;
    /// let x = f64::from(d);
    /// 
    /// let diff = (x - std::f64::consts::PI).abs();
    /// assert!(diff < 1e-15);
    /// # }
    #[inline]
    fn from(a: Double) -> f64 {
        a.0
    }
}

impl From<Double> for (f64, f64) {
    /// Converts a `Double` into a 2-tuple of `f64`s.
    ///
    /// The components of the resulting tuple are simply the components of the `Double`.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let d = Double::PI;
    /// let x = <(f64, f64)>::from(d);
    /// 
    /// assert!(x.0 == 3.141592653589793);
    /// assert!(x.1 == 1.2246467991473532e-16);
    /// # }
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
