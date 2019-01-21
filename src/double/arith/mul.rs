// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, two_prod};
use crate::double::Double;
use std::ops::{Mul, MulAssign};

impl Double {
    /// Creates a new double-double representing the product of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_mul(1.0, 2.0);
    /// assert!(x == dd!(2.0));
    /// # }
    /// ```
    pub fn from_mul(a: f64, b: f64) -> Double {
        Double::from(two_prod(a, b))
    }

    #[inline]
    fn mul_double(self, other: Double) -> (f64, f64) {
        let (p, e) = two_prod(self.0, other.0);
        quick_two_sum(p, e + self.0 * other.1 + self.1 * other.0)
    }
}

impl Mul for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        Double::from(self.mul_double(other))
    }
}

impl<'a> Mul<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: &Double) -> Double {
        Double::from(self.mul_double(*other))
    }
}

impl<'a> Mul<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        Double::from(self.mul_double(other))
    }
}

impl MulAssign for Double {
    #[inline]
    fn mul_assign(&mut self, other: Double) {
        let (a, b) = self.mul_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> MulAssign<&'a Double> for Double {
    #[inline]
    fn mul_assign(&mut self, other: &Double) {
        let (a, b) = self.mul_double(*other);
        self.0 = a;
        self.1 = b;
    }
}
