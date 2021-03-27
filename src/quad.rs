// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ops::Index;

#[macro_use]
mod macros {
    /// Creates a new quad-double from another number or from a string.
    ///
    /// The argument can be any expression that evaluates to a type that this library
    /// defines a `From` implementation for. This includes `&str`, `Double`, `Quad`, any
    /// primitive number, and 2-, 3-, and 4-tuples of any of those primitive number types.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(0) == Quad::ZERO);
    ///
    /// let x = qd!(1) / qd!(2).sqrt();
    /// let expected = qd!("0.7071067811865475244008443621048490392848359376884740365883398690");
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[macro_export]
    macro_rules! qd {
        ($x:expr) => {
            $crate::Quad::from($x)
        };
    }
}

#[cfg(test)]
#[macro_use]
mod tests {
    use super::Quad;

    macro_rules! assert_precision {
        ($expected:expr, $actual:expr, $digits:expr) => {
            let expected = Quad::from($expected);
            let actual = Quad::from($actual);
            let mag = expected.0.abs().log10().floor() as i32;
            let epsilon = Quad(10.0, 0.0, 0.0, 0.0).powi(mag - $digits);
            let diff = (expected - actual).abs();
            let message = format!(
                concat!(
                    "\n",
                    "Expected: {0}\n",
                    "Actual:   {1}\n",
                    "\n",
                    "Delta:    {2:e}\n",
                    "Epsilon:  {3:e}\n",
                    "\n",
                    "Components:\n",
                    "  Expected: {4:<22e} {5:<22e} {6:<22e} {7:e}\n",
                    "  Actual:   {8:<22e} {9:<22e} {10:<22e} {11:e}\n",
                ),
                expected,
                actual,
                diff,
                epsilon,
                expected[0],
                expected[1],
                expected[2],
                expected[3],
                actual[0],
                actual[1],
                actual[2],
                actual[3],
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
            assert_precision!($expected, $actual, 60);
        };
    }

    macro_rules! assert_all_close {
        ($($expected:expr, $actual:expr);* $(;)?) => {
            $(assert_close!($expected, $actual);)*
        }
    }

    macro_rules! assert_exact {
        ($expected:expr, $actual:expr) => {
            let expected = Quad::from($expected);
            let actual = Quad::from($actual);
            let message = format!(
                concat!(
                    "\n",
                    "Expected: {0}\n",
                    "Actual:   {1}\n",
                    "\n",
                    "Components:\n",
                    "  Expected: {2:<22e} {3:<22e} {4:<22e} {5:e}\n",
                    "  Actual:   {6:<22e} {7:<22e} {8:<22e} {9:e}\n",
                ),
                expected,
                actual,
                expected[0],
                expected[1],
                expected[2],
                expected[3],
                actual[0],
                actual[1],
                actual[2],
                actual[3],
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
        let a = Quad::new(0.0, 10.0, -3.0, 5.0);
        assert_exact!(a.0, 0.0);
        assert_exact!(a.1, 10.0);
        assert_exact!(a.2, -3.0);
        assert_exact!(a.3, 5.0);
    }

    #[test]
    fn index() {
        assert_exact!(Quad::PI[0], Quad::PI.0);
        assert_exact!(Quad::PI[1], Quad::PI.1);
        assert_exact!(Quad::PI[2], Quad::PI.2);
        assert_exact!(Quad::PI[3], Quad::PI.3);
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

/// A 256-bit floating-point number implemented as the unevaluated sum of four 64-bit
/// floating-point numbers. Discarding the bits used for exponents, this makes for about
/// 212 bits of mantissa accuracy, or around 63 decimal digits.
///
/// There are several ways to create a new `Quad`:
///
/// * calling the [`new`] function
/// * calling [`from`] with a primitive number or a string
/// * calling [`parse`] on a string (or equivalently using [`from_str`])
/// * using the [`qd!`] macro
///
/// What kind of number you actually end up getting depends on the method called to get it.
/// [`new`] will *not* normalize its result. This means that the arguments must be
/// pre-normalized. [`from`], [`parse`], and [`qd!`] will both account for floating-point
/// rounding error *and* produce normalized results.
///
/// The reason for these two different ways of going about creation is speed. If the number
/// is already pre-computed to take normalization and error into account (as all of the
/// constants in this library are), then [`new`] offers a way to avoid having to pay the
/// efficiency cost of unnecessary normalization.
///
/// For the other methods, shortcuts can be taken if the input is a number and that number
/// is [*dyadic*] (i.e., it can be represented in binary exactly, without rounding). In this
/// case, [`from`] and [`qd!`] can also skip normalization and accounting for rounding, and
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
/// [`qd!`]: macro.qd.html
/// [*dyadic*]: https://en.wikipedia.org/wiki/Dyadic_rational
#[derive(Clone, Copy)]
pub struct Quad(f64, f64, f64, f64);

impl Quad {
    /// Creates a `Quad` with the four arguments as the internal components.
    ///
    /// **Be sure you know what you're doing if you use this function.** It does not
    /// normalize its components, meaning that if they aren't already normalized by the
    /// caller, this number will not work the way one would expect (it'll fail equality
    /// tests that it should pass, it may be classified incorrectly, etc.).
    ///
    /// This function is primarily for creating constants where the normalization is
    /// obviously unnecessary. For example, if a `Quad` version of the number `10` is
    /// needed, `Quad::new(10.0, 0.0, 0.0, 0.0)` is a good way to do it in order to save the
    /// cost of the normalization that is obviously not needed.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let q = Quad::new(0.0, 0.0, 0.0, 0.0);
    /// assert!(q.is_zero());
    /// ```
    pub const fn new(a: f64, b: f64, c: f64, d: f64) -> Quad {
        Quad(a, b, c, d)
    }
}

impl Index<usize> for Quad {
    type Output = f64;

    /// Returns one of the components of the `Quad`.
    ///
    /// Using index `0` will return the first component, using index `1` will return the
    /// second, and so on.
    ///
    /// One capability that is *not* provided is mutable indexing; ensuring that a `Quad` is
    /// normalized would be impossible if they could be individually changed at will.
    /// `Quad`s are immutable like any other number; if you need a new value for a `Quad`,
    /// you should simply create a new `Quad`.
    ///
    /// This is primarily provided for making certain mathematical algorithms easier to
    /// implement. There isn't a lot meaning to an individual component of a `Quad` other
    /// than the first.Quad
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let q = Quad::ONE;
    /// assert!(q[0] == 1.0);
    /// assert!(q[1] == 0.0);
    /// assert!(q[2] == 0.0);
    /// assert!(q[3] == 0.0);
    /// # }
    /// ```
    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!(
                "Index of quad-double out of range (must be in range [0, 3]): {}",
                idx
            ),
        }
    }
}
