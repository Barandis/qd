// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::char;
use std::fmt;

// Adjusts the range of integers in the supplied vector from [9, -9] to [0, 9].
// (This function will handle 'digits' up to 19, but I don't believe in this
// application that they're ever over 9.)
#[inline]
pub fn correct_range(digits: &mut Vec<i32>) {
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

// Rounds the second-to-last digit of an i32 vector based on the value of the
// last digit. This rounding is standard round-to-even in the case of a final
// digit of 5. Any necessary carrying is propagated as far as it needs to,
// adjusting the exponent if the carry goes all the way to the first digit.
#[inline]
pub fn round_vec(digits: &mut Vec<i32>, exp: &mut i32) {
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

    // If the first digit requires carry, insert one more digit to turn 9 into
    // 10 and adjust the exponent
    if digits[0] > 9 {
        *exp += 1;
        digits[0] = 0;
        digits.insert(0, 1);
    }
}

// Converts an integer into a character representation of that integer. This
// assumes that `digit` is between 0 and 9 inclusive. If it's not, there's a bug
// somewhere, so we WANT to panic; hence the unchecked `unwrap`.
#[inline]
pub fn char_from_digit(digit: i32) -> char {
    char::from_digit(digit as u32, 10).unwrap()
}

// Appends "NaN" to the supplied vector.
#[inline]
pub fn push_nan(chars: &mut Vec<char>) {
    chars.append(&mut "NaN".chars().collect());
}

// Appends "inf" to the supplied vector.
#[inline]
pub fn push_inf(chars: &mut Vec<char>) {
    chars.append(&mut "inf".chars().collect());
}

// Pushes the number zero to the supplied vector. If the formatter has a
// precision set, then it will add that many zeros behind the decimal; if none
// is set, it'll just push "0.0".
#[inline]
pub fn push_zero(chars: &mut Vec<char>, formatter: &fmt::Formatter) {
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

// Rounds a vector of digits based on the precision. This is -almost- identical
// to `round_vec` above except that it rounds to precision (not to one past
// precision). It is necessary because fixed- point numbers are calculated (and
// rounded) at greater than their needed precision for accuracy. Hence they need
// to be rounded to the correct number of digits after they return.
//
// `offset` + `precision` is presumed to be positive. If not,
// `push_fixed_digits` won't call this function.
//
// TODO: This can be made more efficient. Exponentials are rounded in
// `round_vec`; fixed are rounded there unnecessarily and then here as well.
#[inline]
pub fn round_fixed_digits(
    digits: &mut Vec<i32>,
    offset: &mut i32,
    precision: Option<usize>,
    def_precision: usize,
) {
    let d = match precision {
        Some(p) => *offset + p as i32,
        None => *offset.min(&mut 0) + def_precision as i32, // no more than 31 digits
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

    // If the first digit requires carry, insert one more digit to turn 9 into
    // 10 and adjust the offset
    if digits[0] > 9 {
        digits[0] = 0;
        digits.insert(0, 1);
        *offset += 1;
    }
}

// Converts all of the digits, up to the number indicated by `precision`, into
// characters and pushes them onto the supplied character vector. `offset`
// determines where the decimal point is placed. This is used to create a
// fixed-point output format.
#[inline]
pub fn push_fixed_digits(
    chars: &mut Vec<char>,
    digits: &mut Vec<i32>,
    exp: i32,
    precision: Option<usize>,
    def_precision: usize,
) {
    let mut offset = exp + 1;
    let pr_value = precision.unwrap_or(def_precision);

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
        round_fixed_digits(digits, &mut offset, precision, def_precision);

        if offset > 0 {
            let offset = offset as usize;
            for digit in &digits[..offset] {
                chars.push(char_from_digit(*digit));
            }
            match precision {
                Some(p) => {
                    if p > 0 {
                        chars.push('.');
                        for digit in &digits[offset..offset + p] {
                            chars.push(char_from_digit(*digit));
                        }
                    }
                }
                None => {
                    if offset < def_precision {
                        chars.push('.');
                        // limit to 31 characters, whatever that precision is
                        for digit in &digits[offset..def_precision] {
                            chars.push(char_from_digit(*digit));
                        }
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
                chars.push(char_from_digit(*digit));
            }
        }
    }
}

// Converts all of the digits, up to the number indicated by `precision`, into
// characters and pushes them onto the supplied character vector. If there is a
// decimal point (i.e, if `precision` is not 0), it will always be after the
// first digit. This is used to create an exponential output format.
#[inline]
pub fn push_exp_digits(
    chars: &mut Vec<char>,
    digits: &[i32],
    precision: Option<usize>,
    def_precision: usize,
) {
    let precision = precision.unwrap_or(def_precision);
    chars.push(char_from_digit(digits[0]));
    if precision > 0 {
        chars.push('.');
    }
    for digit in &digits[1..=precision] {
        chars.push(char_from_digit(*digit));
    }
}

// Drops trailing zeros after the decimal point (and the decimal point as well,
// if necessary). This happens only if no precision was supplied to the
// formatter. In that case the number is given as many decimal places as it
// needs minus the trailing zeros.
#[inline]
pub fn drop_trailing_zeros(chars: &mut Vec<char>, formatter: &fmt::Formatter) {
    if formatter.precision().is_none() && chars.contains(&'.') {
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

// Pushes the exponent to the supplied character vector. It includes a leading
// marker character, which should be either 'e' or 'E'.
#[inline]
pub fn push_exponent(chars: &mut Vec<char>, marker: char, exp: i32) {
    chars.push(marker);
    chars.append(&mut exp.to_string().chars().collect());
}

// Adjusts the character vector for width, precision, alignment, and fill
// characters. The vector is expanded as needed to accomodate the width.
#[inline]
pub fn align_and_fill(
    chars: &mut Vec<char>,
    formatter: &mut fmt::Formatter,
    sign: bool,
) {
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
