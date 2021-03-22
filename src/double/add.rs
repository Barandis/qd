// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::double::Double;
use std::ops::{Add, AddAssign};

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Double {
    type Output = Double;

    /// Adds this `Double` to another, producing a new `Double` as a result.
    ///
    /// This implements the `+` operator between two `Double`s.
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
        match self.pre_add(&other) {
            Some(r) => r,
            None => {
                let (s0, e0) = p::two_sum(self.0, other.0);
                let (s1, e1) = p::two_sum(self.1, other.1);
                let (s2, e2) = p::quick_two_sum(s0, s1 + e0);
                let (a, b) = u::renorm2(s2, e1 + e2);
                Double(a, b)
            }
        }
    }
}

impl Add for &Double {
    type Output = Double;

    /// Adds a reference to this `Double` to another, producing a new `Double` as a result.
    ///
    /// This implements the `+` operator between two references to `Double`s.
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
    /// This implements the `+` operator between a `Double` and a reference to a `Double`.
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
    /// This implements the `+` operator between a refernce to a `Double` and a `Double`.
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
    /// This implements the `+=` operator between two `Double`s.
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
        let r = self.add(other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl AddAssign<&Double> for Double {
    /// Adds a reference to another `Double` to this `Double`, modifying this one to equal
    /// the result.
    ///
    /// This implements the `+=` operator between a `Double` and a reference to a `Double`.
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
        let r = self.add(*other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl Double {
    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_add(&self, other: &Double) -> Option<Double> {
        if self.is_nan() || other.is_nan() {
            Some(Double::NAN)
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Some(Double::INFINITY)
                    } else {
                        Some(Double::NAN)
                    }
                } else if other.is_sign_negative() {
                    Some(Double::NEG_INFINITY)
                } else {
                    Some(Double::NAN)
                }
            } else if self.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else {
            None
        }
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
    fn ref_ref() {
        let expected = dd!("5.8598744820488384738229308546322");
        assert_close!(expected, &Double::PI + &Double::E);
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
    fn inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY + Double::ONE);
        assert_exact!(Double::INFINITY, Double::ONE + Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY + Double::ONE);
        assert_exact!(Double::NEG_INFINITY, Double::ONE + Double::NEG_INFINITY);
    }

    #[test]
    fn infs() {
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
