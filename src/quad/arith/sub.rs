// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Neg, Sub, SubAssign};

impl Neg for Quad {
    type Output = Quad;

    #[inline]
    fn neg(self) -> Quad {
        Quad(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Sub for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: Quad) -> Quad {
        Quad::from(self.add_quad(-other))
    }
}

impl<'a> Sub<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        Quad::from(self.add_quad(-*other))
    }
}

impl<'a> Sub<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: Quad) -> Quad {
        Quad::from(self.add_quad(-other))
    }
}

impl SubAssign for Quad {
    #[inline]
    fn sub_assign(&mut self, other: Quad) {
        let (a, b, c, d) = self.add_quad(-other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

impl<'a> SubAssign<&'a Quad> for Quad {
    #[inline]
    fn sub_assign(&mut self, other: &Quad) {
        let (a, b, c, d) = self.add_quad(-*other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}
