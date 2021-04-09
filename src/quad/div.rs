// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::quad::Quad;
use std::ops::{Div, DivAssign};

// Quad x f64 analogue of full quad x quad multiplication above. This is here because we
// don't want to depend on any Quad::from(x), where x is a single f64 (i.e., a non-tuple),
// in arithmetic. Doing so will create infinite loops because arithmetic is used to parse
// the f64s into quads in the first place. Multiplying the f64s directly into Quads bypasses
// this.
//
// Division is the only place where this is necessary, so this multiplication function is
// dropped nearby.
#[inline]
fn mul_f64(a: Quad, b: f64) -> Quad {
    let (h0, l0) = p::two_prod(a.0, b);
    let (h1, l1) = p::two_prod(a.1, b);
    let (h2, l2) = p::two_prod(a.2, b);
    let h3 = a.3 * b;

    let s0 = h0;
    let (s1, t0) = p::two_sum(h1, l0);
    let (s2, t1, t2) = u::three_three_sum(t0, h2, l1);
    let (s3, t3) = u::three_two_sum(t1, h3, l2);
    let s4 = t2 * t3;

    let (a, b, c, d) = u::renorm5(s0, s1, s2, s3, s4);
    Quad(a, b, c, d)
}

impl Div for Quad {
    type Output = Quad;

    /// Computes $x \div y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `/` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E / Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Quad) -> Quad {
        match self.pre_div(&other) {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // Divide the first component of `self` by the first component of `other`.
                // Then divide the first component of the remainder by the first component
                // of `other`, then the first component of -that- remainder by the first
                // component of `other`, and so on until we have five terms we can
                // renormalize.
                let q0 = self.0 / other.0;
                let mut r = self - mul_f64(other, q0);

                let q1 = r.0 / other.0;
                r -= mul_f64(other, q1);

                let q2 = r.0 / other.0;
                r -= mul_f64(other, q2);

                let q3 = r.0 / other.0;
                r -= mul_f64(other, q3);

                let q4 = r.0 / other.0;

                let (a, b, c, d) = u::renorm5(q0, q1, q2, q3, q4);
                Quad(a, b, c, d)
            }
        }
    }
}

impl Div for &Quad {
    type Output = Quad;

    /// Computes $x \div y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `/` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E / &Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn div(self, other: &Quad) -> Quad {
        (*self).div(*other)
    }
}

impl Div<&Quad> for Quad {
    type Output = Quad;

    /// Computes $x \div y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `/` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E / &Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn div(self, other: &Quad) -> Quad {
        self.div(*other)
    }
}

impl Div<Quad> for &Quad {
    type Output = Quad;

    /// Computes $x \div y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `/` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E / Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn div(self, other: Quad) -> Quad {
        (*self).div(other)
    }
}

