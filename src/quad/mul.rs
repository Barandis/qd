// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::quad::Quad;
use std::ops::{Mul, MulAssign};

impl Mul for Quad {
    type Output = Quad;

    // This is complicated.
    //
    // It closely follows the process described on pp. 11-16 of "Library for Double-Double
    // and  Quad-Double Arithmetic" by Y. Hida, X.S. Li, and D.H. Bailey which can be found
    // at http://web.mit.edu/tabbott/Public/quaddouble-debian/qd-2.3.4-old/docs/qd.pdf. You
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

    /// Computes $x \times y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `*` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E * Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, other: Quad) -> Quad {
        match self.pre_mul(&other) {
            Some(r) => r,
            None => {
                // O(1) term
                let (h0, l0) = p::two_prod(self.0, other.0);

                // O(ε) terms
                let (h1, l1) = p::two_prod(self.0, other.1);
                let (h2, l2) = p::two_prod(self.1, other.0);

                // O(ε²) terms
                let (h3, l3) = p::two_prod(self.0, other.2);
                let (h4, l4) = p::two_prod(self.1, other.1);
                let (h5, l5) = p::two_prod(self.2, other.0);

                // O(ε³) terms
                let (h6, l6) = p::two_prod(self.0, other.3);
                let (h7, l7) = p::two_prod(self.1, other.2);
                let (h8, l8) = p::two_prod(self.2, other.1);
                let (h9, l9) = p::two_prod(self.3, other.0);

                // O(ε⁴) terms - the low words aren't necessary for the accuracy we need
                let ha = self.1 * other.3;
                let hb = self.2 * other.2;
                let hc = self.3 * other.1;

                // Each calculation takes all of the high words for the terms of that level,
                // whatever intermediate words are specified by the algorithm, and whatever
                // low words fit in the remaining input space.

                // O(1) calculation (pass-through)
                let r0 = h0;
                // O(ε) calculation
                let (r1, t0, t1) = u::three_three_sum(h1, h2, l0);
                // O(ε²) calculation
                let (r2, t2, t3) = u::six_three_sum(t0, h3, h4, h5, l1, l2);
                // O(ε³) calculation
                let (r3, t4) = u::nine_two_sum(t1, t2, h6, h7, h8, h9, l3, l4, l5);
                // O(ε⁴) calculation (nine_one_sum)
                let r4 = t3 + t4 + ha + hb + hc + l6 + l7 + l8 + l9;

                // Results of the prior calculations are renormalized into four f64s.
                let (a, b, c, d) = u::renorm5(r0, r1, r2, r3, r4);
                Quad(a, b, c, d)
            }
        }
    }
}

impl Mul for &Quad {
    type Output = Quad;

    /// Computes $x \times y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `*` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E * &Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn mul(self, other: &Quad) -> Quad {
        (*self).mul(*other)
    }
}

impl Mul<&Quad> for Quad {
    type Output = Quad;

    /// Computes $x \times y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `*` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E * &Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn mul(self, other: &Quad) -> Quad {
        self.mul(*other)
    }
}

impl Mul<Quad> for &Quad {
    type Output = Quad;

    /// Computes $x \times y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `*` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E * Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn mul(self, other: Quad) -> Quad {
        (*self).mul(other)
    }
}

