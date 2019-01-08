// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt;

// #region Parsing error

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDoubleDoubleError {
    pub kind: DoubleDoubleErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoubleDoubleErrorKind {
    Empty,
    Invalid,
}

impl fmt::Display for ParseDoubleDoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self.kind {
            DoubleDoubleErrorKind::Empty => "cannot parse double-double from empty string",
            DoubleDoubleErrorKind::Invalid => "invalid double-double literal",
        };
        description.fmt(f)
    }
}

// #endregion
