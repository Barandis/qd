// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::display::*;
use crate::quad::Quad;
use std::fmt;

const DEFAULT_PRECISION: usize = 63;
const TEN: Quad = Quad(10.0, 0.0, 0.0, 0.0);

// Calculates the exponent of the supplied quad-double, adjusting the quad-double to fall
// somewhere in the range [1, 10) (i.e., to have a single non-zero digit before the decimal point).
#[inline]
fn calculate_exponent(r: &mut Quad) -> i32 {
    // Quick calculation of exponent based on the first component of `r`. This could turn out to be
    // off by 1 either direction depending on the second component.
    let mut exp = r.0.abs().log10().floor() as i32;

    // Adjust `r` based on that exponent approximation
    if exp < -300 {
        *r *= TEN.powi(300);
        *r /= TEN.powi(exp + 300);
    } else if exp > 300 {
        *r = r.ldexp(-53);
        *r /= TEN.powi(exp);
        *r = r.ldexp(53);
    } else {
        *r /= TEN.powi(exp);
    }

    // If `r` is outside the range [1, 10), then the exponent was off by 1. Adjust both it and `r`.
    if *r >= TEN {
        *r /= TEN;
        exp += 1;
    } else if *r < Quad::ONE {
        *r *= TEN;
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
fn extract_digits(r: &mut Quad, precision: usize) -> Vec<i32> {
    let mut digits = Vec::with_capacity(precision);
    for _ in 0..precision {
        let digit = r.0 as i32;
        *r -= Quad::from(digit);
        *r *= TEN;
        digits.push(digit);
    }
    digits
}

// Turns a quad-double into a vector of digits and an exponent. Sign is ignored, and no decimal
// appears in the vector; the exponent is calculated based on having the decimal point after the
// first digit.
//
// This function returns a vector of signed integers even though unsigned would make more logical
// sense. That's because internally (with the call to `extract_digits`) the vector has to deal with
// signed integers, and it's more efficient to let the caller cast them to unsigned as needed than
// it is to create a new vector of unsigned integers and copy them over.
fn to_digits(r: &Quad, precision: usize) -> (Vec<i32>, i32) {
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

// Potentially pushes a sign character to the supplied vector. Returns whether or not a character
// was actually added, information that is used later in formatting.
#[inline]
fn push_sign(chars: &mut Vec<char>, value: &Quad, formatter: &fmt::Formatter) -> bool {
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

// Formats `value` as a fixed-point number, with the format defined by `f`.
#[inline]
fn format_fixed(value: &Quad, f: &mut fmt::Formatter) -> fmt::Result {
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
            let width = precision as i32 + value.abs().log10().floor().as_int() as i32 + 1;
            // Higher than the max-length number + max precision so that users can do
            // their format!("{:.60}", Quad::from_str("999999999999999999999999999999...")) in
            // peace
            let extra = width.max(130);

            // Special case: zero precision, |value| < 1.0
            // In this case a number greater than 0.5 prints 0 and should print 1
            if precision == 0 && value.abs().as_float() < 1.0 {
                result.push(if value.abs().as_float() >= 0.5 {
                    '1'
                } else {
                    '0'
                });
            } else if width < 0 {
                push_zero(&mut result, f);
            } else {
                let (mut digits, exp) = to_digits(value, extra as usize);
                push_fixed_digits(
                    &mut result,
                    &mut digits,
                    exp,
                    f.precision(),
                    DEFAULT_PRECISION,
                );
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
fn format_exp(value: &Quad, f: &mut fmt::Formatter, upper: bool) -> fmt::Result {
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
            push_exp_digits(&mut result, &digits, f.precision(), DEFAULT_PRECISION);
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

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_fixed(self, f)
    }
}

impl fmt::LowerExp for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_exp(self, f, false)
    }
}

impl fmt::UpperExp for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_exp(self, f, true)
    }
}

impl fmt::Debug for Quad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alt = f.alternate();
        let mut r = String::from("Quad(");
        if alt {
            r.push_str("\n    ");
        }
        r.push_str(format!("{:e}, ", self.0).as_str());
        if alt {
            r.push_str("\n    ");
        }
        r.push_str(format!("{:e}, ", self.1).as_str());
        if alt {
            r.push_str("\n    ");
        }
        r.push_str(format!("{:e}, ", self.2).as_str());
        if alt {
            r.push_str("\n    ");
        }
        r.push_str(format!("{:e}", self.3).as_str());
        if alt {
            r.push_str("\n");
        }
        r.push_str(")");
        write!(f, "{}", r)
    }
}
