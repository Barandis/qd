// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Add, Sub, SubAssign};

impl Sub for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: Quad) -> Quad {
        self.add(-other)
    }
}

impl<'a> Sub<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        self.add(-*other)
    }
}

impl<'a> Sub<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: Quad) -> Quad {
        (*self).add(-other)
    }
}

impl SubAssign for Quad {
    #[inline]
    fn sub_assign(&mut self, other: Quad) {
        self.assign(self.add(-other).into());
    }
}

impl<'a> SubAssign<&'a Quad> for Quad {
    #[inline]
    fn sub_assign(&mut self, other: &Quad) {
        self.assign(self.add(-*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        let expected = qd!("0.4233108251307480031023559119268403864399223056751462460079769646");
        assert_close!(expected, Quad::PI - Quad::E);
        assert_close!(expected, Quad::PI - &Quad::E);
        assert_close!(expected, &Quad::PI - Quad::E);

        let mut a = Quad::PI;
        a -= Quad::E;
        assert_close!(expected, a);

        let mut b = Quad::PI;
        b -= &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::NAN, Quad::NAN - qd!(1));
        assert_exact!(Quad::NAN, qd!(1) - Quad::NAN);
        assert_exact!(Quad::INFINITY, Quad::INFINITY - qd!(1));
        assert_exact!(Quad::NEG_INFINITY, qd!(1) - Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY - qd!(1));
        assert_exact!(Quad::INFINITY, qd!(1) - Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY - Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY - Quad::NEG_INFINITY);
        assert_exact!(Quad::INFINITY, Quad::INFINITY - Quad::NEG_INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY - Quad::INFINITY);
    }
}
