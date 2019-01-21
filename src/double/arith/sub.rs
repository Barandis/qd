// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, two_diff};
use crate::double::Double;
use std::ops::{Neg, Sub, SubAssign};

impl Double {
    /// Creates a new double-double representing the difference of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_sub(1.0, 2.0);
    /// assert!(x == dd!(-1.0));
    /// # }
    /// ```
    pub fn from_sub(a: f64, b: f64) -> Double {
        Double::from(two_diff(a, b))
    }

    #[inline]
    fn sub_double(self, other: Double) -> (f64, f64) {
        let (s0, e0) = two_diff(self.0, other.0);
        let (s1, e1) = two_diff(self.1, other.1);
        let (s2, e2) = quick_two_sum(s0, s1 + e0);
        quick_two_sum(s2, e1 + e2)
    }
}

impl Sub for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        Double::from(self.sub_double(other))
    }
}

impl<'a> Sub<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: &Double) -> Double {
        Double::from(self.sub_double(*other))
    }
}

impl<'a> Sub<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        Double::from(self.sub_double(other))
    }
}

impl SubAssign for Double {
    #[inline]
    fn sub_assign(&mut self, other: Double) {
        let (a, b) = self.sub_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> SubAssign<&'a Double> for Double {
    #[inline]
    fn sub_assign(&mut self, other: &Double) {
        let (a, b) = self.sub_double(*other);
        self.0 = a;
        self.1 = b;
    }
}

impl Neg for Double {
    type Output = Double;

    fn neg(self) -> Double {
        Double(-self.0, -self.1)
    }
}
