// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::cmp::Ordering;

// #region Equality

impl PartialEq for Double {
    #[inline]
    fn eq(&self, other: &Double) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

// #endregion

// #region Ordering

impl PartialOrd for Double {
    #[inline]
    fn partial_cmp(&self, other: &Double) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            x => x,
        }
    }
}

// #endregion
