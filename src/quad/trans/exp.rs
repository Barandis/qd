// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::{mul_pwr2, INV_FACTS};
use crate::quad::Quad;

impl Quad {
    /// Computes the exponential function, *e*<sup>x</sup>, for the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2.3).exp();
    /// let expected = qd!("9.974182454814720739957615156908858001478701193684029563691421917");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn exp(self) -> Quad {
        // Strategy, as gleaned from MIT papers and Wikipedia:
        //
        // The first step is to reduce the size of the exponent by noting that
        //
        //      exp(kr + m * ln(2)) = 2^m * exp(r)^k
        //
        // where m and k are arbitary integers. By choosing m appropriately we can make |kr| <=
        // ln(2) / 2 = 0.347. Then exp(r) is evaluated using a Taylor series, which is actually
        // reasonably easy to figure out for the exponential function:
        //
        //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
        //
        // Reducing x substantially speeds up the convergence, so we have to use fewer terms to
        // reach the required precision.

        let k = 2f64.powi(16);
        let inv_k = 1.0 / k;

        // Common cases, including numbers too big or small to be represented with Quads
        if self.0 <= -709.0 {
            Quad::ZERO
        } else if self.0 >= 709.0 {
            Quad::INFINITY
        } else if self.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
            Quad::ONE
        } else if self == Quad::ONE {
            Quad::E
        } else {
            let m = (self.0 / Quad::LN_2.0 + 0.5).floor();
            let r = mul_pwr2(self - Quad::LN_2 * Quad::from(m), inv_k);
            let threshold = Quad::from(inv_k) * Quad::EPSILON;

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
                if i >= 9 || t.abs() <= threshold {
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
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s = mul_pwr2(s, 2.0) + s.sqr();
            s += Quad::ONE;

            s.ldexp(m as i32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(
            qd!("14.87973172487283411186899301946839578068879752075547683852481232"),
            qd!(2.7).exp()
        );
        assert_close!(
            qd!("0.001836304777028906825227936299894998089886584890697273635291617797"),
            qd!(-6.3).exp()
        );
    }

    #[test]
    fn special() {
        assert_exact!(Quad::ONE, qd!(0).exp());
        assert_exact!(Quad::NAN, Quad::NAN.exp());
        assert_close!(Quad::E, qd!(1).exp());
        assert_exact!(Quad::ZERO, qd!(-710).exp());
        assert_exact!(Quad::INFINITY, qd!(710).exp());
    }
}
