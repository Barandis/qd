// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common;
use crate::double::Double;

// Compute sin a using the Taylor series. This assumes that |a| <= π/32.
#[allow(clippy::many_single_char_names)]
pub(super) fn sin_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ZERO
    } else {
        let threshold = common::mul_pwr2(a.abs() * Double::EPSILON, 0.5);
        let x = -a.sqr();
        let mut s = a;
        let mut r = a;
        let mut i = 0;

        loop {
            r *= x;
            let t = r * common::INV_FACTS[i];
            s += t;
            i += 2;
            if i >= common::INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Compute cos a using the Taylor series. This assumes that |a| <= π/32.
#[allow(clippy::many_single_char_names)]
pub(super) fn cos_taylor(a: Double) -> Double {
    if a.is_zero() {
        Double::ONE
    } else {
        let threshold = common::mul_pwr2(Double::EPSILON, 0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Double::ONE + common::mul_pwr2(r, 0.5);
        let mut i = 1;

        loop {
            r *= x;
            let t = r * common::INV_FACTS[i];
            s += t;
            i += 2;
            if i >= common::INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Computes both the sine and cosine of a using the Taylor series. This is a bit quicker
// than calling the two functions above separately, since if you have one of them you can
// calculate the other more efficiently.
pub(super) fn sincos_taylor(a: Double) -> (Double, Double) {
    if a.is_zero() {
        (Double::ZERO, Double::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Double::ONE - sin_a.sqr()).sqrt())
    }
}

// Helper function to reduce the input to a value whose sin/cos can be calculated via Taylor
// series. It firsts reduces modulo 2π, then π/2, then π/16. Aside from returning the
// reduced value (`t`), it also returns the group within the next higher modulo in which the
// value fell (`j` and `k`, this is the quadrant for `j`).
#[inline]
#[allow(clippy::many_single_char_names)]
pub(super) fn reduce(a: Double) -> (i32, i32, Double) {
    // reduce modulo 2π
    let z = (a / Double::MUL_2_PI).round();
    let r = a - z * Double::MUL_2_PI;

    // reduce modulo π/2
    let mut q = (r.0 / Double::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - Double(q, 0.0) * Double::FRAC_PI_2;
    let j = q as i32;

    // reduce modulo π/16
    q = (t.0 / Double::FRAC_PI_16.0 + 0.5).floor();
    t -= Double(q, 0.0) * Double::FRAC_PI_16;
    let k = q as i32;

    (j, k, t)
}
