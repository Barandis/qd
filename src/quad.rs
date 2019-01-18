// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
