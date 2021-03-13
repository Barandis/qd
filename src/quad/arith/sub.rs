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

impl Sub<&Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        self.add(-*other)
    }
}

impl Sub<Quad> for &Quad {
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

impl SubAssign<&Quad> for Quad {
    #[inline]
    fn sub_assign(&mut self, other: &Quad) {
        self.assign(self.add(-*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, Quad::PI - Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, Quad::PI - &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, &Quad::PI - Quad::E);
    }

    #[test]
    fn assign_num() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");

        let mut a = Quad::PI;
        a -= Quad::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");

        let mut b = Quad::PI;
        b -= &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY - Quad::ONE);
        assert_exact!(Quad::NEG_INFINITY, Quad::ONE - Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY - Quad::ONE);
        assert_exact!(Quad::INFINITY, Quad::ONE - Quad::NEG_INFINITY);
        assert_exact!(Quad::INFINITY, Quad::INFINITY - Quad::NEG_INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY - Quad::INFINITY);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN - Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE - Quad::NAN);
        assert_exact!(Quad::NAN, Quad::NAN - Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY - Quad::NAN);
        assert_exact!(Quad::NAN, Quad::INFINITY - Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY - Quad::NEG_INFINITY);
    }
}
