// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::{mul_pwr2, INV_FACTS};
use crate::quad::Quad;

// #region Exponential

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

// #endregion

// #region Logarithms

impl Quad {
    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(7).ln();
    /// let expected = qd!("1.945910149055313305105352743443179729637084729581861188459390150");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn ln(self) -> Quad {
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
        // Testing has shown that it requires two iterations to get the required precision.
        if self == Quad::ONE {
            Quad::ZERO
        } else if self.is_zero() {
            Quad::NAN
        } else if self.is_sign_negative() {
            Quad::NAN
        } else {
            let mut x = Quad::from(self.0.ln()); // initial approximation
            let mut i = 0;
            loop {
                let next = x + self * (-x).exp() - Quad::ONE;
                if (x - next).abs() < Quad::EPSILON || i >= 5 {
                    return next;
                }
                x = next;
                i += 1;
            }
        }
    }

    /// Calculates log<sub>10</sub> of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E.log10();
    /// let expected = qd!("0.4342944819032518276511289189166050822943970058036665661144537832");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn log10(self) -> Quad {
        self.ln() / Quad::LN_10
    }

    /// Calculates log<sub>2</sub> of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(10).log2();
    /// let expected = qd!("3.321928094887362347870319429489390175864831393024580612054756396");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn log2(self) -> Quad {
        self.ln() / Quad::LN_2
    }

    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the number.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`, the
    /// specialized functions for those purposes([`ln`], [`log2`], and [`log10`] respectively) will
    /// be more efficient.
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(10).log(7.3);
    /// let expected = qd!("1.158315209978887965104764376269736420106652944692834002126233653");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn log(self, b: f64) -> Quad {
        self.ln() / Quad::from(b).ln()
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_trans_exp() {
        assert_exact!(Quad::ONE, qd!(0).exp());
        assert_exact!(Quad::NAN, Quad::NAN.exp());
        assert_close!(Quad::E, qd!(1).exp());
        assert_exact!(Quad::ZERO, qd!(-710).exp());
        assert_exact!(Quad::INFINITY, qd!(710).exp());
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
    fn quad_trans_ln() {
        assert_exact!(Quad::ZERO, qd!(1).ln());
        assert_exact!(Quad::NAN, qd!(0).ln());
        assert_exact!(Quad::NAN, qd!(-1).ln());
        assert_close!(Quad::ONE, qd!(Quad::E).ln());
        assert_close!(
            qd!("2.302585092994045684017991454684364207601101488628772976033327901"),
            qd!(10).ln()
        );
        assert_close!(
            qd!("5.493061443340548456976226184612628523237452789113747258673471668"),
            qd!(243).ln()
        );
    }

    #[test]
    fn quad_trans_log2() {
        assert_exact!(Quad::ZERO, qd!(1).log2());
        assert_exact!(Quad::NAN, qd!(0).log2());
        assert_exact!(Quad::NAN, qd!(-1).log2());
        assert_close!(Quad::ONE, qd!(2).log2());
        assert_close!(
            qd!("3.321928094887362347870319429489390175864831393024580612054756396"),
            qd!(10).log2()
        );
        assert_close!(
            qd!("7.924812503605780907268694719739082543799072038462405302278763273"),
            qd!(243).log2()
        );
    }

    #[test]
    fn quad_trans_log10() {
        assert_exact!(Quad::ZERO, qd!(1).log10());
        assert_exact!(Quad::NAN, qd!(0).log10());
        assert_exact!(Quad::NAN, qd!(-1).log10());
        assert_close!(Quad::ONE, qd!(10).log10());
        assert_close!(
            qd!("1.623249290397900463220983056572244529451891141976769812643928055"),
            qd!(42).log10()
        );
        assert_close!(
            qd!("2.385606273598312186475139516275576546000644320953479324149328202"),
            qd!(243).log10()
        );
    }

    #[test]
    fn quad_trans_log() {
        assert_exact!(Quad::ZERO, qd!(1).log(6.3));
        assert_exact!(Quad::NAN, qd!(0).log(9.2));
        assert_exact!(Quad::NAN, qd!(-1).log(1.8));
        assert_close!(Quad::ONE, qd!(3.3).log(3.3));
        assert_close!(
            qd!("1.174731503667180022671874948332360514453253860423778048991647180"),
            qd!(10).log(7.1)
        );
        assert_close!(
            qd!("4.224809005935378615289228804344351219807607162037233517389353517"),
            qd!(243).log(3.67)
        );
    }
}
