// Copyright (c) 2021 Thomas Otterson
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
    /// # use qd::{dd, Double};
    /// let x = -Double::PI;
    /// let expected = dd!("-3.1415926535897932384626433832795");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(x < dd!(1e-30));
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
    /// # use qd::{dd, Double};
    /// let x = -&Double::PI;
    /// let expected = dd!("-3.1415926535897932384626433832795");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(x < dd!(1e-30));
    /// ```
    #[inline]
    fn neg(self) -> Double {
        Double(-(*self).0, -(*self).1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_all_near!(
        num_pi:
            dd!("-3.1415926535897932384626433832795028"),
            -Double::PI;
        num_e:
            dd!("-2.7182818284590452353602874713526615"),
            -Double::E;
        ref_pi:
            dd!("-3.1415926535897932384626433832795028"),
            -&Double::PI;
        ref_e:
            dd!("-2.7182818284590452353602874713526615"),
            -&Double::E;
    );
    test_all_exact!(
        zero:
            Double::NEG_ZERO,
            -Double::ZERO;
        neg_zero:
            Double::ZERO,
            -Double::NEG_ZERO;
        inf:
            Double::NEG_INFINITY,
            -Double::INFINITY;
        neg_inf:
            Double::INFINITY,
            -Double::NEG_INFINITY;
        nan:
            Double::NAN,
            -Double::NAN;
    );
}
