// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::{mul_pwr2, INV_FACTS};
use crate::quad::Quad;

const FRAC_PI_1024: Quad = Quad(
    3.0679615757712823e-3,
    1.195944139792337e-19,
    -2.924579892303066e-36,
    1.0863810750618759e-52,
);

// Compute sin a using the Taylor series. This assumes that |a| <= π/2048.
pub(super) fn sin_taylor(a: Quad) -> Quad {
    if a.is_zero() {
        Quad::ZERO
    } else {
        let threshold = mul_pwr2(Quad::EPSILON * a.abs(), 0.5);
        let x = -a.sqr();
        let mut s = a;
        let mut r = a;
        let mut i = 0;

        loop {
            r *= x;
            let t = r * INV_FACTS[i];
            s += t;
            i += 2;
            if i >= INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Compute cos a using the Taylor series. This assumes that |a| <= π/2048.
pub(super) fn cos_taylor(a: Quad) -> Quad {
    if a.is_zero() {
        Quad::ONE
    } else {
        let threshold = mul_pwr2(Quad::EPSILON, 0.5);
        let x = -a.sqr();
        let mut r = x;
        let mut s = Quad::ONE + mul_pwr2(r, 0.5);
        let mut i = 1;

        loop {
            r *= x;
            let t = r * INV_FACTS[i];
            s += t;
            i += 2;
            if i >= INV_FACTS.len() || t.abs() <= threshold {
                break;
            }
        }
        s
    }
}

// Computes both the sine and cosine of a using the Taylor series. This is a bit quicker than
// calling the two functions above separately, since if you have one of them you can calculate the
// other more efficiently.
pub(super) fn sincos_taylor(a: Quad) -> (Quad, Quad) {
    if a.is_zero() {
        (Quad::ZERO, Quad::ONE)
    } else {
        let sin_a = sin_taylor(a);
        (sin_a, (Quad::ONE - sin_a.sqr()).sqrt())
    }
}

// Helper function to reduce the input to a value whose sin/cos can be calculated via Taylor series.
// It firsts reduces modulo 2π, then π/2, then π/1024. Aside from returning the reduced value (`t`),
// it also returns the group within the next higher modulo in which the value fell (`j` and `k`,
// this is the quadrant for `j`).
#[inline]
pub(super) fn reduce(a: Quad) -> (i32, i32, Quad) {
    // reduce modulo 2π
    let z = (a / Quad::MUL_2_PI).round();
    let r = a - z * Quad::MUL_2_PI;

    // reduce modulo π/2
    let mut q = (r.0 / Quad::FRAC_PI_2.0 + 0.5).floor();
    let mut t = r - Quad::from(q) * Quad::FRAC_PI_2;
    let j = q as i32;

    // reduce modulo π/1024
    q = (t.0 / FRAC_PI_1024.0 + 0.5).floor();
    t -= Quad::from(q) * FRAC_PI_1024;
    let k = q as i32;

    (j, k, t)
}
