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
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Double {
    type Output = Double;

    /// Adds this `Double` to another, producing a new `Double` as a result.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E + Double::PI;
    /// let expected = dd!("5.859874482048838473822930854632");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    fn add(self, other: Double) -> Double {
        if self.is_nan() || other.is_nan() {
            Double::NAN
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Double::INFINITY
                    } else {
                        Double::NAN
                    }
                } else if other.is_sign_negative() {
                    Double::NEG_INFINITY
                } else {
                    Double::NAN
                }
            } else if self.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
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

impl Add for &Double {
    type Output = Double;

    /// Adds a reference to this `Double` to another, producing a new `Double` as a result.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::E + &Double::PI;
    /// let expected = dd!("5.859874482048838473822930854632");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn add(self, other: &Double) -> Double {
        (*self).add(*other)
    }
}

impl Add<&Double> for Double {
    type Output = Double;

    /// Adds this `Double` to a reference to another `Double`, producing a new `Double` as a
    /// result.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E + &Double::PI;
    /// let expected = dd!("5.859874482048838473822930854632");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn add(self, other: &Double) -> Double {
        self.add(*other)
    }
}

impl Add<Double> for &Double {
    type Output = Double;

    /// Adds a reference to this `Double` to another `Double`, producing a new `Double` as a
    /// result.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::E + Double::PI;
    /// let expected = dd!("5.859874482048838473822930854632");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn add(self, other: Double) -> Double {
        (*self).add(other)
    }
}

impl AddAssign for Double {
    /// Adds another `Double` to this one, modifying this one to equal the result.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::E;
    /// x += Double::PI;
    /// let expected = dd!("5.859874482048838473822930854632");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Double) {
        let (a, b) = self.add(other).into();
        self.0 = a;
        self.1 = b;
    }
}

impl AddAssign<&Double> for Double {
    /// Adds a reference to another `Double` to this `Double`, modifying this one to equal
    /// the result.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::E;
    /// x += &Double::PI;
    /// let expected = dd!("5.859874482048838473822930854632");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Double) {
        let (a, b) = self.add(*other).into();
        self.0 = a;
        self.1 = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = dd!("5.8598744820488384738229308546322");
        assert_close!(expected, Double::PI + Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = dd!("5.8598744820488384738229308546322");
        assert_close!(expected, Double::PI + &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = dd!("5.8598744820488384738229308546322");
        assert_close!(expected, &Double::PI + Double::E);
    }

    #[test]
    fn assign_num() {
        let expected = dd!("5.8598744820488384738229308546322");
        let mut a = Double::PI;
        a += Double::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = dd!("5.8598744820488384738229308546322");
        let mut b = Double::PI;
        b += &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY + Double::ONE);
        assert_exact!(Double::INFINITY, Double::ONE + Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY + Double::ONE);
        assert_exact!(Double::NEG_INFINITY, Double::ONE + Double::NEG_INFINITY);
    }

    #[test]
    fn infinities() {
        assert_exact!(Double::INFINITY, Double::INFINITY + Double::INFINITY);
        assert_exact!(
            Double::NEG_INFINITY,
            Double::NEG_INFINITY + Double::NEG_INFINITY
        );
        assert_exact!(Double::NAN, Double::INFINITY + Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY + Double::INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY + Double::NAN);
        assert_exact!(Double::NAN, Double::NEG_INFINITY + Double::NAN);
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN + Double::ONE);
        assert_exact!(Double::NAN, Double::ONE + Double::NAN);
    }
}