impl DivAssign for Quad {
    /// Computes $x \div y$, where $x$ is `self` and $y$ is the argument, assigning the
    /// result to `self`.
    ///
    /// This implements the `/=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x /= Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Quad) {
        let r = self.div(other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl DivAssign<&Quad> for Quad {
    /// Computes $x \div y$, where $x$ is `self` and $y$ is the argument, assigning the
    /// result to `self`.
    ///
    /// This implements the `/=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x /= &Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Quad) {
        let r = self.div(*other);
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
    fn pre_div(&self, other: &Quad) -> Option<Quad> {
        if self.is_nan() || other.is_nan() {
            Some(Quad::NAN)
        } else if other.is_zero() {
            if self.is_zero() {
                Some(Quad::NAN)
            } else if self.is_sign_negative() == other.is_sign_positive() {
                Some(Quad::NEG_INFINITY)
            } else {
                Some(Quad::INFINITY)
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Some(Quad::NAN)
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Some(Quad::ZERO)
            } else {
                Some(Quad::NEG_ZERO)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // div tests
    test_all_near!(
        num_num:
            qd!("1.1557273497909217179100931833126962991208510231644158204997065353273"),
            Quad::PI / Quad::E;
        num_ref:
            qd!("1.1557273497909217179100931833126962991208510231644158204997065353273"),
            Quad::PI / &Quad::E;
        ref_num:
            qd!("1.1557273497909217179100931833126962991208510231644158204997065353273"),
            &Quad::PI / Quad::E;
        ref_ref:
            qd!("1.1557273497909217179100931833126962991208510231644158204997065353273"),
            &Quad::PI / &Quad::E;
        num_neg_num:
            qd!("-1.1557273497909217179100931833126962991208510231644158204997065353273"),
            Quad::PI / -Quad::E;
        num_neg_ref:
            qd!("-1.1557273497909217179100931833126962991208510231644158204997065353273"),
            Quad::PI / -&Quad::E;
        ref_neg_num:
            qd!("-1.1557273497909217179100931833126962991208510231644158204997065353273"),
            &Quad::PI / -Quad::E;
        ref_neg_ref:
            qd!("-1.1557273497909217179100931833126962991208510231644158204997065353273"),
            &Quad::PI / -&Quad::E;
        num_id:
            Quad::PI,
            Quad::PI / Quad::ONE;
        id_num:
            Quad::FRAC_1_PI,
            Quad::ONE / Quad::PI;
        num_small:
            qd!("3141592653589793238462643383279502884197169399375105820974944.5923061"),
            Quad::PI / qd!("1e-60");
        small_num:
            qd!("3.1830988618379067153776752674502872406891929148091289749533468811804e-61"),
            qd!("1e-60") / Quad::PI;
        three_nums:
            qd!("1.6673621161631071223063639072253467866814381989438981528114006093878"),
            Quad::PI / Quad::E / Quad::LN_2;
        lassoc:
            qd!("1.6673621161631071223063639072253467866814381989438981528114006093878"),
            (Quad::PI / Quad::E) / Quad::LN_2;
        rassoc:
            qd!("12.320232213560921976987672083576725232192678340447553172224165846265"),
            Quad::PI / (Quad::LN_2 / Quad::E);
    );
    test_all_exact!(
        zero_inf:
            Quad::ZERO,
            Quad::ZERO / Quad::INFINITY;
        zero_neg_inf:
            Quad::NEG_ZERO,
            Quad::ZERO / Quad::NEG_INFINITY;
        inf_zero:
            Quad::INFINITY,
            Quad::INFINITY / Quad::ZERO;
        neg_inf_zero:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY / Quad::ZERO;
        nan_zero:
            Quad::NAN,
            Quad::NAN / Quad::ZERO;
        zero_nan:
            Quad::NAN,
            Quad::ZERO / Quad::NAN;
        zero_zero:
            Quad::NAN,
            Quad::ZERO / Quad::ZERO;

        one_inf:
            Quad::ZERO,
            Quad::ONE / Quad::INFINITY;
        one_neg_inf:
            Quad::NEG_ZERO,
            Quad::ONE / Quad::NEG_INFINITY;
        inf_one:
            Quad::INFINITY,
            Quad::INFINITY / Quad::ONE;
        neg_inf_one:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY / Quad::ONE;
        inf_inf:
            Quad::NAN,
            Quad::INFINITY / Quad::INFINITY;
        inf_neg_inf:
            Quad::NAN,
            Quad::INFINITY / Quad::NEG_INFINITY;
        neg_inf_inf:
            Quad::NAN,
            Quad::NEG_INFINITY / Quad::INFINITY;
        neg_inf_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY / Quad::NEG_INFINITY;
        one_zero:
            Quad::INFINITY,
            Quad::ONE / Quad::ZERO;
        neg_one_zero:
            Quad::NEG_INFINITY,
            Quad::NEG_ONE / Quad::ZERO;

        nan_one:
            Quad::NAN,
            Quad::NAN / Quad::ONE;
        one_nan:
            Quad::NAN,
            Quad::ONE / Quad::NAN;
    );

    // Assign tests. Assign code delegates to div code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Quad::PI;
            a /= Quad::E;
            near!(qd!("1.1557273497909217179100931833126962991208510231644158204997065353273"), a);
        }
        assign_ref: {
            let mut b = Quad::PI;
            b /= &Quad::E;
            near!(qd!("1.1557273497909217179100931833126962991208510231644158204997065353273"), b);
        }
    );
}
