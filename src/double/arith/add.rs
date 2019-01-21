// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, two_sum};
use crate::double::Double;
use std::ops::{Add, AddAssign};

impl Double {
    /// Creates a new double-double representing the sum of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::from_add(1.0, 2.0);
    /// assert!(x == dd!(3.0));
    /// # }
    /// ```
    pub fn from_add(a: f64, b: f64) -> Double {
        Double::from(two_sum(a, b))
    }

    #[inline]
    fn add_double(self, other: Double) -> (f64, f64) {
        let (s0, e0) = two_sum(self.0, other.0);
        let (s1, e1) = two_sum(self.1, other.1);
        let (s2, e2) = quick_two_sum(s0, s1 + e0);
        quick_two_sum(s2, e1 + e2)
    }
}

impl Add for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        Double::from(self.add_double(other))
    }
}

impl<'a> Add<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: &Double) -> Double {
        Double::from(self.add_double(*other))
    }
}

impl<'a> Add<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        Double::from(self.add_double(other))
    }
}

impl AddAssign for Double {
    #[inline]
    fn add_assign(&mut self, other: Double) {
        let (a, b) = self.add_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> AddAssign<&'a Double> for Double {
    #[inline]
    fn add_assign(&mut self, other: &Double) {
        let (a, b) = self.add_double(*other);
        self.0 = a;
        self.1 = b;
    }
}
