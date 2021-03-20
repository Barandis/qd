// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::{tables, Double};

impl Double {
    /// Computes the exponential function, *e*<sup>x</sup>, for the `Double`.
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
            let r = (self - Double::LN_2 * Double(m, 0.0)).mul_pwr2(inv_k);

            let mut p = r.sqr();
            let mut s = r + p.mul_pwr2(0.5);
            p *= r;
            let mut t = p * tables::INV_FACTS[0];
            let mut i = 0;

            loop {
                s += t;
                p *= r;
                i += 1;
                t = p * tables::INV_FACTS[i];
                if i >= 5 || t.abs() <= Double::from(inv_k) * Double::EPSILON {
                    break;
                }
            }

            s += t;

            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s = s.mul_pwr2(2.0) + s.sqr();
            s += Double::ONE;

            s.ldexp(m as i32)
        }
    }

    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the `Double`.
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
        // The Taylor series for logarithms converges much more slowly than that of exp
        // because of the lack of a factorial term in the denominator. Hence this routine
        // instead tries to determine the root of the function
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
        } else if self.is_zero() || self.is_sign_negative() {
            Double::NAN
        } else if self.is_infinite() {
            Double::INFINITY
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

    /// Calculates log<sub>10</sub> of the `Double`.
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

    /// Calculates log<sub>2</sub> of the `Double`.
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

    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the `Double`.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`,
    /// the specialized functions for those purposes([`ln`], [`log2`], and [`log10`]
    /// respectively) will be more efficient.
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
    ///
    /// [`ln`]: #method.ln 
    /// [`log2`]: #method.log2 
    /// [`log10`]: #method.log10
    #[inline]
    pub fn log(self, b: f64) -> Double {
        self.ln() / Double::from(b).ln()
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
    fn exp_zero() {
        assert_exact!(Double::ONE, Double::ZERO.exp());
    }

    #[test]
    fn exp_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.exp());
        assert_exact!(Double::ZERO, Double::NEG_INFINITY.exp());
    }

    #[test]
    fn exp_nan() {
        assert_exact!(Double::NAN, Double::NAN.exp());
    }

    #[test]
    fn exp_extreme() {
        assert_exact!(Double::ZERO, dd!(-710).exp());
        assert_exact!(Double::INFINITY, dd!(710).exp());
    }

    #[test]
    fn ln() {
        assert_close!(dd!("2.30258509299404568401799145468436"), dd!(10).ln());
        assert_close!(dd!("5.49306144334054845697622618461263"), dd!(243).ln());
        assert_close!(Double::ONE, Double::E.ln());
        assert_exact!(Double::ZERO, Double::ONE.ln());
    }

    #[test]
    fn ln_zero() {
        assert_exact!(Double::NAN, Double::ZERO.ln());
        assert_exact!(Double::NAN, Double::NEG_ZERO.ln());
    }

    #[test]
    fn ln_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.ln());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.ln());
    }

    #[test]
    fn ln_nan() {
        assert_exact!(Double::NAN, Double::NAN.ln());
    }

    #[test]
    fn ln_neg() {
        assert_exact!(Double::NAN, dd!(-1).ln());
    }

    #[test]
    fn log10() {
        assert_close!(dd!("1.62324929039790046322098305657224"), dd!(42).log10());
        assert_close!(dd!("2.38560627359831218647513951627558"), dd!(243).log10());
        assert_exact!(Double::ZERO, dd!(1).log10());
        assert_close!(Double::ONE, dd!(10).log10());
    }

    #[test]
    fn log10_zero() {
        assert_exact!(Double::NAN, Double::ZERO.log10());
        assert_exact!(Double::NAN, Double::NEG_ZERO.log10());
    }

    #[test]
    fn log10_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log10());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log10());
    }

    #[test]
    fn log10_nan() {
        assert_exact!(Double::NAN, Double::NAN.log10());
    }

    #[test]
    fn log10_neg() {
        assert_exact!(Double::NAN, dd!(-1).log10());
    }

    #[test]
    fn log2() {
        assert_close!(dd!("3.32192809488736234787031942948939"), dd!(10).log2());
        assert_close!(dd!("7.92481250360578090726869471973908"), dd!(243).log2());
        assert_exact!(Double::ZERO, dd!(1).log2());
        assert_close!(Double::ONE, dd!(2).log2());
    }

    #[test]
    fn log2_zero() {
        assert_exact!(Double::NAN, Double::ZERO.log2());
        assert_exact!(Double::NAN, Double::NEG_ZERO.log2());
    }

    #[test]
    fn log2_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log2());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log2());
    }

    #[test]
    fn log2_nan() {
        assert_exact!(Double::NAN, Double::NAN.log2());
    }

    #[test]
    fn log2_neg() {
        assert_exact!(Double::NAN, dd!(-1).log2());
    }

    #[test]
    fn log() {
        assert_close!(dd!("1.17473150366718002267187494833236"), dd!(10).log(7.1));
        assert_close!(
            dd!("4.22480900593537861528922880434435"),
            dd!(243).log(3.67)
        );
        assert_exact!(Double::ZERO, dd!(1).log(6.3));
        assert_close!(Double::ONE, dd!(3.3).log(3.3));
    }

    #[test]
    fn log_zero() {
        assert_exact!(Double::NAN, Double::ZERO.log(9.2));
        assert_exact!(Double::NAN, Double::NEG_ZERO.log(1.8));
    }

    #[test]
    fn log_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.log(7.3));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.log(7.3));
    }

    #[test]
    fn log_nan() {
        assert_exact!(Double::NAN, Double::NAN.log(3.4));
    }

    #[test]
    fn log_neg() {
        assert_exact!(Double::NAN, dd!(-1).log(1.8));
    }
}

