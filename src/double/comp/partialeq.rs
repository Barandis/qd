// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl PartialEq for Double {
    /// Implements the `==` and `!= operators, testing two double-doubles for equality and
    /// inequality.
    ///
    /// Equality works exactly the same as it does for system floating-point numbers (`f64`,
    /// etc.), including zero equalling negative zero, `NaN` equalling nothing (including
    /// itself), etc. Notably, equality should be used with care since floating-point
    /// rounding, even with the increased precision of double-doubles, will still cause some
    /// numbers that should be equal to not be equal.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(Double::PI == Double::PI);
    /// assert!(Double::E != Double::PI);
    /// assert!(dd!(0.0) == dd!(-0.0));
    /// assert!(Double::NAN != Double::NAN);
    /// # }
    /// ```
    #[inline]
    fn eq(&self, other: &Double) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::eq_op)]
    fn equal() {
        assert!(Double::PI == Double::PI);
    }

    #[test]
    fn not_equal() {
        assert!(Double::PI != Double::E);
    }

    #[test]
    fn zero() {
        assert!(Double::ZERO == Double::NEG_ZERO);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn infinity() {
        assert!(Double::INFINITY == Double::INFINITY);
        assert!(Double::NEG_INFINITY == Double::NEG_INFINITY);
        assert!(Double::INFINITY != Double::NEG_INFINITY);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn nan() {
        assert!(Double::NAN != Double::NAN);
    }
}
