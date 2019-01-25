// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::cmp::Ordering;

impl PartialOrd for Double {
    /// Implements the `<`, `>`, `<=`, and `>=` operators, testing two double-doubles for ordering.
    ///
    /// Ordering works the same as it does for system floating-point numbers, including `NaN`
    /// returning false for any of these operators (including when comparing it to itself).
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
    fn basic() {
        assert!(Double::PI > Double::E);
        assert!(Double::PI >= Double::E);
        assert!(Double::E < Double::PI);
        assert!(Double::E <= Double::PI);
    }

    #[test]
    fn special() {
        assert!(Double::NEG_INFINITY < Double::INFINITY);
        assert!(Double::NEG_INFINITY <= Double::NEG_INFINITY);
        assert!(Double::NEG_INFINITY >= Double::NEG_INFINITY);
        assert!(!(Double::NEG_INFINITY > Double::NEG_INFINITY));
        assert!(Double::ZERO <= Double::NEG_ZERO);
        assert!(Double::ZERO >= Double::NEG_ZERO);
        assert!(!(Double::NAN < Double::NAN));
        assert!(!(Double::NAN <= Double::NAN));
        assert!(!(Double::NAN > Double::NAN));
        assert!(!(Double::NAN >= Double::NAN));
    }
}

