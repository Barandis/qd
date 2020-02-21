// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::mul_pwr2;
use crate::quad::Quad;

impl Quad {
    /// Computes the hyperbolic sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).sinh();
    /// let expected = qd!("1.175201193643801456882381850595600815155717981334095870229565413");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sinh(self) -> Quad {
        if self.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
            Quad::ZERO
        } else if self.abs().as_float() > 0.05 {
            let a = self.exp();
            mul_pwr2(a - a.recip(), 0.5)
        } else {
            // The above formula is not accurate enough with very small numbers.
            // Use a Taylor series instead.
            let mut s = self;
            let mut t = self;
            let r = t.sqr();
            let mut m = 1.0;
            let threshold = (self * Quad::EPSILON).abs();

            loop {
                m += 2.0;
                t *= r;
                t /= Quad::from((m - 1.0) * m);
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
        assert_close!(
            qd!("11.54873935725774837797733431538840968449518906639478945523216336"),
            Quad::PI.sinh()
        );
        assert_close!(
            qd!("7.544137102816975826341820042516532740294985744301671666369136432"),
            Quad::E.sinh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sinh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sinh());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.sinh());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sinh());
    }
}
