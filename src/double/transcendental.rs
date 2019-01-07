// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::DoubleDouble;

// #region Exponential

/// Reciprocals of factorials, rendered as DoubleDoubles. These are used in the Taylor series for
/// calculating the exponentiation function.
const INV_FACT: [DoubleDouble; 15] = [
    DoubleDouble(1.6666666666666666e-1, 9.25185853854297e-18),
    DoubleDouble(4.1666666666666664e-2, 2.3129646346357427e-18),
    DoubleDouble(8.333333333333333e-3, 1.1564823173178714e-19),
    DoubleDouble(1.388888888888889e-3, -5.300543954373577e-20),
    DoubleDouble(1.984126984126984e-4, 1.7209558293420705e-22),
    DoubleDouble(2.48015873015873e-5, 2.1511947866775882e-23),
    DoubleDouble(2.7557319223985893e-6, -1.858393274046472e-22),
    DoubleDouble(2.755731922398589e-7, 2.3767714622250297e-23),
    DoubleDouble(2.505210838544172e-8, -1.448814070935912e-24),
    DoubleDouble(2.08767569878681e-9, -1.20734505911326e-25),
    DoubleDouble(1.6059043836821613e-10, 1.2585294588752098e-26),
    DoubleDouble(1.1470745597729725e-11, 2.0655512752830745e-28),
    DoubleDouble(7.647163731819816e-13, 7.03872877733453e-30),
    DoubleDouble(4.779477332387385e-14, 4.399205485834081e-31),
    DoubleDouble(2.8114572543455206e-15, 1.6508842730861433e-31),
];

/// Helper function that efficiently multiplies a DoubleDouble by a power of 2. This is -much-
/// faster than regular multiplication but only works with powers of 2.
#[inline]
fn mul_pwr2(a: DoubleDouble, b: f64) -> DoubleDouble {
    DoubleDouble(a.0 * b, a.1 * b)
}

impl DoubleDouble {
    /// Computes the exponentional function, e^self, in double-double precision.
    pub fn exp(&self) -> DoubleDouble {
        // Strategy, as gleaned from MIT papers and Wikipedia:
        //
        // The first step is to reduce the size of the exponent by noting that
        //
        //      exp(kr + m * log(2)) = 2^m * exp(r)^k
        //
        // where m and k are arbitary integers. By choosing m appropriately we can make |kr| <=
        // log(2) / 2 = 0.347. Then exp(r) is evaluated using a Taylor series, which is actually
        // reasonably easy to figure out for the exponential function:
        //
        //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
        //
        // Reducing x substantially speeds up the convergence, so we only have to calculate a few
        // terms of this series to reach our maximum precision.

        let k = 512.0;
        let inv_k = 1.0 / k;

        // Common cases, including numbers too big or small to be represented with DoubleDoubles
        if self.0 <= -709.0 {
            return DoubleDouble::from(0.0);
        }
        if self.0 >= 709.0 {
            return DoubleDouble::INFINITY;
        }
        if *self == 0.0 {
            return DoubleDouble::from(1.0);
        }
        if *self == 1.0 {
            return DoubleDouble::E;
        }

        let m = (self.0 / DoubleDouble::LOG10_2.0 + 0.5).floor();
        let r = mul_pwr2(*self - DoubleDouble::LOG10_2 * m, inv_k);

        let mut p = r.square();
        let mut s = r + mul_pwr2(p, 0.5);
        p *= r;
        let mut t = p * INV_FACT[0];
        let mut i = 0;

        loop {
            s += t;
            p *= r;
            i += 1;
            t = p * INV_FACT[i];
            if i >= 5 || t.to_float().abs() <= inv_k * DoubleDouble::EPSILON {
                break;
            }
        }

        s += t;

        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s = mul_pwr2(s, 2.0) + s.square();
        s += 1.0;

        s.ldexp(m as i32)
    }
}

// #endregion

// #region Logarithms

impl DoubleDouble {
    pub fn ln(self) -> DoubleDouble {
        // Strategy:
        //
        // The Taylor series for logarithms converges much more slowly than that of exp because of
        // the lack of a factorial term in the denominator. Hence this routine instead tries to
        // determine the root of the function
        //
        //      f(x) = exp(x) - a
        //
        // using Newton's iteration. This iteration is given by
        //
        //      x' = x - f(x)/f'(x)
        //         = x - (1 - a * exp(-x))
        //         = x + a * exp(-x) - 1
        //
        // So now we're doing a little calculus too. Exciting!
        //
        // Only one iteration is needed because Newton's iteration approximately doubles the number
        // of digits per iteration.
        if self == 1.0 {
            return DoubleDouble::from(0.0);
        }
        if self <= 0.0 {
            return DoubleDouble::NAN;
        }

        let x = self.0.ln(); // initial approximation
        x + self * (-x).exp() - 1.0
    }

    #[inline]
    pub fn log10(self) -> DoubleDouble {
        self.ln() / DoubleDouble::LN_10
    }
}

// #endregion
