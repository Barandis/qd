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

// FROM INTEGER IMPLEMENTATIONS
//
// These are simple enough - since integers are inherently dyadic (as long as they fit into
// `f64`s - see below), they can just be cast to `f64`s and sent directly into the `Double`
// constructor.
//
// The exceptions are `i64` and `u64`, which don't fit into `f64`s. They get their own
// separate (non-macro) functions that split them into two 32-bit parts which are then
// renormalized into a proper `Double`.

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

// Separate implementations for the 64-bit integers because they require splitting to fit
// into 53-bit mantissas, so their code is different.

impl From<u64> for Double {
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
    fn from(a: u64) -> Double {
        from_u64(a)
    }
}

impl From<i64> for Double {
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
    fn from(a: i64) -> Double {
        from_i64(a)
    }
}

// FROM FLOAT IMPLEMENTATIONS
//
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
                    // Parsing digit-by-digit from a string is the only way to do this
                    // accurately.
                    //
                    // `unwrap` is safe because `a.to_string` will never return a string
                    // that can't be parsed into a Double.
                    a.to_string().parse().unwrap()
                }
            }
        }
    )*);
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

impl From<(f64, f64)> for Double {
    /// Generates a `Double` from a 2-tuple of `f64`s.
    ///
    /// This conversion acts like [`new`] does: it assumes that if you're creating a
    /// `Double` out of a pair of numbers, you already know what you want those numbers to
    /// be. Therefore it neither renormalizes or accounts for rounding error.
    ///
    /// No other `From` implementations are provided for tuples. There is no way to provide
    /// a pre-normalized pair of integers, and since tuple conversion doesn't adjust for
    /// rounding error, it's better to make the user explicity cast `f32`s first in the
    /// manner of their choosing.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// // These are the components used to define Double::PI
    /// let d = Double::from((3.141592653589793e0, 1.2246467991473532e-16));
    /// assert!(d == Double::PI);
    /// ```
    ///
    /// [`new`]: #method.new
    #[inline]
    fn from((a, b): (f64, f64)) -> Double {
        Double(a, b)
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

impl From<Double> for (f64, f64) {
    /// Converts a `Double` into a tuple of `f64`s.
    ///
    /// The components of the double become the components of the returned tuple. Note that,
    /// while the value of the first component is simply the `f64` cast of the `Double`
    /// itself, the second component encodes the next digits of the `Double` *plus* the
    /// rounding error in the first component. For that reason, it's not likely to be very
    /// useful outside of a `Double` context.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let (a, b) = <(f64, f64)>::from(Double::PI);
    /// assert!(a == 3.141592653589793e0);
    /// assert!(b == 1.2246467991473532e-16); // *not* the next 16 digits of π
    /// ```
    #[inline]
    fn from(a: Double) -> (f64, f64) {
        (a.0, a.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // f32 tests
    test_all_exact!(
        f32_int:
            Double(1.0, 0.0),
            dd!(1.0f32);
        f32_float:
            Double(1.203125, 0.0),
            dd!(1.203125f32);
        f32_zero:
            Double::ZERO,
            dd!(0f32);
        f32_neg_zero:
            Double::NEG_ZERO,
            dd!(-0.0f32);
        f32_inf:
            Double::INFINITY,
            std::f32::INFINITY;
        f32_neg_inf:
            Double::NEG_INFINITY,
            std::f32::NEG_INFINITY;
        f32_nan:
            Double::NAN,
            std::f32::NAN;
    );
    test!(f32_nonrep: {
        assert_ne!(dd!(1.1f32).1, 0.0);
    });

    // f64 tests
    test_all_exact!(
        f64_int:
            Double(1.0, 0.0),
            dd!(1.0);
        f64_float:
            Double(1.203125, 0.0),
            dd!(1.203125);
        f64_double:
            Double(1.0005645751953125, 0.0),
            dd!(1.0005645751953125);
        f64_zero:
            Double::ZERO,
            dd!(0.0);
        f64_neg_zero:
            Double::NEG_ZERO,
            dd!(-0.0);
        f64_inf:
            Double::INFINITY,
            std::f64::INFINITY;
        f64_neg_inf:
            Double::NEG_INFINITY,
            std::f64::NEG_INFINITY;
        f64_nan:
            Double::NAN,
            std::f64::NAN;
    );
    test!(f64_nonrep: {
        assert_ne!(dd!(1.1).1, 0.0);
    });

    // integer tests
    test_all_eq!(
        i8_min: i8::MIN.to_string(), dd!(i8::MIN).to_string();
        u8_max: u8::MAX.to_string(), dd!(u8::MAX).to_string();
        i16_min: i16::MIN.to_string(), dd!(i16::MIN).to_string();
        u16_max: u16::MAX.to_string(), dd!(u16::MAX).to_string();
        i32_min: i32::MIN.to_string(), dd!(i32::MIN).to_string();
        u32_max: u32::MAX.to_string(), dd!(u32::MAX).to_string();
        i64_min: i64::MIN.to_string(), dd!(i64::MIN).to_string();
        u64_max: u64::MAX.to_string(), dd!(u64::MAX).to_string();
    );
}
