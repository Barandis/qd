// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use crate::error::{ErrorKind, ParseError};
use std::str::FromStr;

const TEN: Double = Double(10.0, 0.0);

impl FromStr for Double {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Double, ParseError> {
        let mut result = Double::ZERO;
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
            Ok(Double::NAN)
        } else if s == "inf" || s == "infinity" {
            Ok(Double::INFINITY)
        } else if s == "-inf" || s == "-infinity" {
            Ok(Double::NEG_INFINITY)
        } else {
            for (index, ch) in s.chars().enumerate() {
                match ch.to_digit(10) {
                    Some(d) => {
                        result *= TEN;
                        result += Double(d as f64, 0.0);
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

// #region Tests
#[cfg(test)]
mod tests {
    use super::*;

    const PI_50: &str = "3.14159265358979323846264338327950288419716939937510";
    const E_50: &str = "2.71828182845904523536028747135266249775724709369995";
    const PI_50_3: &str = "3.14159265358979323846264338327950288419716939937510e3";
    const E_50_NEG_2: &str = "2.71828182845904523536028747135266249775724709369995e-2";
    const PI_TIMES_10_20: &str = "314159265358979323846";
    const E_TIMES_10_25: &str = "27182818284590452353602874";
    const PI_TIMES_10_20_EXP: &str = "3.14159265358979323846e20";
    const E_TIMES_10_25_EXP: &str = "2.7182818284590452353602874e25";

    fn parse(value: &str) -> Double {
        value.parse().unwrap()
    }

    fn parse_error(value: &str) -> ParseError {
        value.parse::<Double>().unwrap_err()
    }

    #[test]
    fn nan() {
        assert!(parse("nan").is_nan());
    }

    #[test]
    fn inf() {
        assert_exact!(parse("inf"), Double::INFINITY);
    }

    #[test]
    fn neg_inf() {
        assert_exact!(parse("-inf"), Double::NEG_INFINITY);
    }

    #[test]
    fn zero() {
        assert_exact!(parse("0"), Double::from(0));
        assert_exact!(parse("-0"), Double::from(-0.0));
    }

    #[test]
    fn integer() {
        assert_exact!(parse("1729"), Double::from(1729));
        assert_exact!(parse("16_777_216"), Double::from(16_777_216));
        assert_exact!(parse("+2317"), Double::from(2317));
        assert_exact!(parse("-42"), Double::from(-42));
    }

    #[test]
    fn long_integer() {
        assert_exact!(
            parse(PI_TIMES_10_20),
            (Double::PI * Double::from(10).powi(20)).floor()
        );
        assert_exact!(
            parse(E_TIMES_10_25),
            (Double::E * Double::from(10).powi(25)).floor()
        );
    }

    #[test]
    fn float() {
        // Using just the first component for comparisons here, because on numbers that don't
        // need that much precision, there is a notable error component in the parsing calculations
        // (much less than the precision necessary, but enough that tests would catch it).
        //
        // This could easily be checked by formatting an output with the proper precision, but that
        // would be testing both parsing and precision, and we want to isolate those.
        assert_exact!(parse("17.29").0, Double::from(17.29).0);
        assert_exact!(parse(".016_777_216").0, Double::from(0.016_777_216).0);
        assert_exact!(parse("0.016_777_216").0, Double::from(0.016_777_216).0);
        assert_exact!(parse("+2.317").0, Double::from(2.317).0);
        assert_exact!(parse("-0.00042").0, Double::from(-0.00042).0);
    }

    #[test]
    fn long_float() {
        // Using closeness for comparisons here, because the input numbers are long enough to
        // warrant it
        assert_close!(parse(PI_50), Double::PI);
        assert_close!(parse(E_50), Double::E);
    }

    #[test]
    fn exp_integer() {
        assert_exact!(parse("1729e0"), Double::from(1729));
        assert_exact!(parse("16_777_216e+1"), Double::from(167772160));
        assert_exact!(parse("+231700000E-5"), Double::from(2317));
        assert_exact!(parse("-42E3"), Double::from(-42000));
    }

    #[test]
    fn long_exp_integer() {
        assert_eq!(
            parse(PI_TIMES_10_20_EXP),
            (Double::PI * Double::from(10).powi(20)).floor()
        );
        assert_eq!(
            parse(E_TIMES_10_25_EXP),
            (Double::E * Double::from(10).powi(25)).floor()
        );
    }

    #[test]
    fn exp_float() {
        assert_close!(parse("17.29e0").0, Double::from(17.29).0);
        assert_close!(parse("1.6777216e-2").0, Double::from(0.016_777_216).0);
        assert_close!(parse("0.16777216e-1").0, Double::from(0.016_777_216).0);
        assert_close!(parse("+2.317E3").0, Double::from(2317).0);
        assert_close!(parse("-4.2e-4").0, Double::from(-0.00042).0);
    }

    #[test]
    fn long_exp_float() {
        assert_close!(parse(PI_50_3), Double::PI * dd!(1000.0));
        assert_close!(parse(E_50_NEG_2), Double::E / dd!(100.0));
    }

    #[test]
    fn error_empty() {
        assert_eq!(parse_error("").kind, ErrorKind::Empty);
    }

    #[test]
    fn error_misplaced_sign() {
        assert_eq!(parse_error("2+317").kind, ErrorKind::Invalid);
    }

    #[test]
    fn error_duplicate_sign() {
        assert_eq!(parse_error("+-2317").kind, ErrorKind::Invalid);
    }

    #[test]
    fn error_duplicate_point() {
        assert_eq!(parse_error("2.31.7").kind, ErrorKind::Invalid);
    }

    #[test]
    fn error_bad_exponent() {
        assert_eq!(parse_error("1.729e4e").kind, ErrorKind::Invalid);
    }

    #[test]
    fn error_bad_character() {
        // Not yet!
        assert_eq!(parse_error("0xcafebabe").kind, ErrorKind::Invalid);
    }
}
