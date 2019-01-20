// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::*;
use crate::quad::Quad;
use std::f64;
use std::ops::{Div, DivAssign};

// Quad x f64 analogue of full quad x quad multiplication above. This is here because we don't want
// to depend on any Quad::from(x), where x is a single f64 (i.e., a non-tuple), in arithmetic. Doing
// so will create infinite loops because arithmetic is used to parse the f64s into quads in the
// first place. Multiplying the f64s directly into Quads bypasses this.
//
// Division is the only place where this is necessary, so this multiplication function is dropped
// nearby.
#[inline]
fn mul_f64(a: Quad, b: f64) -> Quad {
    let (h0, l0) = two_prod(a.0, b);
    let (h1, l1) = two_prod(a.1, b);
    let (h2, l2) = two_prod(a.2, b);
    let h3 = a.3 * b;

    let s0 = h0;
    let (s1, t0) = two_sum(h1, l0);
    let (s2, t1, t2) = three_three_sum(t0, h2, l1);
    let (s3, t3) = three_two_sum(t1, h3, l2);
    let s4 = t2 * t3;

    Quad::from(renorm5(s0, s1, s2, s3, s4))
}

impl Quad {
    #[inline]
    fn div_quad(self, other: Quad) -> (f64, f64, f64, f64) {
        if other.is_zero() {
            if self.is_zero() {
                (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
            } else if self.is_sign_negative() == other.is_sign_positive() {
                (
                    f64::NEG_INFINITY,
                    f64::NEG_INFINITY,
                    f64::NEG_INFINITY,
                    f64::NEG_INFINITY,
                )
            } else {
                (f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY)
            }
        } else {
            // Strategy:
            //
            // Divide the first component of `self` by the first component of `other`. Then divide
            // the first component of the remainder by the first component of `other`, then the
            // first component of -that- remainder by the first component of `other`, and so on
            // until we have five terms we can renormalize.
            let q0 = self.0 / other.0;
            let mut r = self - mul_f64(other, q0);

            let q1 = r.0 / other.0;
            r -= mul_f64(other, q1);

            let q2 = r.0 / other.0;
            r -= mul_f64(other, q2);

            let q3 = r.0 / other.0;
            r -= mul_f64(other, q3);

            let q4 = r.0 / other.0;

            renorm5(q0, q1, q2, q3, q4)
        }
    }

    #[inline]
    pub fn recip(self) -> Quad {
        Quad::ONE / self
    }
}

impl Div for Quad {
    type Output = Quad;

    #[inline]
    fn div(self, other: Quad) -> Quad {
        Quad::from(self.div_quad(other))
    }
}

impl<'a> Div<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn div(self, other: &Quad) -> Quad {
        Quad::from(self.div_quad(*other))
    }
}

impl<'a> Div<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn div(self, other: Quad) -> Quad {
        Quad::from(self.div_quad(other))
    }
}

impl DivAssign for Quad {
    #[inline]
    fn div_assign(&mut self, other: Quad) {
        let (a, b, c, d) = self.div_quad(other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

impl<'a> DivAssign<&'a Quad> for Quad {
    #[inline]
    fn div_assign(&mut self, other: &Quad) {
        let (a, b, c, d) = self.div_quad(*other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}
