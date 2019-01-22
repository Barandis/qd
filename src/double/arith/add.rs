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
    /// # #[macro_use] extern crate dd;
    /// # use dd::Double;
    /// # fn main() {
    /// let x = Double::from_add(1.0, 2.0);
    /// assert!(x == dd!(3.0));
    /// # }
    /// ```
    pub fn from_add(a: f64, b: f64) -> Double {
        Double::from(two_sum(a, b))
    }
}

impl Add for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Double::INFINITY
                    } else {
                        Double::NAN
                    }
                } else {
                    if other.is_sign_negative() {
                        Double::NEG_INFINITY
                    } else {
                        Double::NAN
                    }
                }
            } else {
                if self.is_sign_positive() {
                    Double::INFINITY
                } else {
                    Double::NEG_INFINITY
                }
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else {
            let (s0, e0) = two_sum(self.0, other.0);
            let (s1, e1) = two_sum(self.1, other.1);
            let (s2, e2) = quick_two_sum(s0, s1 + e0);
            Double::from(quick_two_sum(s2, e1 + e2))
        }
    }
}

impl<'a> Add<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn add(self, other: &Double) -> Double {
        self.add(*other)
    }
}

impl<'a> Add<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn add(self, other: Double) -> Double {
        (*self).add(other)
    }
}

impl AddAssign for Double {
    #[inline]
    fn add_assign(&mut self, other: Double) {
        self.assign(self.add(other).into());
    }
}

impl<'a> AddAssign<&'a Double> for Double {
    #[inline]
    fn add_assign(&mut self, other: &Double) {
        self.assign(self.add(*other).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        let expected = dd!("5.8598744820488384738229308546322");
        assert_close!(expected, Double::PI + Double::E);
        assert_close!(expected, Double::PI + &Double::E);
        assert_close!(expected, &Double::PI + Double::E);

        let mut a = Double::PI;
        a += Double::E;
        assert_close!(expected, a);

        let mut b = Double::PI;
        b += &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, Double::NAN + dd!(1));
        assert_exact!(Double::NAN, dd!(1) + Double::NAN);
        assert_exact!(Double::INFINITY, Double::INFINITY + dd!(1));
        assert_exact!(Double::INFINITY, dd!(1) + Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY + dd!(1));
        assert_exact!(Double::NEG_INFINITY, dd!(1) + Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY + Double::INFINITY);
        assert_exact!(
            Double::NEG_INFINITY,
            Double::NEG_INFINITY + Double::NEG_INFINITY
        );
        assert_exact!(Double::NAN, Double::INFINITY + Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY + Double::INFINITY);
    }
}
