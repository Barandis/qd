// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::renorm4;
use std::ops::{Index, IndexMut};

#[macro_use]
mod macros {
    /// Creates a new quad-double from another number or from a string.
    ///
    /// The argument can be any expression that evaluates to a type that this library defines a
    /// `From` implementation for. This includes `&str`, `Double`, `Quad`, any primitive number, and
    /// 2-, 3-, and 4-tuples of any of those primitive number types.
    ///
    /// # Panics
    ///
    /// Passing an expression that evaluates to a type that does not have a `From` implementation
    /// will cause a panic.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(0) == Quad::ZERO);
    ///
    /// let x = qd!(1) / qd!(2).sqrt();
    /// let expected = qd!("0.7071067811865475244008443621048490392848359376884740365883398690");
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[macro_export]
    macro_rules! qd {
        ($x:expr) => {
            Quad::from($x)
        };
    }
}

mod alg;
mod arith;
mod common;
mod comp;
mod consts;
mod conv;
mod hyper;
mod misc;
mod parse;
mod trans;
mod trig;

#[derive(Clone, Copy, Debug)]
pub struct Quad(f64, f64, f64, f64);

impl Quad {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Quad {
        Quad(a, b, c, d)
    }

    pub fn norm(a: f64, b: f64, c: f64, d: f64) ->  Quad {
        Quad::from(renorm4(a, b, c, d))
    }
}

impl Index<usize> for Quad {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!(
                "Index of quad-double out of range (must be in range [0, 3]): {}",
                idx
            ),
        }
    }
}

impl IndexMut<usize> for Quad {
    fn index_mut(&mut self, idx: usize) -> &mut f64 {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!(
                "Index of quad-double out of range (must be in range [0, 3]): {}",
                idx
            ),
        }
    }
}