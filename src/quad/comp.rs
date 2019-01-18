// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::cmp::Ordering;

impl PartialEq for Quad {
    #[inline]
    fn eq(&self, other: &Quad) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl PartialOrd for Quad {
    #[inline]
    fn partial_cmp(&self, other: &Quad) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => match self.1.partial_cmp(&other.1) {
                Some(Ordering::Equal) => match self.2.partial_cmp(&other.2) {
                    Some(Ordering::Equal) => self.3.partial_cmp(&other.3),
                    x => x,
                },
                x => x,
            },
            x => x,
        }
    }
}
