// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! This module implements all of the low-level "primitives" used in the algorithms of both
//! double-doubles and quad-doubles. None of the functions presented here are aware of the
//! higher-precision numbers; they all work with `f64`s and return tuples of `f64`s, which
//! are then used in higher-precision operations.
//!
//! This module is aware of the `no_fma` feature. If it's enabled, further constants and
//! functions are used to split floats into high and low words, necessary for the
//! multiplication algorithm. If the feature is not enabled, multiplication instead depends
//! on `f64`'s `mul_add` function, which uses FMA under the hood if it's available.

#![allow(clippy::many_single_char_names)]

/// The factor used to multiply a number to split it into high and low components.
///
/// It essentially acts as a mask to isolate one half of the mantissa from the other, and is
/// therefore chosen to be a point halfway into the mantissa's bit field.
///
/// This value is 2<sup>27</sup> + 1.
#[cfg(no_fma)]
const SPLIT_FACTOR: f64 = 134217729.0; // = 2^27 + 1

/// The threshold over which special handling is done when splitting an `f64`.
///
/// If the absolute value of the value being split exceeds this, the number is reduced
/// before processing and then increased by the same factor after processing. This increases
/// accuracy in very large (and small) numbers where accuracy is naturally lessened in
/// floating-point numbers.
///
/// This value is 2<sup>996</sup>, which is the highest power of two that is less than
/// 10<sup>300</sup>.
#[cfg(no_fma)]
const SPLIT_THRESHOLD: f64 = 6.69692879491417e+299; // = 2^996

/// The factor by which a very large number is multiplied before being split.
///
/// This value is 2<sup>-28</sup>.
#[cfg(no_fma)]
const SPLIT_SHIFT_DOWN: f64 = 3.7252902984619140625e-9; // = 2^-28

/// The factor by which a very large number is multiplied after being split.
///
/// This value is 2<sup>28</sup>, or the inverse of the value used before splitting.
#[cfg(no_fma)]
const SPLIT_SHIFT_UP: f64 = 268435456.0; // = 2^28

/// Calculates fl(a + b) and err(a + b).
///
/// This calculation performs 3 floating-point operations. This is more efficient than
/// [`two_sum`](#fn.two_sum) but carries the restriction that |a| >= |b|, which
/// [`two_sum`](#fn.two_sum) does not.
#[inline]
pub fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let e = b - (s - a);
    (s, e)
}

// #[inline]
// pub fn quick_two_diff(a: f64, b: f64) -> (f64, f64) {
//     let s = a - b;
//     let e = (a - s) - b;
//     (s, e)
// }

/// Calculates fl(a + b) and err(a + b).
///
/// This calculation performs 6 floating-point operations. It is less efficient than
/// [`quick_two_sum`](#fn.quick_two_sum) but it carries no restrictions on its input values.
#[inline]
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let v = s - a;
    let e = (a - (s - v)) + (b - v);
    (s, e)
}

/// Calculates fl(a - b) and err(a - b).
///
/// This calculation performs 6 floating-point operations.
pub fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let v = s - a;
    let e = (a - (s - v)) - (b + v);
    (s, e)
}

/// Splits a number into equal-length high and low components.
///
/// This is a helper function for use in multiplication functions when FMA is not enabled
/// (i.e., when the `no_fma` feature is enabled). It performs 4 floating-point operations,
/// unless the number's absolute value is greater than
/// [`SPLIT_THRESHOLD`](constant.SPLIT_THRESHOLD.html). In this case it performs 7
/// floating-point operations to increase precision in the large number.
#[cfg(no_fma)]
#[inline]
fn split(a: f64) -> (f64, f64) {
    if a > SPLIT_THRESHOLD || a < -SPLIT_THRESHOLD {
        let s = a * SPLIT_SHIFT_DOWN;
        let t = SPLIT_FACTOR * s;
        let hi = t - (t - s);
        let lo = s - hi;
        (hi * SPLIT_SHIFT_UP, lo * SPLIT_SHIFT_UP)
    } else {
        let t = SPLIT_FACTOR * a;
        let hi = t - (t - a);
        let lo = a - hi;
        (hi, lo)
    }
}

/// Calculates fl(a * b) and err(a * b).
///
/// This implementation uses FMA and requires 2 floating-point operations because of it. If
/// FMA is not available but the `no_fma` feature is not enabled, it will use considerably
/// more operations.
#[cfg(not(no_fma))]
#[inline]
pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let e = a.mul_add(b, -p);
    (p, e)
}

/// Calculates fl(a * b) and err(a * b).
///
/// This implementation does not use FMA and consequently requires at least 17
/// floating-point operations. If the arguments are particularly large, it can require as
/// many as 23 floating-point operations.
#[cfg(no_fma)]
#[inline]
pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let (ahi, alo) = split(a);
    let (bhi, blo) = split(b);
    let e = ahi * bhi - p + ahi * blo + alo * bhi + alo * blo;
    (p, e)
}

/// Calculates fl(a * a) and err(a * a).
///
/// This implementation uses FMA and therefore requires 2 floating-point instructions, the
/// same as for multiplication. If FMA is not available but the `no_fma` feature is not
/// enabled, it will use considerably more operations.
#[cfg(not(no_fma))]
#[inline]
pub fn two_sqr(a: f64) -> (f64, f64) {
    let p = a * a;
    let e = a.mul_add(a, -p);
    (p, e)
}

/// Calculates fl(a * a) and err(a * a).
///
/// This implementation does not use FMA but takes advantage of optimizations that can be
/// made in multiplication due to the multiplied numbers being the same. It therefore only
/// uses a minimum of 12 floating-point operations, though with a very large argument it can
/// be 15.
#[cfg(no_fma)]
#[inline]
pub fn two_sqr(a: f64) -> (f64, f64) {
    let p = a * a;
    let (hi, lo) = split(a);
    let e = hi * hi - p + 2.0 * hi * lo + lo * lo;
    (p, e)
}
