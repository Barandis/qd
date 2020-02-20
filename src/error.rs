// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt;

// #region Parsing error

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Empty,
    Invalid,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self.kind {
            ErrorKind::Empty => {
                "cannot parse composite double from empty string"
            }
            ErrorKind::Invalid => "invalid composite double literal",
        };
        description.fmt(f)
    }
}

// #endregion
