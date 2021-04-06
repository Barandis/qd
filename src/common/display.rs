// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::fmt::Formatter;
use std::{char, fmt::Alignment};

// Add a "not-a-number" representation to the input vector.
pub fn push_nan(chars: &mut Vec<char>) {
    chars.append(&mut "NaN".chars().collect());
}

// Add an "infinity" representation to the input vector.
pub fn push_inf(chars: &mut Vec<char>) {
    chars.append(&mut "inf".chars().collect());
}

// Add zero to the input vector. The number of characters in the zero is determined by the
// supplied formatter.
pub fn push_zero(chars: &mut Vec<char>, f: &Formatter) {
    chars.push('0');
    if let Some(p) = f.precision() {
        if p > 0 {
            chars.push('.');
            for _ in 0..p {
                chars.push('0');
            }
        }
    }
}

// Add an exponent to the input vector.
pub fn push_exp(chars: &mut Vec<char>, marker: char, exp: i32) {
    chars.push(marker);
    chars.append(&mut exp.to_string().chars().collect());
}

// Rounds the digits in a vector to a certain index and then truncates the vector at that
// index.
pub fn round_and_trunc(digits: &mut Vec<u8>, len: usize) {
    if digits[len] >= 5 {
        // Round up if the digit after the last desired digit is 5 or higher
        let mut i: usize = 1;
        digits[len - 1] += 1;
        // Round up until there are no more 9's, if it's 9's all the way, leave
        // the first element as a 10, which we'll handle later
        while digits[len - i] == 10 && len != i {
            digits[len - i] = 0;
            i += 1;
            digits[len - i] += 1;
        }
    }
    digits.truncate(len);
}

// Adds or removes zeros to the vector depending on the exponent. If the exponent is
// positive, this will only add zeros to the end if zeros need to be added in order for the
// number to reach the decimal point position. If the exponent is negative, zeros will be
// added to the beginning of the vector.
//
// The exponent parameter is used *only* for determining the number of zeros that need to be
// added or removed. If this is being used for a number that is to be in exponential form,
// use 0 for the exponent.
pub fn adjust_zeros(digits: &mut Vec<u8>, exp: i32) {
    let absexp = exp.abs() as usize;
    let accuracy = digits.len() as isize;
    // First we want to drop trailing zeros that make the width of the vector higher than
    // the exponent + 1 (i.e., trailing zeros that would be to the right of the decimal
    // point, after we put in a decimal point). These will be re-added later if the
    // specified precision requires it.
    if digits.len() as i32 > exp + 1 {
        let mut new_len = digits.len();
        while digits[new_len - 1] == 0 && new_len > 1 && new_len as i32 > exp + 1 {
            new_len -= 1;
        }
        digits.truncate(new_len);
    }

    // Add zeros to the left (if exp is negative) or the right (if non-negative) to make
    // the whole number reach the decimal point if it doesn't already.
    if exp < 0 {
        digits.splice(..0, vec![0; absexp]);
    } else {
        let zero_length = 0.max(absexp as isize - accuracy + 1) as usize;
        if zero_length > 0 {
            digits.append(&mut vec![0; zero_length]);
        }
    }
}

// Adds zeros or removes digits from the vector to correspond to the supplied precision. If
// the vector is not long enough to reach that precision, zeros will be added to the end; if
// it's too long, the vector will be truncated. In this case, the last removed digit will be
// used to round the rest of the digits.
//
// In the case of a number consisting of all 9's that must be truncated, the first element
// will become a 10 while the rest will be 0's. This will happen only with numbers with
// positive exponents, as the ones with negative exponents already had the pre-decimal-point
// zero added which will halt rounding cascades on those numbers. This makes it easy to tell
// later when an exponent needs to be adjusted to place the decimal point correctly. This is
// also the only time when any vector element will be something other than a single-digit
// integer.
pub fn adjust_prec(digits: &mut Vec<u8>, exp: i32, prec: Option<usize>) {
    if let Some(p) = prec {
        // If exp < 0, we add 1 for the zero before the decimal point
        let desired = if exp < 0 { p + 1 } else { exp as usize + p + 1 };

        if desired > digits.len() {
            digits.append(&mut vec![0; desired - digits.len()]);
        } else if desired < digits.len() {
            // The only other option is desired == digits.len(), as zeros would have
            // already been appended if desired > digits.len()
            round_and_trunc(digits, desired);
        }
    }
}

// Positions a decimal point at the correct location dependiong on the exponent. Since the
// decimal point is not a `u8` like the digits are, this function returns a character vector
// rather than manipulating the input vector in place.
//
// The function also handles the `10` "digit" that can be in the first position for a
// positive-exponent number that cascade rounded all the way to the first digit.
//
// A decimal point will *not* be placed if the length of the vector indicates that the last
// digit is right at the decimal point location. This means we'll end up with "10" rather
// than "10.", for instance.
pub fn place_decimal(digits: Vec<u8>, exp: i32) -> Vec<char> {
    let offset = if digits[0] == 10 { 1 } else { 0 };
    let mut result = Vec::with_capacity(digits.len() + offset + 1);

    for d in digits {
        if d == 10 {
            // The only "digit" that might be 10 is the first one, if cascading rounding
            // reached all the way to the beginning of the number. When this happens we have
            // to push both characters.
            result.push('1');
            result.push('0');
        } else {
            result.push(char::from_digit(d as u32, 10).unwrap());
        }
    }

    let position = 0.max(exp) as usize + offset + 1;
    // Don't add a decimal point if the number doesn't have any digits after the decimal
    // point
    if position < result.len() {
        result.insert(position, '.');
    }

    result
}

// Adjust the width of the number based on alignment, width, and fill settings. This
// function also handles the sign-aware zero fill.
//
// This function does nothing if either no width is specified or if the specified width is
// not greater than the current vector length. A width setting can increase the number of
// characters in the vector, but it cannot decrease it. As a consequence, align and fill are
// ignored if there isn't a width specified that is higher than the vector length.
pub fn align_and_fill(chars: &mut Vec<char>, signed: bool, f: &mut Formatter) {
    if let Some(width) = f.width() {
        let len = chars.len();
        if len < width {
            let delta = width - len;
            let fill = f.fill();

            match f.align() {
                Some(Alignment::Left) => {
                    for _ in 0..delta {
                        chars.push(fill);
                    }
                }
                Some(Alignment::Right) => {
                    for _ in 0..delta {
                        chars.insert(0, fill);
                    }
                }
                Some(Alignment::Center) => {
                    let left = delta / 2;
                    let right = delta - left;
                    for _ in 0..left {
                        chars.insert(0, fill);
                    }
                    for _ in 0..right {
                        chars.push(fill);
                    }
                }
                None => {
                    if f.sign_aware_zero_pad() {
                        let index = if signed { 1 } else { 0 };
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
