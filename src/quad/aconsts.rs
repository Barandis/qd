// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::f64;

impl Quad {
    /// The radix or base of the internal representation of `Double`. This is the same as
    /// the representation in the underlying f64.
    pub const RADIX: u32 = 2;

    /// Number of significant digits in base 2.
    pub const MANTISSA_DIGITS: u32 = 212;

    /// Approximate number of significant digits in base 10.
    pub const DIGITS: u32 = 62;

    /// [Machine epsilon] value for `Quad`.
    ///
    /// This is the difference between `1.0` and the next largest representable number.
    ///
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
    pub const EPSILON: Quad = Quad(1.2154326714572542e-63, 4.2261844194902035e-129, 0e0, 0e0);

    /// Smallest finite `Quad` value.
    pub const MIN: Quad = Quad(
        -1.7976931348623157e308,
        -9.979201547673598e291,
        -5.5395696628011126e275,
        -3.075078893078405e259,
    );

    /// Smallest positive normal `Quad` value.
    pub const MIN_POSITIVE: Quad = Quad(1.6259745436952323e-260, 0e0, 0e0, 0e0);

    /// Largest finite `Quad` value.
    pub const MAX: Quad = Quad(
        1.7976931348623157e308,
        9.979201547673598e291,
        5.5395696628011126e275,
        3.075078893078405e259,
    );

    /// One greater than the minimum possible normal power of 2 exponent.
    pub const MIN_EXP: i32 = -1021;

    /// Maximum possible power of 2 exponent.
    pub const MAX_EXP: i32 = 1024;

    /// Minimum possible normal power of 10 exponent.
    pub const MIN_10_EXP: i32 = -307;

    /// Maximum possible power of 10 exponent.
    pub const MAX_10_EXP: i32 = 308;

    /// Not a Number (NaN).
    pub const NAN: Quad = Quad(f64::NAN, 0.0, 0.0, 0.0);

    /// Infinity (∞).
    pub const INFINITY: Quad = Quad(f64::INFINITY, 0.0, 0.0, 0.0);

    /// Negative infinity (-∞).
    pub const NEG_INFINITY: Quad = Quad(f64::NEG_INFINITY, 0.0, 0.0, 0.0);

    /// Zero (0)
    pub const ZERO: Quad = Quad(0.0, 0.0, 0.0, 0.0);

    /// Negative zero (-0)
    pub const NEG_ZERO: Quad = Quad(-0.0, 0.0, 0.0, 0.0);

    /// One (1)
    pub const ONE: Quad = Quad(1.0, 0.0, 0.0, 0.0);

    /// Negative one (-1)
    pub const NEG_ONE: Quad = Quad(-1.0, 0.0, 0.0, 0.0);

    /// Archimedes' constant (π)
    pub const PI: Quad = Quad(
        3.141592653589793e0,
        1.2246467991473532e-16,
        -2.9947698097183397e-33,
        1.1124542208633655e-49,
    );

    /// The full circle constant (τ), or 2π
    pub const TAU: Quad = Quad(
        6.283185307179586e0,
        2.4492935982947064e-16,
        -5.989539619436679e-33,
        2.224908441726731e-49,
    );

    /// π/2
    pub const FRAC_PI_2: Quad = Quad(
        1.5707963267948966e0,
        6.123233995736766e-17,
        -1.4973849048591698e-33,
        5.562271104316827e-50,
    );

    /// π/3
    pub const FRAC_PI_3: Quad = Quad(
        1.0471975511965979e0,
        -1.072081766451091e-16,
        -9.982566032394464e-34,
        -7.69561536018215e-50,
    );

    /// π/4
    pub const FRAC_PI_4: Quad = Quad(
        7.853981633974483e-1,
        3.061616997868383e-17,
        -7.486924524295849e-34,
        2.781135552158413e-50,
    );

    /// π/6
    pub const FRAC_PI_6: Quad = Quad(
        5.235987755982989e-1,
        -5.360408832255455e-17,
        -4.991283016197232e-34,
        -3.847807680091076e-50,
    );

