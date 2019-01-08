// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt;

// #region Parsing error

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseQdFloatError {
    pub kind: QdFloatErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QdFloatErrorKind {
    Empty,
    Invalid,
}

impl fmt::Display for ParseQdFloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self.kind {
            QdFloatErrorKind::Empty => "cannot parse composite double from empty string",
            QdFloatErrorKind::Invalid => "invalid composite double literal",
        };
        description.fmt(f)
    }
}

// #endregion
