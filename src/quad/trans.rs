// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use crate::quad::common::{INV_FACTS, mul_pwr2};

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
            return Quad::ZERO;
        }
        if self.0 >= 709.0 {
            return Quad::INFINITY;
        }
        if self.is_zero() {
            return Quad::ONE;
        }
        if self == Quad::ONE {
            return Quad::E;
        }

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

// #endregion

// #region Logarithms

impl Quad {/// Calculates the natural logarithm, log<sub>*e*</sub>, of the number.
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
            return Quad::ZERO;
        }
        if self.is_zero() {
            return Quad::NAN;
        }
        if self.is_sign_negative() {
            return Quad::NAN;
        }

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
