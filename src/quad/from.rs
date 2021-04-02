// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::utils as u;
use crate::double::Double;
use crate::quad::Quad;
use std::f64;

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
    let (a, b, c, d) = u::renorm4(x as f64 * 2f64.powi(32), y as f64, 0.0, 0.0);
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
    let (a, b, c, d) = u::renorm4(x as f64 * 2f64.powi(32), y as f64, 0.0, 0.0);
    if sign == -1 {
        Quad(-a, -b, -c, -d)
    } else {
        Quad(a, b, c, d)
    }
}

#[allow(clippy::many_single_char_names)]
fn from_u128(a: u128) -> Quad {
    let (w, x, y, z) = split_u128(a);
    let (a, b, c, d) = u::renorm4(
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
    let (a, b, c, d) = u::renorm4(
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

// FROM INTEGER IMPLEMENTATIONS
//
// These are simple enough - since integers are inherently dyadic (as long as they fit into
// `f64`s - see below), they can just be cast to `f64`s and sent directly into the `Quad`
// constructor.
//
// The exceptions are the 64- and 128-bit integers, which don't fit into `f64`s. They get
// their own separate functions that split them into 32-bit parts which are then
// renormalized into a proper `Quad`.

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

from_int_impl! {
    /// Generates a `Quad` from an `i8`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = i8::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-128");
    /// ```
    i8
    /// Generates a `Quad` from a `u8`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = u8::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "255");
    /// ```
    u8
    /// Generates a `Quad` from an `i16`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = i16::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-32768");
    /// ```
    i16
    /// Generates a `Quad` from a `u8`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = u16::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "65535");
    /// ```
    u16
    /// Generates a `Quad` from an `i32`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = i32::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-2147483648");
    /// ```
    i32
    /// Generates a `Quad` from a `u32`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = u32::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "4294967295");
    /// ```
    u32
}

// Separate implementations for the 64-bit and 128-bit integers because they require
// splitting to fit into 53-bit mantissas, so their code is different.

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

from_long_int_impl! {
    /// Generates a `Quad` from an `i64`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = i64::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-9223372036854775808");
    /// ```
    i64 from_i64
    /// Generates a `Quad` from a `u64`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = u64::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "18446744073709551615");
    /// ```
    u64 from_u64
    /// Generates a `Quad` from an `i128`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = i128::MIN;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "-170141183460469231731687303715884105728");
    /// ```
    i128 from_i128
    /// Generates a `Quad` from a `u128`.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let x = u128::MAX;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "340282366920938463463374607431768211455");
    /// ```
    u128 from_u128
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
                } else if u::is_dyadic(a as f64) {
                    Quad(a.into(), 0.0, 0.0, 0.0)
                } else {
                    // Parsing digit-by-digit from a string is the only way to do this
                    // accurately.
                    //
                    // `unwrap` is safe because `a.to_string` will never return a string
                    // that can't be parsed into a Quad.
                    a.to_string().parse().unwrap()
                }
            }
        }
    )*);
}

from_float_impl! {
    /// Generates a `Quad` from an `f32`.
    ///
    /// This function *does* account for floating point rounding error. Even though the
    /// first component of a `Quad` is enough to fit an `f32`, if that `f32` is not exactly
    /// representable in binary, then the second component of the `Quad` will account for
    /// the rounding error.
    ///
    /// Note that in order to do this, the `f32` needs to be parsed digit by digit. While
    /// the parser does work quite fast with integers or any `f32` that is represented
    /// perfectly in binary (any number that can be represented as a fraction with a power
    /// of 2 in the denominator), it's not a particularly fast operation otherwise.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// // Exactly representable in binary
    /// let x = 0.9921875f32;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.9921875");
    ///
    /// // Xot exactly representable in binary
    /// let x = 0.9921876f32;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.9921876");
    /// ```
    f32
    /// Generates a `Quad` from an `f64`.
    ///
    /// This function *does* account for floating point rounding error. Even though the
    /// first component of a `Quad` is enough to fit an `f64`, if that `f64` is not exactly
    /// representable in binary, then the second component of the `Quad` will account for
    /// the rounding error.
    ///
    /// Note that in order to do this, the `f64` needs to be parsed digit by digit. While
    /// the parser does work quite fast with integers or any `f64` that is represented
    /// perfectly in binary (any number that can be represented as a fraction with a power
    /// of 2 in the denominator), it's not a particularly fast operation otherwise.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// // Exactly representable in binary
    /// let x = 0.999969482421875f64;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.999969482421875");
    ///
    /// // Not exactly representable in binary
    /// let x = 0.999969482421876f64;
    /// let a = Quad::from(x);
    /// assert!(a.to_string() == "0.999969482421876");
    /// ```
    f64
}

