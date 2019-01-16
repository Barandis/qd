// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common::mul_pwr2;
use crate::double::Double;

impl Double {
    pub fn sinh(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.abs() > 0.05 {
            let a = self.exp();
            mul_pwr2(a - a.recip(), 0.5)
        } else {
            // The above formula is not accurate enough with very small numbers. Use a Taylor
            // series instead.
            let mut s = self;
            let mut t = self;
            let r = t.sqr();
            let mut m = 1.0;
            let threshold = (self.to_float() * Double::EPSILON).abs();

            loop {
                m += 2.0;
                t *= r;
                t /= (m - 1.0) * m;
                s += t;
                if t.abs() <= threshold {
                    break;
                }
            }
            s
        }
    }

    pub fn cosh(self) -> Double {
        if self.is_zero() {
            Double::ONE
        } else {
            let a = self.exp();
            mul_pwr2(a + a.recip(), 0.5)
        }
    }

    pub fn tanh(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.abs() > 0.05 {
            let a = self.exp();
            let inv_a = a.recip();
            (a - inv_a) / (a + inv_a)
        } else {
            let s = self.sinh();
            let c = (1.0 + s.sqr()).sqrt();
            s / c
        }
    }

    pub fn sinh_cosh(self) -> (Double, Double) {
        if self.abs() <= 0.05 {
            let s = self.sinh();
            let c = (1.0 + s.sqr()).sqrt();
            (s, c)
        } else {
            let a = self.exp();
            let inv_a = a.recip();
            let s = mul_pwr2(a - inv_a, 0.5);
            let c = mul_pwr2(a + inv_a, 0.5);
            (s, c)
        }
    }

    pub fn asinh(self) -> Double {
        (self + (self.sqr() + 1.0).sqrt()).ln()
    }

    pub fn acosh(self) -> Double {
        if self < 1.0 {
            Double::NAN
        } else {
            (self + (self.sqr() - 1.0).sqrt()).ln()
        }
    }

    pub fn atanh(self) -> Double {
        if self.abs() >= 1.0 {
            Double::NAN
        } else {
            mul_pwr2(((1.0 + self) / (1.0 - self)).ln(), 0.5)
        }
    }
}
