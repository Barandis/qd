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
    pub const EPSILON: Double = Double(4.930_380_657_631_32e-32, 0.0); // 2^-104

    /// Smallest finite `Double` value.
    pub const MIN: Double = Double(-1.797_693_134_862_315_7e308, -9.979_201_547_673_598e291);

    /// Smallest positive normal `Double` value.
    pub const MIN_POSITIVE: Double = Double(2.225_073_858_507_201_4e-308, 0e0);

    /// Largest finite `Double` value.
    pub const MAX: Double = Double(1.797_693_134_862_315_7e308, 9.979_201_547_673_598e291);

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
    pub const PI: Double = Double(3.141_592_653_589_793e0, 1.224_646_799_147_353_2e-16);

    /// 2π
    pub const MUL_2_PI: Double = Double(6.283_185_307_179_586e0, 2.449_293_598_294_706e-16);

    /// π/2
    pub const FRAC_PI_2: Double = Double(1.570_796_326_794_896_6e0, 6.123_233_995_736_766e-17);

    /// π/3
    pub const FRAC_PI_3: Double = Double(1.047_197_551_196_597_9e0, -1.072_081_766_451_091e-16);

    /// π/4
    pub const FRAC_PI_4: Double = Double(7.853_981_633_974_483e-1, 3.061_616_997_868_386e-17);

    /// π/6
    pub const FRAC_PI_6: Double = Double(5.235_987_755_982_989e-1, -5.360_408_832_255_455e-17);

    /// π/8
    pub const FRAC_PI_8: Double = Double(3.926_990_816_987_241_4e-1, 1.530_808_498_934_191_5e-17);

    /// π/16
    pub const FRAC_PI_16: Double = Double(1.963_495_408_493_620_7e-1, 7.654_042_494_670_959e-18);

    /// 3π/4
    pub const FRAC_3_PI_4: Double = Double(2.356_194_490_192_345e0, 9.184_850_993_605_146e-17);

    /// 1/π
    pub const FRAC_1_PI: Double = Double(3.183_098_861_837_907e-1, -1.967_867_667_518_248_6e-17);

    /// 2/π
    pub const FRAC_2_PI: Double = Double(6.366_197_723_675_814e-1, -3.935_735_335_036_498_4e-17);

    /// 2/√π
    pub const FRAC_2_SQRT_PI: Double = Double(1.128_379_167_095_512_6e0, 1.533_545_961_316_587e-17);

    /// √2
    pub const SQRT_2: Double = Double(1.414_213_562_373_095_1e0, -9.667_293_313_452_916e-17);

    /// 1/√2
    pub const FRAC_1_SQRT_2: Double = Double(7.071_067_811_865_476e-1, -4.833_646_656_726_457e-17);

    /// Euler's number (e)
    pub const E: Double = Double(2.718_281_828_459_045e0, 1.445_646_891_729_25e-16);

    /// log₂ 10
    pub const LOG2_10: Double = Double(3.321_928_094_887_362e0, 1.661_617_516_973_591_8e-16);

    /// log₂ e
    pub const LOG2_E: Double = Double(1.442_695_040_888_963_4e0, 2.035_527_374_093_102_7e-17);

    /// log 2
    pub const LOG10_2: Double = Double(3.010_299_956_639_812e-1, -2.803_728_127_785_168_5e-18);

    /// log e
    pub const LOG10_E: Double = Double(4.342_944_819_032_518e-1, 1.098_319_650_216_765_2e-17);

    /// ln 2
    pub const LN_2: Double = Double(6.931_471_805_599_453e-1, 2.319_046_813_846_301e-17);

    /// ln 10
    pub const LN_10: Double = Double(2.302_585_092_994_046e0, -2.170_756_223_382_249_4e-16);
}
