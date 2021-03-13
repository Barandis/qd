// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::{quick_two_sum, two_diff};
use crate::double::Double;
use std::ops::{Sub, SubAssign};

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
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        if self.is_nan() || other.is_nan() {
            Double::NAN
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Double::NAN
                    } else {
                        Double::INFINITY
                    }
                } else if other.is_sign_negative() {
                    Double::NAN
                } else {
                    Double::NEG_INFINITY
                }
            } else if self.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Double::NEG_INFINITY
            } else {
                Double::INFINITY
            }
        } else {
            let (s0, e0) = two_diff(self.0, other.0);
            let (s1, e1) = two_diff(self.1, other.1);
            let (s2, e2) = quick_two_sum(s0, s1 + e0);
            Double::from(quick_two_sum(s2, e1 + e2))
        }
    }
}

impl Sub<&Double> for Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: &Double) -> Double {
        self.sub(*other)
    }
}

impl Sub<Double> for &Double {
    type Output = Double;

    #[inline]
    fn sub(self, other: Double) -> Double {
        (*self).sub(other)
    }
}

impl SubAssign for Double {
    #[inline]
    fn sub_assign(&mut self, other: Double) {
        self.assign(self.sub(other).into());
    }
}

impl SubAssign<&Double> for Double {
    #[inline]
    fn sub_assign(&mut self, other: &Double) {
        self.assign(self.sub(*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, Double::PI - Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, Double::PI - &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, &Double::PI - Double::E);
    }

    #[test]
    fn assign_num() {
        let expected = dd!("0.42331082513074800310235591192684");
        let mut a = Double::PI;
        a -= Double::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = dd!("0.42331082513074800310235591192684");
        let mut b = Double::PI;
        b -= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY - Double::ONE);
        assert_exact!(Double::NEG_INFINITY, Double::ONE - Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY - Double::ONE);
        assert_exact!(Double::INFINITY, Double::ONE - Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY - Double::NEG_INFINITY);
        assert_exact!(
            Double::NEG_INFINITY,
            Double::NEG_INFINITY - Double::INFINITY
        );
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN - Double::ONE);
        assert_exact!(Double::NAN, Double::ONE - Double::NAN);
        assert_exact!(Double::NAN, Double::NAN - Double::INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY - Double::NAN);
        assert_exact!(Double::NAN, Double::INFINITY - Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY - Double::NEG_INFINITY);
    }
}
