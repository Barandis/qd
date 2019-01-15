// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::*;
use crate::double::Double;
use std::f64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// #region Addition

impl Double {
    /// Creates a new `Double` representing the sum of two numbers.
    ///
    /// This acts differently from the basic `Double(a, b)` constructor in that the internal
    /// values are completely normalized by this function. This is only relevant if no other
    /// operation is done on the number afterwards, as all mathematical ops in this library
    /// normalize their values.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from_add(1.0, 2.0);
    /// assert!(dd == 3.0);
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

    #[inline]
    fn add_f64(self, other: f64) -> (f64, f64) {
        let (s, e) = two_sum(self.0, other);
        quick_two_sum(s, e + self.1)
    }
}

impl Add for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        Double::from(self.add_double(other))
    }
}

impl Add<f64> for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: f64) -> Double {
        Double::from(self.add_f64(other))
    }
}

impl Add<Double> for f64 {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        other + self
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

impl AddAssign<f64> for Double {
    #[inline]
    fn add_assign(&mut self, other: f64) {
        let (a, b) = self.add_f64(other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Subtraction

impl Double {
    /// Creates a new `Double` representing the difference of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from_sub(1.0, 2.0);
    /// assert!(dd == -1.0);
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

    #[inline]
    fn sub_f64(self, other: f64) -> (f64, f64) {
        let (s, e) = two_diff(self.0, other);
        quick_two_sum(s, e + self.1)
    }
}

impl Sub for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        Double::from(self.sub_double(other))
    }
}

impl Sub<f64> for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: f64) -> Double {
        Double::from(self.sub_f64(other))
    }
}

impl Sub<Double> for f64 {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        let (s, e) = two_diff(self, other.0);
        Double::from(quick_two_sum(s, e - other.1))
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

impl SubAssign<f64> for Double {
    #[inline]
    fn sub_assign(&mut self, other: f64) {
        let (a, b) = self.sub_f64(other);
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

// #endregion

// #region Multiplication

impl Double {
    /// Creates a new `Double` representing the product of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from_mul(1.0, 2.0);
    /// assert!(dd == 2.0);
    /// ```
    pub fn from_mul(a: f64, b: f64) -> Double {
        Double::from(two_prod(a, b))
    }

    #[inline]
    fn mul_double(self, other: Double) -> (f64, f64) {
        let (p, e) = two_prod(self.0, other.0);
        quick_two_sum(p, e + self.0 * other.1 + self.1 * other.0)
    }

    #[inline]
    fn mul_f64(self, other: f64) -> (f64, f64) {
        let (p, e) = two_prod(self.0, other);
        quick_two_sum(p, e + self.1 * other)
    }
}

impl Mul for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        Double::from(self.mul_double(other))
    }
}

impl Mul<f64> for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: f64) -> Double {
        Double::from(self.mul_f64(other))
    }
}

impl Mul<Double> for f64 {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        other * self
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

impl MulAssign<f64> for Double {
    #[inline]
    fn mul_assign(&mut self, other: f64) {
        let (a, b) = self.mul_f64(other);
        self.0 = a;
        self.1 = b;
    }
}

// #endregion

// #region Division

impl Double {
    /// Creates a new `Double` representing the quotient of two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let dd = Double::from_div(1.0, 2.0);
    /// assert!(dd == 0.5);
    /// ```
    pub fn from_div(a: f64, b: f64) -> Double {
        if b == 0.0 {
            if a == 0.0 {
                Double::NAN
            } else if a.is_sign_negative() {
                Double::NEG_INFINITY
            } else {
                Double::INFINITY
            }
        } else {
            let q1 = a / b;

            let (p1, p2) = two_prod(q1, b);
            let (s, e) = two_diff(a, p1);

            let q2 = (s + e - p2) / b;
            Double::from(quick_two_sum(q1, q2))
        }
    }

    #[inline]
    fn div_double(self, other: Double) -> Double {
        if other.is_zero() {
            if self.is_zero() {
                Double::NAN
            } else if self.is_sign_negative() {
                Double::NEG_INFINITY
            } else {
                Double::INFINITY
            }
        } else {
            let q1 = self.0 / other.0;
            let mut r = self - q1 * other;

            let q2 = r.0 / other.0;
            r -= q2 * other;

            let q3 = r.0 / other.0;
            Double::from(quick_two_sum(q1, q2)) + q3
        }
    }

