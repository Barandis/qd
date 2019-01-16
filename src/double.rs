// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::quick_two_sum;
use std::f64;

#[macro_use]
mod macros {
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
            let mag = expected.abs().log10().floor().to_int();
            let epsilon = 10f64.powi(mag - $digits);
            let message = format!(
                "\nExpected: {0} ({0:?})\nActual:   {1} ({1:?})",
                expected, actual
            );
            assert!((expected - actual).abs() < epsilon, message);
        };
    }

    macro_rules! assert_exact {
        ($expected:expr, $actual:expr) => {
            let expected = Double::from($expected);
            let actual = Double::from($actual);
            let message = format!(
                "\nExpected: {0} ({0:?})\nActual:   {1} ({1:?})",
                expected, actual
            );
            assert!(expected == actual, message);
        }
    }
}

mod consts;
mod algebraic;
mod arithmetic;
mod comparison;
mod conversion;
mod hyperbolic;
mod misc;
mod parsing;
mod transcendental;
mod trigonometric;

/// A 128-bit floating-point number implemented as the unevaluated sum of two 64-bit floating-point
/// numbers.
///
/// There are several ways to create a new `Double`: the [`new`](#method.new) or
/// [`norm`](#method.norm) functions, the various `From` implementations, the `FromStr`
/// implementation, or one of the mathematical `from_xxx` functions. See the [module-level
/// documentation](index.html) for more information.
#[derive(Clone, Copy, Debug)]
pub struct Double(f64, f64);

impl Double {
    /// Creates a `Double` with the two arguments as the internal components.
    ///
    /// This function is only useful in the simplest of cases, as it does not do normalization and
    /// therefore does not account for floating-point error in the first component (meaning the user
    /// has to). One of its primary functions is declaration of `Double` constants that have
    /// been pre-computed.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::new(0.0, 0.0);
    /// assert!(dd.is_zero());
    /// ```
    pub fn new(a: f64, b: f64) -> Double {
        Double(a, b)
    }

    /// Creates a `Double` by normalizing the sum of two arguments.
    ///
    /// This is a quick and efficient function, but it carries the restriction that the absolute
    /// value of `a` must be greater than or equal to the absolute value of `b`. If that cannot be
    /// guaranteed, it would be better to use the slightly slower but more robust
    /// [`from_add`](#method.from_add) instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::norm(2.0, 1.0);
    /// assert!(dd == 3.0);
    /// ```
    pub fn norm(a: f64, b: f64) -> Double {
        Double::from(quick_two_sum(a, b))
    }
}
