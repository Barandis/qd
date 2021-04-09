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

    /// Infinity ($\infin$).
    pub const INFINITY: Double = Double(f64::INFINITY, 0.0);

    /// Negative infinity ($-\infin$).
    pub const NEG_INFINITY: Double = Double(f64::NEG_INFINITY, 0.0);

    /// Zero ($0$).
    pub const ZERO: Double = Double(0.0, 0.0);

    /// Negative zero ($-0$).
    pub const NEG_ZERO: Double = Double(-0.0, 0.0);

    /// One ($1$).
    pub const ONE: Double = Double(1.0, 0.0);

    /// Negative one ($-1$).
    pub const NEG_ONE: Double = Double(-1.0, 0.0);

    /// Archimedes' constant ($\pi$). The value of this constant is
    /// 3.141592653589793238462643383280.
    pub const PI: Double = Double(3.141592653589793e0, 1.2246467991473532e-16);

    /// The full circle constant ($\tau$), or $2\pi$. The value of this constant is
    /// 6.283185307179586476925286766559.
    pub const TAU: Double = Double(6.283185307179586e0, 2.449293598294704e-16);

    /// The constant $\pi/2$. The value of this constant is
    /// 1.570796326794896619231321691640.
    pub const FRAC_PI_2: Double = Double(1.5707963267948966e0, 6.12323399573676e-17);

    /// The constant $\pi/3$. The value of this constant is
    /// 1.047197551196597746154214461093.
    pub const FRAC_PI_3: Double = Double(1.0471975511965979e0, -1.0720817664510912e-16);

    /// The constant $\pi/4$. The value of this constant is
    /// 0.7853981633974483096156608458199.
    pub const FRAC_PI_4: Double = Double(7.853981633974483e-1, 3.0616169978683836e-17);

    /// The constant $\pi/6$. The value of this constant is
    /// 0.5235987755982988730771072305466.
    pub const FRAC_PI_6: Double = Double(5.235987755982989e-1, -5.360408832255455e-17);

    /// The constant $\pi/8$. The value of this constant is
    /// 0.3926990816987241548078304229099.
    pub const FRAC_PI_8: Double = Double(3.9269908169872414e-1, 1.5308084989341906e-17);

    /// The constant $\pi/16$. The value of this constant is
    /// 0.1963495408493620774039152114550.
    pub const FRAC_PI_16: Double = Double(1.9634954084936207e-1, 7.654042494670953e-18);

    /// The constant $3\pi/2$. The value of this constant is
    /// 4.712388980384689857693965074919.
    pub const FRAC_3_PI_2: Double = Double(4.71238898038469e0, 1.8369701987210292e-16);

    /// The constant $3\pi/4$. The value of this constant is
    /// 2.356194490192344928846982537460.
    pub const FRAC_3_PI_4: Double = Double(2.356194490192345e0, 9.184850993605146e-17);

    /// The constant $5\pi/4$. The value of this constant is
    /// 3.926990816987241548078304229099.
    pub const FRAC_5_PI_4: Double = Double(3.9269908169872414e0, 1.5308084989341908e-16);

    /// The constant $7\pi/4$. The value of this constant is
    /// 5.497787143782138167309625920739.
    pub const FRAC_7_PI_4: Double = Double(5.497787143782138e0, 2.143131898507869e-16);

    /// The constant $1/\pi$. The value of this constant is
    /// 0.3183098861837906715377675267450.
    pub const FRAC_1_PI: Double = Double(3.183098861837907e-1, -1.967867667518248e-17);

    /// The constant $2/\pi$. The value of this constant is
    /// 0.6366197723675813430755350534901.
    pub const FRAC_2_PI: Double = Double(6.366197723675814e-1, -3.935735335036497e-17);

    /// The constant $2/\sqrt{\pi}$. The value of this constant is
    /// 1.128379167095512573896158903122.
    pub const FRAC_2_SQRT_PI: Double = Double(1.1283791670955126e0, 1.533545961316588e-17);

    /// The constant $\sqrt{2}$. The value of this constant is
    /// 1.414213562373095048801688724210.
    pub const SQRT_2: Double = Double(1.4142135623730951e0, -9.667293313452915e-17);

    /// The constant $1/\sqrt{2}$. The value of this constant is
    /// 0.7071067811865475244008443621048.
    pub const FRAC_1_SQRT_2: Double = Double(7.071067811865476e-1, -4.833646656726457e-17);

    /// Euler's number ($e$). The value of this constant is
    /// 2.718281828459045235360287471353.
    pub const E: Double = Double(2.718281828459045e0, 1.4456468917292497e-16);

    /// The constant $\log_2 10$. The value of this constant is
    /// 3.321928094887362347870319429489.
    pub const LOG2_10: Double = Double(3.321928094887362e0, 1.6616175169735918e-16);

    /// The constant $\log_2 e$. The value of this constant is
    /// 1.442695040888963407359924681002.
    pub const LOG2_E: Double = Double(1.4426950408889634e0, 2.035527374093102e-17);

    /// The constant $\log_{10} 2$. The value of this constant is
    /// 0.3010299956639811952137388947245.
    pub const LOG10_2: Double = Double(3.010299956639812e-1, -2.8037281277851654e-18);

    /// The constant $\log_{10} e$. The value of this constant is
    /// 0.4342944819032518276511289189166.
    pub const LOG10_E: Double = Double(4.342944819032518e-1, 1.0983196502167654e-17);

    /// The constant $\ln 2$. The value of this constant is
    /// 0.6931471805599453094172321214582.
    pub const LN_2: Double = Double(6.931471805599453e-1, 2.319046813846301e-17);

    /// The constant $\ln 10$. The value of this constant is
    /// 2.302585092994045684017991454684.
    pub const LN_10: Double = Double(2.302585092994046e0, -2.1707562233822496e-16);
}
