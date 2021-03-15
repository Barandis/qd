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
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for Double {
    type Output = Double;

    fn mul(self, other: Double) -> Double {
        if self.is_nan() || other.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            if other.is_infinite() {
                Double::NAN
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Double::ZERO
            } else {
                Double::NEG_ZERO
            }
        } else if self.is_infinite() {
            if other.is_zero() {
                Double::NAN
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else {
            let (p, e) = two_prod(self.0, other.0);
            Double::from(quick_two_sum(p, e + self.0 * other.1 + self.1 * other.0))
        }
    }
}

impl Mul<&Double> for Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: &Double) -> Double {
        self.mul(*other)
    }
}

impl Mul<Double> for &Double {
    type Output = Double;

    #[inline]
    fn mul(self, other: Double) -> Double {
        (*self).mul(other)
    }
}

impl MulAssign for Double {
    #[inline]
    fn mul_assign(&mut self, other: Double) {
        self.assign(self.mul(other).into());
    }
}

impl MulAssign<&Double> for Double {
    #[inline]
    fn mul_assign(&mut self, other: &Double) {
        self.assign(self.mul(*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = dd!("8.5397342226735670654635508695466");
        assert_close!(expected, Double::PI * Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = dd!("8.5397342226735670654635508695466");
        assert_close!(expected, Double::PI * &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = dd!("8.5397342226735670654635508695466");
        assert_close!(expected, &Double::PI * Double::E);
    }

    #[test]
    fn assign_num() {
        let expected = dd!("8.5397342226735670654635508695466");
        let mut a = Double::PI;
        a *= Double::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = dd!("8.5397342226735670654635508695466");
        let mut b = Double::PI;
        b *= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::NAN * Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO * Double::NAN);
        assert_exact!(Double::NAN, Double::INFINITY * Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO * Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY * Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO * Double::NEG_INFINITY);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY * Double::ONE);
        assert_exact!(Double::INFINITY, Double::ONE * Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY * Double::ONE);
        assert_exact!(Double::NEG_INFINITY, Double::ONE * Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY * Double::INFINITY);
        assert_exact!(
            Double::NEG_INFINITY,
            Double::INFINITY * Double::NEG_INFINITY
        );
        assert_exact!(
            Double::NEG_INFINITY,
            Double::NEG_INFINITY * Double::INFINITY
        );
        assert_exact!(
            Double::INFINITY,
            Double::NEG_INFINITY * Double::NEG_INFINITY
        );
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN * Double::ONE);
        assert_exact!(Double::NAN, Double::ONE * Double::NAN);
    }
}
