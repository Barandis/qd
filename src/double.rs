// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
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
    use super::Double;

    macro_rules! assert_precision {
        ($expected:expr, $actual:expr, $digits:expr) => {
            let expected = Double::from($expected);
            let actual = Double::from($actual);
            let mag = f64::from(expected.abs().log10().floor()) as i32;
            let epsilon = Double(10.0, 0.0).powi(mag - $digits);
            let diff = (expected - actual).abs();
            let message = format!(
                concat!(
                    "\n",
                    "Expected: {0}\n",
                    "          ({0:?})\n",
                    "Actual:   {1}\n",
                    "          ({1:?})\n",
                    "Delta:    {2:e}"
                ),
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
                concat!(
                    "\n",
                    "Expected: {0}\n",
                    "          ({0:?})\n",
                    "Actual:   {1}\n",
                    "          ({1:?})"
                ),
                expected, actual
            );
            if expected.is_nan() {
                assert!(actual.is_nan(), message);
            } else {
                assert!(expected == actual, message);
            }
        };
    }

    #[test]
    fn raw() {
        let a = Double::raw(0.0, 10.0);
        assert_exact!(a.0, 0.0);
        assert_exact!(a.1, 10.0);
    }

    #[test]
    fn new() {
        let a = Double::new(0.0, 10.1);
        assert_exact!(a.0, 10.1);
        assert_exact!(a.1, 0.0);
    }

    #[test]
    fn index() {
        assert_exact!(Double::PI[0], Double::PI.0);
        assert_exact!(Double::PI[1], Double::PI.1);
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
/// 106 bits of mantissa accuracy, or around 31 decimal digits.
///
/// There are several ways to create a new `Double`:
///
/// * calling the [`new`] or [`raw`] functions
/// * calling [`from`] with a primitive number (except for `u128` and `i128`) or a string
/// * calling [`parse`] on a string (or equivalently using [`from_str`])
/// * calling [`from_add`], [`from_sub`], [`from_mul`], or [`from_div`]
/// * using the [`dd!`] macro
///
/// What kind of number you actually end up getting depends on the method called to get it.
/// 
/// * [`raw`] will *not* normalize its result. This is for speed, but it means that the
///   arguments must be pre-normalized.
/// * [`new`], [`from_add`], [`from_sub`], [`from_mul`], and [`from_div`] will normalize
///   their results but will *not* account for floating-point rounding error. `f64`s passed
///   to these functions are assumed to be exactly what's desired, including the rounding
///   error.
/// * [`from`], [`parse`], and [`dd!`] will both account for floating-point rounding error
///   *and* produce normalized results. This is the slowest of the three choices but also
///   the most accurate.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`new`]: #method.new
/// [`raw`]: #method.raw
/// [`from`]: #impl-From<f64>
/// [`parse`]: #impl-FromStr
/// [`from_str`]: #method.from_str
/// [`from_add`]: #method.from_add
/// [`from_sub`]: #method.from_sub
/// [`from_mul`]: #method.from_mul
/// [`from_div`]: #method.from_div
/// [`dd!`]: macro.dd.html
#[derive(Clone, Copy, Default)]
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

    /// Creates a `Double` by normalizing its two arguments.
    ///
    /// This function normalizes the input arguments (if this is obviously unnecessary, use
    /// [`raw`] instead) and assigns the normalized values to the new `Double`'s components.
    ///
    /// It's assumed that the two numbers passed in are exactly what's desired, and aside
    /// from normalization, they will not be manipulated further. That means that any
    /// floating-point rounding error will be retained. For instance, `Double::new(1.1,
    /// 0.0)` actually produces the number `1.10000000000000008881784197001253`. To account
    /// for that rounding error, use [`from`] or the [`dd!`] macro; `dd!(1.1)` is
    /// effectively the same as `Double::new(1.1, -8.881784197001253e-17)`.
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
    ///
    /// [`raw`]: #method.raw
    /// [`from`]: #impl-From<f64>
    /// [`dd!`]: macro.dd.html
    pub fn new(a: f64, b: f64) -> Double {
        let (a, b) = if a.abs() > b.abs() {
            core::renorm2(a, b)
        } else {
            core::renorm2(b, a)
        };
        Double(a, b)
    }
}

impl Index<usize> for Double {
    type Output = f64;

    /// Returns one of the components of the `Double`.
    ///
    /// Using index `0` will return the first component; using index `1` will return the
    /// second. This capability is provided mostly to make some algorithms easier to
    /// implement. If the components of the `Double` are needed, pattern matching with the
    /// 2-tuple's [`from`] is likely to be the better way to go.
    ///
    /// One capability that is *not* provided is mutable indexing; ensuring that a `Double`
    /// is normalized would be impossible if they could be individually changed at will. If
    /// you need to modify the components of an existing mutable `Double`, use [`assign`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let d = Double::ONE;
    /// assert!(d[0] == 1.0);
    /// assert!(d[1] == 0.0);
    /// # }
    /// ```
    ///
    /// [`from`]: #method.from-22
    /// [`assign`]: #method.assign
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
