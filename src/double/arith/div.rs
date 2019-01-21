// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, renorm3, two_diff, two_prod};
use crate::double::Double;
use std::f64;
use std::ops::{Div, DivAssign};

// Helper function needed to avoid the only place in this arithmetic where Double::from must be
// called on a non-tuple, non-integer number. With the current parsing of floats, calling
// Double::from this way in the basic arithmetic would cause a stack overflow.
#[inline]
fn mul_f64(a: Double, b: f64) -> Double {
    let (p, e) = two_prod(a.0, b);
    Double::from(quick_two_sum(p, e + a.1 * b))
}

impl Double {
    /// Creates a new double-double representing the quotient of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate dd;
    /// # use dd::Double;
    /// # fn main() {
    /// let x = Double::from_div(1.0, 2.0);
    /// assert!(x == dd!(0.5));
    /// # }
    /// ```
    pub fn from_div(a: f64, b: f64) -> Double {
        if b == 0.0 {
            if a == 0.0 {
                Double::NAN
            } else if a.is_sign_negative() == b.is_sign_positive() {
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
    fn div_double(self, other: Double) -> (f64, f64) {
        if self.is_nan() || other.is_nan() {
            (f64::NAN, f64::NAN)
        } else if other.is_zero() {
            if self.is_zero() {
                (f64::NAN, f64::NAN)
            } else if self.is_sign_negative() == other.is_sign_positive() {
                (
                    f64::NEG_INFINITY,
                    f64::NEG_INFINITY,
                )
            } else {
                (f64::INFINITY, f64::INFINITY)
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                (f64::NAN, f64::NAN)
            } else {
                if self.is_sign_positive() == other.is_sign_positive() {
                    (f64::INFINITY, f64::INFINITY)
                } else {
                    (
                        f64::NEG_INFINITY,
                        f64::NEG_INFINITY,
                    )
                }
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                (0.0, 0.0)
            } else {
                (-0.0, 0.0)
            }
        } else {
            let q1 = self.0 / other.0;
            let mut r = self - mul_f64(other, q1);

            let q2 = r.0 / other.0;
            r -= mul_f64(other, q2);

            let q3 = r.0 / other.0;
            renorm3(q1, q2, q3)
        }
    }

    /// Calculates the reciprocal of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate dd;
    /// # use dd::Double;
    /// # fn main() {
    /// let r = Double::from(2.0).recip();
    /// assert!(r == dd!(0.5));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Double {
        Double::ONE / self
    }
}

impl Div for Double {
    type Output = Double;

    #[inline]
    fn div(self, other: Double) -> Double {
        Double::from(self.div_double(other))
    }
}

impl<'a> Div<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn div(self, other: &Double) -> Double {
        Double::from(self.div_double(*other))
    }
}

impl<'a> Div<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn div(self, other: Double) -> Double {
        Double::from(self.div_double(other))
    }
}

impl DivAssign for Double {
    #[inline]
    fn div_assign(&mut self, other: Double) {
        let (a, b) = self.div_double(other);
        self.0 = a;
        self.1 = b;
    }
}

impl<'a> DivAssign<&'a Double> for Double {
    #[inline]
    fn div_assign(&mut self, other: &Double) {
        let (a, b) = self.div_double(*other);
        self.0 = a;
        self.1 = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        let expected = dd!("1.1557273497909217179100931833127");
        assert_close!(expected, Double::PI / Double::E);
        assert_close!(expected, Double::PI / &Double::E);
        assert_close!(expected, &Double::PI / Double::E);

        let mut a = Double::PI;
        a /= Double::E;
        assert_close!(expected, a);

        let mut b = Double::PI;
        b /= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, Double::NAN / dd!(0));
        assert_exact!(Double::NAN, dd!(0) / Double::NAN);
        assert_exact!(Double::NAN, Double::NAN / dd!(1));
        assert_exact!(Double::NAN, dd!(1) / Double::NAN);
        assert_exact!(Double::INFINITY, Double::INFINITY / dd!(1));
        assert_exact!(Double::ZERO, dd!(1) / Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY / dd!(1));
        assert_exact!(Double::NEG_ZERO, dd!(1) / Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY / Double::INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY / Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY / Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY / Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY / Double::ZERO);
        assert_exact!(Double::ZERO, Double::ZERO / Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY / Double::ZERO);
        assert_exact!(Double::NEG_ZERO, Double::ZERO / Double::NEG_INFINITY);
    }
}