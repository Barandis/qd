// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Div, Rem, RemAssign};

impl Rem for Quad {
    type Output = Quad;

    #[inline]
    fn rem(self, other: Quad) -> Quad {
        let n = self.div(other).trunc();
        self - other * n
    }
}

impl<'a> Rem<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn rem(self, other: &Quad) -> Quad {
        self.rem(*other)
    }
}

impl<'a> Rem<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn rem(self, other: Quad) -> Quad {
        (*self).rem(other)
    }
}

impl RemAssign for Quad {
    #[inline]
    fn rem_assign(&mut self, other: Quad) {
        self.assign(self.rem(other).into());
    }
}

impl<'a> RemAssign<&'a Quad> for Quad {
    #[inline]
    fn rem_assign(&mut self, other: &Quad) {
        self.assign(self.rem(*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        let expected = qd!("0.4233108251307480031023559119268403864399223056751462460079769646");
        assert_close!(expected, Quad::PI % Quad::E);
        assert_close!(expected, Quad::PI % &Quad::E);
        assert_close!(expected, &Quad::PI % Quad::E);

        let mut a = Quad::PI;
        a %= Quad::E;
        assert_close!(expected, a);

        let mut b = Quad::PI;
        b %= &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::NAN, Quad::NAN % qd!(0));
        assert_exact!(Quad::NAN, qd!(0) % Quad::NAN);
        assert_exact!(Quad::NAN, Quad::NAN % qd!(1));
        assert_exact!(Quad::NAN, qd!(1) % Quad::NAN);
        assert_exact!(Quad::NAN, Quad::INFINITY % qd!(1));
        assert_exact!(Quad::NAN, qd!(1) % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % qd!(1));
        assert_exact!(Quad::NAN, qd!(1) % Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO % Quad::NEG_INFINITY);
    }
}
