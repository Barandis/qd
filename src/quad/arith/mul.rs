// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::quad::Quad;
use std::ops::{Mul, MulAssign};

impl Mul for Quad {
    type Output = Quad;

    // This is complicated.
    //
    // It closely follows the process described on pp. 11-16 of "Library for Double-Double
    // and  Quad-Double Arithmetic" by Y. Hida, X.S. Li, and D.H. Bailey which can be found
    // at http://web.mit.edu/tabbott/Public/quadQuad-debian/qd-2.3.4-old/docs/qd.pdf. You
    // should be able to see the way the source code works from the diagrams there.
    //
    // TERMS (a = self, b = other):
    // Order   Components   Group (hx, lx)
    // O(1)    a0 * b0      0
    // O(ε)    a0 * b1      1
    //         a1 * b0      2
    // O(ε²)   a0 * b2      3
    //         a1 * b1      4
    //         a2 * b0      5
    // O(ε³)   a0 * b3      6
    //         a1 * b2      7
    //         a2 * b1      8
    //         a3 * b0      9
    // O(ε⁴)   a1 * b3      a  (high word only)
    //         a2 * b2      b  (high word only)
    //         a3 * b1      c  (high word only)
    //
    // Other terms, including the remaining O(ε⁴) terms and the low words of the O(ε⁴) that
    // are calculated, are not necessary to provide 212 bits of accuracy.

    /// Multiplies this `Quad` by another, producing a new `Quad` as a result.
    ///
    /// This implements the `*` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E * Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: Quad) -> Quad {
        if self.is_nan() || other.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
            if other.is_infinite() {
                Quad::NAN
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Quad::ZERO
            } else {
                Quad::NEG_ZERO
            }
        } else if self.is_infinite() {
            if other.is_zero() {
                Quad::NAN
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::NEG_INFINITY
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::NEG_INFINITY
            }
        } else {
            // O(1) term
            let (h0, l0) = core::two_prod(self.0, other.0);

            // O(ε) terms
            let (h1, l1) = core::two_prod(self.0, other.1);
            let (h2, l2) = core::two_prod(self.1, other.0);

            // O(ε²) terms
            let (h3, l3) = core::two_prod(self.0, other.2);
            let (h4, l4) = core::two_prod(self.1, other.1);
            let (h5, l5) = core::two_prod(self.2, other.0);

            // O(ε³) terms
            let (h6, l6) = core::two_prod(self.0, other.3);
            let (h7, l7) = core::two_prod(self.1, other.2);
            let (h8, l8) = core::two_prod(self.2, other.1);
            let (h9, l9) = core::two_prod(self.3, other.0);

            // O(ε⁴) terms - the low words aren't necessary for the accuracy we need
            let ha = self.1 * other.3;
            let hb = self.2 * other.2;
            let hc = self.3 * other.1;

            // Each calculation takes all of the high words for the terms of that level,
            // whatever intermediate words are specified by the algorithm, and whatever low
            // words fit in the remaining input space.

            // O(1) calculation (pass-through)
            let r0 = h0;
            // O(ε) calculation
            let (r1, t0, t1) = core::three_three_sum(h1, h2, l0);
            // O(ε²) calculation
            let (r2, t2, t3) = core::six_three_sum(t0, h3, h4, h5, l1, l2);
            // O(ε³) calculation
            let (r3, t4) = core::nine_two_sum(t1, t2, h6, h7, h8, h9, l3, l4, l5);
            // O(ε⁴) calculation (nine_one_sum)
            let r4 = t3 + t4 + ha + hb + hc + l6 + l7 + l8 + l9;

            // Results of the prior calculations are renormalized into four f64s.
            let (a, b, c, d) = core::renorm5(r0, r1, r2, r3, r4);
            Quad(a, b, c, d)
        }
    }
}

impl Mul for &Quad {
    type Output = Quad;

    /// Multiplies a reference to this `Quad` by another, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the `*` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E * &Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn mul(self, other: &Quad) -> Quad {
        (*self).mul(*other)
    }
}

impl Mul<&Quad> for Quad {
    type Output = Quad;

    /// Multiplies this `Quad` by a reference to another, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the `*` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E * &Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn mul(self, other: &Quad) -> Quad {
        self.mul(*other)
    }
}

impl Mul<Quad> for &Quad {
    type Output = Quad;

    /// Multiplies a reference to this `Quad` by another `Quad`, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the `*` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E * Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn mul(self, other: Quad) -> Quad {
        (*self).mul(other)
    }
}

impl MulAssign for Quad {
    /// Multiples this `Quad` by another one, modifying this one to equal the result.
    ///
    /// This implements the `*=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x *= Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Quad) {
        let r = self.mul(other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl MulAssign<&Quad> for Quad {
    /// Multiples this `Quad` by a reference to another one, modifying this one to equal the
    /// result.
    ///
    /// This implements the `*=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x *= &Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Quad) {
        let r = self.mul(*other);
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
        let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
        assert_close!(expected, Quad::PI * Quad::E);
    }

    #[test]
    fn ref_ref() {
        let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
        assert_close!(expected, &Quad::PI * &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
        assert_close!(expected, Quad::PI * &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
        assert_close!(expected, &Quad::PI * Quad::E);
    }

    #[test]
    fn assign_num() {
        let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");

        let mut a = Quad::PI;
        a *= Quad::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");

        let mut b = Quad::PI;
        b *= &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::NAN * Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO * Quad::NAN);
        assert_exact!(Quad::NAN, Quad::INFINITY * Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO * Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY * Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO * Quad::NEG_INFINITY);
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY * Quad::ONE);
        assert_exact!(Quad::INFINITY, Quad::ONE * Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY * Quad::ONE);
        assert_exact!(Quad::NEG_INFINITY, Quad::ONE * Quad::NEG_INFINITY);
        assert_exact!(Quad::INFINITY, Quad::INFINITY * Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::INFINITY * Quad::NEG_INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY * Quad::INFINITY);
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY * Quad::NEG_INFINITY);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN * Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE * Quad::NAN);
    }
}
