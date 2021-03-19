// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use std::ops::Index;

#[macro_use]
mod macros {
    /// Creates a new quad-double from another number or from a string.
    ///
    /// The argument can be any expression that evaluates to a type that this library
    /// defines a `From` implementation for. This includes `&str`, `Double`, `Quad`, any
    /// primitive number, and 2-, 3-, and 4-tuples of any of those primitive number types.
    ///
    /// # Panics
    ///
    /// Passing an expression that evaluates to a type that does not have a `From`
    /// implementation will cause a panic.
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
            Quad::from($x)
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
            let mag = expected.abs().log10().floor().as_int() as i32;
            let epsilon = Quad(10.0, 0.0, 0.0, 0.0).powi(mag - $digits);
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
            assert_precision!($expected, $actual, 60);
        };
    }

    macro_rules! assert_exact {
        ($expected:expr, $actual:expr) => {
            let expected = Quad::from($expected);
            let actual = Quad::from($actual);
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
        let a = Quad::raw(0.0, 10.0, -3.0, 5.0);
        assert_exact!(a.0, 0.0);
        assert_exact!(a.1, 10.0);
        assert_exact!(a.2, -3.0);
        assert_exact!(a.3, 5.0);
    }

    #[test]
    fn new() {
        let a = Quad::new(0.0, 10.1, -3.0, 5.0);
        assert_exact!(a.0, 12.1);
        assert_exact!(a.1, 0.0);
        assert_exact!(a.2, 0.0);
        assert_exact!(a.3, 0.0);
    }

    #[test]
    fn index() {
        assert_exact!(Quad::PI[0], Quad::PI.0);
        assert_exact!(Quad::PI[1], Quad::PI.1);
        assert_exact!(Quad::PI[2], Quad::PI.2);
        assert_exact!(Quad::PI[3], Quad::PI.3);
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

/// A 256-bit floating-point number implemented as the unevaluated sum of four 64-bit
/// floating-point numbers. Discarding the bits used for exponents, this makes for about
/// 212 bits of mantissa accuracy, or around 63 decimal digits.
///
/// There are several ways to create a new `Quad`:
///
/// * calling the [`new`] or [`raw`] functions
/// * calling [`from`] and passing a type that has a `From` implementation
/// * calling [`parse`] on a string (or equivalently using [`from_str`])
/// * using the [`qd!`] macro
///
/// What kind of number you actually end up getting depends on the method called to get it.
///
/// * [`raw`] will *not* normalize its result. This is for speed, but it means that the
///   arguments must be pre-normalized.
/// * [`new`] and [`from`] (when used with tuples) will normalize their results but will
///   *not* account for floating-point rounding error. `f64`s passed to these functions are
///   assumed to be exactly what's desired, including the rounding error.
/// * [`from`] (when used with non-tuples), [`parse`], and [`qd!`] will both account for
///   floating-point rounding error *and* produce normalized results. This is the slowest of
///   the three choices but also the most accurate.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`new`]: #method.new
/// [`raw`]: #method.raw
/// [`from`]: #impl-From<f64>
/// [`parse`]: #impl-FromStr
/// [`from_str`]: #method.from_str
/// [`qd!`]: macro.qd.html
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
    /// needed, `Quad::raw(10.0, 0.0, 0.0, 0.0)` is a good way to do it in order to save the
    /// cost of the normalization that is obviously not needed.
    ///
    /// # Examples
    /// ```
    /// # use qd::Quad;
    /// let q = Quad::raw(0.0, 0.0, 0.0, 0.0);
    /// assert!(q.is_zero());
    /// ```
    pub fn raw(a: f64, b: f64, c: f64, d: f64) -> Quad {
        Quad(a, b, c, d)
    }

    /// Creates a `Quad` by normalizing its four arguments.
    ///
    /// This function normalizes the input arguments (if this is obviously unnecessary, use
    /// [`raw`] instead) and assigns the normalized values to the new `Quad`'s components.
    ///
    /// It's assumed that the four numbers passed in are exactly what's desired, and aside
    /// from normalization, they will not be manipulated further. That means that any
    /// floating-point rounding error will be retained. For instance, `Quad::new(1.1, 0.0,
    /// 0.0, 0.0)` actually produces the number
    /// `1.100000000000000088817841970012523233890533447265625`. To account for that
    /// rounding error, use [`from`] or the [`qd!`] macro; `qd!(1.1)` is effectively the
    /// same as `Quad::new(1.1, -8.881784197001253e-17, 4.930380657631324e-33,
    /// -2.7369110631344085e-49)`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let d = Quad::new(2.0, 1.0, 3.0, -4.5);
    /// assert!(d == qd!(1.5));
    /// # }
    /// ```
    ///
    /// [`raw`]: #method.raw
    /// [`from`]: #impl-From<f64>
    /// [`qd!`]: macro.qd.html
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Quad {
        let mut comps = [a, b, c, d];
        if comps.iter().any(|&x| x.is_nan()) {
            // Returns NaN if any component is NaN
            Quad::NAN
        } else {
            // Reverse sort, largest absolute value to smallest. Normalization depends on
            // the components being in this order. The `unwrap` is safe because we've
            // already dealt with the one case where ordering can fail (NaN).
            comps.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
            Quad::from(core::renorm4(comps[0], comps[1], comps[2], comps[3]))
        }
    }
}

impl Index<usize> for Quad {
    type Output = f64;

    /// Returns one of the components of the `Quad`.
    ///
    /// Using index `0` will return the first component, using index `1` will return the
    /// second, and so on. This capability is provided mostly to make some algorithms easier
    /// to implement. If the components of the `Double` are needed, pattern matching with
    /// the 4-tuple's [`from`] is likely to be the better way to go.
    ///
    /// One capability that is *not* provided is mutable indexing; ensuring that a `Quad`
    /// is normalized would be impossible if they could be individually changed at will. If
    /// you need to modify the components of an existing mutable `Quad`, use [`assign`].
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
    ///
    /// [`assign`]: #method.assign
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
