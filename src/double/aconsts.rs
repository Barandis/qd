// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::f64;

impl Double {
    /// The radix or base of the internal representation of `Double`. This is the same as
    /// the representation in the underlying f64.
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
    pub const EPSILON: Double = Double(4.93038065763132e-32, 0.0); // 2^-104

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
    pub const NAN: Double = Double(f64::NAN, 0.0);

    /// Infinity (∞).
    pub const INFINITY: Double = Double(f64::INFINITY, 0.0);

    /// Negative infinity (-∞).
    pub const NEG_INFINITY: Double = Double(f64::NEG_INFINITY, 0.0);

    /// Zero (0)
    pub const ZERO: Double = Double(0.0, 0.0);

    /// Negative zero (-0)
    pub const NEG_ZERO: Double = Double(-0.0, 0.0);

    /// One (1)
    pub const ONE: Double = Double(1.0, 0.0);

    /// Negative one (-1)
    pub const NEG_ONE: Double = Double(-1.0, 0.0);

    /// Archimedes' constant (π)
    pub const PI: Double = Double(3.141592653589793e0, 1.2246467991473532e-16);

    /// The full circle constant (τ), or 2π
    pub const TAU: Double = Double(6.283185307179586e0, 2.449293598294704e-16);

    /// π/2
    pub const FRAC_PI_2: Double = Double(1.5707963267948966e0, 6.12323399573676e-17);

    /// π/3
    pub const FRAC_PI_3: Double = Double(1.0471975511965979e0, -1.0720817664510912e-16);

    /// π/4
    pub const FRAC_PI_4: Double = Double(7.853981633974483e-1, 3.0616169978683836e-17);

    /// π/6
    pub const FRAC_PI_6: Double = Double(5.235987755982989e-1, -5.360408832255455e-17);

    /// π/8
    pub const FRAC_PI_8: Double = Double(3.9269908169872414e-1, 1.5308084989341906e-17);

    /// π/16
    pub const FRAC_PI_16: Double = Double(1.9634954084936207e-1, 7.654042494670953e-18);

    /// 3π/2
    pub const FRAC_3_PI_2: Double = Double(4.71238898038469e0, 1.8369701987210292e-16);

    /// 3π/4
    pub const FRAC_3_PI_4: Double = Double(2.356194490192345e0, 9.184850993605146e-17);

    /// 5π/4
    pub const FRAC_5_PI_4: Double = Double(3.9269908169872414e0, 1.5308084989341908e-16);

    /// 7π/4
    pub const FRAC_7_PI_4: Double = Double(5.497787143782138e0, 2.143131898507869e-16);

    /// 1/π
    pub const FRAC_1_PI: Double = Double(3.183098861837907e-1, -1.967867667518248e-17);

    /// 2/π
    pub const FRAC_2_PI: Double = Double(6.366197723675814e-1, -3.935735335036497e-17);

    /// 2/√π
    pub const FRAC_2_SQRT_PI: Double = Double(1.1283791670955126e0, 1.533545961316588e-17);

    /// √2
    pub const SQRT_2: Double = Double(1.4142135623730951e0, -9.667293313452915e-17);

    /// 1/√2
    pub const FRAC_1_SQRT_2: Double = Double(7.071067811865476e-1, -4.833646656726457e-17);

    /// Euler's number (*e*)
    pub const E: Double = Double(2.718281828459045e0, 1.4456468917292497e-16);

    /// log<sub>2</sub> 10
    pub const LOG2_10: Double = Double(3.321928094887362e0, 1.6616175169735918e-16);

    /// log<sub>2</sub> *e*
    pub const LOG2_E: Double = Double(1.4426950408889634e0, 2.035527374093102e-17);

    /// log<sub>10</sub> 2
    pub const LOG10_2: Double = Double(3.010299956639812e-1, -2.8037281277851654e-18);

    /// log<sub>10</sub> *e*
    pub const LOG10_E: Double = Double(4.342944819032518e-1, 1.0983196502167654e-17);

    /// log<sub>*e*</sub> 2
    pub const LN_2: Double = Double(6.931471805599453e-1, 2.319046813846301e-17);

    /// log<sub>*e*</sub> 10
    pub const LN_10: Double = Double(2.302585092994046e0, -2.1707562233822496e-16);
}
