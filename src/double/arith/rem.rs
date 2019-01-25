// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::ops::{Div, Rem, RemAssign};

impl Rem for Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = self.div(other).trunc();
        self - other * n
    }
}

impl<'a> Rem<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: &Double) -> Double {
        let n = self.div(*other).trunc();
        self - *other * n
    }
}

impl<'a> Rem<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = self.div(other).trunc();
        (*self) - other * n
    }
}

impl RemAssign for Double {
    #[inline]
    fn rem_assign(&mut self, other: Double) {
        self.assign(self.rem(other).into());
    }
}

impl<'a> RemAssign<&'a Double> for Double {
    #[inline]
    fn rem_assign(&mut self, other: &Double) {
        self.assign(self.rem(*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, Double::PI % Double::E);
        assert_close!(expected, Double::PI % &Double::E);
        assert_close!(expected, &Double::PI % Double::E);

        let mut a = Double::PI;
        a %= Double::E;
        assert_close!(expected, a);

        let mut b = Double::PI;
        b %= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn special() {
        assert_exact!(Double::NAN, Double::NAN % dd!(0));
        assert_exact!(Double::NAN, dd!(0) % Double::NAN);
        assert_exact!(Double::NAN, Double::NAN % dd!(1));
        assert_exact!(Double::NAN, dd!(1) % Double::NAN);
        assert_exact!(Double::NAN, Double::INFINITY % dd!(1));
        assert_exact!(Double::NAN, dd!(1) % Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % dd!(1));
        assert_exact!(Double::NAN, dd!(1) % Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY % Double::INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY % Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY % Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO % Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO % Double::NEG_INFINITY);
    }
}
