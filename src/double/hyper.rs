// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    /// Computes the hyperbolic sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).sinh();
    /// let expected = dd!("1.1752011936438014568823818505956");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn sinh(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.abs().as_float() > 0.05 {
            let a = self.exp();
            mul_pwr2(a - a.recip(), 0.5)
        } else {
            // The above formula is not accurate enough with very small numbers. Use a Taylor
            // series instead.
            let mut s = self;
            let mut t = self;
            let r = t.sqr();
            let mut m = 1.0;
            let threshold = (self * Double::EPSILON).abs();

            loop {
                m += 2.0;
                t *= r;
                t /= Double::from_mul(m - 1.0, m);
                s += t;
                if t.abs() <= threshold {
                    break;
                }
            }
            s
        }
    }

    /// Computes the hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).cosh();
    /// let expected = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn cosh(self) -> Double {
        if self.is_zero() {
            Double::ONE
        } else {
            let a = self.exp();
            mul_pwr2(a + a.recip(), 0.5)
        }
    }

    /// Computes the hyperbolic tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).tanh();
    /// let expected = dd!("0.76159415595576488811945828260479");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn tanh(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.abs().as_float() > 0.05 {
            let a = self.exp();
            let inv_a = a.recip();
            (a - inv_a) / (a + inv_a)
        } else {
            let s = self.sinh();
            let c = (Double::ONE + s.sqr()).sqrt();
            s / c
        }
    }

    /// Simultaneously computes the hyperbolic sine and cosine of the number.
    ///
    /// This method is more efficient to run than [`sinh`] and [`cosh`] individually and is useful
    /// when both numbers are needed.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let (sin_h, cos_h) = dd!(1).sinh_cosh();
    /// let esin = dd!("1.1752011936438014568823818505956");
    /// let ecos = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff1 = (sin_h - esin).abs();
    /// let diff2 = (cos_h - ecos).abs();
    ///
    /// assert!(diff1 < dd!(1e-30));
    /// assert!(diff2 < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`sinh`]: #method.sinh
    /// [`cosh`]: #method.cosh
    pub fn sinh_cosh(self) -> (Double, Double) {
        if self.abs().as_float() <= 0.05 {
            let s = self.sinh();
            let c = (Double::ONE + s.sqr()).sqrt();
            (s, c)
        } else {
            let a = self.exp();
            let inv_a = a.recip();
            let s = mul_pwr2(a - inv_a, 0.5);
            let c = mul_pwr2(a + inv_a, 0.5);
            (s, c)
        }
    }

    /// Calculates the inverse hyperbolic sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).asinh();
    /// let expected = dd!("1.1947632172871093041119308285191");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn asinh(self) -> Double {
        (self + (self.sqr() + Double::ONE).sqrt()).ln()
    }

    /// Calculates the inverse hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).acosh();
    /// let expected = dd!("0.96242365011920689499551782684874");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn acosh(self) -> Double {
        if self < Double::ONE {
            Double::NAN
        } else {
            (self + (self.sqr() - Double::ONE).sqrt()).ln()
        }
    }

    /// Calculates the inverse hyperbolic tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(0.5).atanh();
    /// let expected = dd!("0.54930614433405484569762261846126");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn atanh(self) -> Double {
        if self.abs() >= Double::ONE {
            Double::NAN
        } else {
            mul_pwr2(((Double::ONE + self) / (Double::ONE - self)).ln(), 0.5)
        }
    }
}
