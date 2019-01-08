// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::DoubleDouble;
use crate::error::{ParseQdFloatError, QdFloatErrorKind};
use std::char;
use std::fmt;
use std::str::FromStr;

// #region Parsing

impl FromStr for DoubleDouble {
    type Err = ParseQdFloatError;

    fn from_str(s: &str) -> Result<DoubleDouble, ParseQdFloatError> {
        let mut result = DoubleDouble::from(0);
        let mut digits = 0;
        let mut point = -1;
        let mut sign = 0;
        let mut exp = 0;

        let s = s.trim();

        if s.is_empty() {
            return Err(ParseQdFloatError {
                kind: QdFloatErrorKind::Empty,
            });
        }

        if s.to_ascii_lowercase() == "nan" {
            return Ok(DoubleDouble::NAN);
        }
        if s.to_ascii_lowercase() == "inf" {
            return Ok(DoubleDouble::INFINITY);
        }
        if s.to_ascii_lowercase() == "-inf" {
            return Ok(DoubleDouble::NEG_INFINITY);
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
                            return Err(ParseQdFloatError {
                                kind: QdFloatErrorKind::Invalid,
                            });
                        }
                        point = digits;
                    }
                    '-' => {
                        if sign != 0 || digits > 0 {
                            return Err(ParseQdFloatError {
                                kind: QdFloatErrorKind::Invalid,
                            });
                        }
                        sign = -1;
                    }
                    '+' => {
                        if sign != 0 || digits > 0 {
                            return Err(ParseQdFloatError {
                                kind: QdFloatErrorKind::Invalid,
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
                                return Err(ParseQdFloatError {
                                    kind: QdFloatErrorKind::Invalid,
                                });
                            }
                        }
                    }
                    '_' => {
                        // just continue; _ is a no-op but not an error
                    }
                    _ => {
                        return Err(ParseQdFloatError {
                            kind: QdFloatErrorKind::Invalid,
                        });
                    }
                },
            }
        }

        if point >= 0 {
            exp -= digits - point;
        }
        if exp != 0 {
            result *= DoubleDouble::from(10.0).powi(exp);
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
fn calculate_exponent(r: &mut DoubleDouble) -> i32 {
    // Quick calculation of exponent based on the first component of `r`. This could turn out to be
    // off by 1 either direction depending on the second component.
    let mut exp = r.0.abs().log10().floor() as i32;

    // Adjust `r` based on that exponent approximation
    if exp < -300 {
        *r *= DoubleDouble::from(10.0).powi(300);
        *r /= DoubleDouble::from(10.0).powi(exp + 300);
    } else if exp > 300 {
        *r = r.ldexp(-53);
        *r /= DoubleDouble::from(10.0).powi(exp);
        *r = r.ldexp(53);
    } else {
        *r /= DoubleDouble::from(10.0).powi(exp);
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
fn extract_digits(r: &mut DoubleDouble, precision: usize) -> Vec<i32> {
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
fn to_digits(r: &DoubleDouble, precision: usize) -> (Vec<i32>, i32) {
    let mut r = r.abs();

    if r == 0.0 {
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
fn push_sign(chars: &mut Vec<char>, value: &DoubleDouble, formatter: &fmt::Formatter) -> bool {
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
        Some(p) if p == 0 => {
            chars.push('0');
        }
        Some(p) => {
            chars.push('0');
            chars.push('.');
            for _ in 0..p {
                chars.push('0');
            }
        }
        None => {
            chars.append(&mut "0.0".chars().collect());
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
// TODO: This can likely be made more efficient. Exponentials are rounded in `round_vec`; fixed are
// rounded there unnecessarily and then here as well.
fn round_fixed_digits(digits: &mut Vec<i32>, offset: &mut i32, precision: usize) {
    let d = (*offset + precision as i32) as usize;
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
fn push_fixed_digits(chars: &mut Vec<char>, digits: &mut Vec<i32>, exp: i32, precision: usize) {
    let mut offset = exp + 1;
    if precision as i32 + offset <= 0 {
        chars.push('0');
        if precision > 0 {
            chars.push('.');
            for _ in 0..precision {
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
            if precision > 0 {
                chars.push('.');
                for digit in &digits[offset..offset + precision] {
                    chars.push(char_from_digit(digit));
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
            let max = (offset + precision as i32) as usize;
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
fn push_exp_digits(chars: &mut Vec<char>, digits: &Vec<i32>, precision: usize) {
    chars.push(char_from_digit(&digits[0]));
    if precision > 0 {
        chars.push('.');
    }
    for digit in &digits[1..precision] {
        chars.push(char_from_digit(digit));
    }
}

// It was suggested in the MIT implementation that the decimal point is placed incorrectly in
// large numbers of the form 10^x - 1, for x > 28. This function adjusts the decimal point if that
// actually happens.
#[inline]
fn improper_offset_fix(chars: &mut Vec<char>, target_value: f64, precision: usize) {
    if precision > 0 {
        let current_value: f64 = chars
            .clone()
            .into_iter()
            .collect::<String>()
            .parse()
            .unwrap();
        if (current_value / target_value).abs() > 3.0 {
            let index = chars.clone().into_iter().position(|c| c == '.').unwrap();
            let t = chars[index - 1];
            chars[index - 1] = '.';
            chars[index] = t;
        }
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
fn format_fixed(value: &DoubleDouble, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = Vec::new();
    let mut sign = true;
    let precision = match f.precision() {
        Some(p) => p,
        None => DEFAULT_PRECISION,
    };

    if value.is_nan() {
        push_nan(&mut result);
    } else {
        sign = push_sign(&mut result, value, f);

        if value.is_infinite() {
            push_inf(&mut result);
        } else if *value == 0.0 {
            push_zero(&mut result, f);
        } else {
            let width = precision as i32 + value.abs().log10().floor().to_int() + 1;
            // Meant to be much wider than highest precision for better rounding
            let extra = width.max(60);

            // Special case: zero precision, |value| < 1.0
            // In this case a number greater than 0.5 prints 0 and should print 1
            if precision == 0 && value.abs() < 1.0 {
                result.push(if value.abs() >= 0.5 { '1' } else { '0' });
            }

            if width < 0 {
                push_zero(&mut result, f);
            } else {
                let (mut digits, exp) = to_digits(value, extra as usize);
                push_fixed_digits(&mut result, &mut digits, exp, precision);
            }
        }

        improper_offset_fix(&mut result, value.0, precision);
        drop_trailing_zeros(&mut result, f);
    }
    align_and_fill(&mut result, f, sign);

    write!(f, "{}", result.into_iter().collect::<String>())
}

// Formats `value` as a exponential number, with the format defined by `f`.
#[inline]
fn format_exp(value: &DoubleDouble, f: &mut fmt::Formatter, upper: bool) -> fmt::Result {
    let mut result = Vec::new();
    let mut sign = true;
    let mut exp = 0;
    let precision = match f.precision() {
        Some(p) => p,
        None => DEFAULT_PRECISION,
    };

    if value.is_nan() {
        push_nan(&mut result);
    } else {
        sign = push_sign(&mut result, value, f);

        if value.is_infinite() {
            push_inf(&mut result);
        } else if *value == 0.0 {
            push_zero(&mut result, f);
        } else {
            let width = precision + 1;
            let (digits, e) = to_digits(value, width as usize);
            exp = e;
            push_exp_digits(&mut result, &digits, precision);
        }

        drop_trailing_zeros(&mut result, f);
        let marker = if upper { 'E' } else { 'e' };
        push_exponent(&mut result, marker, exp);
    }
    align_and_fill(&mut result, f, sign);

    write!(f, "{}", result.into_iter().collect::<String>())
}

impl fmt::Display for DoubleDouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_fixed(self, f)
    }
}

impl fmt::LowerExp for DoubleDouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_exp(self, f, false)
    }
}

impl fmt::UpperExp for DoubleDouble {
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
    const E_50: &str  = "2.71828182845904523536028747135266249775724709369995";
    const PI_50_3: &str = "3.14159265358979323846264338327950288419716939937510e3";
    const E_50_NEG_2: &str  = "2.71828182845904523536028747135266249775724709369995e-2";
    const PI_TIMES_10_20: &str = "314159265358979323846";
    const E_TIMES_10_25: &str = "27182818284590452353602874";
    const PI_TIMES_10_20_EXP: &str = "3.14159265358979323846e20";
    const E_TIMES_10_25_EXP: &str = "2.7182818284590452353602874e25";

    fn parse(value: &str) -> DoubleDouble {
        value.parse().unwrap()
    }

    fn parse_error(value: &str) -> ParseQdFloatError {
        value.parse::<DoubleDouble>().unwrap_err()
    }

    fn close(a: DoubleDouble, b: DoubleDouble) -> bool {
        (a - b).abs() < 1e-28
    }

    // #region Parsing tests

    #[test]
    fn parse_nan() {
        assert!(parse("nan").is_nan());
    }

    #[test]
    fn parse_inf() {
        assert_eq!(parse("inf"), DoubleDouble::INFINITY);
    }

    #[test]
    fn parse_neg_inf() {
        assert_eq!(parse("-inf"), DoubleDouble::NEG_INFINITY);
    }

    #[test]
    fn parse_zero() {
        assert_eq!(parse("0"), DoubleDouble::from(0));
    }

    #[test]
    fn parse_integer() {
        assert_eq!(parse("1729"), DoubleDouble::from(1729));
        assert_eq!(parse("16_777_216"), DoubleDouble::from(16_777_216));
        assert_eq!(parse("+2317"), DoubleDouble::from(2317));
        assert_eq!(parse("-42"), DoubleDouble::from(-42));
    }

    #[test]
    fn parse_long_integer() {
        assert_eq!(
            parse(PI_TIMES_10_20),
            (DoubleDouble::PI * DoubleDouble::from(10).powi(20)).floor()
        );
        assert_eq!(
            parse(E_TIMES_10_25),
            (DoubleDouble::E * DoubleDouble::from(10).powi(25)).floor()
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
        assert_eq!(parse("17.29").0, DoubleDouble::from(17.29).0);
        assert_eq!(parse(".016_777_216").0, DoubleDouble::from(0.016_777_216).0);
        assert_eq!(parse("0.016_777_216").0, DoubleDouble::from(0.016_777_216).0);
        assert_eq!(parse("+2.317").0, DoubleDouble::from(2.317).0);
        assert_eq!(parse("-0.00042").0, DoubleDouble::from(-0.00042).0);
    }

    #[test]
    fn parse_long_float() {
        // Using closeness for comparisons here, because the input numbers are long enough to
        // warrant it
        assert!(close(parse(PI_50), DoubleDouble::PI));
        assert!(close(parse(E_50), DoubleDouble::E));
    }

    #[test]
    fn parse_exp_integer() {
        assert_eq!(parse("1729e0"), DoubleDouble::from(1729));
        assert_eq!(parse("16_777_216e+1"), DoubleDouble::from(167772160));
        assert_eq!(parse("+231700000E-5"), DoubleDouble::from(2317));
        assert_eq!(parse("-42E3"), DoubleDouble::from(-42000));
    }

    #[test]
    fn parse_long_exp_integer() {
        assert_eq!(
            parse(PI_TIMES_10_20_EXP),
            (DoubleDouble::PI * DoubleDouble::from(10).powi(20)).floor()
        );
        assert_eq!(
            parse(E_TIMES_10_25_EXP),
            (DoubleDouble::E * DoubleDouble::from(10).powi(25)).floor()
        );
    }

    #[test]
    fn parse_exp_float() {
        assert_eq!(parse("17.29e0").0, DoubleDouble::from(17.29).0);
        assert_eq!(parse("1.6777216e-2").0, DoubleDouble::from(0.016_777_216).0);
        assert_eq!(parse("0.16777216e-1").0, DoubleDouble::from(0.016_777_216).0);
        assert_eq!(parse("+2.317E3").0, DoubleDouble::from(2317).0);
        assert_eq!(parse("-4.2e-4").0, DoubleDouble::from(-0.00042).0);
    }

    #[test]
    fn parse_long_exp_float() {
        assert!(close(parse(PI_50_3), DoubleDouble::PI * 1000.0));
        assert!(close(parse(E_50_NEG_2), DoubleDouble::E / 100.0));
    }

    #[test]
    fn parse_error_empty() {
        assert_eq!(parse_error("").kind, QdFloatErrorKind::Empty);
    }

    #[test]
    fn parse_error_misplaced_sign() {
        assert_eq!(parse_error("2+317").kind, QdFloatErrorKind::Invalid);
    }

    #[test]
    fn parse_error_duplicate_sign() {
        assert_eq!(parse_error("+-2317").kind, QdFloatErrorKind::Invalid);
    }

    #[test]
    fn parse_error_duplicate_point() {
        assert_eq!(parse_error("2.31.7").kind, QdFloatErrorKind::Invalid);
    }

    #[test]
    fn parse_error_bad_exponent() {
        assert_eq!(parse_error("1.729e4e").kind, QdFloatErrorKind::Invalid);
    }

    #[test]
    fn parse_error_bad_character() {
        // Not yet!
        assert_eq!(parse_error("0xcafebabe").kind, QdFloatErrorKind::Invalid);
    }

    // #endregion
}

// #endregion
