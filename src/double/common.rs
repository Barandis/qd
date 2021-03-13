// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

/// Reciprocals of factorials, rendered as Doubles. These are used in Taylor series
/// calculations.
pub const INV_FACTS: [Double; 15] = [
    Double(1.666_666_666_666_666_6e-1, 9.251_858_538_542_97e-18),
    Double(4.166_666_666_666_666_4e-2, 2.312_964_634_635_742_7e-18),
    Double(8.333_333_333_333_333e-3, 1.156_482_317_317_871_4e-19),
    Double(1.388_888_888_888_889e-3, -5.300_543_954_373_577e-20),
    Double(1.984_126_984_126_984e-4, 1.720_955_829_342_070_5e-22),
    Double(2.480_158_730_158_73e-5, 2.151_194_786_677_588_2e-23),
    Double(2.755_731_922_398_589_3e-6, -1.858_393_274_046_472e-22),
    Double(2.755_731_922_398_589e-7, 2.376_771_462_225_029_7e-23),
    Double(2.505_210_838_544_172e-8, -1.448_814_070_935_912e-24),
    Double(2.087_675_698_786_81e-9, -1.207_345_059_113_26e-25),
    Double(1.605_904_383_682_161_3e-10, 1.258_529_458_875_209_8e-26),
    Double(1.147_074_559_772_972_5e-11, 2.065_551_275_283_074_5e-28),
    Double(7.647_163_731_819_816e-13, 7.038_728_777_334_53e-30),
    Double(4.779_477_332_387_385e-14, 4.399_205_485_834_081e-31),
    Double(2.811_457_254_345_520_6e-15, 1.650_884_273_086_143_3e-31),
];

/// Helper function that efficiently multiplies a Double by a power of 2. This is -much-
/// faster than regular multiplication but only works with powers of 2.
#[inline]
pub fn mul_pwr2(a: Double, b: f64) -> Double {
    Double(a.0 * b, a.1 * b)
}
