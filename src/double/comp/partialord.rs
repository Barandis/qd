// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::cmp::Ordering;

impl PartialOrd for Double {
    /// Implements the `<`, `>`, `<=`, and `>=` operators, testing two `Double`s for
    /// ordering.
    ///
    /// Ordering works the same as it does for system floating-point numbers, including
    /// [`NAN`] returning false for any of these operators (including when comparing it to
    /// itself).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(Double::PI > Double::E);
    /// assert!(dd!(0.0) <= dd!(-0.0));
    /// assert!(!(Double::NAN >= Double::NAN));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    fn partial_cmp(&self, other: &Double) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            x => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn gt() {
        assert!(Double::PI > Double::E);
        assert!(!(Double::PI > Double::PI));
        assert!(!(Double::E > Double::PI));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn gte() {
        assert!(Double::PI >= Double::E);
        assert!(Double::PI >= Double::PI);
        assert!(!(Double::E >= Double::PI));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn lt() {
        assert!(Double::E < Double::PI);
        assert!(!(Double::E < Double::E));
        assert!(!(Double::PI < Double::E));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn lte() {
        assert!(Double::E <= Double::PI);
        assert!(Double::E <= Double::E);
        assert!(!(Double::PI <= Double::E));
    }

    #[test]
    fn zero() {
        assert!(Double::ZERO <= Double::NEG_ZERO);
        assert!(Double::ZERO >= Double::NEG_ZERO);
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn infinity() {
        assert!(Double::NEG_INFINITY < Double::INFINITY);
        assert!(Double::NEG_INFINITY <= Double::NEG_INFINITY);
        assert!(Double::NEG_INFINITY >= Double::NEG_INFINITY);
        assert!(!(Double::NEG_INFINITY > Double::NEG_INFINITY));
    }

    #[test]
    #[allow(clippy::eq_op, clippy::neg_cmp_op_on_partial_ord)]
    fn nan() {
        assert!(!(Double::NAN < Double::NAN));
        assert!(!(Double::NAN <= Double::NAN));
        assert!(!(Double::NAN > Double::NAN));
        assert!(!(Double::NAN >= Double::NAN));
    }
}
