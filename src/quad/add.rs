// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::utils as u;
use crate::quad::Quad;
use std::ops::{Add, AddAssign};

// Utility function that returns the quad component with the specified index and then
// increments the index. This is how we do `a[i++]` without the `++` operator.
#[inline]
fn index_and_inc(a: Quad, i: &mut usize) -> f64 {
    let r = a[*i];
    *i += 1;
    r
}

impl Add for Quad {
    type Output = Quad;

    // This function is the real reason indexing was added to quads. Unlike multiplication,
    // where every component has a specific function and appears in a specific place in the
    // algorithm, addition is just a repeated iteration over each successive component.

    /// Computes $x + y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `+` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E + Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[allow(clippy::suspicious_arithmetic_impl, clippy::many_single_char_names)]
    fn add(self, other: Quad) -> Quad {
        match self.pre_add(&other) {
            Some(r) => r,
            None => {
                let mut i = 0;
                let mut j = 0;
                let mut k = 0;

                let mut x = [0.0; 4];

                // These two assignments, along with the reassignments of the same variables
                // in the `accumulate` call below, act as a merge sort. The largest
                // component between the two quads is operated on first, then the second
                // largest, and so on.
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
                let (mut u, mut v) = u::renorm2(u, v);

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
                    } else if j >= 4 || self[i].abs() > other[j].abs() {
                        index_and_inc(self, &mut i)
                    } else {
                        index_and_inc(other, &mut j)
                    };

                    let (s, y, z) = u::accumulate(u, v, t);
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
                let (a, b, c, d) = u::renorm4(x[0], x[1], x[2], x[3]);
                Quad(a, b, c, d)
            }
        }
    }
}

impl Add for &Quad {
    type Output = Quad;

    /// Computes $x + y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `+` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E + &Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn add(self, other: &Quad) -> Quad {
        (*self).add(*other)
    }
}

impl Add<&Quad> for Quad {
    type Output = Quad;

    /// Computes $x + y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `+` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E + &Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn add(self, other: &Quad) -> Quad {
        self.add(*other)
    }
}

impl Add<Quad> for &Quad {
    type Output = Quad;

    /// Computes $x + y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Quad` as the result.
    ///
    /// This implements the `+` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::E + Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn add(self, other: Quad) -> Quad {
        (*self).add(other)
    }
}

impl AddAssign for Quad {
    /// Computes $x + y$, where $x$ is `self` and $y$ is the argument, assigning the result
    /// to `self`.
    ///
    /// This implements the `+=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x += Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn add_assign(&mut self, other: Quad) {
        let r = self.add(other);
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl AddAssign<&Quad> for Quad {
    /// Computes $x + y$, where $x$ is `self` and $y$ is the argument, assigning the result
    /// to `self`.
    ///
    /// This implements the `+=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::E;
    /// x += &Quad::PI;
    /// let expected = qd!("5.859874482048838473822930854632165381954416493075065395941912220");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    fn add_assign(&mut self, other: &Quad) {
        let r = self.add(*other);
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
    fn pre_add(&self, other: &Quad) -> Option<Quad> {
        if self.is_nan() || other.is_nan() {
            Some(Quad::NAN)
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Some(Quad::INFINITY)
                    } else {
                        Some(Quad::NAN)
                    }
                } else if other.is_sign_negative() {
                    Some(Quad::NEG_INFINITY)
                } else {
                    Some(Quad::NAN)
                }
            } else if self.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
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

    // add tests
    test_all_near!(
        num_num:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            Quad::PI + Quad::E;
        num_ref:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            Quad::PI + &Quad::E;
        ref_num:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            &Quad::PI + Quad::E;
        ref_ref:
            qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"),
            &Quad::PI + &Quad::E;
        num_neg_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI + -Quad::E;
        num_neg_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI + -&Quad::E;
        ref_neg_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI + -Quad::E;
        ref_neg_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI + -&Quad::E;
        num_id:
            Quad::PI,
            Quad::PI + Quad::ZERO;
        id_num:
            Quad::PI,
            Quad::ZERO + Quad::PI;
        num_small:
            qd!("3.1415926535897932384626433832795028841971693993751058209749455923065"),
            Quad::PI + qd!("1e-60");
        small_num:
            qd!("3.1415926535897932384626433832795028841971693993751058209749455923065"),
            qd!("1e-60") + Quad::PI;
        three_nums:
            qd!("6.5530216626087837832401629760903419500299166274353206500625922295256"),
            Quad::PI + Quad::E + Quad::LN_2;
        lassoc:
            qd!("6.5530216626087837832401629760903419500299166274353206500625922295256"),
            (Quad::PI + Quad::E) + Quad::LN_2;
        rassoc:
            qd!("6.5530216626087837832401629760903419500299166274353206500625922295256"),
            Quad::PI + (Quad::LN_2 + Quad::E);
    );
    test_all_exact!(
        inf_num:
            Quad::INFINITY,
            Quad::INFINITY + Quad::ONE;
        num_inf:
            Quad::INFINITY,
            Quad::ONE + Quad::INFINITY;
        neg_inf_num:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY + Quad::ONE;
        num_neg_inf:
            Quad::NEG_INFINITY,
            Quad::ONE + Quad::NEG_INFINITY;
        inf_inf:
            Quad::INFINITY,
            Quad::INFINITY + Quad::INFINITY;
        neg_inf_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY + Quad::NEG_INFINITY;
        inf_neg_inf:
            Quad::NAN,
            Quad::INFINITY + Quad::NEG_INFINITY;
        neg_inf_inf:
            Quad::NAN,
            Quad::NEG_INFINITY + Quad::INFINITY;
        inf_nan:
            Quad::NAN,
            Quad::INFINITY + Quad::NAN;
        neg_inf_nan:
            Quad::NAN,
            Quad::NEG_INFINITY + Quad::NAN;
        nan_num:
            Quad::NAN,
            Quad::NAN + Quad::ONE;
        num_nan:
            Quad::NAN,
            Quad::ONE + Quad::NAN;
        nan_nan:
            Quad::NAN,
            Quad::NAN + Quad::NAN;
    );

    // Assign tests. Assign code delegates to add code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Quad::PI;
            a += Quad::E;
            near!(qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"), a);
        }
        assign_ref: {
            let mut b = Quad::PI;
            b += &Quad::E;
            near!(qd!("5.8598744820488384738229308546321653819544164930750653959419122200308"), b);
        }
    );
}
