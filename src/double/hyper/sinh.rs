// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common;
use crate::double::Double;

impl Double {
    /// Computes the hyperbolic sine (sinh) of the `Double`.
    /// 
    /// The domain and range of this function are both (-∞, ∞).
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
        if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ZERO
        } else if self.abs().0 > 0.05 {
            let a = self.exp();
            common::mul_pwr2(a - a.recip(), 0.5)
        } else {
            // The above formula is not accurate enough with very small numbers. Use a
            // Taylor series instead.
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sinh() {
        assert_close!(dd!("11.548739357257748377977334315388"), Double::PI.sinh());
        assert_close!(dd!("7.5441371028169758263418200425165"), Double::E.sinh());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sinh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sinh());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.sinh());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.sinh());
    }
}
