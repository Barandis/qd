// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::*;
use crate::quad::Quad;
use std::ops::{Add, AddAssign};

// Utility function that returns the quad component with the specified index and
// then increments the index. This is how we do `a[i++]` without the `++`
// operator.
#[inline]
fn index_and_inc(a: Quad, i: &mut usize) -> f64 {
    let r = a[*i];
    *i += 1;
    r
}

impl Add for Quad {
    type Output = Quad;

    // This function is the real reason indexing was added to quads. Unlike
    // multiplication, where every component has a specific function and appears
    // in a specific place in the algorithm, addition is just a repeated
    // iteration over each successive component.
    #[allow(clippy::suspicious_arithmetic_impl, clippy::many_single_char_names)]
    #[inline]
    fn add(self, other: Quad) -> Quad {
        if self.is_nan() || other.is_nan() {
            Quad::NAN
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Quad::INFINITY
                    } else {
                        Quad::NAN
                    }
                } else if other.is_sign_negative() {
                    Quad::NEG_INFINITY
                } else {
                    Quad::NAN
                }
            } else if self.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::NEG_INFINITY
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::NEG_INFINITY
            }
        } else {
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;

            let mut x = [0.0, 0.0, 0.0, 0.0];

            // These two assignments, along with the reassignments of the same
            // variables in the `accumulate` call below, act as a merge sort.
            // The largest component between the two quads is operated on first,
            // then the second largest, and so on.
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
                } else if j >= 4 || self[i].abs() > other[j].abs() {
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

            Quad::from(renorm4(x[0], x[1], x[2], x[3]))
        }
    }
}

impl<'a> Add<&'a Quad> for Quad {
    type Output = Quad;

    #[inline]
    fn add(self, other: &Quad) -> Quad {
        self.add(*other)
    }
}

impl<'a> Add<Quad> for &'a Quad {
    type Output = Quad;

    #[inline]
    fn add(self, other: Quad) -> Quad {
        (*self).add(other)
    }
}

impl AddAssign for Quad {
    #[inline]
    fn add_assign(&mut self, other: Quad) {
        self.assign(self.add(other).into());
    }
}

impl<'a> AddAssign<&'a Quad> for Quad {
    #[inline]
    fn add_assign(&mut self, other: &Quad) {
        self.assign(self.add(*other).into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_num() {
        let expected = qd!(
            "5.859874482048838473822930854632165381954416493075065395941912220"
        );
        assert_close!(expected, Quad::PI + Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn num_ref() {
        let expected = qd!(
            "5.859874482048838473822930854632165381954416493075065395941912220"
        );
        assert_close!(expected, Quad::PI + &Quad::E);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn ref_num() {
        let expected = qd!(
            "5.859874482048838473822930854632165381954416493075065395941912220"
        );
        assert_close!(expected, &Quad::PI + Quad::E);
    }

    #[test]
    fn assign_num() {
        let expected = qd!(
            "5.859874482048838473822930854632165381954416493075065395941912220"
        );

        let mut a = Quad::PI;
        a += Quad::E;
        assert_close!(expected, a);
    }

    #[test]
    fn assign_ref() {
        let expected = qd!(
            "5.859874482048838473822930854632165381954416493075065395941912220"
        );

        let mut b = Quad::PI;
        b += &Quad::E;
        assert_close!(expected, b);
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY + Quad::ONE);
        assert_exact!(Quad::INFINITY, Quad::ONE + Quad::INFINITY);
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY + Quad::ONE);
        assert_exact!(Quad::NEG_INFINITY, Quad::ONE + Quad::NEG_INFINITY);
    }

    #[test]
    fn infinities() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY + Quad::INFINITY);
        assert_exact!(
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY + Quad::NEG_INFINITY
        );
        assert_exact!(Quad::NAN, Quad::INFINITY + Quad::NEG_INFINITY);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY + Quad::INFINITY);
        assert_exact!(Quad::NAN, Quad::INFINITY + Quad::NAN);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY + Quad::NAN);
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN + Quad::ONE);
        assert_exact!(Quad::NAN, Quad::ONE + Quad::NAN);
    }
}
