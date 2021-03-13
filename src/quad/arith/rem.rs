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
    fn num_num() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, Quad::PI % Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, Quad::PI % &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, &Quad::PI % Quad::E);
    }

    #[test]
    fn assign_num() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");

        let mut a = Quad::PI;
        a %= Quad::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");

        let mut b = Quad::PI;
        b %= &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::NAN % Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO % Quad::NAN);
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO % Quad::NEG_INFINITY);
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE % Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY % Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY % Quad::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN % Quad::NAN);
        assert_exact!(Quad::NAN, Quad::NAN % Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE % Quad::NAN);
    }
}