    /// π/8
    pub const FRAC_PI_8: Quad = Quad(
        3.9269908169872414e-1,
        1.5308084989341915e-17,
        -3.7434622621479246e-34,
        1.3905677760792066e-50,
    );

    /// π/16
    pub const FRAC_PI_16: Quad = Quad(
        1.9634954084936207e-1,
        7.654042494670958e-18,
        -1.8717311310739623e-34,
        6.952838880396033e-51,
    );

    /// 3π/2
    pub const FRAC_3_PI_2: Quad = Quad(
        4.71238898038469e0,
        1.8369701987210297e-16,
        7.8337969295008e-33,
        -5.173596326540972e-49,
    );

    /// 3π/4
    pub const FRAC_3_PI_4: Quad = Quad(
        2.356194490192345e0,
        9.184850993605148e-17,
        3.9168984647504e-33,
        -2.586798163270486e-49,
    );

    /// 5π/4
    pub const FRAC_5_PI_4: Quad = Quad(
        3.9269908169872414e0,
        1.5308084989341916e-16,
        -9.90643808418708e-33,
        4.811706604997218e-49,
    );

    /// 7π/4
    pub const FRAC_7_PI_4: Quad = Quad(
        5.497787143782138e0,
        2.143131898507868e-16,
        9.221286550320605e-34,
        2.3622547205188384e-50,
    );

    /// 1/π
    pub const FRAC_1_PI: Quad = Quad(
        3.183098861837907e-1,
        -1.9678676675182486e-17,
        -1.0721436282893004e-33,
        8.053563926594112e-50,
    );

    /// 2/π
    pub const FRAC_2_PI: Quad = Quad(
        6.366197723675814e-1,
        -3.935735335036497e-17,
        -2.1442872565786008e-33,
        1.6107127853188224e-49,
    );

    /// 2/√π
    pub const FRAC_2_SQRT_PI: Quad = Quad(
        1.1283791670955126e0,
        1.533545961316588e-17,
        -4.765684596693686e-34,
        -2.0077946616552623e-50,
    );

    /// √2
    pub const SQRT_2: Quad = Quad(
        1.4142135623730951e0,
        -9.667293313452913e-17,
        4.1386753086994136e-33,
        4.935546991468354e-50,
    );

    /// 1/√2
    pub const FRAC_1_SQRT_2: Quad = Quad(
        7.071067811865476e-1,
        -4.833646656726457e-17,
        2.0693376543497068e-33,
        2.467773495734177e-50,
    );

    /// Euler's number (*e*)
    pub const E: Quad = Quad(
        2.718281828459045e0,
        1.4456468917292502e-16,
        -2.1277171080381768e-33,
        1.5156301598412193e-49,
    );

    /// log<sub>2</sub> 10
    pub const LOG2_10: Quad = Quad(
        3.321928094887362e0,
        1.661617516973592e-16,
        1.2215512178458181e-32,
        5.95511897027825e-49,
    );

    /// log<sub>2</sub> *e*
    pub const LOG2_E: Quad = Quad(
        1.4426950408889634e0,
        2.0355273740931033e-17,
        -1.0614659956117258e-33,
        -1.3836716780181397e-50,
    );

    /// log<sub>10</sub> 2
    pub const LOG10_2: Quad = Quad(
        3.010299956639812e-1,
        -2.8037281277851704e-18,
        5.471948402314639e-35,
        5.105138983107093e-51,
    );

    /// log<sub>10</sub> *e*
    pub const LOG10_E: Quad = Quad(
        4.342944819032518e-1,
        1.098319650216765e-17,
        3.717181233110959e-34,
        7.73448434650429e-51,
    );

    /// log<sub>*e*</sub> 2
    pub const LN_2: Quad = Quad(
        6.931471805599453e-1,
        2.3190468138462996e-17,
        5.707708438416212e-34,
        -3.582432210601812e-50,
    );

    /// log<sub>*e*</sub> 10
    pub const LN_10: Quad = Quad(
        2.302585092994046e0,
        -2.1707562233822494e-16,
        -9.984262454465777e-33,
        -4.023357454450206e-49,
    );
}
