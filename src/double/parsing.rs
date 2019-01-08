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
            // result *= 10f64.powi(exp);
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Fixed,
    Upper,
    Lower,
}

impl DoubleDouble {
    // Converts a DoubleDouble to a vector of its digits and an exponent. Sign and decimal point are
    // not included, though the exponent assumes the decimal point to be after the first digit.
    fn to_digits(&self, precision: usize) -> (Vec<char>, i32) {
        let mut r = self.abs();

        // Handle the zero case
        if r == 0.0 {
            return (vec!['0'; precision], 0);
        }

        // Determine the approximate exponent
        let mut exp = self.0.abs().log10().floor() as i32;
        if exp < -300 {
            r *= DoubleDouble::from(10.0).powi(300);
            r /= DoubleDouble::from(10.0).powi(exp + 300);
        } else if exp > 300 {
            r = r.ldexp(-53);
            r /= DoubleDouble::from(10.0).powi(exp);
            r = r.ldexp(53);
        } else {
            r /= DoubleDouble::from(10.0).powi(exp);
        }

        // Adjust the exponent by 1 in either direction, if necessary
        if r >= 10.0 {
            r /= 10.0;
            exp += 1;
        } else if r < 1.0 {
            r *= 10.0;
            exp -= 1;
        }

        // Extract digits into a vector of values from 0-9
        // Initially there are likely to "digits" out of this range, but that will be corrected
        // below
        let digits = precision + 1; // One more than the precision, so we can round the last digit
        let mut s = Vec::with_capacity(digits);
        for _ in 0..digits {
            let digit = r.0 as i32;
            r -= digit as f64;
            r *= 10.0;
            s.push(digit);
        }

        // Correct out of range digits
        for i in (1..digits).rev() {
            if s[i] < 0 {
                s[i - 1] -= 1;
                s[i] += 10;
            } else if s[i] > 9 {
                s[i - 1] += 1;
                s[i] -= 10;
            }
        }

        // Round the last digit and handle carry through the vector
        if s[digits - 1] > 5 {
            s[digits - 2] += 1;
            let mut i = digits - 2;
            while i > 0 && s[i] > 9 {
                s[i] -= 10;
                s[i - 1] += 1;
                i -= 1;
            }
        }

        // If the first digit requires carry, insert one more digit to turn 9 into 10
        // and adjust the exponent
        if s[0] > 9 {
            exp += 1;
            s[0] = 0;
            s.insert(0, 1);
        }

        // Transfer into a vector of chars. Using the vec slice drops the last item, which had only
        // been used for rounding.
        let result: Vec<char> = s[0..precision]
            .into_iter()
            .map(|d| char::from_digit(*d as u32, 10).unwrap())
            .collect();

        (result, exp)
    }

