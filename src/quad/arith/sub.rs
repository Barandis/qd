// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Add, Sub, SubAssign};

impl Sub for Quad {
    type Output = Quad;

    /// Subtracts another `Quad` from this one, producing a new `Quad` as a result.
    /// 
    /// This implements the binary `-` operator between two `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E - Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn sub(self, other: Quad) -> Quad {
        self.add(-other)
    }
}

impl Sub for &Quad {
    type Output = Quad;

    /// Subtracts a reference to another `Quad` from this one, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the binary `-` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E - &Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        (*self).add(-*other)
    }
}

impl Sub<&Quad> for Quad {
    type Output = Quad;

    /// Subtracts a reference to another `Quad` from this `Quad`, producing a new `Quad` as
    /// a result.
    ///
    /// This implements the binary `-` operator between a `Quad` and a reference to a
    /// `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E - &Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        self.add(-*other)
    }
}

impl Sub<Quad> for &Quad {
    type Output = Quad;

    /// Subtracts another `Quad` from a reference to this one, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the binary `-` operator between a reference to a `Quad` and a
    /// `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E - Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn sub(self, other: Quad) -> Quad {
        (*self).add(-other)
    }
}

impl SubAssign for Quad {
    /// Subtracts another `Quad` from this one, modifying this one to equal the result.
    /// 
    /// This implements the `-=` operator between two `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x -= Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Quad) {
        let r = self.add(-other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl SubAssign<&Quad> for Quad {
    /// Subtracts a reference to another `Quad` from this `Quad`, modifying this one to
    /// equal the result.
    ///
    /// This implements the `-=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x -= &Quad::PI;
    /// let expected = qd!("-0.4233108251307480031023559119268403864399223056751462460079769646");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Quad) {
        let r = self.add(-*other);
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
        assert_close!(expected, Quad::PI - Quad::E);
    }

    #[test]
    fn ref_ref() {
        let expected = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
        assert_close!(expected, &Quad::PI - &Quad::E);
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
