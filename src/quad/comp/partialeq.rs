// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

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
    fn zero() {
        assert!(Quad::ZERO == Quad::NEG_ZERO);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn infinity() {
        assert!(Quad::INFINITY == Quad::INFINITY);
        assert!(Quad::NEG_INFINITY == Quad::NEG_INFINITY);
        assert!(Quad::INFINITY != Quad::NEG_INFINITY);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn nan() {
        assert!(Quad::NAN != Quad::NAN);
    }
}