    fn format(&self, f: &mut fmt::Formatter, mode: Mode) -> String {
        let mut result = Vec::new();
        let mut sign = true;
        let mut exp = 0;
        let precision = match f.precision() {
            Some(p) => p,
            None => 32,
        };

        if self.is_nan() {
            result.append(&mut "NaN".chars().collect());
        } else {
            if self.is_sign_negative() {
                result.push('-');
            } else if f.sign_plus() {
                result.push('+');
            } else {
                sign = false;
            }

            if self.is_infinite() {
                result.append(&mut "inf".chars().collect());
            } else if *self == 0.0 {
                result.push('0');
                if precision > 0 {
                    result.push('.');
                    for _ in 0..precision {
                        result.push('0');
                    }
                }
            } else {
                let width = precision as i32
                    + match mode {
                        Mode::Fixed => 1 + self.abs().log10().floor().to_int(),
                        _ => 1,
                    };
                let extra = match mode {
                    Mode::Fixed => width.max(60),
                    _ => width,
                };

                // Special case: fixed mode, zero precision, |self| < 1.0
                // In this case a number greater than 0.5 prints 0 and should print 1
                if mode == Mode::Fixed && precision == 0 && self.abs() < 1.0 {
                    result.push(if self.abs() >= 0.5 { '1' } else { '0' });
                }

                if mode == Mode::Fixed && width < 0 {
                    result.push('0');
                    if precision > 0 {
                        result.push('.');
                        for _ in 0..precision {
                            result.push('0');
                        }
                    }
                } else {
                    let (digits, e) = match mode {
                        // These casts are safe because we handled width < 0 above
                        Mode::Fixed => self.to_digits(extra as usize),
                        _ => self.to_digits(width as usize),
                    };
                    exp = e;

                    let offset = e + 1;
                    match mode {
                        Mode::Fixed => {
                            if offset > 0 {
                                let offset = offset as usize;
                                for digit in &digits[..offset] {
                                    result.push(*digit);
                                }
                                if precision > 0 {
                                    result.push('.');
                                    for digit in &digits[offset..precision] {
                                        result.push(*digit);
                                    }
                                }
                            } else {
                                result.push('0');
                                result.push('.');
                                if offset < 0 {
                                    for _ in 0..-offset {
                                        result.push('0');
                                    }
                                }
                                for digit in &digits[..precision] {
                                    result.push(*digit);
                                }
                            }
                        }
                        _ => {
                            result.push(digits[0]);
                            if precision > 0 {
                                result.push('.');
                            }
                            for digit in &digits[1..precision] {
                                result.push(*digit);
                            }
                        }
                    }
                }
            }

            // Fix a fixed number because of an improper offset with large values.
            // This affects values of 10^x - 1 for x > 28, causing them to put the point in the
            // wrong place.
            if mode == Mode::Fixed && precision > 0 {
                let from_string: f64 = result
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .parse()
                    .unwrap();
                if (from_string / self.0).abs() > 3.0 {
                    let index = result.clone().into_iter().position(|c| c == '.').unwrap();
                    let t = result[index - 1];
                    result[index - 1] = '.';
                    result[index] = t;
                }
            }

            // If no precision was specified and there is a decimal point, drop all trailing zeroes
            // after the decimal point.
            if let None = f.precision() {
                if result.contains(&'.') {
                    if let Some(index) = result.clone().into_iter().rposition(|c| c != '0') {
                        // Drop the decimal point itself if everything after it is a zero
                        let new_length = match result[index] {
                            '.' => index,
                            _ => index + 1,
                        };
                        result.truncate(new_length);
                    }
                }
            }

            if mode != Mode::Fixed && !self.is_infinite() {
                result.push(if mode == Mode::Upper { 'E' } else { 'e' });
                result.append(&mut exp.to_string().chars().collect());
            }
        }

        let len = result.len();
        if let Some(width) = f.width() {
            if len < width {
                let delta = width - len;
                let fill = f.fill();
                match f.align() {
                    Some(fmt::Alignment::Left) => {
                        for _ in 0..delta {
                            result.push(fill);
                        }
                    }
                    Some(fmt::Alignment::Right) => {
                        for _ in 0..delta {
                            result.insert(0, fill);
                        }
                    }
                    Some(fmt::Alignment::Center) => {
                        let left_delta = delta / 2;
                        let right_delta = delta - left_delta;
                        for _ in 0..left_delta {
                            result.insert(0, fill);
                        }
                        for _ in 0..right_delta {
                            result.push(fill);
                        }
                    }
                    None => {
                        if f.sign_aware_zero_pad() {
                            let index = if sign { 1 } else { 0 };
                            for _ in index..delta {
                                result.insert(index, '0');
                            }
                        } else {
                            for _ in 0..delta {
                                result.insert(0, fill);
                            }
                        }
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}

impl fmt::Display for DoubleDouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.format(f, Mode::Fixed);
        write!(f, "{}", text)
    }
}

impl fmt::LowerExp for DoubleDouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.format(f, Mode::Lower);
        write!(f, "{}", text)
    }
}

impl fmt::UpperExp for DoubleDouble {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = self.format(f, Mode::Upper);
        write!(f, "{}", text)
    }
}

// #endregion
