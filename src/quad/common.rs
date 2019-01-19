// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

pub const INV_FACTS: [Quad; 15] = [
    Quad(
        1.6666666666666666e-1,
        9.25185853854297e-18,
        5.135813185032629e-34,
        2.8509490240983423e-50,
    ),
    Quad(
        4.1666666666666664e-2,
        2.3129646346357427e-18,
        1.2839532962581572e-34,
        7.127372560245855e-51,
    ),
    Quad(
        8.333333333333333e-3,
        1.1564823173178714e-19,
        1.6049416203226965e-36,
        2.22730392507682e-53,
    ),
    Quad(
        1.388888888888889e-3,
        -5.300543954373577e-20,
        -1.7386867553495878e-36,
        -1.6333562117230084e-52,
    ),
    Quad(
        1.984126984126984e-4,
        1.7209558293420705e-22,
        1.4926912391394127e-40,
        1.2947032674766573e-58,
    ),
    Quad(
        2.48015873015873e-5,
        2.1511947866775882e-23,
        1.865864048924266e-41,
        1.6183790843843425e-59,
    ),
    Quad(
        2.7557319223985893e-6,
        -1.858393274046472e-22,
        8.491754604881993e-39,
        -5.726616407894298e-55,
    ),
    Quad(
        2.755731922398589e-7,
        2.3767714622250297e-23,
        -3.263188903340883e-40,
        1.6143511186040432e-56,
    ),
    Quad(
        2.505210838544172e-8,
        -1.448814070935912e-24,
        2.0426735146714455e-41,
        -8.49632672007163e-58,
    ),
    Quad(
        2.08767569878681e-9,
        -1.20734505911326e-25,
        1.702227928892871e-42,
        1.4160953215039672e-58,
    ),
    Quad(
        1.6059043836821613e-10,
        1.2585294588752098e-26,
        -5.31334602762985e-43,
        3.540214725976055e-59,
    ),
    Quad(
        1.1470745597729725e-11,
        2.0655512752830745e-28,
        6.889079232466646e-45,
        5.7292000265510916e-61,
    ),
    Quad(
        7.647163731819816e-13,
        7.03872877733453e-30,
        -7.827539277162583e-48,
        1.9213864944378356e-64,
    ),
    Quad(
        4.779477332387385e-14,
        4.399205485834081e-31,
        -4.892212048226615e-49,
        1.2008665590236055e-65,
    ),
    Quad(
        2.8114572543455206e-15,
        1.6508842730861433e-31,
        -2.877771793074479e-50,
        4.271106892562081e-67,
    ),
];

/// Helper function that efficiently multiplies a Quad by a power of 2. This is -much- faster than
/// regular multiplication but only works with powers of 2.
#[inline]
pub fn mul_pwr2(a: Quad, b: f64) -> Quad {
    Quad(a.0 * b, a.1 * b, a.2 * b, a.3 * b)
}