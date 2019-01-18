// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::*;
use crate::quad::Quad;
use std::f64;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Utility function that returns the quad component with the specified index and then increments
// the index. This is how we do `a[i++]` without the `++` operator.
#[inline]
fn index_and_inc(a: Quad, i: &mut usize) -> f64 {
    let r = a[*i];
    *i += 1;
    r
}

// #region Addition

impl Quad {
    // This function is the real reason indexing was added to quads. Unlike multiplication, where
    // every component has a specific function and appears in a specific place in the algorithm,
    // addition is just a repeated iteration over each successive component.
    #[inline]
    fn add_quad(self, other: Quad) -> (f64, f64, f64, f64) {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        let mut x = [0.0, 0.0, 0.0, 0.0];

        // These two assignments, along with the reassignments of the same variables in the
        // `accumulate` call below, act as a merge sort. The largest component between the two quads
        // is operated on first, then the second largest, and so on.
        let u = if self[i].abs() > other[j].abs() {
            index_and_inc(self, &mut i)
        } else {
            index_and_inc(other, &mut j)
        };
        let v = if self[i].abs() > other[j].abs() {
            index_and_inc(self, &mut i)
        } else {
            index_and_inc(other, &mut j)
        };
        let (mut u, mut v) = renorm2(u, v);

        while k < 4 {
            if i >= 4 && j >= 4 {
                x[k] = u;
                if k < 3 {
                    k += 1;
                    x[k] = v;
                }
                break;
            }

            let t = if i >= 4 {
                index_and_inc(other, &mut j)
            } else if j >= 4 {
                index_and_inc(self, &mut i)
            } else if self[i].abs() > other[j].abs() {
                index_and_inc(self, &mut i)
            } else {
                index_and_inc(other, &mut j)
            };

            let (s, y, z) = accumulate(u, v, t);
            u = y;
            v = z;

            if s != 0.0 {
                x[k] = s;
                k += 1;
            }
        }

        for k in i..4 {
            x[3] += self[k];
        }
        for k in j..4 {
            x[3] += other[k];
        }

        renorm4(x[0], x[1], x[2], x[3])
    }
}

impl Add for Quad {
    type Output = Quad;

    #[inline]
    fn add(self, other: Quad) -> Quad {
        Quad::from(self.add_quad(other))
    }
}

impl<'a> Add<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn add(self, other: &Quad) -> Quad {
        Quad::from(self.add_quad(*other))
    }
}

impl<'a> Add<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn add(self, other: Quad) -> Quad {
        Quad::from(self.add_quad(other))
    }
}

impl AddAssign for Quad {
    #[inline]
    fn add_assign(&mut self, other: Quad) {
        let (a, b, c, d) = self.add_quad(other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

impl<'a> AddAssign<&'a Quad> for Quad {
    #[inline]
    fn add_assign(&mut self, other: &Quad) {
        let (a, b, c, d) = self.add_quad(*other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

// #endregion

// #region Subtraction

impl Neg for Quad {
    type Output = Quad;

    #[inline]
    fn neg(self) -> Quad {
        Quad(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Sub for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: Quad) -> Quad {
        Quad::from(self.add_quad(-other))
    }
}

impl<'a> Sub<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: &Quad) -> Quad {
        Quad::from(self.add_quad(-*other))
    }
}

impl<'a> Sub<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn sub(self, other: Quad) -> Quad {
        Quad::from(self.add_quad(-other))
    }
}

impl SubAssign for Quad {
    #[inline]
    fn sub_assign(&mut self, other: Quad) {
        let (a, b, c, d) = self.add_quad(-other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

impl<'a> SubAssign<&'a Quad> for Quad {
    #[inline]
    fn sub_assign(&mut self, other: &Quad) {
        let (a, b, c, d) = self.add_quad(-*other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

// #endregion

// #region Multiplication

impl Quad {
    // This is complicated.
    //
    // It closely follows the process described on pp. 11-16 of "Library for Double-Double and
    // Quad-Double Arithmetic" by Y. Hida, X.S. Li, and D.H. Bailey which can be found at
    // http://web.mit.edu/tabbott/Public/quaddouble-debian/qd-2.3.4-old/docs/qd.pdf. You should be
    // able to see the way the source code works from the diagrams there.
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
    // Other terms, including the remaining O(ε⁴) terms and the low words of the O(ε⁴) that are
    // calculated, are not necessary to provide 212 bits of accuracy.
    #[inline]
    fn mul_quad(self, other: Quad) -> (f64, f64, f64, f64) {
        // O(1) term
        let (h0, l0) = two_prod(self.0, other.0);

        // O(ε) terms
        let (h1, l1) = two_prod(self.0, other.1);
        let (h2, l2) = two_prod(self.1, other.0);

        // O(ε²) terms
        let (h3, l3) = two_prod(self.0, other.2);
        let (h4, l4) = two_prod(self.1, other.1);
        let (h5, l5) = two_prod(self.2, other.0);

        // O(ε³) terms
        let (h6, l6) = two_prod(self.0, other.3);
        let (h7, l7) = two_prod(self.1, other.2);
        let (h8, l8) = two_prod(self.2, other.1);
        let (h9, l9) = two_prod(self.3, other.0);

        // O(ε⁴) terms - the low words aren't necessary for the accuracy we need
        let ha = self.1 * other.3;
        let hb = self.2 * other.2;
        let hc = self.3 * other.1;

        // Each calculation takes all of the high words for the terms of that level, whatever
        // intermediate words are specified by the algorithm, and whatever low words fit in the
        // remaining input space.

        // O(1) calculation (pass-through)
        let r0 = h0;
        // O(ε) calculation
        let (r1, t0, t1) = three_three_sum(h1, h2, l0);
        // O(ε²) calculation
        let (r2, t2, t3) = six_three_sum(t0, h3, h4, h5, l1, l2);
        // O(ε³) calculation
        let (r3, t4) = nine_two_sum(t1, t2, h6, h7, h8, h9, l3, l4, l5);
        // O(ε⁴) calculation (nine_one_sum)
        let r4 = t3 + t4 + ha + hb + hc + l6 + l7 + l8 + l9;

        // Results of the prior calculations are renormalized into four f64s.
        renorm5(r0, r1, r2, r3, r4)
    }
}

impl Mul for Quad {
    type Output = Quad;

    #[inline]
    fn mul(self, other: Quad) -> Quad {
        Quad::from(self.mul_quad(other))
    }
}

impl<'a> Mul<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn mul(self, other: &Quad) -> Quad {
        Quad::from(self.mul_quad(*other))
    }
}

impl<'a> Mul<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn mul(self, other: Quad) -> Quad {
        Quad::from(self.mul_quad(other))
    }
}

impl MulAssign for Quad {
    #[inline]
    fn mul_assign(&mut self, other: Quad) {
        let (a, b, c, d) = self.mul_quad(other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

impl<'a> MulAssign<&'a Quad> for Quad {
    #[inline]
    fn mul_assign(&mut self, other: &Quad) {
        let (a, b, c, d) = self.mul_quad(*other);
        self.0 = a;
        self.1 = b;
        self.2 = c;
        self.3 = d;
    }
}

// #endregion

// #region Division

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

// #endregion