impl From<(f64, f64)> for Quad {
    /// Generates a `Quad` from a 2-tuple of `f64`s.
    ///
    /// This conversion acts like [`new`] does: it assumes that if you're creating a `Quad`
    /// out of a pair of numbers, you already know what you want those numbers to be.
    /// Therefore it neither renormalizes or accounts for rounding error.
    ///
    /// The third and fourth components of the new `Quad` are set to 0.
    ///
    /// No `From` implementations are provided for 2-tuples of other types. There is no way
    /// to provide a pre-normalized pair of integers, and since tuple conversion doesn't
    /// adjust for rounding error, it's better to make the user explicity cast `f32`s first
    /// in the manner of their choosing.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// // These are the first two components used to define Quad::PI
    /// let a = Quad::from((3.141592653589793e0, 1.2246467991473532e-16));
    /// let diff = (a - Quad::PI).abs();
    /// assert!(diff < qd!(1e-30));
    /// ```
    ///
    /// [`new`]: #method.new
    #[inline]
    fn from((a, b): (f64, f64)) -> Quad {
        Quad(a, b, 0.0, 0.0)
    }
}

impl From<(f64, f64, f64)> for Quad {
    /// Generates a `Quad` from a 3-tuple of `f64`s.
    ///
    /// This conversion acts like [`new`] does: it assumes that if you're creating a `Quad`
    /// out of a pair of numbers, you already know what you want those numbers to be.
    /// Therefore it neither renormalizes or accounts for rounding error.
    ///
    /// The fourth component of the new `Quad` is set to 0.
    ///
    /// No `From` implementations are provided for 3-tuples of other types. There is no way
    /// to provide a pre-normalized pair of integers, and since tuple conversion doesn't
    /// adjust for rounding error, it's better to make the user explicity cast `f32`s first
    /// in the manner of their choosing.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// // These are the first three components used to define Quad::PI
    /// let a = Quad::from((3.141592653589793e0, 1.2246467991473532e-16, -2.9947698097183397e-33));
    /// let diff = (a - Quad::PI).abs();
    /// assert!(diff < qd!(1e-45));
    /// ```
    ///
    /// [`new`]: #method.new
    #[inline]
    fn from((a, b, c): (f64, f64, f64)) -> Quad {
        Quad(a, b, c, 0.0)
    }
}

impl From<(f64, f64, f64, f64)> for Quad {
    /// Generates a `Quad` from a 4-tuple of `f64`s.
    ///
    /// This conversion acts like [`new`] does: it assumes that if you're creating a `Quad`
    /// out of a pair of numbers, you already know what you want those numbers to be.
    /// Therefore it neither renormalizes or accounts for rounding error.
    ///
    /// No `From` implementations are provided for 4-tuples of other types. There is no way
    /// to provide a pre-normalized pair of integers, and since tuple conversion doesn't
    /// adjust for rounding error, it's better to make the user explicity cast `f32`s first
    /// in the manner of their choosing.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// // These are the components used to define Quad::PI
    /// let a = Quad::from((
    ///     3.141592653589793e0,
    ///     1.2246467991473532e-16,
    ///     -2.9947698097183397e-33,
    ///     1.1124542208633655e-49,
    /// ));
    /// assert!(a == Quad::PI);
    /// ```
    ///
    /// [`new`]: #method.new
    #[inline]
    fn from((a, b, c, d): (f64, f64, f64, f64)) -> Quad {
        Quad(a, b, c, d)
    }
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
    /// # use qd::{dd, qd, Quad};
    /// let expected = Quad::from("0.9999999303082806237436760862691");
    /// let a = (dd!(3).powi(15) - dd!(1)) / dd!(3).powi(15);
    /// let x = Quad::from(a);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
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
    /// # use qd::{qd, Quad};
    /// let expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
    /// let x = Quad::from("0.9999999303082806237436760862691492808476631704421807180156648865");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    /// [`from_str`]: #method.from_str
    fn from(s: &str) -> Quad {
        s.parse().unwrap_or(Quad::NAN)
    }
}

impl From<Quad> for f64 {
    /// Converts a `Quad` into an `f64`.
    ///
    /// This will lose precision if the second component of the `Quad` is not 0, but it
    /// will not lose range.
    ///
    /// No other conversions from `Quad` to numeric types are provided, as every other one
    /// has the capability of losing range (for example, no other type could be used to
    /// represent `qd!(1e308)`). Casts can be made from the `f64` provided by this function
    /// to other numeric types as needed.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let a = Quad::PI;
    /// let x = f64::from(a);
    ///
    /// let diff = (x - std::f64::consts::PI).abs();
    /// assert!(diff < 1e-15);
    /// ```
    #[inline]
    fn from(a: Quad) -> f64 {
        a.0
    }
}

