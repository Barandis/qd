// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::*;
use crate::double::DoubleDouble;
use std::f64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// #region Addition

impl DoubleDouble {
    /// Creates a new `DoubleDouble` representing the sum of two numbers.
    ///
    /// This acts differently from the basic `DoubleDouble(a, b)` constructor in that the internal
    /// values are completely normalized by this function. This is only relevant if no other
    /// operation is done on the number afterwards, as all mathematical ops in this library
    /// normalize their values.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from_add(1.0, 2.0);
    /// assert!(dd == 3.0);
    /// ```
    pub fn from_add(a: f64, b: f64) -> DoubleDouble {
        DoubleDouble::from(two_sum(a, b))
    }

    /// Creates a new `DoubleDouble` by summing two numbers and normalizing them.
    ///
    /// This is a higher-performance, more limited form of [from_add](#method.from_add). It is only
    /// guaranteed to work if |a| >= |b|.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::norm(2.0, 1.0);
    /// assert!(dd == 3.0);
    /// ```
    pub fn norm(a: f64, b: f64) -> DoubleDouble {
        DoubleDouble::from(quick_two_sum(a, b))
    }

    #[inline]
    fn add_double(self, other: DoubleDouble) -> (f64, f64) {
        let (s0, e0) = two_sum(self.0, other.0);
        let (s1, e1) = two_sum(self.1, other.1);
        let (s2, e2) = quick_two_sum(s0, s1 + e0);
        quick_two_sum(s2, e1 + e2)
    }

    #[inline]
    fn add_f64(self, other: f64) -> (f64, f64) {
        let (s, e) = two_sum(self.0, other);
        quick_two_sum(s, e + self.1)
    }
}

impl Add for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn add(self, other: DoubleDouble) -> DoubleDouble {
        DoubleDouble::from(self.add_double(other))
    }
}

impl Add<f64> for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn add(self, other: f64) -> DoubleDouble {
        DoubleDouble::from(self.add_f64(other))
    }
}

impl Add<DoubleDouble> for f64 {
    type Output = DoubleDouble;

    #[inline]
    fn add(self, other: DoubleDouble) -> DoubleDouble {
        other + self
    }
}

