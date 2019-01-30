// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::error::{ErrorKind, ParseError};
use crate::quad::Quad;
use std::str::FromStr;

const TEN: Quad = Quad(10.0, 0.0, 0.0, 0.0);

impl FromStr for Quad {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Quad, ParseError> {
        let mut result = Quad::ZERO;
        let mut digits = 0;
        let mut point = -1;
        let mut sign = 0;
        let mut exp = 0;

        let s = s.trim().to_ascii_lowercase();

        if s.is_empty() {
            Err(ParseError {
                kind: ErrorKind::Empty,
            })
        } else if s == "nan" {
            Ok(Quad::NAN)
        } else if s == "inf" || s == "infinity"  {
            Ok(Quad::INFINITY)
        } else if s == "-inf" || s == "-infinity" {
            Ok(Quad::NEG_INFINITY)
        } else {
            for (index, ch) in s.chars().enumerate() {
                match ch.to_digit(10) {
                    Some(d) => {
                        result *= TEN;
                        result += Quad(d as f64, 0.0, 0.0, 0.0);
                        digits += 1;
                    }
                    None => match ch {
                        '.' => {
                            if point >= 0 {
                                return Err(ParseError {
                                    kind: ErrorKind::Invalid,
                                });
                            }
                            point = digits;
                        }
                        '-' => {
                            if sign != 0 || digits > 0 {
                                return Err(ParseError {
                                    kind: ErrorKind::Invalid,
                                });
                            }
                            sign = -1;
                        }
                        '+' => {
                            if sign != 0 || digits > 0 {
                                return Err(ParseError {
                                    kind: ErrorKind::Invalid,
                                });
                            }
                            sign = 1;
                        }
                        'e' => {
                            let end = &s[(index + 1)..];
                            match end.parse::<i32>() {
                                Ok(e) => {
                                    exp = e;
                                    break;
                                }
                                Err(_) => {
                                    return Err(ParseError {
                                        kind: ErrorKind::Invalid,
                                    });
                                }
                            }
                        }
                        '_' => {
                            // just continue; _ is a no-op but not an error
                        }
                        _ => {
                            return Err(ParseError {
                                kind: ErrorKind::Invalid,
                            });
                        }
                    },
                }
            }

            if point >= 0 {
                exp -= digits - point;
            }
            if exp != 0 {
                // Do this in two stages if the exponent is too small
                // For exmaple, a number with 30 digits could have an exponent as low as -337 and
                // still not overflow, but doing the -337 all at once WOULD overflow
                if exp < -307 {
                    let adjust = exp + 307;
                    result *= TEN.powi(adjust);
                    exp -= adjust;
                }
                result *= TEN.powi(exp);
            }
            if sign == -1 {
                result = -result;
            }

            Ok(result)
        }
    }
}
