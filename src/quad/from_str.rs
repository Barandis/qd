// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::error::{ErrorKind, ParseQuadError};
use crate::quad::Quad;
use std::str::FromStr;

const TEN: Quad = Quad(10.0, 0.0, 0.0, 0.0);

impl FromStr for Quad {
    type Err = ParseQuadError;

    /// Parses a string to create a `Quad`.
    ///
    /// The parser works pretty similarly to parsers for `f32` and `f64`. It will fail if
    /// characters are present that are not digits, decimal points, signs, or exponent
    /// markers. It will also fail if there are multiples of these or if they're in the
    /// wrong places; two decimal points or a negative sign after the number will both be
    /// rejected, for instance.
    ///
    /// Failure will return a [`ParseQuadError`] of some kind.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// use std::str::FromStr;
    ///
    /// let expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
    ///
    /// let x1 = Quad::from_str(
    ///     "0.9999999303082806237436760862691492808476631704421807180156648865"
    /// ).unwrap();
    /// // `parse` calls `from_str` in the background, so this is equivalent. In fact it's
    /// // probably preferred because it doesn't require importing `FromStr`. The turbofish
    /// // (or type annotation on x2, if you prefer) is required instead if the type can't
    /// // otherwise be inferred.
    /// let x2 = "0.9999999303082806237436760862691492808476631704421807180156648865"
    ///     .parse::<Quad>()
    ///     .unwrap();
    ///
    /// let diff1 = (x1 - expected).abs();
    /// assert!(diff1 < qd!(1e-60));
    ///
    /// let diff2 = (x2 - expected).abs();
    /// assert!(diff2 < qd!(1e-60));
    /// ```
    ///
    /// [`ParseQuadError`]: error/struct.ParseQuadError.html
    fn from_str(s: &str) -> Result<Quad, ParseQuadError> {
        let mut result = Quad::ZERO;
        let mut digits = 0;
        let mut point = -1;
        let mut sign = 0;
        let mut exp = 0;

        let s = s.trim().to_ascii_lowercase();

        match pre_from_str(&s) {
            Some(r) => r,
            None => {
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
                                    return Err(ParseQuadError {
                                        kind: ErrorKind::Invalid,
                                    });
                                }
                                point = digits;
                            }
                            '-' => {
                                if sign != 0 || digits > 0 {
                                    return Err(ParseQuadError {
                                        kind: ErrorKind::Invalid,
                                    });
                                }
                                sign = -1;
                            }
                            '+' => {
                                if sign != 0 || digits > 0 {
                                    return Err(ParseQuadError {
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
                                        return Err(ParseQuadError {
                                            kind: ErrorKind::Invalid,
                                        });
                                    }
                                }
                            }
                            '_' => {
                                // just continue; _ is a no-op but not an error
                            }
                            _ => {
                                return Err(ParseQuadError {
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
                    // Do this in two stages if the exponent is too small For exmaple, a
                    // number with 30 digits could have an exponent as low as -337 and still
                    // not overflow, but doing the -337 all at once WOULD overflow
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
}

#[inline]
fn pre_from_str(s: &str) -> Option<Result<Quad, ParseQuadError>> {
    if s.is_empty() {
        Some(Err(ParseQuadError {
            kind: ErrorKind::Empty,
        }))
    } else if s == "nan" {
        Some(Ok(Quad::NAN))
    } else if s == "inf" || s == "infinity" {
        Some(Ok(Quad::INFINITY))
    } else if s == "-inf" || s == "-infinity" {
        Some(Ok(Quad::NEG_INFINITY))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! single {
        ($e:expr, $a:expr) => {
            assert!($a[0] - $e < 1e-60, "component 0 not equal");
            assert!($a[1] < 1e-60, "component 1 not equal");
            assert!($a[2] < 1e-60, "component 2 not equal");
            assert!($a[3] < 1e-60, "component 3 not equal");
        };
    }

    fn parse(s: &str) -> Quad {
        s.parse().unwrap()
    }

    fn parse_err(s: &str) -> ErrorKind {
        s.parse::<Quad>().unwrap_err().kind
    }

    // error tests
    test_all_eq!(
        empty:
            ErrorKind::Empty,
            parse_err("");
        double_sign:
            ErrorKind::Invalid,
            parse_err("++2317");
        double_point:
            ErrorKind::Invalid,
            parse_err("2.31.7");
        mid_sign:
            ErrorKind::Invalid,
            parse_err("2-317");
        end_letter:
            ErrorKind::Invalid,
            parse_err("2.317err");
        mid_letter:
            ErrorKind::Invalid,
            parse_err("2.3j7");
    );

    // zero tests
    test_all_exact!(
        zero_int:
            Quad::ZERO,
            parse("0");
        zero_float:
            Quad::ZERO,
            parse("0.0");
        zero_plus_int:
            Quad::ZERO,
            parse("+0");
        zero_plus_float:
            Quad::ZERO,
            parse("+0.0");
        zero_minus_int:
            Quad::NEG_ZERO,
            parse("-0");
        zero_minus_float:
            Quad::NEG_ZERO,
            parse("-0.0");
    );

    test!(single_int: {
        single!(1.0, parse("1"));
        single!(2317.0, parse("2317"));
        single!(16_777_216.0, parse("16_777_216"));
    });

    // With any number big enough to use more than one component, the half-ulp normalization
    // requirement and the possibility of having differing floating-point precisions between
    // the components means that the components will not simply be their part of the whole
    // integer. For example, in the first test below, one might expect that the components
    // will be
    //
    //      1.234567890123456e31
    //      1.234567890123456e15
    //
    // Instead, the real values are
    //
    //      1.2345678901234562e31
    //      -1.064442023724352e15
    //
    // This makes it prohibitively difficult to write tests for the exact component values.
    // Instead we construct one value by parsing a string and construct the other value
    // directly through math between double-precision values. The components of each should
    // be the same if the parsing is being done correctly.

    test!(double_int: {
        let s = parse("12345678901234561234567890123456");
        let a = qd!(1_234_567_890_123_456.0);

        let mut n = qd!(a);
        n *= qd!(10).powi(16);
        n += qd!(a);
        exact!(n, s);
    });

    test!(triple_int: {
        let s = parse("123456789012345612345678901234561234567890123456");
        let a = qd!(1_234_567_890_123_456.0);

        let mut n = qd!(a) * qd!(10).powi(32);
        n += qd!(a) * qd!(10).powi(16);
        n += qd!(a);
        exact!(n, s);
    });

    test!(quadruple_int: {
        let s = parse("1234567890123456123456789012345612345678901234561234567890123456");
        let a = qd!(1_234_567_890_123_456.0);

        let mut n = qd!(a) * qd!(10).powi(48);
        n += qd!(a) * qd!(10).powi(32);
        n += qd!(a) * qd!(10).powi(16);
        n += qd!(a);
        exact!(n, s);
    });

    // The parsed values in the first asserts in each test below are of the form (2ⁿ - 1) /
    // 2ⁿ. Since this is the same as the sum of the series 1/2⁰ + 1/2¹ + 1/2² + ... 1/2ⁿ,
    // these numbers are exactly representable in binary.
    //
    // The second asserts use numbers in the form (3ⁿ - 1) / 3ⁿ where n = 15, rounded to the
    // correct number of digits. Since these are not sums of powers of 2, they are *not*
    // exactly representable in binary.
    //
    // Parsing any floating-point number will introduce inexactness just because of the
    // nature of the math used in parsing. However this error will be less than the best
    // precision offered by the type (most of them are accurate to about 68 digits when only
    // 63-64 is offered). Therefore `assert_close` is used rather than `assert_exact`.

    test!(single_float: {
        // n = 15
        single!(0.999_084_472_656_25, parse("0.99908447265625"));
        let three_expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
        prec!(three_expected, parse("0.9999999303082806"), 15);
    });

    test!(double_float: {
        // n = 31
        let s = parse("0.9999999995343387126922607421875");
        let t = qd!(2).powi(31);
        let x = (t - qd!(1)) / t;
        near!(x, s);

        let three_expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
        prec!(
            three_expected,
            parse("0.9999999303082806237436760862691"),
            30
        );
    });

    test!(triple_float: {
        // n = 47
        let s = parse("0.99999999999999289457264239899814128875732421875");
        let t = qd!(2).powi(47);
        let x = (t - qd!(1)) / t;
        near!(x, s);

        let three_expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
        prec!(
            three_expected,
            parse("0.99999993030828062374367608626914928084766317044"),
            45
        );
    });

    #[test]
    fn quadruple_float() {
        // n = 63
        let s = parse("0.999999999999999999891579782751449556599254719913005828857421875");
        let t = qd!(2).powi(63);
        let x = (t - qd!(1)) / t;
        near!(x, s);

        let three_expected = (qd!(3).powi(15) - qd!(1)) / qd!(3).powi(15);
        prec!(
            three_expected,
            parse("0.999999930308280623743676086269149280847663170442180718015664887"),
            60
        );
    }

    test!(exponent: {
        let s = parse("0.999999999999999999891579782751449556599254719913005828857421875e100");
        let t = qd!(2).powi(63);
        let x = ((t - qd!(1)) / t) * qd!(10).powi(100);
        near!(x, s);

        let s = parse("0.999999999999999999891579782751449556599254719913005828857421875e-100");
        let t = qd!(2).powi(63);
        let x = ((t - qd!(1)) / t) * qd!(10).powi(-100);
        near!(x, s);
    });
}
