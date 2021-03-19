// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

//! Errors that may occur while parsing a string into a [`Double`] or a [`Quad`].
//! 
//! [`Double`]: struct.Double.html
//! [`Quad`]: struct.Quad.html

use std::fmt::{Display, Formatter, Result};

/// An error generated when a problem is encountered parsing a string into a [`Double`].
/// 
/// [`Double`]: struct.Double.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDoubleError {
    pub kind: ErrorKind,
}

/// An error generated when a problem is encountered parsing a string into a [`Quad`].
/// 
/// [`Quad`]: struct.Quad.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseQuadError {
    pub kind: ErrorKind,
}

/// The different kinds of errors that might be generated during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// An error indicating that an attempt was made to parse an empty string.
    Empty,
    /// An error indicating that the format of a parsed string is not a legal number.
    Invalid,
}

impl Display for ParseDoubleError {
    /// Displays an English-language message describing the kind of the error.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let description = match self.kind {
            ErrorKind::Empty => "cannot parse double-double from empty string",
            ErrorKind::Invalid => "invalid double-double literal",
        };
        description.fmt(f)
    }
}

impl Display for ParseQuadError {
    /// Displays an English-language message describing the kind of the error.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let description = match self.kind {
            ErrorKind::Empty => "cannot parse quad-double from empty string",
            ErrorKind::Invalid => "invalid quad-double literal",
        };
        description.fmt(f)
    }
}
