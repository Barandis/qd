// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::ops::{Div, Rem, RemAssign};

impl Rem for Double {
    type Output = Double;

    /// Divides this `Double` by another, producing a new `Double` of the remainder as a
    /// result.
    ///
    /// This implements the `%` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI % Double::E;
    /// let expected = dd!("0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = self.div(other).trunc();
        self - other * n
    }
}

impl Rem for &Double {
    type Output = Double;

    /// Divides a reference to this `Double` by another, producing a new `Double` of the
    /// remainder as a result.
    ///
    /// This implements the `%` operator between two references to `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::PI % &Double::E;
    /// let expected = dd!("0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: &Double) -> Double {
        let n = self.div(*other).trunc();
        *self - *other * n
    }
}

impl Rem<&Double> for Double {
    type Output = Double;

    /// Divides this `Double` by a reference to another, producing a new `Double` of the
    /// remainder as a result.
    ///
    /// This implements the `%` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI % &Double::E;
    /// let expected = dd!("0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: &Double) -> Double {
        let n = self.div(*other).trunc();
        self - *other * n
    }
}

impl Rem<Double> for &Double {
    type Output = Double;

    /// Divides a reference to this `Double` by another `Double`, producing a new `Double`
    /// of the remainder as a result.
    ///
    /// This implements the `%` operator between a reference to a `Double`s and a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::PI % Double::E;
    /// let expected = dd!("0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn rem(self, other: Double) -> Double {
        let n = self.div(other).trunc();
        *self - other * n
    }
}

impl RemAssign for Double {
    /// Divides this `Double` by another, modifying this one to equal the remainder.
    /// 
    /// This implements the `%=` operator between two `Double`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::PI;
    /// x %= Double::E;
    /// let expected = dd!("0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Double) {
        let (a, b) = self.rem(other).into();
        self.0 = a;
        self.1 = b;
    }
}

impl RemAssign<&Double> for Double {
    /// Divides this `Double` by a reference to another, modifying this one to equal the
    /// remainder.
    ///
    /// This implements the `%=` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::PI;
    /// x %= &Double::E;
    /// let expected = dd!("0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Double) {
        let (a, b) = self.rem(*other).into();
        self.0 = a;
        self.1 = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::op_ref)]
    fn num_num() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, Double::PI % Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_ref() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, &Double::PI % &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, Double::PI % &Double::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = dd!("0.42331082513074800310235591192684");
        assert_close!(expected, &Double::PI % Double::E);
    }

    #[test]
    fn assign_num() {
        let expected = dd!("0.42331082513074800310235591192684");
        let mut a = Double::PI;
        a %= Double::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = dd!("0.42331082513074800310235591192684");
        let mut b = Double::PI;
        b %= &Double::E;
        assert_close!(expected, b);
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::NAN % Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO % Double::NAN);
        assert_exact!(Double::NAN, Double::INFINITY % Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO % Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::ZERO);
        assert_exact!(Double::NAN, Double::ZERO % Double::NEG_INFINITY);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY % Double::ONE);
        assert_exact!(Double::NAN, Double::ONE % Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::ONE);
        assert_exact!(Double::NAN, Double::ONE % Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY % Double::INFINITY);
        assert_exact!(Double::NAN, Double::INFINITY % Double::NEG_INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::INFINITY);
        assert_exact!(Double::NAN, Double::NEG_INFINITY % Double::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN % Double::NAN);
        assert_exact!(Double::NAN, Double::NAN % Double::ONE);
        assert_exact!(Double::NAN, Double::ONE % Double::NAN);
    }
}
