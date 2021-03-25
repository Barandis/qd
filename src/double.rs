// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
            $crate::Double::from($x)
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
            let mag = expected.0.abs().log10().ceil() as i32;
            let epsilon = Double(10.0, 0.0).powi(mag - $digits);
            let diff = (expected - actual).abs();
            let message = format!(
                concat!(
                    "\n",
                    "Expected: {0}\n",
                    "          {0:?}\n",
                    "Actual:   {1}\n",
                    "          {1:?}\n",
                    "Delta:    {2:e}\n",
                    "Epsilon:  {3:e}\n",
                ),
                expected, actual, diff, epsilon
            );
            assert!(diff < epsilon, message);
        };
    }

    macro_rules! assert_precision_all {
        ($($expected:expr, $actual:expr, $digits:expr);* $(;)?) => {
            $(assert_precision!($expected, $actual, $digits);)*
        }
    }

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 30);
        };
    }

    macro_rules! assert_all_close {
        ($($expected:expr, $actual:expr);* $(;)?) => {
            $(assert_close!($expected, $actual);)*
        }
    }

    macro_rules! assert_exact {
        ($expected:expr, $actual:expr) => {
            let expected = Double::from($expected);
            let actual = Double::from($actual);
            let diff = (expected - actual).abs();
            let message = format!(
                concat!(
                    "\n",
                    "Expected: {0}\n",
                    "          {0:?}\n",
                    "Actual:   {1}\n",
                    "          {1:?}\n",
                    "Delta:    {2:e}\n",
                    "Epsilon:  0\n",
                ),
                expected, actual, diff
            );
            if expected.is_nan() {
                assert!(actual.is_nan(), message);
            } else {
                assert!(expected == actual, message);
            }
        };
    }

    macro_rules! assert_all_exact {
        ($($expected:expr, $actual:expr);* $(;)?) => {
            $(assert_exact!($expected, $actual);)*
        }
    }

    #[test]
    fn new() {
        let a = Double::new(0.0, 10.0);
        assert_exact!(a.0, 0.0);
        assert_exact!(a.1, 10.0);
    }

    #[test]
    fn index() {
        assert_exact!(Double::PI[0], Double::PI.0);
        assert_exact!(Double::PI[1], Double::PI.1);
    }
}

mod aconsts; // "a" for "associated", or maybe just to make it appear first in docs
mod add;
mod alg;
mod comp;
mod display;
mod div;
mod from;
mod from_str;
mod hyper;
mod iter;
mod misc;
mod mul;
mod neg;
mod rem;
mod sub;
mod tables;
mod trans;
mod trig;

/// A 128-bit floating-point number implemented as the unevaluated sum of two 64-bit
/// floating-point numbers. Discarding the bits used for exponents, this makes for about
/// 106 bits of mantissa accuracy, or around 31 decimal digits.
///
/// There are several ways to create a new `Double`:
///
/// * calling the [`new`] function
/// * calling [`from`] with a primitive number (except for `u128` and `i128`) or a string
/// * calling [`parse`] on a string (or equivalently using [`from_str`])
/// * using the [`dd!`] macro
///
/// What kind of number you actually end up getting depends on the method called to get it.
/// [`new`] will *not* normalize its result. This means that the arguments must be
/// pre-normalized. [`from`], [`parse`], and [`dd!`] will both account for floating-point
/// rounding error *and* produce normalized results.
///
/// The reason for these two different ways of going about creation is speed. If the number
/// is already pre-computed to take normalization and error into account (as all of the
/// constants in this library are), then [`new`] offers a way to avoid having to pay the
/// efficiency cost of unnecessary normalization.
///
/// For the other methods, shortcuts can be taken if the input is a number and that number
/// is [*dyadic*] (i.e., it can be represented in binary exactly, without rounding). In this
/// case, [`from`] and [`dd!`] can also skip normalization and accounting for rounding, and
/// they won't be much slower than [`new`].
///
/// Parsing from strings or from numbers that are not dyadic cannot take these shortcuts.
/// The results will be precise, but at the cost of speed.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`new`]: #method.new
/// [`from`]: #impl-From<f64>
/// [`parse`]: #impl-FromStr
/// [`from_str`]: #method.from_str
/// [`dd!`]: macro.dd.html
/// [*dyadic*]: https://en.wikipedia.org/wiki/Dyadic_rational
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
    /// needed, `Double::new(10.0, 0.0)` is a good way to do it in order to save the cost
    /// of the normalization that is obviously not needed.
    ///
    /// # Examples
    /// ```
    /// # use qd::Double;
    /// let d = Double::new(0.0, 0.0);
    /// assert!(d.is_zero());
    /// ```
    pub const fn new(a: f64, b: f64) -> Double {
        Double(a, b)
    }
}

impl Index<usize> for Double {
    type Output = f64;

    /// Returns one of the components of the `Double`.
    ///
    /// Using index `0` will return the first component and using index `1` will return the
    /// second.
    ///
    /// One capability that is *not* provided is mutable indexing; ensuring that a `Double`
    /// is normalized would be impossible if they could be individually changed at will.
    /// `Double`s are immutable like any other number; if you need a new value for a
    /// `Double`, you should simply create a new `Double`.
    ///
    /// This is primarily provided for making certain mathematical algorithms easier to
    /// implement. There isn't a lot meaning to an individual component of a `Double` other
    /// than the first.
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
