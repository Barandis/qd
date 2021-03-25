// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;

/// Calculates the sum of three `f64`s in double-double precision.
#[inline]
pub fn three_two_sum(a: f64, b: f64, c: f64) -> (f64, f64) {
    let (u, v) = p::two_sum(a, b);
    let (s, w) = p::two_sum(c, u);
    (s, v + w)
}

/// Calculates the sum of three `f64`s in triple-double precision.
#[inline]
pub fn three_three_sum(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let (u, v) = p::two_sum(a, b);
    let (s, w) = p::two_sum(c, u);
    let (e1, e2) = p::two_sum(v, w);
    (s, e1, e2)
}

/// Calculates the sum of four `f64`s in double-double precision.
#[inline]
pub fn four_two_sum(a: f64, b: f64, c: f64, d: f64) -> (f64, f64) {
    let (s0, s1) = p::two_sum(a, c);
    (s0, s1 + b + d)
}

/// Calculates the sum of six `f64`s in triple-double precision.
#[inline]
pub fn six_three_sum(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> (f64, f64, f64) {
    let (p0, p1, p2) = three_three_sum(a, b, c);
    let (q0, q1, q2) = three_three_sum(d, e, f);
    let (r0, r1) = p::two_sum(p0, q0);
    let (s0, s1) = p::two_sum(p1, q1);
    let (t0, t1) = p::two_sum(s0, r1);
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
    let (p0, p1) = p::two_sum(a, b);
    let (q0, q1) = p::two_sum(c, d);
    let (r0, r1) = p::two_sum(e, f);
    let (s0, s1) = p::two_sum(g, h);
    let (t0, t1) = four_two_sum(p0, p1, q0, q1);
    let (u0, u1) = four_two_sum(r0, r1, s0, s1);
    let (v0, v1) = four_two_sum(t0, t1, u0, u1);
    let (w0, w1) = p::two_sum(v0, i);
    (w0, w1 + v1)
}

/// Adds a float to an value/error pair.
///
/// If the result of this addition doesn't fit in two `f64`s, the sum is output as the first
/// tuple component and the second and third contain the remainder. Otherwise, the first
/// tuple component is `0.0` and the sum is in the other two components.
#[inline]
pub fn accumulate(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let (s, b) = p::two_sum(b, c);
    let (s, a) = p::two_sum(a, s);

    let za = a == 0.0;
    let zb = b == 0.0;

    if !(za || zb) {
        (s, a, b)
    } else {
        (0.0, s, if zb { a } else { b })
    }
}

/// Renormalizes two components into a two-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of the last component is no more than half the ULP of the
/// first.
#[inline]
pub fn renorm2(a: f64, b: f64) -> (f64, f64) {
    p::quick_two_sum(a, b)
}

/// Renormalizes three components into a two-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of the last component is no more than half the ULP of the
/// first.
#[inline]
pub fn renorm3(a: f64, b: f64, c: f64) -> (f64, f64) {
    let (u, v) = p::quick_two_sum(a, b);
    let (s, w) = p::quick_two_sum(c, u);
    p::quick_two_sum(s, v + w)
}

/// Renormalizes four components into a four-component value.
///
/// Renormalization ensures that the components of the returned tuple are arranged in such a
/// way that the absolute value of each component is no more than half of the ULP of the
/// prior component.
#[inline]
pub fn renorm4(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64) {
    let (x, s3) = p::quick_two_sum(c, d);
    let (x, s2) = p::quick_two_sum(b, x);
    let (s0, s1) = p::quick_two_sum(a, x);

    if s1 != 0.0 {
        let (s1, s2) = p::quick_two_sum(s1, s2);
        if s2 != 0.0 {
            let (s2, s3) = p::quick_two_sum(s2, s3);
            (s0, s1, s2, s3)
        } else {
            let (s1, s2) = p::quick_two_sum(s1, s3);
            (s0, s1, s2, 0.0)
        }
    } else {
        let (s0, s1) = p::quick_two_sum(s0, s2);
        if s1 != 0.0 {
            let (s1, s2) = p::quick_two_sum(s1, s3);
            (s0, s1, s2, 0.0)
        } else {
            let (s0, s1) = p::quick_two_sum(s0, s3);
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
    let (x, s4) = p::quick_two_sum(d, e);
    let (x, s3) = p::quick_two_sum(c, x);
    let (x, s2) = p::quick_two_sum(b, x);
    let (s0, s1) = p::quick_two_sum(a, x);

    if s1 != 0.0 {
        let (s1, s2) = p::quick_two_sum(s1, s2);
        if s2 != 0.0 {
            let (s2, s3) = p::quick_two_sum(s2, s3);
            if s3 != 0.0 {
                (s0, s1, s2, s3 + s4)
            } else {
                let (s2, s3) = p::quick_two_sum(s2, s4);
                (s0, s1, s2, s3)
            }
        } else {
            let (s1, s2) = p::quick_two_sum(s1, s3);
            if s2 != 0.0 {
                let (s2, s3) = p::quick_two_sum(s2, s4);
                (s0, s1, s2, s3)
            } else {
                let (s1, s2) = p::quick_two_sum(s1, s4);
                (s0, s1, s2, 0.0)
            }
        }
    } else {
        let (s0, s1) = p::quick_two_sum(s0, s2);
        if s1 != 0.0 {
            let (s1, s2) = p::quick_two_sum(s1, s3);
            if s2 != 0.0 {
                let (s2, s3) = p::quick_two_sum(s2, s4);
                (s0, s1, s2, s3)
            } else {
                let (s1, s2) = p::quick_two_sum(s1, s4);
                (s0, s1, s2, 0.0)
            }
        } else {
            let (s0, s1) = p::quick_two_sum(s0, s3);
            if s1 != 0.0 {
                let (s1, s2) = p::quick_two_sum(s1, s4);
                (s0, s1, s2, 0.0)
            } else {
                let (s0, s1) = p::quick_two_sum(s0, s4);
                (s0, s1, 0.0, 0.0)
            }
        }
    }
}

/// Determines whether a number is exact (true) or has floating-point error (false).
///
/// A number is exactly representable in binary if it can be rendered as a fraction with a
/// power of two as an exponent. If so, then floating-point error doesn't exist and the
/// number can be turned into a quad- or double-double much more efficiently.
pub fn is_dyadic(n: f64) -> bool {
    let f = n.fract();
    if f == 0.0 {
        true
    } else {
        let len = f.to_string().len() - 2; // ignore the leading "0."
        let base = 2f64.powi(-(len as i32));
        f % base == 0.0
    }
}
