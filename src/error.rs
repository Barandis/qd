// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDoubleError {
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseQuadError {
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Empty,
    Invalid,
}

impl Display for ParseDoubleError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let description = match self.kind {
            ErrorKind::Empty => "cannot parse double-double from empty string",
            ErrorKind::Invalid => "invalid double-double literal",
        };
        description.fmt(f)
    }
}

impl Display for ParseQuadError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let description = match self.kind {
            ErrorKind::Empty => "cannot parse quad-double from empty string",
            ErrorKind::Invalid => "invalid quad-double literal",
        };
        description.fmt(f)
    }
}
