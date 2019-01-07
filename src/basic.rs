// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

#[cfg(no_fma)]
const SPLIT_FACTOR: f64 = 134217729.0;                  // = 2^27 + 1

#[cfg(no_fma)]
const SPLIT_THRESHOLD: f64 = 6.69692879491417e+299;     // = 2^996

#[cfg(no_fma)]
const SPLIT_SHIFT_DOWN: f64 = 3.7252902984619140625e-9; // = 2^-28

#[cfg(no_fma)]
const SPLIT_SHIFT_UP: f64 = 268435456.0;                // = 2^28

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

#[inline]
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let v = s - a;
    let e = (a - (s - v)) + (b - v);
    (s, e)
}

pub fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let s = a - b;
    let v = s - a;
    let e = (a - (s - v)) - (b + v);
    (s, e)
}

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

#[cfg(not(no_fma))]
#[inline]
pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let e = a.mul_add(b, -p);
    (p, e)
}

#[cfg(no_fma)]
#[inline]
pub fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let (ahi, alo) = split(a);
    let (bhi, blo) = split(b);
    let e = ahi * bhi - p + ahi * blo + alo * bhi + alo * blo;
    (p, e)
}

#[cfg(not(no_fma))]
#[inline]
pub fn two_square(a: f64) -> (f64, f64) {
    let p = a * a;
    let e = a.mul_add(a, -p);
    (p, e)
}

#[cfg(no_fma)]
#[inline]
pub fn two_square(a: f64) -> (f64, f64) {
    let p = a * a;
    let (hi, lo) = split(a);
    let e = hi * hi - p + 2.0 * hi * lo + lo * lo;
    (p, e)
}
