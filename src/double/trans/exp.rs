// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::{mul_pwr2, INV_FACTS};
use crate::double::Double;

impl Double {
    /// Computes the exponential function, *e*<sup>x</sup>, for the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).exp();
    /// let expected = dd!("7.3890560989306502272304274605750");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn exp(self) -> Double {
        // Strategy, as gleaned from MIT papers and Wikipedia:
        //
        // The first step is to reduce the size of the exponent by noting that
        //
        //      exp(kr + m * ln(2)) = 2^m * exp(r)^k
        //
        // where m and k are arbitary integers. By choosing m appropriately we can make |kr|
        // <= ln(2) / 2 = 0.347. Then exp(r) is evaluated using a Taylor series, which is
        // actually reasonably easy to figure out for the exponential function:
        //
        //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
        //
        // Reducing x substantially speeds up the convergence, so we have to use fewer terms
        // to reach the required precision.

        let k = 512.0;
        let inv_k = 1.0 / k;

        // Common cases, including numbers too big or small to be represented with Doubles
        if self.0 <= -709.0 {
            Double::ZERO
        } else if self.0 >= 709.0 {
            Double::INFINITY
        } else if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ONE
        } else if self == Double::ONE {
            Double::E
        } else {
            let m = (self.0 / Double::LN_2.0 + 0.5).floor();
            let r = mul_pwr2(self - Double::LN_2 * Double::from(m), inv_k);

            let mut p = r.sqr();
            let mut s = r + mul_pwr2(p, 0.5);
            p *= r;
            let mut t = p * INV_FACTS[0];
            let mut i = 0;

            loop {
                s += t;
                p *= r;
                i += 1;
                t = p * INV_FACTS[i];
                if i >= 5 || t.abs() <= Double::from(inv_k) * Double::EPSILON {
                    break;
                }
            }

            s += t;

            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s += Double::ONE;

            s.ldexp(m as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp() {
        assert_close!(dd!("14.87973172487283411186899301946840"), dd!(2.7).exp());
        assert_close!(dd!("0.0018363047770289068252279362998950"), dd!(-6.3).exp());
        assert_close!(Double::E, Double::ONE.exp());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ONE, Double::ZERO.exp());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.exp());
        assert_exact!(Double::ZERO, Double::NEG_INFINITY.exp());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.exp());
    }

    #[test]
    fn over_limit() {
        assert_exact!(Double::ZERO, dd!(-710).exp());
        assert_exact!(Double::INFINITY, dd!(710).exp());
    }
}