    #[inline]
    fn div_f64(self, other: f64) -> (f64, f64) {
        if other == 0.0 {
            if self.is_zero() {
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

    /// Calculates the reciprocal of `self`, returning it as a new `Double`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::Double;
    ///
    /// let r = Double::from(2.0).recip();
    /// assert!(r == 0.5);
    /// ```
    #[inline]
    pub fn recip(self) -> Double {
        1.0 / self
    }
}

impl Div for Double {
    type Output = Double;

    #[inline]
    fn div(self, other: Double) -> Double {
        self.div_double(other)
    }
}

impl Div<f64> for Double {
    type Output = Double;

    #[inline]
    fn div(self, other: f64) -> Double {
        Double::from(self.div_f64(other))
    }
}

impl Div<Double> for f64 {
    type Output = Double;

    #[inline]
    fn div(self, other: Double) -> Double {
        Double::from(self) / other
    }
}

impl DivAssign for Double {
    #[inline]
    fn div_assign(&mut self, other: Double) {
        let result = *self / other;
        self.0 = result.0;
        self.1 = result.1;
    }
}

impl DivAssign<f64> for Double {
    #[inline]
    fn div_assign(&mut self, other: f64) {
        let result = *self / other;
        self.0 = result.0;
        self.1 = result.1;
    }
}

// #endregion

// #region Tests

// Tests are all to be done with these types of numbers:
//
// 1 Whole numbers (integers with .0)
// 2 Representable numbers (numbers with even binary fractions)
// 3 Unrepresentable numbers
// 4 High-precision whole numbers
// 5 High-precision representable numbers
// 6 High-precision unrepresentable numbers
// 7 Exponentials with whole numbers
// 8 Exponentials with representable numbers
// 9 Exponentials with unrepresentable numbers
#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Double, b: Double, places: i32) -> bool {
        a.0 == b.0 && (a.1 - b.1).abs() < 10f64.powi(-places)
    }

    fn message(a: Double, b: Double) -> String {
        format!("\nActual:   {:?}\nExpected: {:?}", a, b)
    }

    fn check(a: Double, b: Double, places: i32) {
        assert!(close(a, b, places), message(a, b));
    }

    #[test]
    fn add_whole() {
        assert_eq!(Double::from(13) + Double::from(14), 27.0);
        assert_eq!(
            "1357913579135791357913579".parse::<Double>().unwrap()
                + "8642086420864208642086420".parse::<Double>().unwrap(),
            "9999999999999999999999999".parse::<Double>().unwrap()
        );
        assert_eq!(Double::from(3e10) + Double::from(5e10), 8e10);
    }

    #[test]
    fn add_repr() {
        check(
            Double::from(6.25) + Double::from(5.5),
            Double::from(11.75),
            28
        );
        check(
            "13579135791357913579.25".parse::<Double>().unwrap()
                + "86420864208642086420.5".parse::<Double>().unwrap(),
            "99999999999999999999.75".parse::<Double>().unwrap(),
            10
        );
        check(Double::from(6.25e20) + Double::from(5.5e20), Double::from(1.175e21), 8);
    }

    #[test]
    fn add_unrepr() {
        check(
            Double::from(6.3) + Double::from(4.2),
            Double::from(10.5),
            28
        );
        check(
            "135791357913579.1357913579".parse::<Double>().unwrap()
                + "864208642086420.8642086420".parse::<Double>().unwrap(),
            "999999999999999.9999999999".parse::<Double>().unwrap(),
            16
        );
        check(
            Double::from(1.35e20) + Double::from(8.64e19),
            Double::from(2.214e20),
            8
        );
    }

    #[test]
    fn sub_whole() {
        assert_eq!(Double::from(13) - Double::from(14), -1.0);
        assert_eq!(
            "1357913579135791357913579".parse::<Double>().unwrap()
                - "8642086420864208642086420".parse::<Double>().unwrap(),
            "-7284172841728417284172841".parse::<Double>().unwrap()
        );
        assert_eq!(Double::from(3e10) - Double::from(5e10), -2e10);
    }

    #[test]
    fn sub_repr() {
        check(
            Double::from(6.25) - Double::from(5.5),
            Double::from(0.75),
            28
        );
        check(
            "13579135791357913579.5".parse::<Double>().unwrap()
                - "86420864208642086420.25".parse::<Double>().unwrap(),
            "-72841728417284172840.75".parse::<Double>().unwrap(),
            10
        );
        check(Double::from(6.25e20) - Double::from(5.5e20), Double::from(7.5e19), 8);
    }

    #[test]
    fn sub_unrepr() {
        check(
            Double::from(6.3) - Double::from(4.2),
            Double::from(2.1),
            28
        );
        check(
            "135791357913579.1357913579".parse::<Double>().unwrap()
                - "864208642086420.8642086420".parse::<Double>().unwrap(),
            "-728417284172841.7284172841".parse::<Double>().unwrap(),
            16
        );
        check(
            Double::from(1.35e20) - Double::from(8.64e19),
            Double::from(4.86e19),
            8
        );
    }
}

// #endregion
