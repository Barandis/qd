// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::{mul_pwr2, INV_FACTS};
use crate::double::Double;

// #region Exponential

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
    pub fn exp(self) -> Double {
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

// #endregion

// #region Logarithms

impl Double {
    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(7).ln();
    /// let expected = dd!("1.9459101490553133051053527434432");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn ln(self) -> Double {
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
        if self == Double::ONE {
            Double::ZERO
        } else if self.is_zero() {
            Double::NAN
        } else if self.is_sign_negative() {
            Double::NAN
        } else {
            let mut x = Double::from(self.0.ln()); // initial approximation
            let mut i = 0;
            loop {
                let next = x + self * (-x).exp() - Double::ONE;
                if (x - next).abs() < Double::EPSILON || i >= 5 {
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
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E.log10();
    /// let expected = dd!("0.434294481903251827651128918916605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log10(self) -> Double {
        self.ln() / Double::LN_10
    }

    /// Calculates log<sub>2</sub> of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log2();
    /// let expected = dd!("3.32192809488736234787031942948939");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log2(self) -> Double {
        self.ln() / Double::LN_2
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
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(10).log(7.0);
    /// let expected = dd!("1.18329466245493832681792856164686");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn log(self, b: f64) -> Double {
        self.ln() / Double::from(b).ln()
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_close {
        ($expected:expr, $actual:expr $(,)*) => {
            assert_precision!($expected, $actual, 30);
        };
    }



    #[test]
    fn double_trans_exp() {
        assert_exact!(Double::ONE, dd!(0).exp());
        assert_exact!(Double::NAN, Double::NAN.exp());
        assert_close!(Double::E, dd!(1).exp());
        assert_exact!(Double::ZERO, dd!(-710).exp());
        assert_exact!(Double::INFINITY, dd!(710).exp());
        assert_close!(
            dd!("14.87973172487283411186899301946839578068879752075547683852481232"),
            dd!(2.7).exp()
        );
        assert_close!(
            dd!("0.001836304777028906825227936299894998089886584890697273635291617797"),
            dd!(-6.3).exp()
        );
    }

    #[test]
    fn double_trans_ln() {
        assert_exact!(Double::ZERO, dd!(1).ln());
        assert_exact!(Double::NAN, dd!(0).ln());
        assert_exact!(Double::NAN, dd!(-1).ln());
        assert_close!(Double::ONE, dd!(Double::E).ln());
        assert_close!(
            dd!("2.302585092994045684017991454684364207601101488628772976033327901"),
            dd!(10).ln()
        );
        assert_close!(
            dd!("5.493061443340548456976226184612628523237452789113747258673471668"),
            dd!(243).ln()
        );
    }

    #[test]
    fn double_trans_log2() {
        assert_exact!(Double::ZERO, dd!(1).log2());
        assert_exact!(Double::NAN, dd!(0).log2());
        assert_exact!(Double::NAN, dd!(-1).log2());
        assert_close!(Double::ONE, dd!(2).log2());
        assert_close!(
            dd!("3.321928094887362347870319429489390175864831393024580612054756396"),
            dd!(10).log2()
        );
        assert_close!(
            dd!("7.924812503605780907268694719739082543799072038462405302278763273"),
            dd!(243).log2()
        );
    }

    #[test]
    fn double_trans_log10() {
        assert_exact!(Double::ZERO, dd!(1).log10());
        assert_exact!(Double::NAN, dd!(0).log10());
        assert_exact!(Double::NAN, dd!(-1).log10());
        assert_close!(Double::ONE, dd!(10).log10());
        assert_close!(
            dd!("1.623249290397900463220983056572244529451891141976769812643928055"),
            dd!(42).log10()
        );
        assert_close!(
            dd!("2.385606273598312186475139516275576546000644320953479324149328202"),
            dd!(243).log10()
        );
    }

    #[test]
    fn double_trans_log() {
        assert_exact!(Double::ZERO, dd!(1).log(6.3));
        assert_exact!(Double::NAN, dd!(0).log(9.2));
        assert_exact!(Double::NAN, dd!(-1).log(1.8));
        assert_close!(Double::ONE, dd!(3.3).log(3.3));
        assert_close!(
            dd!("1.174731503667180022671874948332360514453253860423778048991647180"),
            dd!(10).log(7.1)
        );
        assert_close!(
            dd!("4.224809005935378615289228804344351219807607162037233517389353517"),
            dd!(243).log(3.67)
        );
    }
}
