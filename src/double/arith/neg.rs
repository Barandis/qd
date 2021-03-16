// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::ops::Neg;

impl Neg for Double {
    type Output = Double;

    /// Negates this `Double`, producing a new `Double`.
    /// 
    /// This implements the unary `-` operator for `Double`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = -Double::PI;
    /// let expected = dd!("-3.1415926535897932384626433832795");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(x < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn neg(self) -> Double {
        Double(-self.0, -self.1)
    }
}

impl Neg for &Double {
    type Output = Double;

    /// Negates a reference to this `Double`, producing a new `Double`.
    /// 
    /// This implements the unary `-` operator for references to `Double`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = -&Double::PI;
    /// let expected = dd!("-3.1415926535897932384626433832795");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(x < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn neg(self) -> Double {
        Double(-(*self).0, -(*self).1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neg() {
        assert_close!(dd!("-3.1415926535897932384626433832795"), -Double::PI);
        assert_close!(dd!("-2.7182818284590452353602874713527"), -Double::E);
    }

    #[test]
    fn neg_ref() {
        assert_close!(dd!("-3.1415926535897932384626433832795"), -&Double::PI);
        assert_close!(dd!("-2.7182818284590452353602874713527"), -&Double::E);
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NEG_ZERO, -Double::ZERO);
        assert_exact!(Double::ZERO, -Double::NEG_ZERO);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NEG_INFINITY, -Double::INFINITY);
        assert_exact!(Double::INFINITY, -Double::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, -Double::NAN);
    }
}
