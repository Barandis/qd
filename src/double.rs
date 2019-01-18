// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::renorm2;
use std::f64;
use std::ops::{Index, IndexMut};

#[macro_use]
mod macros {
    /// Creates a new double-double from another number or from a string.
    ///
    /// The argument can be any expression that evaluates to a type that this library defines a
    /// `From` implementation for. This includes `&str`, `Double`, any primitive number that is not
    /// a `u128` or `i128`, and 2-tuples of any of those primitive number types.
    ///
    /// # Panics
    ///
    /// Passing an expression that evaluates to a type that does not have a `From` implementation
    /// will cause a panic.
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
        };
    }
}

mod alg;
mod arith;
mod common;
mod comp;
mod consts;
mod conv;
mod hyper;
mod misc;
mod parse;
mod trans;
mod trig;

/// A 128-bit floating-point number implemented as the unevaluated sum of two 64-bit floating-point
/// numbers.
///
/// There are several ways to create a new `Double`:
///
/// * calling the [`new`] or [`norm`] functions
/// * calling `Double::from` and passing a type that has a `From` implementation
/// * calling `parse` on a string
/// * calling [`from_add`], [`from_sub`], [`from_mul`], or [`from_div`]
/// * using the [`dd`] macro
///
/// See the [module-level documentation] (index.html) for more information.
///
/// [`new`]: #method.new
/// [`norm`]: #method.norm
/// [`from_add`]: #method.from_add
/// [`from_sub`]: #method.from_sub
/// [`from_mul`]: #method.from_mul
/// [`from_div`]: #method.from_div
/// [`dd`]: macro.dd.html
/// [module-level documentation]: index.html
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
    /// ```
    /// # use qd::Double;
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
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let dd = Double::norm(2.0, 1.0);
    /// assert!(dd == dd!(3.0));
    /// # }
    /// ```
    pub fn norm(a: f64, b: f64) -> Double {
        Double::from(renorm2(a, b))
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

impl IndexMut<usize> for Double {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!(
                "Index of double-double out of range (must be in range [0, 1]): {}",
                idx
            ),
        }
    }
}
