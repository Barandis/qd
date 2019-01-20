// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use crate::common::basic::*;
use std::ops::{Add, AddAssign};

// Utility function that returns the quad component with the specified index and then increments
// the index. This is how we do `a[i++]` without the `++` operator.
#[inline]
fn index_and_inc(a: Quad, i: &mut usize) -> f64 {
    let r = a[*i];
    *i += 1;
    r
}

impl Quad {
    // This function is the real reason indexing was added to quads. Unlike multiplication, where
    // every component has a specific function and appears in a specific place in the algorithm,
    // addition is just a repeated iteration over each successive component.
    #[inline]
    pub(super) fn add_quad(self, other: Quad) -> (f64, f64, f64, f64) {
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
