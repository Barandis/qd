// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, two_diff};
use crate::double::Double;
use std::f64;
use std::ops::{Neg, Sub, SubAssign};

impl Double {
    /// Creates a new double-double representing the difference of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate dd;
    /// # use dd::Double;
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
        if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        (f64::NAN, f64::NAN)
                    } else {
                        (f64::INFINITY, f64::INFINITY)
                    }
                } else {
                    if other.is_sign_negative() {
                        (f64::NAN, f64::NAN)
                    } else {
                        (f64::NEG_INFINITY, f64::NEG_INFINITY)
                    }
                }
            } else {
                if self.is_sign_positive() {
                    (f64::INFINITY, f64::INFINITY)
                } else {
                    (f64::NEG_INFINITY, f64::NEG_INFINITY)
                }
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                (f64::NEG_INFINITY, f64::NEG_INFINITY)
            } else {
                (f64::INFINITY, f64::INFINITY)
            }
        } else {
            let (s0, e0) = two_diff(self.0, other.0);
            let (s1, e1) = two_diff(self.1, other.1);
            let (s2, e2) = quick_two_sum(s0, s1 + e0);
            quick_two_sum(s2, e1 + e2)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, Double::PI - Double::E);
        assert_close!(expected, Double::PI - &Double::E);
        assert_close!(expected, &Double::PI - Double::E);

        let mut a = Double::PI;
        a -= Double::E;
        assert_close!(expected, a);

        let mut b = Double::PI;
        b -= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, Double::NAN - dd!(1));
        assert_exact!(Double::NAN, dd!(1) - Double::NAN);
        assert_exact!(Double::INFINITY, Double::INFINITY - dd!(1));
        assert_exact!(Double::NEG_INFINITY, dd!(1) - Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY - dd!(1));
        assert_exact!(Double::INFINITY, dd!(1) - Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY - Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY - Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY - Double::NEG_INFINITY);
        assert_exact!(
            Double::NEG_INFINITY,
            Double::NEG_INFINITY - Double::INFINITY
        );
    }
}
