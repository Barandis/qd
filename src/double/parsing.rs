// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use crate::error::{ErrorKind, ParseError};
use std::char;
use std::fmt;
use std::str::FromStr;

// #region Parsing

impl FromStr for Double {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Double, ParseError> {
        let mut result = Double::from(0);
        let mut digits = 0;
        let mut point = -1;
        let mut sign = 0;
        let mut exp = 0;

        let s = s.trim();

        if s.is_empty() {
            return Err(ParseError {
                kind: ErrorKind::Empty,
            });
        }

        if s.to_ascii_lowercase() == "nan" {
            return Ok(Double::NAN);
        }
        if s.to_ascii_lowercase() == "inf" {
            return Ok(Double::INFINITY);
        }
        if s.to_ascii_lowercase() == "-inf" {
            return Ok(Double::NEG_INFINITY);
        }

        for (index, ch) in s.chars().enumerate() {
            match ch.to_digit(10) {
                Some(d) => {
                    result *= 10.0;
                    result += d as f64;
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
                    'e' | 'E' => {
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
            result *= Double::from(10.0).powi(exp);
        }
        if sign == -1 {
            result = -result;
        }

        Ok(result)
    }
}

// #endregion

// #region Display implementations

const DEFAULT_PRECISION: usize = 31;

// Calculates the exponent of the supplied double-double, adjusting the double-double to fall
// somewhere in the range [1, 10) (i.e., to have a single non-zero digit before the decimal point).
#[inline]
fn calculate_exponent(r: &mut Double) -> i32 {
    // Quick calculation of exponent based on the first component of `r`. This could turn out to be
    // off by 1 either direction depending on the second component.
    let mut exp = r.0.abs().log10().floor() as i32;

    // Adjust `r` based on that exponent approximation
    if exp < -300 {
        *r *= Double::from(10.0).powi(300);
        *r /= Double::from(10.0).powi(exp + 300);
    } else if exp > 300 {
        *r = r.ldexp(-53);
        *r /= Double::from(10.0).powi(exp);
        *r = r.ldexp(53);
    } else {
        *r /= Double::from(10.0).powi(exp);
    }

    // If `r` is outside the range [1, 10), then the exponent was off by 1. Adjust both it and `r`.
    if *r >= 10.0 {
        *r /= 10.0;
        exp += 1;
    } else if *r < 1.0 {
        *r *= 10.0;
        exp -= 1;
    }

    exp
}

// Extracts the digits of `r` into a vector of integers. These integers will fall in the range [-9,
// 9]. Even if `r` is always positive as a whole, its second component can be negative which will
// generate negative 'digits'.
//
// `r` is modified throughout to extract the digits and contains nothing of value when this function
// is complete.
#[inline]
fn extract_digits(r: &mut Double, precision: usize) -> Vec<i32> {
    let mut digits = Vec::with_capacity(precision);
    for _ in 0..precision {
        let digit = r.0 as i32;
        *r -= digit as f64;
        *r *= 10.0;
        digits.push(digit);
    }
    digits
}

// Adjusts the range of integers in the supplied vector from [9, -9] to [0, 9]. (This function will
// handle 'digits' up to 19, but I don't believe in this application that they're ever over 9.)
#[inline]
fn correct_range(digits: &mut Vec<i32>) {
    for i in (1..digits.len()).rev() {
        if digits[i] < 0 {
            digits[i - 1] -= 1;
            digits[i] += 10;
        } else if digits[i] > 9 {
            digits[i - 1] += 1;
            digits[i] -= 10;
        }
    }
}

// Rounds the second-to-last digit of an i32 vector based on the value of the last digit. This
// rounding is standard round-to-even in the case of a final digit of 5. Any necessary carrying is
// propagated as far as it needs to, adjusting the exponent if the carry goes all the way to the
// first digit.
#[inline]
fn round_vec(digits: &mut Vec<i32>, exp: &mut i32) {
    let len = digits.len();
    if digits[len - 1] > 5 || digits[len - 1] == 5 && digits[len - 2] % 2 == 1 {
        digits[len - 2] += 1;
        let mut i = len - 2;
        while i > 0 && digits[i] > 9 {
            digits[i] -= 10;
            digits[i - 1] += 1;
            i -= 1;
        }
    }

    // If the first digit requires carry, insert one more digit to turn 9 into 10
    // and adjust the exponent
    if digits[0] > 9 {
        *exp += 1;
        digits[0] = 0;
        digits.insert(0, 1);
    }
}

// Turns a double-double into a vector of digits and an exponent. Sign is ignored, and no decimal
// appears in the vector; the exponent is calculated based on having the decimal point after the
// first digit.
//
// This function returns a vector of signed integers even though unsigned would make more logical
// sense. That's because internally (with the call to `extract_digits`) the vector has to deal with
// signed integers, and it's more efficient to let the caller cast them to unsigned as needed than
// it is to create a new vector of unsigned integers and copy them over.
fn to_digits(r: &Double, precision: usize) -> (Vec<i32>, i32) {
    let mut r = r.abs();

    if r.is_zero() {
        return (vec![0; precision], 0);
    }

    let mut exp = calculate_exponent(&mut r);
    // We pass one more than the actual precision to leave an extra digit at the end to do rounding
    let mut digits = extract_digits(&mut r, precision + 1);
    correct_range(&mut digits);
    round_vec(&mut digits, &mut exp);

    (digits, exp)
}
// Converts an integer into a character representation of that integer. This assumes that `digit` is
// between 0 and 9 inclusive. If it's not, there's a bug somewhere, so we WANT to panic; hence the
// unchecked `unwrap`.
#[inline]
fn char_from_digit(digit: &i32) -> char {
    char::from_digit(*digit as u32, 10).unwrap()
}

// Potentially pushes a sign character to the supplied vector. Returns whether or not a character
// was actually added, information that is used later in formatting.
#[inline]
fn push_sign(chars: &mut Vec<char>, value: &Double, formatter: &fmt::Formatter) -> bool {
    let mut sign = true;
    if value.is_sign_negative() {
        chars.push('-');
    } else if formatter.sign_plus() {
        chars.push('+');
    } else {
        sign = false;
    }
    sign
}

// Appends "NaN" to the supplied vector.
#[inline]
fn push_nan(chars: &mut Vec<char>) {
    chars.append(&mut "NaN".chars().collect());
}

// Appends "inf" to the supplied vector.
#[inline]
fn push_inf(chars: &mut Vec<char>) {
    chars.append(&mut "inf".chars().collect());
}

// Pushes the number zero to the supplied vector. If the formatter has a precision set, then it will
// add that many zeros behind the decimal; if none is set, it'll just push "0.0".
#[inline]
fn push_zero(chars: &mut Vec<char>, formatter: &fmt::Formatter) {
    match formatter.precision() {
        Some(p) if p > 0 => {
            chars.push('0');
            chars.push('.');
            for _ in 0..p {
                chars.push('0');
            }
        }
        _ => {
            chars.push('0');
        }
    }
}

// Rounds a vector of digits based on the precision. This is -almost- identical to `round_vec` above
// except that it rounds to precision (not to one past precision). It is necessary because fixed-
// point numbers are calculated (and rounded) at greater than their needed precision for accuracy.
// Hence they need to be rounded to the correct number of digits after they return.
//
// `offset` + `precision` is presumed to be positive. If not, `push_fixed_digits` won't call this
// function.
//
// TODO: This can be made more efficient. Exponentials are rounded in `round_vec`; fixed are rounded
// there unnecessarily and then here as well.
#[inline]
fn round_fixed_digits(digits: &mut Vec<i32>, offset: &mut i32, precision: Option<usize>) {
    let d = match precision {
        Some(p) => *offset + p as i32,
        None => *offset.min(&mut 0) + DEFAULT_PRECISION as i32, // no more than 31 digits
    } as usize;

    if digits[d] > 5 || digits[d] == 5 && digits[d - 1] % 2 == 1 {
        digits[d - 1] += 1;
        let mut i = d - 1;
        while i > 0 && digits[i] > 9 {
            digits[i] -= 10;
            digits[i - 1] += 1;
            i -= 1;
        }
    }

    // If the first digit requires carry, insert one more digit to turn 9 into 10
    // and adjust the offset
    if digits[0] > 9 {
        digits[0] = 0;
        digits.insert(0, 1);
        *offset += 1;
    }
}

// Converts all of the digits, up to the number indicated by `precision`, into characters and
// pushes them onto the supplied character vector. `offset` determines where the decimal point is
// placed. This is used to create a fixed-point output format.
#[inline]
fn push_fixed_digits(
    chars: &mut Vec<char>,
    digits: &mut Vec<i32>,
    exp: i32,
    precision: Option<usize>,
) {
    let mut offset = exp + 1;
    let pr_value = precision.unwrap_or(DEFAULT_PRECISION);

    if pr_value as i32 + offset <= 0 {
        // Offset is greater than precision, give zero at given precision
        chars.push('0');
        if pr_value > 0 {
            chars.push('.');
            for _ in 0..pr_value {
                chars.push('0');
            }
        }
    } else {
        round_fixed_digits(digits, &mut offset, precision);

        if offset > 0 {
            let offset = offset as usize;
            for digit in &digits[..offset] {
                chars.push(char_from_digit(digit));
            }
            match precision {
                Some(p) => {
                    if p > 0 {
                        chars.push('.');
                        for digit in &digits[offset..offset + p] {
                            chars.push(char_from_digit(digit));
                        }
                    }
                }
                None => {
                    chars.push('.');
                    // limit to 31 characters, whatever that precision is
                    for digit in &digits[offset..DEFAULT_PRECISION] {
                        chars.push(char_from_digit(digit));
                    }
                }
            }
        } else {
            chars.push('0');
            chars.push('.');
            if offset < 0 {
                for _ in 0..-offset {
                    chars.push('0');
                }
            }
            let max = (offset + pr_value as i32) as usize;
            for digit in &digits[..max] {
                chars.push(char_from_digit(digit));
            }
        }
    }
}

// Converts all of the digits, up to the number indicated by `precision`, into characters and
// pushes them onto the supplied character vector. If there is a decimal point (i.e, if `precision`
// is not 0), it will always be after the first digit. This is used to create an exponential output
// format.
#[inline]
fn push_exp_digits(chars: &mut Vec<char>, digits: &Vec<i32>, precision: Option<usize>) {
    let precision = precision.unwrap_or(DEFAULT_PRECISION);
    chars.push(char_from_digit(&digits[0]));
    if precision > 0 {
        chars.push('.');
    }
    for digit in &digits[1..precision + 1] {
        chars.push(char_from_digit(digit));
    }
}

// Drops trailing zeros after the decimal point (and the decimal point as well, if necessary). This
// happens only if no precision was supplied to the formatter. In that case the number is given
// as many decimal places as it needs minus the trailing zeros.
#[inline]
fn drop_trailing_zeros(chars: &mut Vec<char>, formatter: &fmt::Formatter) {
    if let None = formatter.precision() {
        if chars.contains(&'.') {
            if let Some(index) = chars.clone().into_iter().rposition(|c| c != '0') {
                // Drop the decimal point itself if everything after it is a zero
                let new_length = match chars[index] {
                    '.' => index,
                    _ => index + 1,
                };
                chars.truncate(new_length);
            }
        }
    }
}

// Pushes the exponent to the supplied character vector. It includes a leading marker character,
// which should be either 'e' or 'E'.
#[inline]
fn push_exponent(chars: &mut Vec<char>, marker: char, exp: i32) {
    chars.push(marker);
    chars.append(&mut exp.to_string().chars().collect());
}

// Adjusts the character vector for width, precision, alignment, and fill characters. The vector is
// expanded as needed to accomodate the width.
#[inline]
fn align_and_fill(chars: &mut Vec<char>, formatter: &mut fmt::Formatter, sign: bool) {
    if let Some(width) = formatter.width() {
        let len = chars.len();
        if len < width {
            let delta = width - chars.len();
            let fill = formatter.fill();
            match formatter.align() {
                Some(fmt::Alignment::Left) => {
                    for _ in 0..delta {
                        chars.push(fill);
                    }
                }
                Some(fmt::Alignment::Right) => {
                    for _ in 0..delta {
                        chars.insert(0, fill);
                    }
                }
                Some(fmt::Alignment::Center) => {
                    let left_delta = delta / 2;
                    let right_delta = delta - left_delta;
                    for _ in 0..left_delta {
                        chars.insert(0, fill);
                    }
                    for _ in 0..right_delta {
                        chars.push(fill);
                    }
                }
                None => {
                    if formatter.sign_aware_zero_pad() {
                        let index = if sign { 1 } else { 0 };
                        for _ in 0..delta {
                            chars.insert(index, '0');
                        }
                    } else {
                        for _ in 0..delta {
                            chars.insert(0, fill);
                        }
                    }
                }
            }
        }
    }
}

// Formats `value` as a fixed-point number, with the format defined by `f`.
#[inline]
fn format_fixed(value: &Double, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = Vec::new();
    let mut sign = true;
    let precision = f.precision().unwrap_or(DEFAULT_PRECISION);

    if value.is_nan() {
        push_nan(&mut result);
    } else {
        sign = push_sign(&mut result, value, f);

        if value.is_infinite() {
            push_inf(&mut result);
        } else if value.is_zero() {
            push_zero(&mut result, f);
        } else {
            let width = precision as i32 + value.abs().log10().floor().to_int() + 1;
            // Higher than the max-length number + max precision so that users can do
            // their format!("{:.30}", Double::from_str("999999999999999999999999999999")) in
            // peace
            let extra = width.max(65);

            // Special case: zero precision, |value| < 1.0
            // In this case a number greater than 0.5 prints 0 and should print 1
            if precision == 0 && value.abs() < 1.0 {
                result.push(if value.abs() >= 0.5 { '1' } else { '0' });
            } else if width < 0 {
                push_zero(&mut result, f);
            } else {
                let (mut digits, exp) = to_digits(value, extra as usize);
                push_fixed_digits(&mut result, &mut digits, exp, f.precision());
            }
        }

        if !value.is_infinite() {
            drop_trailing_zeros(&mut result, f);
        }
    }
    align_and_fill(&mut result, f, sign);

    write!(f, "{}", result.into_iter().collect::<String>())
}

// Formats `value` as a exponential number, with the format defined by `f`.
#[inline]
fn format_exp(value: &Double, f: &mut fmt::Formatter, upper: bool) -> fmt::Result {
    let mut result = Vec::new();
    let mut sign = true;
    let mut exp = 0;

    if value.is_nan() {
        push_nan(&mut result);
    } else {
        sign = push_sign(&mut result, value, f);

        if value.is_infinite() {
            push_inf(&mut result);
        } else if value.is_zero() {
            push_zero(&mut result, f);
        } else {
            let width = f.precision().unwrap_or(DEFAULT_PRECISION) + 1;
            let (digits, e) = to_digits(value, width);
            exp = e;
            push_exp_digits(&mut result, &digits, f.precision());
        }

        if !value.is_infinite() {
            drop_trailing_zeros(&mut result, f);
            let marker = if upper { 'E' } else { 'e' };
            push_exponent(&mut result, marker, exp);
        }
    }
    align_and_fill(&mut result, f, sign);

    write!(f, "{}", result.into_iter().collect::<String>())
}

impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_fixed(self, f)
    }
}

impl fmt::LowerExp for Double {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_exp(self, f, false)
    }
}

