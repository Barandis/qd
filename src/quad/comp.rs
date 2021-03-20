// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::cmp::Ordering;

impl PartialEq for Quad {
    /// Implements the `==` and `!= operators, testing two `Quad`s for equality and
    /// inequality.
    ///
    /// Equality works exactly the same as it does for system floating-point numbers (`f64`,
    /// etc.), including zero equalling negative zero, `NaN` equalling nothing (including
    /// itself), etc. Notably, equality should be used with care since floating-point
    /// rounding, even with the increased precision of `Quad`s, will still cause some
    /// numbers that should be equal to not be equal.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::PI == Quad::PI);
    /// assert!(Quad::E != Quad::PI);
    /// assert!(qd!(0.0) == qd!(-0.0));
    /// assert!(Quad::NAN != Quad::NAN);
    /// # }
    /// ```
    #[inline]
    fn eq(&self, other: &Quad) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl PartialOrd for Quad {
    /// Implements the `<`, `>`, `<=`, and `>=` operators, testing two `Quad`s for ordering.
    ///
    /// Ordering works the same as it does for system floating-point numbers, including
    /// `NaN` returning false for any of these operators (including when comparing it to
    /// itself).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::PI > Quad::E);
    /// assert!(qd!(0.0) <= qd!(-0.0));
    /// assert!(!(Quad::NAN >= Quad::NAN));
    /// # }
    /// ```
    ///
    #[inline]
    fn partial_cmp(&self, other: &Quad) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => match self.1.partial_cmp(&other.1) {
                Some(Ordering::Equal) => match self.2.partial_cmp(&other.2) {
                    Some(Ordering::Equal) => self.3.partial_cmp(&other.3),
                    x => x,
                },
                x => x,
            },
            x => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::eq_op)]
    fn equal() {
        assert!(Quad::PI == Quad::PI);
    }

    #[test]
    fn not_equal() {
        assert!(Quad::PI != Quad::E);
    }

    #[test]
    fn equal_zero() {
        assert!(Quad::ZERO == Quad::NEG_ZERO);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn equal_inf() {
        assert!(Quad::INFINITY == Quad::INFINITY);
        assert!(Quad::NEG_INFINITY == Quad::NEG_INFINITY);
        assert!(Quad::INFINITY != Quad::NEG_INFINITY);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn equal_nan() {
        assert!(Quad::NAN != Quad::NAN);
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn gt() {
        assert!(Quad::PI > Quad::E);
        assert!(!(Quad::PI > Quad::PI));
        assert!(!(Quad::E > Quad::PI));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn gte() {
        assert!(Quad::PI >= Quad::E);
        assert!(Quad::PI >= Quad::PI);
        assert!(!(Quad::E >= Quad::PI));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn lt() {
        assert!(Quad::E < Quad::PI);
        assert!(!(Quad::E < Quad::E));
        assert!(!(Quad::PI < Quad::E));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn lte() {
        assert!(Quad::E <= Quad::PI);
        assert!(Quad::E <= Quad::E);
        assert!(!(Quad::PI <= Quad::E));
    }

    #[test]
    fn ord_zero() {
        assert!(Quad::ZERO <= Quad::NEG_ZERO);
        assert!(Quad::ZERO >= Quad::NEG_ZERO);
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn ord_inf() {
        assert!(Quad::NEG_INFINITY < Quad::INFINITY);
        assert!(Quad::NEG_INFINITY <= Quad::NEG_INFINITY);
        assert!(Quad::NEG_INFINITY >= Quad::NEG_INFINITY);
        assert!(!(Quad::NEG_INFINITY > Quad::NEG_INFINITY));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn ord_nan() {
        assert!(!(Quad::NAN < Quad::NAN));
        assert!(!(Quad::NAN <= Quad::NAN));
        assert!(!(Quad::NAN > Quad::NAN));
        assert!(!(Quad::NAN >= Quad::NAN));
    }
}