impl From<Quad> for (f64, f64) {
    /// Converts a `Quad` into a 2-tuple of `f64`s.
    ///
    /// The first two components of the `Quad` become the components of the returned tuple.
    /// Note that, while the value of the first component is simply the `f64` cast of the
    /// `Quad` itself, the second component encodes the next digits of the `Quad` *plus*
    /// the rounding error in the first component. For that reason, it's not likely to be
    /// very useful outside of a `Quad` context.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let (a, b) = <(f64, f64)>::from(Quad::PI);
    /// assert!(a == 3.141592653589793e0);
    /// assert!(b == 1.2246467991473532e-16); // *not* the next 16 digits of π
    /// ```
    #[inline]
    fn from(a: Quad) -> (f64, f64) {
        (a.0, a.1)
    }
}

impl From<Quad> for (f64, f64, f64) {
    /// Converts a `Quad` into a 3-tuple of `f64`s.
    ///
    /// The first three components of the `Quad` become the components of the returned
    /// tuple. Note that, while the value of the first component is simply the `f64` cast of
    /// the `Quad` itself, the other components encode the next digits of the `Quad` *plus*
    /// the rounding error in the prior components. For that reason, they're not likely to
    /// be very useful outside of a `Quad` context.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let (a, b, c) = <(f64, f64, f64)>::from(Quad::PI);
    /// assert!(a == 3.141592653589793e0);
    /// assert!(b == 1.2246467991473532e-16);
    /// assert!(c == -2.9947698097183397e-33);
    /// ```
    #[inline]
    fn from(a: Quad) -> (f64, f64, f64) {
        (a.0, a.1, a.2)
    }
}

impl From<Quad> for (f64, f64, f64, f64) {
    /// Converts a `Quad` into a 4-tuple of `f64`s.
    ///
    /// The components of the `Quad` become the components of the returned tuple. Note that,
    /// while the value of the first component is simply the `f64` cast of the `Quad`
    /// itself, the other components encode the next digits of the `Quad` *plus* the
    /// rounding error in the prior components. For that reason, they're not likely to be
    /// very useful outside of a `Quad` context.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let (a, b, c, d) = <(f64, f64, f64, f64)>::from(Quad::PI);
    /// assert!(a == 3.141592653589793e0);
    /// assert!(b == 1.2246467991473532e-16);
    /// assert!(c == -2.9947698097183397e-33);
    /// assert!(d == 1.1124542208633655e-49);
    /// ```
    #[inline]
    fn from(a: Quad) -> (f64, f64, f64, f64) {
        (a.0, a.1, a.2, a.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // f32 tests
    test_all_exact!(
        f32_int:
            Quad(1.0, 0.0, 0.0, 0.0),
            qd!(1.0f32);
        f32_float:
            Quad(1.203125, 0.0, 0.0, 0.0),
            qd!(1.203125f32);
        f32_zero:
            Quad::ZERO,
            qd!(0f32);
        f32_neg_zero:
            Quad::NEG_ZERO,
            qd!(-0.0f32);
        f32_inf:
            Quad::INFINITY,
            std::f32::INFINITY;
        f32_neg_inf:
            Quad::NEG_INFINITY,
            std::f32::NEG_INFINITY;
        f32_nan:
            Quad::NAN,
            std::f32::NAN;
    );
    test!(f32_nonrep: {
        assert_ne!(qd!(1.1f32).1, 0.0);
    });

    // f64 tests
    test_all_exact!(
        f64_int:
            Quad(1.0, 0.0, 0.0, 0.0),
            qd!(1.0);
        f64_float:
            Quad(1.203125, 0.0, 0.0, 0.0),
            qd!(1.203125);
        f64_double:
            Quad(1.0005645751953125, 0.0, 0.0, 0.0),
            qd!(1.0005645751953125);
        f64_zero:
            Quad::ZERO,
            qd!(0.0);
        f64_neg_zero:
            Quad::NEG_ZERO,
            qd!(-0.0);
        f64_inf:
            Quad::INFINITY,
            std::f64::INFINITY;
        f64_neg_inf:
            Quad::NEG_INFINITY,
            std::f64::NEG_INFINITY;
        f64_nan:
            Quad::NAN,
            std::f64::NAN;
    );
    test!(f64_nonrep: {
        assert_ne!(qd!(1.1).1, 0.0);
    });

    // integer tests
    test_all_eq!(
        i8_min: i8::MIN.to_string(), qd!(i8::MIN).to_string();
        u8_max: u8::MAX.to_string(), qd!(u8::MAX).to_string();
        i16_min: i16::MIN.to_string(), qd!(i16::MIN).to_string();
        u16_max: u16::MAX.to_string(), qd!(u16::MAX).to_string();
        i32_min: i32::MIN.to_string(), qd!(i32::MIN).to_string();
        u32_max: u32::MAX.to_string(), qd!(u32::MAX).to_string();
        i64_min: i64::MIN.to_string(), qd!(i64::MIN).to_string();
        u64_max: u64::MAX.to_string(), qd!(u64::MAX).to_string();
        i128_min: i128::MIN.to_string(), qd!(i128::MIN).to_string();
        u128_max: u128::MAX.to_string(), qd!(u128::MAX).to_string();
    );
}
