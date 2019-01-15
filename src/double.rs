// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::quick_two_sum;
use std::f64;

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

// #region Constants

impl Double {
    /// The radix or base of the internal representation of `Double`. This is the same as the
    /// representation in the underlying f64.
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = 106;

    /// Approximate number of significant digits in base 10.
    pub const DIGITS: u32 = 31;

    /// [Machine epsilon] value for `Double`.
    ///
    /// This is the difference between `1.0` and the next largest representable number.
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: f64 = 4.93038065763132e-32; // 2^-104

    /// Smallest finite `Double` value.
    pub const MIN: Double = Double(-1.7976931348623157e308, -9.979201547673598e291);

    /// Smallest positive normal `Double` value.
    pub const MIN_POSITIVE: Double = Double(2.2250738585072014e-308, 0e0);

    /// Largest finite `Double` value.
    pub const MAX: Double = Double(1.7976931348623157e308, 9.979201547673598e291);

    /// One greater than the minimum possible normal power of 2 exponent.
    pub const MIN_EXP: i32 = -1021;

    /// Maximum possible power of 2 exponent.
    pub const MAX_EXP: i32 = 1024;

    /// Minimum possible normal power of 10 exponent.
    pub const MIN_10_EXP: i32 = -307;

    /// Maximum possible power of 10 exponent.
    pub const MAX_10_EXP: i32 = 308;

    /// Not a Number (NaN).
    pub const NAN: Double = Double(f64::NAN, f64::NAN);

    /// Infinity (∞).
    pub const INFINITY: Double = Double(f64::INFINITY, f64::INFINITY);

    /// Negative infinity (-∞).
    pub const NEG_INFINITY: Double = Double(f64::NEG_INFINITY, f64::NEG_INFINITY);

    /// Archimedes' constant (π)
    pub const PI: Double = Double(3.141592653589793e0, 1.2246467991473532e-16);

    /// Zero (0)
    pub const ZERO: Double = Double(0.0, 0.0);

    /// Negative zero (-0)
    pub const NEG_ZERO: Double = Double(-0.0, 0.0);

    /// One (1)
    pub const ONE: Double = Double(1.0, 0.0);

    /// π/2
    pub const FRAC_PI_2: Double = Double(1.5707963267948966e0, 6.123233995736766e-17);

    /// π/3
    pub const FRAC_PI_3: Double = Double(1.0471975511965979e0, -1.072081766451091e-16);

    /// π/4
    pub const FRAC_PI_4: Double = Double(7.853981633974483e-1, 3.061616997868386e-17);

    /// π/6
    pub const FRAC_PI_6: Double = Double(5.235987755982989e-1, -5.360408832255455e-17);

    /// π/8
    pub const FRAC_PI_8: Double = Double(3.9269908169872414e-1, 1.5308084989341915e-17);

    /// 1/π
    pub const FRAC_1_PI: Double = Double(3.183098861837907e-1, -1.9678676675182486e-17);

    /// 2/π
    pub const FRAC_2_PI: Double = Double(6.366197723675814e-1, -3.9357353350364984e-17);

    /// 2/√π
    pub const FRAC_2_SQRT_PI: Double = Double(1.1283791670955126e0, 1.533545961316587e-17);

    /// √2
    pub const SQRT_2: Double = Double(1.4142135623730951e0, -9.667293313452916e-17);

    /// 1/√2
    pub const FRAC_1_SQRT_2: Double = Double(7.071067811865476e-1, -4.833646656726457e-17);

    /// Euler's number (e)
    pub const E: Double = Double(2.718281828459045e0, 1.44564689172925e-16);

    /// log₂ 10
    pub const LOG2_10: Double = Double(3.321928094887362e0, 1.6616175169735918e-16);

    /// log₂ e
    pub const LOG2_E: Double = Double(1.4426950408889634e0, 2.0355273740931027e-17);

    /// log 2
    pub const LOG10_2: Double = Double(3.010299956639812e-1, -2.8037281277851685e-18);

    /// log e
    pub const LOG10_E: Double = Double(4.342944819032518e-1, 1.0983196502167652e-17);

    /// ln 2
    pub const LN_2: Double = Double(6.931471805599453e-1, 2.319046813846301e-17);

    /// ln 10
    pub const LN_10: Double = Double(2.302585092994046e0, -2.1707562233822494e-16);
}

// #endregion
