// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::Neg;

impl Neg for Quad {
    type Output = Quad;

    /// Negates this `Quad`, producing a new `Quad`.
    /// 
    /// This implements the unary `-` operator for `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = -Quad::PI;
    /// let expected = qd!("-3.141592653589793238462643383279502884197169399375105820974944592");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(x < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn neg(self) -> Quad {
        Quad(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Neg for &Quad {
    type Output = Quad;

    /// Negates a reference to this `Quad`, producing a new `Quad`.
    /// 
    /// This implements the unary `-` operator for references to `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = -&Quad::PI;
    /// let expected = qd!("-3.141592653589793238462643383279502884197169399375105820974944592");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(x < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn neg(self) -> Quad {
        Quad(-(*self).0, -(*self).1, -(*self).2, -(*self).3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        assert_close!(
            qd!("-3.141592653589793238462643383279502884197169399375105820974944592"),
            -Quad::PI
        );
        assert_close!(
            qd!("-2.718281828459045235360287471352662497757247093699959574966967628"),
            -Quad::E
        );
    }

    #[test]
    fn neg_ref() {
        assert_close!(
            qd!("-3.141592653589793238462643383279502884197169399375105820974944592"),
            -&Quad::PI
        );
        assert_close!(
            qd!("-2.718281828459045235360287471352662497757247093699959574966967628"),
            -&Quad::E
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NEG_ZERO, -Quad::ZERO);
        assert_exact!(Quad::ZERO, -Quad::NEG_ZERO);
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::NEG_INFINITY, -Quad::INFINITY);
        assert_exact!(Quad::INFINITY, -Quad::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, -Quad::NAN);
    }
}
