// Copyright (c) 2019 Thomas Otterson
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
    /// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon 2^-209
    pub const EPSILON: Quad = Quad(
        1.215_432_671_457_254_2e-63,
        4.226_184_419_490_203_5e-129,
        0e0,
        0e0,
    );

    /// Smallest finite `Quad` value.
    pub const MIN: Quad = Quad(
        -1.797_693_134_862_315_7e308,
        -9.979_201_547_673_598e291,
        -5.539_569_662_801_112_6e275,
        -3.075_078_893_078_405e259,
    );

    /// Smallest positive normal `Quad` value.
    pub const MIN_POSITIVE: Quad = Quad(1.625_974_543_695_232_3e-260, 0e0, 0e0, 0e0);

    /// Largest finite `Quad` value.
    pub const MAX: Quad = Quad(
        1.797_693_134_862_315_7e308,
        9.979_201_547_673_598e291,
        5.539_569_662_801_112_6e275,
        3.075_078_893_078_405e259,
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

    /// Archimedes' constant (π)
    pub const PI: Quad = Quad(
        3.141_592_653_589_793e0,
        1.224_646_799_147_353_2e-16,
        -2.994_769_809_718_339_7e-33,
        1.112_454_220_863_365_5e-49,
    );

    /// 2π
    pub const MUL_2_PI: Quad = Quad(
        6.283_185_307_179_586e0,
        2.449_293_598_294_706_4e-16,
        -5.989_539_619_436_679e-33,
        2.224_908_441_726_731e-49,
    );

    /// π/2
    pub const FRAC_PI_2: Quad = Quad(
        1.570_796_326_794_896_6e0,
        6.123_233_995_736_766e-17,
        -1.497_384_904_859_169_8e-33,
        5.562_271_104_316_827e-50,
    );

    /// π/3
    pub const FRAC_PI_3: Quad = Quad(
        1.047_197_551_196_597_9e0,
        -1.072_081_766_451_091e-16,
        -9.982_566_032_394_464e-34,
        -7.695_615_360_182_150_5e-50,
    );

    /// π/4
    pub const FRAC_PI_4: Quad = Quad(
        7.853_981_633_974_483e-1,
        3.061_616_997_868_383e-17,
        -7.486_924_524_295_849e-34,
        2.781_135_552_158_414e-50,
    );

    /// π/6
    pub const FRAC_PI_6: Quad = Quad(
        5.235_987_755_982_989e-1,
        -5.360_408_832_255_455e-17,
        -4.991_283_016_197_232e-34,
        -3.847_807_680_091_074_3e-50,
    );

    /// π/8
    pub const FRAC_PI_8: Quad = Quad(
        3.926_990_816_987_241_4e-1,
        1.530_808_498_934_191_5e-17,
        -3.743_462_262_147_924_6e-34,
        1.390_567_776_079_207_8e-50,
    );

    /// π/16
    pub const FRAC_PI_16: Quad = Quad(
        1.963_495_408_493_620_7e-1,
        7.654_042_494_670_958e-18,
        -1.871_731_131_073_962_3e-34,
        6.952_838_880_396_035e-51,
    );

    /// 3π/4
    pub const FRAC_3_PI_4: Quad = Quad(
        2.356_194_490_192_345e0,
        9.184_850_993_605_148e-17,
        3.916_898_464_750_4e-33,
        -2.586_798_163_270_486_4e-49,
    );

    /// 1/π
    pub const FRAC_1_PI: Quad = Quad(
        3.183_098_861_837_907e-1,
        -1.967_867_667_518_248_6e-17,
        -1.072_143_628_289_300_4e-33,
        8.053_563_926_594_113e-50,
    );

    /// 2/π
    pub const FRAC_2_PI: Quad = Quad(
        6.366_197_723_675_814e-1,
        -3.935_735_335_036_497e-17,
        -2.144_287_256_578_600_8e-33,
        1.610_712_785_318_822_4e-49,
    );

    /// 2/√π
    pub const FRAC_2_SQRT_PI: Quad = Quad(
        1.128_379_167_095_512_6e0,
        1.533_545_961_316_588e-17,
        -4.765_684_596_693_686e-34,
        -2.007_794_661_655_262_5e-50,
    );

    /// √2
    pub const SQRT_2: Quad = Quad(
        1.414_213_562_373_095_1e0,
        -9.667_293_313_452_913e-17,
        4.138_675_308_699_413_6e-33,
        4.935_546_991_468_353e-50,
    );

    /// 1/√2
    pub const FRAC_1_SQRT_2: Quad = Quad(
        7.071_067_811_865_476e-1,
        -4.833_646_656_726_457e-17,
        2.069_337_654_349_706_8e-33,
        2.467_773_495_734_178_3e-50,
    );

    /// Euler's number (e)
    pub const E: Quad = Quad(
        2.718_281_828_459_045e0,
        1.445_646_891_729_250_2e-16,
        -2.127_717_108_038_176_8e-33,
        1.515_630_159_841_219_3e-49,
    );

    /// log₂ 10
    pub const LOG2_10: Quad = Quad(
        3.321_928_094_887_362e0,
        1.661_617_516_973_592e-16,
        1.221_551_217_845_818_1e-32,
        5.955_118_970_278_25e-49,
    );

    /// log₂ e
    pub const LOG2_E: Quad = Quad(
        1.442_695_040_888_963_4e0,
        2.035_527_374_093_103_3e-17,
        -1.061_465_995_611_725_8e-33,
        -1.383_671_678_018_14e-50,
    );

    /// log 2
    pub const LOG10_2: Quad = Quad(
        3.010_299_956_639_812e-1,
        -2.803_728_127_785_170_4e-18,
        5.471_948_402_314_639e-35,
        5.105_138_983_107_098_4e-51,
    );

    /// log e
    pub const LOG10_E: Quad = Quad(
        4.342_944_819_032_518e-1,
        1.098_319_650_216_765e-17,
        3.717_181_233_110_959e-34,
        7.734_484_346_504_308e-51,
    );

    /// ln 2
    pub const LN_2: Quad = Quad(
        6.931_471_805_599_453e-1,
        2.319_046_813_846_299_6e-17,
        5.707_708_438_416_212e-34,
        -3.582_432_210_601_810_5e-50,
    );

    /// ln 10
    pub const LN_10: Quad = Quad(
        2.302_585_092_994_046e0,
        -2.170_756_223_382_249_4e-16,
        -9.984_262_454_465_777e-33,
        -4.023_357_454_450_205_6e-49,
    );
}
