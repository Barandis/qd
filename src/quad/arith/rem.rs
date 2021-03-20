// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Div, Rem, RemAssign};

impl Rem for Quad {
    type Output = Quad;

    /// Divides this `Quad` by another, producing a new `Quad` of the remainder as a
    /// result.
    ///
    /// This implements the `%` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI % Quad::E;
    /// let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: Quad) -> Quad {
        let n = self.div(other).trunc();
        self - other * n
    }
}

impl Rem for &Quad {
    type Output = Quad;

    /// Divides a reference to this `Quad` by another, producing a new `Quad` of the
    /// remainder as a result.
    ///
    /// This implements the `%` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::PI % &Quad::E;
    /// let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: &Quad) -> Quad {
        (*self).rem(*other)
    }
}

impl Rem<&Quad> for Quad {
    type Output = Quad;

    /// Divides this `Quad` by a reference to another, producing a new `Quad` of the
    /// remainder as a result.
    ///
    /// This implements the `%` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI % &Quad::E;
    /// let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: &Quad) -> Quad {
        self.rem(*other)
    }
}

impl Rem<Quad> for &Quad {
    type Output = Quad;

    /// Divides a reference to this `Quad` by another `Quad`, producing a new `Quad` of the
    /// remainder as a result.
    ///
    /// This implements the `%` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::PI % Quad::E;
    /// let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: Quad) -> Quad {
        (*self).rem(other)
    }
}

impl RemAssign for Quad {
    /// Divides this `Quad` by another, modifying this one to equal the remainder.
    /// 
    /// This implements the `%=` operator between two `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::PI;
    /// x %= Quad::E;
    /// let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Quad) {
        let r = self.rem(other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl RemAssign<&Quad> for Quad {
    /// Divides this `Quad` by a reference to another, modifying this one to equal the
    /// remainder.
    ///
    /// This implements the `%=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::PI;
    /// x %= &Quad::E;
    /// let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Quad) {
        let r = self.rem(*other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
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
    fn ref_ref() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, &Quad::PI % &Quad::E);
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
