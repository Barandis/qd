// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::two_sum;
use std::f64;
use std::ops::Index;

#[macro_use]
mod macros {
    /// Creates a new double-double from another number or from a string.
    ///
    /// The argument can be any expression that evaluates to a type that this library
    /// defines a `From` implementation for. This includes `&str`, `Double`, any primitive
    /// number that is not a `u128` or `i128`, and 2-tuples of any of those primitive number
    /// types.
    ///
    /// # Panics
    ///
    /// Passing an expression that evaluates to a type that does not have a `From`
    /// implementation will cause a panic.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(dd!(0) == Double::ZERO);
    ///
    /// let x = dd!(1) / dd!(2).sqrt();
    /// let expected = dd!("0.70710678118654752440084436210485");
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[macro_export]
    macro_rules! dd {
        ($x:expr) => {
            Double::from($x)
        };
    }
}

#[cfg(test)]
#[macro_use]
mod tests {
    macro_rules! assert_precision {
        ($expected:expr, $actual:expr, $digits:expr) => {
            let expected = Double::from($expected);
            let actual = Double::from($actual);
            let mag = expected.abs().log10().floor().as_int() as i32;
            let epsilon = Double(10.0, 0.0).powi(mag - $digits);
            let diff = (expected - actual).abs();
            let message = format!(
                "\nExpected: {0}\n\
                          ({0:?})\n\
                Actual:   {1}\n\
                          ({1:?})\n\
                Delta:    {2:e}",
                expected, actual, diff
            );
            assert!(diff < epsilon, message);
        };
    }

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 30);
        };
    }

    macro_rules! assert_exact {
        ($expected:expr, $actual:expr) => {
            let expected = Double::from($expected);
            let actual = Double::from($actual);
            let message = format!(
                "\nExpected: {0}\n          ({0:?})\nActual:   {1}\n          ({1:?})",
                expected, actual
            );
            if expected.is_nan() {
                assert!(actual.is_nan(), message);
            } else {
                assert!(expected == actual, message);
            }
        };
    }
}

mod alg;
mod arith;
mod common;
mod comp;
mod consts;
mod display;
mod from;
mod from_str;
mod hyper;
mod iter;
mod misc;
mod trans;
mod trig;

/// A 128-bit floating-point number implemented as the unevaluated sum of two 64-bit
/// floating-point numbers. Discarding the bits used for exponents, this makes for about
/// 112 bits of accuracy, or around 31 decimal digits.
///
/// There are several ways to create a new `Double`:
///
/// * calling the [`new`] or [`raw`] functions
/// * calling `Double::from` and passing a type that has a `From` implementation
/// * calling `parse` on a string
/// * calling [`from_add`], [`from_sub`], [`from_mul`], or [`from_div`]
/// * using the [`dd!`] macro
/// 
/// If a `Double` is created directly from an `f32` or an `f64` (which can be done either
/// with `Double::from` or [`dd!`]), then floating-point error is calculated and accounted
/// for. This is slower, as the `f32`/`f64` needs to be parsed digit by digit, but it is
/// vital to accuracy.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`new`]: #method.new
/// [`raw`]: #method.raw
/// [`from_add`]: #method.from_add
/// [`from_sub`]: #method.from_sub
/// [`from_mul`]: #method.from_mul
/// [`from_div`]: #method.from_div
/// [`dd!`]: macro.dd.html
#[derive(Clone, Copy)]
pub struct Double(f64, f64);

impl Double {
    /// Creates a `Double` with the two arguments as the internal components.
    ///
    /// **Be sure you know what you're doing if you use this function.** It does not
    /// normalize its components, meaning that if they aren't already normalized by the
    /// caller, this number will not work the way one would expect (it'll fail equality
    /// tests that it should pass, it may be classified incorrectly, etc.).
    ///
    /// This function is primarily for creating constants where the normalization is
    /// obviously unnecessary. For example, if a `Double` version of the number `10` is
    /// needed, `Double::raw(10.0, 0.0)` is a good way to do it in order to save the cost
    /// of the normalization that is obviously not needed.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let d = Double::raw(0.0, 0.0);
    /// assert!(d.is_zero());
    /// ```
    pub fn raw(a: f64, b: f64) -> Double {
        Double(a, b)
    }

    /// Creates a `Double` by normalizing the sum of two arguments.
    ///
    /// This function normalizes its components (if this is obviously unnecessary, use
    /// [`raw`](#method.raw) instead). The normalization is effective no matter the values
    /// of the components; while it's possible to have more efficient normalization if we
    /// know that |`a`| >= |`b`|, the "safe" normalization is still less expensive than the
    /// conditional required to know whether the quick one can be used.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let d = Double::new(2.0, 1.0);
    /// assert!(d == dd!(3.0));
    /// # }
    /// ```
    pub fn new(a: f64, b: f64) -> Double {
        Double::from(two_sum(a, b))
    }
}

impl Index<usize> for Double {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            _ => panic!(
                "Index of double-double out of range (must be in range [0, 1]): {}",
                idx
            ),
        }
    }
}
