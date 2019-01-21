// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::ops::{Rem, RemAssign};

impl Rem for Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = (self / other).trunc();
        self - other * n
    }
}

impl<'a> Rem<&'a Double> for Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: &Double) -> Double {
        let n = (self / *other).trunc();
        self - *other * n
    }
}

impl<'a> Rem<Double> for &'a Double {
    type Output = Double;

    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = (self / other).trunc();
        self - other * n
    }
}

impl RemAssign for Double {
    #[inline]
    fn rem_assign(&mut self, other: Double) {
        let a = *self % other;
        self.0 = a.0;
        self.1 = a.1;
    }
}

impl<'a> RemAssign<&'a Double> for Double {
    #[inline]
    fn rem_assign(&mut self, other: &Double) {
        let a = *self % *other;
        self.0 = a.0;
        self.1 = a.1;
    }
}
