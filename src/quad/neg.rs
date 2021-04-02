// Copyright (c) 2021 Thomas Otterson
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
    /// # use qd::{qd, Quad};
    /// let x = -Quad::PI;
    /// let expected = qd!("-3.141592653589793238462643383279502884197169399375105820974944592");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(x < qd!(1e-60));
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
    /// # use qd::{qd, Quad};
    /// let x = -&Quad::PI;
    /// let expected = qd!("-3.141592653589793238462643383279502884197169399375105820974944592");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(x < qd!(1e-60));
    /// ```
    #[inline]
    fn neg(self) -> Quad {
        Quad(-(*self).0, -(*self).1, -(*self).2, -(*self).3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_all_near!(
        num_pi:
            qd!("-3.1415926535897932384626433832795028841971693993751058209749445923069"),
            -Quad::PI;
        num_e:
            qd!("-2.7182818284590452353602874713526624977572470936999595749669676277239"),
            -Quad::E;
        ref_pi:
            qd!("-3.1415926535897932384626433832795028841971693993751058209749445923069"),
            -&Quad::PI;
        ref_e:
            qd!("-2.7182818284590452353602874713526624977572470936999595749669676277239"),
            -&Quad::E;
    );
    test_all_exact!(
        zero:
            Quad::NEG_ZERO,
            -Quad::ZERO;
        neg_zero:
            Quad::ZERO,
            -Quad::NEG_ZERO;
        inf:
            Quad::NEG_INFINITY,
            -Quad::INFINITY;
        neg_inf:
            Quad::INFINITY,
            -Quad::NEG_INFINITY;
        nan:
            Quad::NAN,
            -Quad::NAN;
    );
}
