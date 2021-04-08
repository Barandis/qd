// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

/// Helper function that efficiently multiplies a Double by a power of 2. This is -much-
/// faster than regular multiplication but only works with powers of 2.
#[inline]
pub fn mul_pwr2(a: Double, b: f64) -> Double {
    Double(a.0 * b, a.1 * b)
}

// pub fn agm(x: Double, y: Double, eps: Double) -> Double {
//     let mut a = x;
//     let mut g = y;

//     while (a - g).abs() > eps {
//         let an = mul_pwr2(a + g, 0.5); // arithmetic mean
//         let gn = (a * g).sqrt(); //       geometric mean
//         a = an;
//         g = gn;
//     }
//     a
// }

/// Precomputed reciprocals of numbers up to 30. This starts with 1/3, as the reciprocals
/// before that (1 and 1/2) are trivial. These are used in the Maclaurin series calculations
/// for ln1p.
pub const INV_INTS: [Double; 28] = [
    Double(3.333333333333333e-1, 1.850371707708594e-17),
    Double(2.5e-1, 0e0),
    Double(2e-1, -1.1102230246251566e-17),
    Double(1.6666666666666666e-1, 9.25185853854297e-18),
    Double(1.4285714285714285e-1, 7.93016446160826e-18),
    Double(1.25e-1, 0e0),
    Double(1.111111111111111e-1, 6.1679056923619804e-18),
    Double(1e-1, -5.551115123125783e-18),
    Double(9.090909090909091e-2, -2.523234146875356e-18),
    Double(8.333333333333333e-2, 4.625929269271485e-18),
    Double(7.692307692307693e-2, -4.270088556250602e-18),
    Double(7.142857142857142e-2, 3.96508223080413e-18),
    Double(6.666666666666667e-2, 9.251858538542971e-19),
    Double(6.25e-2, 0e0),
    Double(5.8823529411764705e-2, 8.163404592832033e-19),
    Double(5.555555555555555e-2, 3.0839528461809902e-18),
    Double(5.263157894736842e-2, 2.921639538487254e-18),
    Double(5e-2, -2.7755575615628915e-18),
    Double(4.7619047619047616e-2, 2.64338815386942e-18),
    Double(4.5454545454545456e-2, -1.261617073437678e-18),
    Double(4.3478260869565216e-2, 1.206764157201257e-18),
    Double(4.1666666666666664e-2, 2.3129646346357427e-18),
    Double(4e-2, -8.326672684688674e-19),
    Double(3.8461538461538464e-2, -2.135044278125301e-18),
    Double(3.7037037037037035e-2, 2.05596856412066e-18),
    Double(3.571428571428571e-2, 1.982541115402065e-18),
    Double(3.4482758620689655e-2, 4.785444071660157e-19),
    Double(3.333333333333333e-2, 4.625929269271486e-19),
];

/// Table of the reciprocals of factorials. This starts with 1/3!, as the inverse factorials
/// before that are trivial (1/1! is 1 and 1/2! is 1/2). These are used in Maclaurin series
/// calculations for exp, sin, and cos.
pub const INV_FACTS: [Double; 28] = [
    Double(1.6666666666666666e-1, 9.25185853854297e-18),
    Double(4.1666666666666664e-2, 2.3129646346357427e-18),
    Double(8.333333333333333e-3, 1.1564823173178714e-19),
    Double(1.388888888888889e-3, -5.300543954373577e-20),
    Double(1.984126984126984e-4, 1.7209558293420705e-22),
    Double(2.48015873015873e-5, 2.1511947866775882e-23),
    Double(2.7557319223985893e-6, -1.858393274046472e-22),
    Double(2.755731922398589e-7, 2.3767714622250297e-23),
    Double(2.505210838544172e-8, -1.448814070935912e-24),
    Double(2.08767569878681e-9, -1.20734505911326e-25),
    Double(1.6059043836821613e-10, 1.2585294588752098e-26),
    Double(1.1470745597729725e-11, 2.0655512752830745e-28),
    Double(7.647163731819816e-13, 7.03872877733453e-30),
    Double(4.779477332387385e-14, 4.399205485834081e-31),
    Double(2.8114572543455206e-15, 1.6508842730861433e-31),
    Double(1.5619206968586225e-16, 1.1910679660273754e-32),
    Double(8.22063524662433e-18, 2.2141894119604265e-34),
    Double(4.110317623312165e-19, 1.4412973378659527e-36),
    Double(1.9572941063391263e-20, -1.3643503830087908e-36),
    Double(8.896791392450574e-22, -7.911402614872376e-38),
    Double(3.868170170630684e-23, -8.843177655482339e-40),
    Double(1.6117375710961184e-24, -3.6846573564509766e-41),
    Double(6.446950284384474e-26, -1.9330404233703468e-42),
    Double(2.4795962632247976e-27, -1.2953730964765229e-43),
    Double(9.183689863795546e-29, 1.430315039678732e-45),
    Double(3.279889237069838e-30, 1.5117542744029879e-46),
    Double(1.1309962886447716e-31, 1.0498015412959505e-47),
    Double(3.7699876288159054e-33, 2.5870347832750324e-49),
];

/// Table of sines of kπ/16, where k is in [1, 4].
pub const SINES: [Double; 4] = [
    Double(1.9509032201612828e-1, -7.991079068461734e-18),
    Double(3.826834323650898e-1, -1.005077269646159e-17),
    Double(5.555702330196022e-1, 4.7094109405616756e-17),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];

/// Table of cosines of kπ/16, where k is in [1, 4].
pub const COSINES: [Double; 4] = [
    Double(9.807852804032304e-1, 1.8546939997824996e-17),
    Double(9.238795325112867e-1, 1.764504708433667e-17),
    Double(8.314696123025452e-1, 1.4073856984728008e-18),
    Double(7.071067811865476e-1, -4.8336466567264573e-17),
];
