// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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

#[derive(Clone, Copy, Debug)]
pub struct DoubleDouble(f64, f64);

// #region Constants

impl DoubleDouble {
    /// The radix or base of the internal representation of `DoubleDouble`. This is the same as the
    /// representation in the underlying f64.
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = 106;

    /// Approximate number of significant digits in base 10.
    pub const DIGITS: u32 = 31;

    /// [Machine epsilon] value for `DoubleDouble`.
    ///
    /// This is the difference between `1.0` and the next largest representable number.
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: f64 = 4.93038065763132e-32; // 2^-104

    /// Smallest finite `DoubleDouble` value.
    pub const MIN: DoubleDouble = DoubleDouble(-1.7976931348623157e308, -9.979201547673598e291);

    /// Smallest positive normal `DoubleDouble` value.
    pub const MIN_POSITIVE: DoubleDouble = DoubleDouble(2.2250738585072014e-308, 0e0);

    /// Largest finite `DoubleDouble` value.
    pub const MAX: DoubleDouble = DoubleDouble(1.7976931348623157e308, 9.979201547673598e291);

    /// One greater than the minimum possible normal power of 2 exponent.
    pub const MIN_EXP: i32 = -1021;

    /// Maximum possible power of 2 exponent.
    pub const MAX_EXP: i32 = 1024;

    /// Minimum possible normal power of 10 exponent.
    pub const MIN_10_EXP: i32 = -307;

    /// Maximum possible power of 10 exponent.
    pub const MAX_10_EXP: i32 = 308;

    /// Not a Number (NaN).
    pub const NAN: DoubleDouble = DoubleDouble(f64::NAN, f64::NAN);

    /// Infinity (∞).
    pub const INFINITY: DoubleDouble = DoubleDouble(f64::INFINITY, f64::INFINITY);

    /// Negative infinity (-∞).
    pub const NEG_INFINITY: DoubleDouble = DoubleDouble(f64::NEG_INFINITY, f64::NEG_INFINITY);

    /// Archimedes' constant (π)
    pub const PI: DoubleDouble = DoubleDouble(3.141592653589793e0, 1.2246467991473532e-16);

    /// π/2
    pub const FRAC_PI_2: DoubleDouble = DoubleDouble(1.5707963267948966e0, 6.123233995736766e-17);

    /// π/3
    pub const FRAC_PI_3: DoubleDouble = DoubleDouble(1.0471975511965979e0, -1.072081766451091e-16);

    /// π/4
    pub const FRAC_PI_4: DoubleDouble = DoubleDouble(7.853981633974483e-1, 3.061616997868386e-17);

    /// π/6
    pub const FRAC_PI_6: DoubleDouble = DoubleDouble(5.235987755982989e-1, -5.360408832255455e-17);

    /// π/8
    pub const FRAC_PI_8: DoubleDouble = DoubleDouble(3.9269908169872414e-1, 1.5308084989341915e-17);

    /// 1/π
    pub const FRAC_1_PI: DoubleDouble = DoubleDouble(3.183098861837907e-1, -1.9678676675182486e-17);

    /// 2/π
    pub const FRAC_2_PI: DoubleDouble = DoubleDouble(6.366197723675814e-1, -3.9357353350364984e-17);

    /// 2/√π
    pub const FRAC_2_SQRT_PI: DoubleDouble =
        DoubleDouble(1.1283791670955126e0, 1.533545961316587e-17);

    /// √2
    pub const SQRT_2: DoubleDouble = DoubleDouble(1.4142135623730951e0, -9.667293313452916e-17);

    /// 1/√2
    pub const FRAC_1_SQRT_2: DoubleDouble =
        DoubleDouble(7.071067811865476e-1, -4.833646656726457e-17);

    /// Euler's number (e)
    pub const E: DoubleDouble = DoubleDouble(2.718281828459045e0, 1.44564689172925e-16);

    /// log₂ 10
    pub const LOG2_10: DoubleDouble = DoubleDouble(3.321928094887362e0, 1.6616175169735918e-16);

    /// log₂ e
    pub const LOG2_E: DoubleDouble = DoubleDouble(1.4426950408889634e0, 2.0355273740931027e-17);

    /// log 2
    pub const LOG10_2: DoubleDouble = DoubleDouble(3.010299956639812e-1, -2.8037281277851685e-18);

    /// log e
    pub const LOG10_E: DoubleDouble = DoubleDouble(4.342944819032518e-1, 1.0983196502167652e-17);

    /// ln 2
    pub const LN_2: DoubleDouble = DoubleDouble(6.931471805599453e-1, 2.319046813846301e-17);

    /// ln 10
    pub const LN_10: DoubleDouble = DoubleDouble(2.302585092994046e0, -2.1707562233822494e-16);
}

// #endregion
