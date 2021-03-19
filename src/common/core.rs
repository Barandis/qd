// Copyright (c) 2019 Thomas Otterson
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

// #region Basic f64 arithmetic

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

// #endregion

// #region Specific f64 arithmetic for other algorithms

// Each of the following functions is used as a component of a larger algorithm.

/// Calculates the sum of three `f64`s in double-double precision.
#[inline]
pub fn three_two_sum(a: f64, b: f64, c: f64) -> (f64, f64) {
    let (u, v) = two_sum(a, b);
    let (s, w) = two_sum(c, u);
    (s, v + w)
}

/// Calculates the sum of three `f64`s in triple-double precision.
#[inline]
pub fn three_three_sum(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let (u, v) = two_sum(a, b);
    let (s, w) = two_sum(c, u);
    let (e1, e2) = two_sum(v, w);
    (s, e1, e2)
}

/// Calculates the sum of four `f64`s in double-double precision.
#[inline]
pub fn four_two_sum(a: f64, b: f64, c: f64, d: f64) -> (f64, f64) {
    let (s0, s1) = two_sum(a, c);
    (s0, s1 + b + d)
}

/// Calculates the sum of six `f64`s in triple-double precision.
#[inline]
pub fn six_three_sum(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> (f64, f64, f64) {
    let (p0, p1, p2) = three_three_sum(a, b, c);
    let (q0, q1, q2) = three_three_sum(d, e, f);
    let (r0, r1) = two_sum(p0, q0);
    let (s0, s1) = two_sum(p1, q1);
    let (t0, t1) = two_sum(s0, r1);
    let u0 = p2 + q2 + s1 + t1;
    (r0, t0, u0)
}

/// Calculates the sum of nine `f64`s in double-double precision.
#[allow(clippy::too_many_arguments)]
#[inline]
pub fn nine_two_sum(
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    i: f64,
) -> (f64, f64) {
    let (p0, p1) = two_sum(a, b);
    let (q0, q1) = two_sum(c, d);
    let (r0, r1) = two_sum(e, f);
    let (s0, s1) = two_sum(g, h);
    let (t0, t1) = four_two_sum(p0, p1, q0, q1);
    let (u0, u1) = four_two_sum(r0, r1, s0, s1);
    let (v0, v1) = four_two_sum(t0, t1, u0, u1);
    let (w0, w1) = two_sum(v0, i);
    (w0, w1 + v1)
}

/// Adds a float to an value/error pair.
///
/// If the result of this addition doesn't fit in two `f64`s, the sum is output as the first
/// tuple component and the second and third contain the remainder. Otherwise, the first
/// tuple component is `0.0` and the sum is in the other two components.
#[inline]
pub fn accumulate(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let (s, b) = two_sum(b, c);
    let (s, a) = two_sum(a, s);

    let za = a == 0.0;
    let zb = b == 0.0;

    if !(za || zb) {
        (s, a, b)
    } else {
        (0.0, s, if zb { a } else { b })
    }
}

// #endregion

// #region Renormalization functions

/// Renormalizes two components into a two-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of the last component is no more than half the ULP of the
/// first.
#[inline]
pub fn renorm2(a: f64, b: f64) -> (f64, f64) {
    quick_two_sum(a, b)
}

/// Renormalizes three components into a two-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of the last component is no more than half the ULP of the
/// first.
#[inline]
pub fn renorm3(a: f64, b: f64, c: f64) -> (f64, f64) {
    let (u, v) = quick_two_sum(a, b);
    let (s, w) = quick_two_sum(c, u);
    quick_two_sum(s, v + w)
}

/// Renormalizes four components into a four-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of each component is no more than half of the ULP of the
/// prior component.
#[inline]
pub fn renorm4(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64) {
    let (x, s3) = quick_two_sum(c, d);
    let (x, s2) = quick_two_sum(b, x);
    let (s0, s1) = quick_two_sum(a, x);

    if s1 != 0.0 {
        let (s1, s2) = quick_two_sum(s1, s2);
        if s2 != 0.0 {
            let (s2, s3) = quick_two_sum(s2, s3);
            (s0, s1, s2, s3)
        } else {
            let (s1, s2) = quick_two_sum(s1, s3);
            (s0, s1, s2, 0.0)
        }
    } else {
        let (s0, s1) = quick_two_sum(s0, s2);
        if s1 != 0.0 {
            let (s1, s2) = quick_two_sum(s1, s3);
            (s0, s1, s2, 0.0)
        } else {
            let (s0, s1) = quick_two_sum(s0, s3);
            (s0, s1, 0.0, 0.0)
        }
    }
}

/// Renormalizes five components into a four-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of each component is no more than half of the ULP of the
/// prior component.
#[inline]
pub fn renorm5(a: f64, b: f64, c: f64, d: f64, e: f64) -> (f64, f64, f64, f64) {
    let (x, s4) = quick_two_sum(d, e);
    let (x, s3) = quick_two_sum(c, x);
    let (x, s2) = quick_two_sum(b, x);
    let (s0, s1) = quick_two_sum(a, x);

    if s1 != 0.0 {
        let (s1, s2) = quick_two_sum(s1, s2);
        if s2 != 0.0 {
            let (s2, s3) = quick_two_sum(s2, s3);
            if s3 != 0.0 {
                (s0, s1, s2, s3 + s4)
            } else {
                let (s2, s3) = quick_two_sum(s2, s4);
                (s0, s1, s2, s3)
            }
        } else {
            let (s1, s2) = quick_two_sum(s1, s3);
            if s2 != 0.0 {
                let (s2, s3) = quick_two_sum(s2, s4);
                (s0, s1, s2, s3)
            } else {
                let (s1, s2) = quick_two_sum(s1, s4);
                (s0, s1, s2, 0.0)
            }
        }
    } else {
        let (s0, s1) = quick_two_sum(s0, s2);
        if s1 != 0.0 {
            let (s1, s2) = quick_two_sum(s1, s3);
            if s2 != 0.0 {
                let (s2, s3) = quick_two_sum(s2, s4);
                (s0, s1, s2, s3)
            } else {
                let (s1, s2) = quick_two_sum(s1, s4);
                (s0, s1, s2, 0.0)
            }
        } else {
            let (s0, s1) = quick_two_sum(s0, s3);
            if s1 != 0.0 {
                let (s1, s2) = quick_two_sum(s1, s4);
                (s0, s1, s2, 0.0)
            } else {
                let (s0, s1) = quick_two_sum(s0, s4);
                (s0, s1, 0.0, 0.0)
            }
        }
    }
}

// #endregion