impl AddAssign for DoubleDouble {
    #[inline]
    fn add_assign(&mut self, other: DoubleDouble) {
        let (a, b) = self.add_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl AddAssign<f64> for DoubleDouble {
    #[inline]
    fn add_assign(&mut self, other: f64) {
        let (a, b) = self.add_f64(other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Subtraction

impl DoubleDouble {
    /// Creates a new `DoubleDouble` representing the difference of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from_sub(1.0, 2.0);
    /// assert!(dd == -1.0);
    /// ```
    pub fn from_sub(a: f64, b: f64) -> DoubleDouble {
        DoubleDouble::from(two_diff(a, b))
    }

    #[inline]
    fn sub_double(self, other: DoubleDouble) -> (f64, f64) {
        let (s0, e0) = two_diff(self.0, other.0);
        let (s1, e1) = two_diff(self.1, other.1);
        let (s2, e2) = quick_two_sum(s0, s1 + e0);
        quick_two_sum(s2, e1 + e2)
    }

    #[inline]
    fn sub_f64(self, other: f64) -> (f64, f64) {
        let (s, e) = two_diff(self.0, other);
        quick_two_sum(s, e + self.1)
    }
}

impl Sub for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn sub(self, other: DoubleDouble) -> DoubleDouble {
        DoubleDouble::from(self.sub_double(other))
    }
}

impl Sub<f64> for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn sub(self, other: f64) -> DoubleDouble {
        DoubleDouble::from(self.sub_f64(other))
    }
}

impl Sub<DoubleDouble> for f64 {
    type Output = DoubleDouble;

    #[inline]
    fn sub(self, other: DoubleDouble) -> DoubleDouble {
        let (s, e) = two_diff(self, other.0);
        DoubleDouble::from(quick_two_sum(s, e - other.1))
    }
}

impl SubAssign for DoubleDouble {
    #[inline]
    fn sub_assign(&mut self, other: DoubleDouble) {
        let (a, b) = self.sub_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl SubAssign<f64> for DoubleDouble {
    #[inline]
    fn sub_assign(&mut self, other: f64) {
        let (a, b) = self.sub_f64(other);
        self.0 = a;
        self.1 = b;
    }
}

impl Neg for DoubleDouble {
    type Output = DoubleDouble;

    fn neg(self) -> DoubleDouble {
        DoubleDouble(-self.0, -self.1)
    }
}

// #endregion

// #region Multiplication

impl DoubleDouble {
    /// Creates a new `DoubleDouble` representing the product of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from_mul(1.0, 2.0);
    /// assert!(dd == 2.0);
    /// ```
    pub fn from_mul(a: f64, b: f64) -> DoubleDouble {
        DoubleDouble::from(two_prod(a, b))
    }

    #[inline]
    fn mul_double(self, other: DoubleDouble) -> (f64, f64) {
        let (p, e) = two_prod(self.0, other.0);
        quick_two_sum(p, e + self.0 * other.1 + self.1 * other.0)
    }

    #[inline]
    fn mul_f64(self, other: f64) -> (f64, f64) {
        let (p, e) = two_prod(self.0, other);
        quick_two_sum(p, e + self.1 * other)
    }
}

impl Mul for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn mul(self, other: DoubleDouble) -> DoubleDouble {
        DoubleDouble::from(self.mul_double(other))
    }
}

impl Mul<f64> for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn mul(self, other: f64) -> DoubleDouble {
        DoubleDouble::from(self.mul_f64(other))
    }
}

impl Mul<DoubleDouble> for f64 {
    type Output = DoubleDouble;

    #[inline]
    fn mul(self, other: DoubleDouble) -> DoubleDouble {
        other * self
    }
}

impl MulAssign for DoubleDouble {
    #[inline]
    fn mul_assign(&mut self, other: DoubleDouble) {
        let (a, b) = self.mul_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl MulAssign<f64> for DoubleDouble {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        let (a, b) = self.mul_f64(other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Division

impl DoubleDouble {
    /// Creates a new `DoubleDouble` representing the quotient of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from_div(1.0, 2.0);
    /// assert!(dd == 0.5);
    /// ```
    pub fn from_div(a: f64, b: f64) -> DoubleDouble {
        if b == 0.0 {
            if a == 0.0 {
                DoubleDouble::NAN
            } else if a.is_sign_negative() {
                DoubleDouble::NEG_INFINITY
            } else {
                DoubleDouble::INFINITY
            }
        } else {
            let q1 = a / b;

            let (p1, p2) = two_prod(q1, b);
            let (s, e) = two_diff(a, p1);

            let q2 = (s + e - p2) / b;
            DoubleDouble::from(quick_two_sum(q1, q2))
        }
    }

    #[inline]
    pub fn div_double(self, other: DoubleDouble) -> DoubleDouble {
        if other == 0.0 {
            if self == 0.0 {
                DoubleDouble::NAN
            } else if self.is_sign_negative() {
                DoubleDouble::NEG_INFINITY
            } else {
                DoubleDouble::INFINITY
            }
        } else {
            let q1 = self.0 / other.0;
            let mut r = self - q1 * other;

            let q2 = r.0 / other.0;
            r -= q2 * other;

            let q3 = r.0 / other.0;
            DoubleDouble::from(quick_two_sum(q1, q2)) + q3
        }
    }

    #[inline]
    fn div_f64(self, other: f64) -> (f64, f64) {
        if other == 0.0 {
            if self == 0.0 {
                (f64::NAN, f64::NAN)
            } else if self.is_sign_negative() {
                (f64::NEG_INFINITY, f64::NEG_INFINITY)
            } else {
                (f64::INFINITY, f64::INFINITY)
            }
        } else {
            let q1 = self.0 / other;

            let (p1, p2) = two_prod(q1, other);
            let (s, e) = two_diff(self.0, p1);

            let q2 = (s + e + self.1 - p2) / other;
            quick_two_sum(q1, q2)
        }
    }

    #[inline]
    pub fn recip(self) -> DoubleDouble {
        1.0 / self
    }
}

impl Div for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn div(self, other: DoubleDouble) -> DoubleDouble {
        self.div_double(other)
    }
}

impl Div<f64> for DoubleDouble {
    type Output = DoubleDouble;

    #[inline]
    fn div(self, other: f64) -> DoubleDouble {
        DoubleDouble::from(self.div_f64(other))
    }
}

impl Div<DoubleDouble> for f64 {
    type Output = DoubleDouble;

    #[inline]
    fn div(self, other: DoubleDouble) -> DoubleDouble {
        DoubleDouble::from(self) / other
    }
}

impl DivAssign for DoubleDouble {
    #[inline]
    fn div_assign(&mut self, other: DoubleDouble) {
        let result = *self / other;
        self.0 = result.0;
        self.1 = result.1;
    }
}

impl DivAssign<f64> for DoubleDouble {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        let result = *self / other;
        self.0 = result.0;
        self.1 = result.1;
    }
}

// #endregion
