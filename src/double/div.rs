// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::double::Double;
use std::ops::{Div, DivAssign};

// Helper function needed to avoid the only place in this arithmetic where Double::from must
// be called on a non-tuple, non-integer number. With the current parsing of floats, calling
// Double::from this way in the basic arithmetic would cause a stack overflow.
#[inline]
fn mul_f64(a: Double, b: f64) -> Double {
    let (p, e) = core::two_prod(a.0, b);
    let (a, b) = core::renorm2(p, e + a.1 * b);
    Double(a, b)
}

impl Double {
    /// Creates a new `Double` representing the quotient of two floats.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
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

            let (p1, p2) = core::two_prod(q1, b);
            let (s, e) = core::two_diff(a, p1);

            let q2 = (s + e - p2) / b;

            let (a, b) = core::renorm2(q1, q2);
            Double(a, b)
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Double {
    type Output = Double;

    /// Divides this `Double` by another, producing a new `Double` as a result.
    ///
    /// This implements the `/` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E / Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    fn div(self, other: Double) -> Double {
        if self.is_nan() || other.is_nan() {
            Double::NAN
        } else if other.is_zero() {
            if self.is_zero() {
                Double::NAN
            } else if self.is_sign_negative() == other.is_sign_positive() {
                Double::NEG_INFINITY
            } else {
                Double::INFINITY
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Double::NAN
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Double::ZERO
            } else {
                Double::NEG_ZERO
            }
        } else {
            let q1 = self.0 / other.0;
            let mut r = self - mul_f64(other, q1);

            let q2 = r.0 / other.0;
            r -= mul_f64(other, q2);

            let q3 = r.0 / other.0;

            let (a, b) = core::renorm3(q1, q2, q3);
            Double(a, b)
        }
    }
}

impl Div for &Double {
    type Output = Double;

    /// Divides a reference to this `Double` by another, producing a new `Double` as a result.
    ///
    /// This implements the `/` operator between two references to `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::E / &Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    fn div(self, other: &Double) -> Double {
        (*self).div(*other)
    }
}

impl Div<&Double> for Double {
    type Output = Double;

    /// Divides this `Double` by a reference to another `Double`, producing a new `Double`
    /// as a result.
    ///
    /// This implements the `/` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E / &Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn div(self, other: &Double) -> Double {
        self.div(*other)
    }
}

impl Div<Double> for &Double {
    type Output = Double;

    /// Divides a reference to this `Double` by another `Double`, producing a new `Double`
    /// as a result.
    ///
    /// This implements the `/` operator between a reference to a `Double` and a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::E / Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn div(self, other: Double) -> Double {
        (*self).div(other)
    }
}

impl DivAssign for Double {
    /// Divides this `Double` by another, modifying this one to equal the result.
    ///
    /// This implements the `/=` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::E;
    /// x /= Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Double) {
        let r = self.div(other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl DivAssign<&Double> for Double {
    /// Divides this `Double` by a reference to another, modifying this one to equal the
    /// result.
    ///
    /// This implements the `/=` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::E;
    /// x /= &Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Double) {
        let r = self.div(*other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = dd!("1.1557273497909217179100931833127");
        assert_close!(expected, Double::PI / Double::E);
    }

    #[test]
    fn ref_ref() {
        let expected = dd!("1.1557273497909217179100931833127");
        assert_close!(expected, &Double::PI / &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = dd!("1.1557273497909217179100931833127");
        assert_close!(expected, Double::PI / &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = dd!("1.1557273497909217179100931833127");
        assert_close!(expected, &Double::PI / Double::E);
    }

    #[test]
    fn assign_num() {
        let expected = dd!("1.1557273497909217179100931833127");
        let mut a = Double::PI;
        a /= Double::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = dd!("1.1557273497909217179100931833127");
        let mut b = Double::PI;
        b /= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO / Double::INFINITY);
        assert_exact!(Double::NEG_ZERO, Double::ZERO / Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY / Double::ZERO);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY / Double::ZERO);
        assert_exact!(Double::NAN, Double::NAN / Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO / Double::NAN);
        assert_exact!(Double::NAN, Double::ZERO / Double::ZERO);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn inf() {
        assert_exact!(Double::ZERO, Double::ONE / Double::INFINITY);
        assert_exact!(Double::NEG_ZERO, Double::ONE / Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::INFINITY / Double::ONE);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY / Double::ONE);
        assert_exact!(Double::NAN, Double::INFINITY / Double::INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY / Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY / Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY / Double::NEG_INFINITY);
        assert_exact!(Double::INFINITY, Double::ONE / Double::ZERO);
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ONE / Double::ZERO);
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN / Double::ONE);
        assert_exact!(Double::NAN, Double::ONE / Double::NAN);
    }
}
