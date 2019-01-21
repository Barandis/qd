// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::iter::{Product, Sum};

impl Sum for Double {
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Double> for Double {
    fn sum<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ZERO, |a, b| a + *b)
    }
}

impl Product for Double {
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = Double>,
    {
        iter.fold(Double::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Double> for Double {
    fn product<I>(iter: I) -> Double
    where
        I: Iterator<Item = &'a Double>,
    {
        iter.fold(Double::ONE, |a, b| a * *b)
    }
}
