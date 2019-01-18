// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::ops::{Index, IndexMut};

mod common;
mod consts;
mod parse;
mod arith;
mod comp;
mod conv;
mod alg;
mod trans;
mod trig;
mod hyper;
mod misc;

#[derive(Clone, Copy, Debug)]
pub struct Quad(f64, f64, f64, f64);

impl Quad {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Quad {
        Quad(a, b, c, d)
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
            _ => panic!("Index of quad-double out of range: {}", idx),
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
            _ => panic!("Index of quad-double out of range: {}", idx),
        }
    }
}