impl MulAssign for Quad {
    /// Computes $x \times y$, where $x$ is `self` and $y$ is the argument, assigning the
    /// result to `self`.
    ///
    /// This implements the `*=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x *= Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
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
    /// Computes $x \times y$, where $x$ is `self` and $y$ is the argument, assigning the
    /// result to `self`.
    ///
    /// This implements the `*=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x *= &Quad::PI;
    /// let expected = qd!("8.539734222673567065463550869546574495034888535765114961879601130");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
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

impl Quad {
    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_mul(&self, other: &Quad) -> Option<Quad> {
        if self.is_nan() || other.is_nan() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            if other.is_infinite() {
                Some(Quad::NAN)
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Some(Quad::ZERO)
            } else {
                Some(Quad::NEG_ZERO)
            }
        } else if self.is_infinite() {
            if other.is_zero() {
                Some(Quad::NAN)
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // mul tests
    test_all_near!(
        num_num:
            qd!("8.5397342226735670654635508695465744950348885357651149618796011301762"),
            Quad::PI * Quad::E;
        num_ref:
            qd!("8.5397342226735670654635508695465744950348885357651149618796011301762"),
            Quad::PI * &Quad::E;
        ref_num:
            qd!("8.5397342226735670654635508695465744950348885357651149618796011301762"),
            &Quad::PI * Quad::E;
        ref_ref:
            qd!("8.5397342226735670654635508695465744950348885357651149618796011301762"),
            &Quad::PI * &Quad::E;
        num_neg_num:
            qd!("-8.5397342226735670654635508695465744950348885357651149618796011301762"),
            Quad::PI * -Quad::E;
        num_neg_ref:
            qd!("-8.5397342226735670654635508695465744950348885357651149618796011301762"),
            Quad::PI * -&Quad::E;
        ref_neg_num:
            qd!("-8.5397342226735670654635508695465744950348885357651149618796011301762"),
            &Quad::PI * -Quad::E;
        ref_neg_ref:
            qd!("-8.5397342226735670654635508695465744950348885357651149618796011301762"),
            &Quad::PI * -&Quad::E;
        num_id:
            Quad::PI,
            Quad::PI * Quad::ONE;
        id_num:
            Quad::PI,
            Quad::ONE * Quad::PI;
        num_small:
            qd!("3.1415926535897932384626433832795028841971693993751058209749445923069e-60"),
            Quad::PI * qd!("1e-60");
        small_num:
            qd!("3.1415926535897932384626433832795028841971693993751058209749445923069e-60"),
            qd!("1e-60") * Quad::PI;
        three_nums:
            qd!("5.9192926991774591936228124210310520055594093367707051307052021348108"),
            Quad::PI * Quad::E * Quad::LN_2;
        lassoc:
            qd!("5.9192926991774591936228124210310520055594093367707051307052021348108"),
            (Quad::PI * Quad::E) * Quad::LN_2;
        rassoc:
            qd!("5.9192926991774591936228124210310520055594093367707051307052021348108"),
            Quad::PI * (Quad::LN_2 * Quad::E);
    );
    test_all_exact!(
        nan_zero:
            Quad::NAN,
            Quad::NAN * Quad::ZERO;
        zero_nan:
            Quad::NAN,
            Quad::ZERO * Quad::NAN;
        inf_zero:
            Quad::NAN,
            Quad::INFINITY * Quad::ZERO;
        zero_inf:
            Quad::NAN,
            Quad::ZERO * Quad::INFINITY;
        inf_neg_zero:
            Quad::NAN,
            Quad::NEG_INFINITY * Quad::ZERO;
        zero_neg_inf:
            Quad::NAN,
            Quad::ZERO * Quad::NEG_INFINITY;

        inf_one:
            Quad::INFINITY,
            Quad::INFINITY * Quad::ONE;
        one_inf:
            Quad::INFINITY,
            Quad::ONE * Quad::INFINITY;
        neg_inf_one:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY * Quad::ONE;
        one_neg_inf:
            Quad::NEG_INFINITY,
            Quad::ONE * Quad::NEG_INFINITY;
        inf_inf:
            Quad::INFINITY,
            Quad::INFINITY * Quad::INFINITY;
        inf_neg_inf:
            Quad::NEG_INFINITY,
            Quad::INFINITY * Quad::NEG_INFINITY;
        neg_inf_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY * Quad::INFINITY;
        neg_inf_neg_inf:
            Quad::INFINITY,
            Quad::NEG_INFINITY * Quad::NEG_INFINITY;

        nan_one:
            Quad::NAN,
            Quad::NAN * Quad::ONE;
        one_nan:
            Quad::NAN,
            Quad::ONE * Quad::NAN;
    );

    // Assign tests. Assign code delegates to mul code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Quad::PI;
            a *= Quad::E;
            near!(qd!("8.5397342226735670654635508695465744950348885357651149618796011301762"), a);
        }
        assign_ref: {
            let mut b = Quad::PI;
            b *= &Quad::E;
            near!(qd!("8.5397342226735670654635508695465744950348885357651149618796011301762"), b);
        }
    );
}
