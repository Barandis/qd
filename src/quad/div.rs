// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
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
    let (h0, l0) = core::two_prod(a.0, b);
    let (h1, l1) = core::two_prod(a.1, b);
    let (h2, l2) = core::two_prod(a.2, b);
    let h3 = a.3 * b;

    let s0 = h0;
    let (s1, t0) = core::two_sum(h1, l0);
    let (s2, t1, t2) = core::three_three_sum(t0, h2, l1);
    let (s3, t3) = core::three_two_sum(t1, h3, l2);
    let s4 = t2 * t3;

    let (a, b, c, d) = core::renorm5(s0, s1, s2, s3, s4);
    Quad(a, b, c, d)
}

impl Div for Quad {
    type Output = Quad;

    /// Divides this `Quad` by another, producing a new `Quad` as a result.
    /// 
    /// This implements the `/` operator between two `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E / Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Quad) -> Quad {
        if self.is_nan() || other.is_nan() {
            Quad::NAN
        } else if other.is_zero() {
            if self.is_zero() {
                Quad::NAN
            } else if self.is_sign_negative() == other.is_sign_positive() {
                Quad::NEG_INFINITY
            } else {
                Quad::INFINITY
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Quad::NAN
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::NEG_INFINITY
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Quad::ZERO
            } else {
                Quad::NEG_ZERO
            }
        } else {
            // Strategy:
            //
            // Divide the first component of `self` by the first component of `other`. Then
            // divide the first component of the remainder by the first component of
            // `other`, then the first component of -that- remainder by the first component
            // of `other`, and so on until we have five terms we can renormalize.
            let q0 = self.0 / other.0;
            let mut r = self - mul_f64(other, q0);

            let q1 = r.0 / other.0;
            r -= mul_f64(other, q1);

            let q2 = r.0 / other.0;
            r -= mul_f64(other, q2);

            let q3 = r.0 / other.0;
            r -= mul_f64(other, q3);

            let q4 = r.0 / other.0;

            let (a, b, c, d) = core::renorm5(q0, q1, q2, q3, q4);
            Quad(a, b, c, d)
        }
    }
}

    /// Divides a reference to this `Quad` by another, producing a new `Quad` as a result.
    /// 
    /// This implements the `/` operator between two references to `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E / &Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
impl Div for &Quad {
    type Output = Quad;

    #[inline]
    fn div(self, other: &Quad) -> Quad {
        (*self).div(*other)
    }
}

impl Div<&Quad> for Quad {
    type Output = Quad;

    /// Divides this `Quad` by a reference to another, producing a new `Quad` as a result.
    /// 
    /// This implements the `/` operator between a `Quad` and a reference to a `Quad`.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E / &Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn div(self, other: &Quad) -> Quad {
        self.div(*other)
    }
}

impl Div<Quad> for &Quad {
    type Output = Quad;

    /// Divides a reference to this `Quad` by another `Quad`, producing a new `Quad` as a
    /// result.
    ///
    /// This implements the `/` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = &Quad::E / Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    fn div(self, other: Quad) -> Quad {
        (*self).div(other)
    }
}

impl DivAssign for Quad {
    /// Divides this `Quad` by another, modifying this one to equal the result.
    /// 
    /// This implements the `/=` operator between two `Quad`s.
    /// 
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x /= Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
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
    /// Divides this `Quad` by a reference to another, modifying this one to equal the
    /// result.
    ///
    /// This implements the `/=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let mut x = Quad::E;
    /// x /= &Quad::PI;
    /// let expected = qd!("0.8652559794322650872177747896460896174287446239085155394543302889");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = qd!("1.155727349790921717910093183312696299120851023164415820499706535");
        assert_close!(expected, Quad::PI / Quad::E);
    }

    #[test]
    fn ref_ref() {
        let expected = qd!("1.155727349790921717910093183312696299120851023164415820499706535");
        assert_close!(expected, &Quad::PI / &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = qd!("1.155727349790921717910093183312696299120851023164415820499706535");
        assert_close!(expected, Quad::PI / &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = qd!("1.155727349790921717910093183312696299120851023164415820499706535");
        assert_close!(expected, &Quad::PI / Quad::E);
    }

    #[test]
    fn assign_num() {
        let expected = qd!("1.155727349790921717910093183312696299120851023164415820499706535");

        let mut a = Quad::PI;
        a /= Quad::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = qd!("1.155727349790921717910093183312696299120851023164415820499706535");

        let mut b = Quad::PI;
        b /= &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO / Quad::INFINITY);
        assert_exact!(Quad::NEG_ZERO, Quad::ZERO / Quad::NEG_INFINITY);
        assert_exact!(Quad::INFINITY, Quad::INFINITY / Quad::ZERO);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY / Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::NAN / Quad::ZERO);
        assert_exact!(Quad::NAN, Quad::ZERO / Quad::NAN);
        assert_exact!(Quad::NAN, Quad::ZERO / Quad::ZERO);
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn inf() {
        assert_exact!(Quad::ZERO, Quad::ONE / Quad::INFINITY);
        assert_exact!(Quad::NEG_ZERO, Quad::ONE / Quad::NEG_INFINITY);
        assert_exact!(Quad::INFINITY, Quad::INFINITY / Quad::ONE);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY / Quad::ONE);
        assert_exact!(Quad::NAN, Quad::INFINITY / Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY / Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY / Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY / Quad::NEG_INFINITY);
        assert_exact!(Quad::INFINITY, Quad::ONE / Quad::ZERO);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ONE / Quad::ZERO);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN / Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE / Quad::NAN);
    }
}