impl fmt::UpperExp for Double {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_exp(self, f, true)
    }
}

// #endregion

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

    fn close(a: Double, b: Double) -> bool {
        (a - b).abs() < 1e-28
    }

    // #region Parsing tests

    #[test]
    fn parse_nan() {
        assert!(parse("nan").is_nan());
    }

    #[test]
    fn parse_inf() {
        assert_eq!(parse("inf"), Double::INFINITY);
    }

    #[test]
    fn parse_neg_inf() {
        assert_eq!(parse("-inf"), Double::NEG_INFINITY);
    }

    #[test]
    fn parse_zero() {
        assert_eq!(parse("0"), Double::from(0));
        assert_eq!(parse("-0"), Double::from(-0.0));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(parse("1729"), Double::from(1729));
        assert_eq!(parse("16_777_216"), Double::from(16_777_216));
        assert_eq!(parse("+2317"), Double::from(2317));
        assert_eq!(parse("-42"), Double::from(-42));
    }

    #[test]
    fn parse_long_integer() {
        assert_eq!(
            parse(PI_TIMES_10_20),
            (Double::PI * Double::from(10).powi(20)).floor()
        );
        assert_eq!(
            parse(E_TIMES_10_25),
            (Double::E * Double::from(10).powi(25)).floor()
        );
    }

    #[test]
    fn parse_float() {
        // Using just the first component for comparisons here, because on numbers that don't
        // need that much precision, there is a notable error component in the parsing calculations
        // (much less than the precision necessary, but enough that tests would catch it).
        //
        // This could easily be checked by formatting an output with the proper precision, but that
        // would be testing both parsing and precision, and we want to isolate those.
        assert_eq!(parse("17.29").0, Double::from(17.29).0);
        assert_eq!(parse(".016_777_216").0, Double::from(0.016_777_216).0);
        assert_eq!(parse("0.016_777_216").0, Double::from(0.016_777_216).0);
        assert_eq!(parse("+2.317").0, Double::from(2.317).0);
        assert_eq!(parse("-0.00042").0, Double::from(-0.00042).0);
    }

    #[test]
    fn parse_long_float() {
        // Using closeness for comparisons here, because the input numbers are long enough to
        // warrant it
        assert!(close(parse(PI_50), Double::PI));
        assert!(close(parse(E_50), Double::E));
    }

    #[test]
    fn parse_exp_integer() {
        assert_eq!(parse("1729e0"), Double::from(1729));
        assert_eq!(parse("16_777_216e+1"), Double::from(167772160));
        assert_eq!(parse("+231700000E-5"), Double::from(2317));
        assert_eq!(parse("-42E3"), Double::from(-42000));
    }

    #[test]
    fn parse_long_exp_integer() {
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
    fn parse_exp_float() {
        assert_eq!(parse("17.29e0").0, Double::from(17.29).0);
        assert_eq!(parse("1.6777216e-2").0, Double::from(0.016_777_216).0);
        assert_eq!(parse("0.16777216e-1").0, Double::from(0.016_777_216).0);
        assert_eq!(parse("+2.317E3").0, Double::from(2317).0);
        assert_eq!(parse("-4.2e-4").0, Double::from(-0.00042).0);
    }

    #[test]
    fn parse_long_exp_float() {
        assert!(close(parse(PI_50_3), Double::PI * 1000.0));
        assert!(close(parse(E_50_NEG_2), Double::E / 100.0));
    }

    #[test]
    fn parse_error_empty() {
        assert_eq!(parse_error("").kind, ErrorKind::Empty);
    }

    #[test]
    fn parse_error_misplaced_sign() {
        assert_eq!(parse_error("2+317").kind, ErrorKind::Invalid);
    }

    #[test]
    fn parse_error_duplicate_sign() {
        assert_eq!(parse_error("+-2317").kind, ErrorKind::Invalid);
    }

    #[test]
    fn parse_error_duplicate_point() {
        assert_eq!(parse_error("2.31.7").kind, ErrorKind::Invalid);
    }

    #[test]
    fn parse_error_bad_exponent() {
        assert_eq!(parse_error("1.729e4e").kind, ErrorKind::Invalid);
    }

    #[test]
    fn parse_error_bad_character() {
        // Not yet!
        assert_eq!(parse_error("0xcafebabe").kind, ErrorKind::Invalid);
    }

    // #endregion

    // #region Formatting tests

    // #region Plain formatting

    fn plain(value: Double) -> String {
        format!("{}", value)
    }

    fn close_str(actual: &str, expected: &str) -> bool {
        let len = expected.len() - 1;
        &actual[0..len] == &expected[0..len]
    }

    #[test]
    fn format_integer() {
        assert_eq!(format!("{}", Double::from(23)), "23");
        assert_eq!(format!("{}", Double::from(-17)), "-17");
        assert_eq!(
            format!("{}", Double::from_str(PI_TIMES_10_20).unwrap()),
            PI_TIMES_10_20
        );
        assert_eq!(format!("{}", Double::from(0)), "0");
        assert_eq!(format!("{}", Double::from(-0.0)), "-0");
    }

    #[test]
    fn format_special() {
        assert_eq!(plain(Double::NAN), "NaN");
        assert_eq!(plain(Double::INFINITY), "inf");
        assert_eq!(plain(Double::NEG_INFINITY), "-inf");
    }

    #[test]
    fn format_float() {
        // Floating point error will keep these from being displayed exactly when no precision is
        // defined, because the default precision will extend into the deep bits of these numbers.
        // So we're checking to see if they're close.
        assert!(close_str(plain(Double::from(17.29)).as_str(), "17.29"));
        assert!(close_str(
            plain(Double::from(0.016_777_216)).as_str(),
            "0.016777216"
        ));
        assert!(close_str(plain(Double::from(2.317)).as_str(), "2.317"));
        assert!(close_str(plain(Double::from(0.00042)).as_str(), "0.00042"));
    }

    #[test]
    fn format_integer_exp() {
        assert_eq!(plain(Double::from(1729e0)), "1729");
        assert_eq!(plain(Double::from(16_777_216e+1)), "167772160");
        assert_eq!(plain(Double::from(231700000E-5)), "2317");
        assert_eq!(plain(Double::from(-42e3)), "-42000");
    }

    #[test]
    fn format_float_exp() {
        assert!(close_str(plain(Double::from(17.29e0)).as_str(), "17.29"));
        assert!(close_str(
            plain(Double::from(1.6777216e-1)).as_str(),
            "0.16777216"
        ));
        assert!(close_str(plain(Double::from(2.317e2)).as_str(), "231.7"));
        assert!(close_str(plain(Double::from(-4.2e-4)).as_str(), "-0.00042"));
    }

    // This is a test for an issue that I have seen mentioned nowhere except in the source code
    // of the MIT library source code. It claims that for numbers of the form 10^x - 1, the decimal
    // point can be printed in the wrong place.
    //
    // I have not seen evidence of this, and it's one otherwise-unmentioned block of code in
    // software that was written more than a decade ago. The "fix" has been taken out of the code
    // but I'm leaving in the test just in case.
    #[test]
    fn format_offset_10_x_minus_1() {
        assert_eq!(
            plain(Double::from(10).powi(29) - 1.0),
            "99999999999999999999999999999"
        );
        assert_eq!(
            plain(Double::from(10).powi(30) - 1.0),
            "999999999999999999999999999999"
        );
        assert_eq!(
            plain(Double::from(10).powi(29) - 2.0),
            "99999999999999999999999999998"
        );
    }

    // #endregion

    // #region Exponential formatting

    fn exp(value: Double) -> String {
        format!("{:e}", value)
    }

    fn close_exp(actual: &str, expected: &str) -> bool {
        let ex_parts: Vec<&str> = expected.split('e').collect();
        let ac_parts: Vec<&str> = actual.split('e').collect();

        let len = ex_parts[0].len() - 1;
        &ac_parts[0][0..len] == &ex_parts[0][0..len] && &ac_parts[1] == &ex_parts[1]
    }

    #[test]
    fn format_exp_integer() {
        assert_eq!(format!("{:e}", Double::from(23)), "2.3e1");
        assert_eq!(format!("{:e}", Double::from(-17)), "-1.7e1");
        assert_eq!(
            format!("{:e}", Double::from_str(PI_TIMES_10_20).unwrap()),
            PI_TIMES_10_20_EXP
        );
        assert_eq!(format!("{:e}", Double::from(0)), "0e0");
    }

    #[test]
    fn format_exp_special() {
        assert_eq!(exp(Double::NAN), "NaN");
        assert_eq!(exp(Double::INFINITY), "inf");
        assert_eq!(exp(Double::NEG_INFINITY), "-inf");
    }

    #[test]
    fn format_exp_float() {
        // Floating point error will keep these from being displayed exactly when no precision is
        // defined, because the default precision will extend into the deep bits of these numbers.
        // So we're checking to see if they're close.
        assert!(close_exp(exp(Double::from(17.29)).as_str(), "1.729e1"));
        assert!(close_exp(
            exp(Double::from(0.016_777_216)).as_str(),
            "1.6777216e-2"
        ));
        assert!(close_exp(exp(Double::from(2.317)).as_str(), "2.317e0"));
        assert!(close_exp(exp(Double::from(-0.00042)).as_str(), "-4.2e-4"));
    }

    #[test]
    fn format_exp_integer_exp() {
        assert_eq!(exp(Double::from(1729e0)), "1.729e3");
        assert_eq!(exp(Double::from(16_777_216e+1)), "1.6777216e8");
        assert_eq!(exp(Double::from(231700000E-5)), "2.317e3");
        assert_eq!(exp(Double::from(-42e3)), "-4.2e4");
    }

    #[test]
    fn format_exp_float_exp() {
        assert!(close_exp(exp(Double::from(17.29e0)).as_str(), "1.729e1"));
        assert!(close_exp(
            exp(Double::from(1.6777216e-1)).as_str(),
            "1.6777216e-1"
        ));
        assert!(close_exp(exp(Double::from(2.317e2)).as_str(), "2.317e2"));
        assert!(close_exp(exp(Double::from(-4.2e-4)).as_str(), "-4.2e-4"));
    }

    // #endregion

    // #region Precision formatting

    #[test]
    fn format_precision_integer() {
        assert_eq!(format!("{:.3}", Double::from(23)), "23.000");
        assert_eq!(format!("{:.0}", Double::from(-17)), "-17");
        assert_eq!(format!("{}", Double::from(0)), "0");
        assert_eq!(format!("{:.0}", Double::from(0)), "0");
        assert_eq!(format!("{:.10}", Double::from(0)), "0.0000000000");
    }

    #[test]
    fn format_precision_float() {
        assert_eq!(format!("{:.0}", Double::from(17.29)), "17");
        assert_eq!(format!("{:.6}", Double::from(0.016_777_216)), "0.016777");
        assert_eq!(format!("{:.5}", Double::from(0.016_777_216)), "0.01678");
        assert_eq!(
            format!("{:.12}", Double::from(0.016_777_216)),
            "0.016777216000"
        );
        assert_eq!(format!("{:.0}", Double::from(0.016_777_216)), "0");
        assert_eq!(format!("{:.0}", Double::from(-0.016_777_216)), "-0");
        assert_eq!(format!("{:.4}", Double::from(0.0000016_777_216)), "0.0000");
    }

    #[test]
    fn format_precision_exp() {
        let value = Double::from(0.016_777_216);
        assert_eq!(format!("{:.3e}", value), "1.678e-2");
        assert_eq!(format!("{:.4e}", value), "1.6777e-2");
        assert_eq!(format!("{:.10e}", value), "1.6777216000e-2");
        assert_eq!(format!("{:.0e}", value), "2e-2");
    }

    #[test]
    fn format_precision_alt() {
        let value = Double::from(0.016_777_216);
        assert_eq!(format!("{:.*e}", 3, value), "1.678e-2");
        assert_eq!(format!("{0:.1$e}", value, 4), "1.6777e-2");
        assert_eq!(format!("{:.prec$e}", value, prec = 10), "1.6777216000e-2");
    }

    // #endregion

    // #region Width, fill, and alignment formatting

    #[test]
    fn format_width_default_align() {
        let value = Double::from(123456);
        assert_eq!(format!("{:3}", value), "123456");
        assert_eq!(format!("{:6}", value), "123456");
        assert_eq!(format!("{:10}", value), "    123456");
        assert_eq!(format!("{:10}", -value), "   -123456");
        assert_eq!(format!("{:10e}", value), " 1.23456e5");
    }

    #[test]
    fn format_width_right_align() {
        let value = Double::from(123456);
        assert_eq!(format!("{:>3}", value), "123456");
        assert_eq!(format!("{:>6}", value), "123456");
        assert_eq!(format!("{:>10}", value), "    123456");
        assert_eq!(format!("{:>10}", -value), "   -123456");
        assert_eq!(format!("{:>10e}", value), " 1.23456e5");
    }

    #[test]
    fn format_width_left_align() {
        let value = Double::from(123456);
        assert_eq!(format!("{:<3}", value), "123456");
        assert_eq!(format!("{:<6}", value), "123456");
        assert_eq!(format!("{:<10}", value), "123456    ");
        assert_eq!(format!("{:<10}", -value), "-123456   ");
        assert_eq!(format!("{:<10e}", value), "1.23456e5 ");
    }

    #[test]
    fn format_width_center_align() {
        let value = Double::from(123456);
        assert_eq!(format!("{:^3}", value), "123456");
        assert_eq!(format!("{:^6}", value), "123456");
        assert_eq!(format!("{:^10}", value), "  123456  ");
        assert_eq!(format!("{:^10}", -value), " -123456  ");
        assert_eq!(format!("{:^11}", value), "  123456   ");
        assert_eq!(format!("{:^11e}", value), " 1.23456e5 ");
    }

    #[test]
    fn format_width_fill() {
        let value = Double::from(123456);
        assert_eq!(format!("{:*^3}", value), "123456");
        assert_eq!(format!("{:*^10}", value), "**123456**");
        assert_eq!(format!("{:*>10}", value), "****123456");
        assert_eq!(format!("{:*<10}", value), "123456****");
        assert_eq!(format!("{:*>10}", -value), "***-123456");
        assert_eq!(format!("{:*>10e}", value), "*1.23456e5");
    }

    #[test]
    fn format_width_sign_aware_zero_fill() {
        let value = Double::from(123456);
        assert_eq!(format!("{:03}", value), "123456");
        assert_eq!(format!("{:010}", value), "0000123456");
        assert_eq!(format!("{:010}", -value), "-000123456");
        assert_eq!(format!("{:0>10}", -value), "000-123456");
        assert_eq!(format!("{:012e}", -value), "-001.23456e5");
    }

    // #endregion

    // #region Miscellaneous formatting

    #[test]
    fn format_misc_plus_sign() {
        let value = Double::from(123456);
        assert_eq!(format!("{:+}", value), "+123456");
        assert_eq!(format!("{:+e}", value), "+1.23456e5");
        assert_eq!(format!("{:+12e}", value), "  +1.23456e5");
        assert_eq!(format!("{:*^+12e}", value), "*+1.23456e5*");
        assert_eq!(format!("{:0>+12e}", value), "00+1.23456e5");
        assert_eq!(format!("{:+012e}", value), "+001.23456e5");
    }

    #[test]
    fn format_misc_big_number() {
        let value = Double::from_str("123456789012345678901234567890").unwrap();
        // Not checking the value here because we don't even do 60 digits of precision, just
        // checking that formatting will actually print out 60 digits (and the decimal point)
        assert_eq!(format!("{:.30}", value).len(), 61);
    }

    // #endregion

    // #endregion
}

// #endregion
