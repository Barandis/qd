// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::DoubleDouble;
use std::cmp::Ordering;

// #region Equality

impl PartialEq for DoubleDouble {
    #[inline]
    fn eq(&self, other: &DoubleDouble) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialEq<f64> for DoubleDouble {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.0 == *other && self.1 == 0.0
    }
}

impl PartialEq<DoubleDouble> for f64 {
    #[inline]
    fn eq(&self, other: &DoubleDouble) -> bool {
        other == self
    }
}

// #endregion

// #region Ordering

impl PartialOrd for DoubleDouble {
    #[inline]
    fn partial_cmp(&self, other: &DoubleDouble) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            None => None,
        }
    }
}

impl PartialOrd<f64> for DoubleDouble {
    #[inline]
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        match self.0.partial_cmp(other) {
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Equal) => self.1.partial_cmp(&0f64),
            None => None,
        }
    }
}

impl PartialOrd<DoubleDouble> for f64 {
    #[inline]
    fn partial_cmp(&self, other: &DoubleDouble) -> Option<Ordering> {
        match self.partial_cmp(&other.0) {
            Some(Ordering::Greater) => Some(Ordering::Greater),
            Some(Ordering::Less) => Some(Ordering::Less),
            Some(Ordering::Equal) => 0f64.partial_cmp(&other.1),
            None => None,
        }
    }
}

// #endregion
