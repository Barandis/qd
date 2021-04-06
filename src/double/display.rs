// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::fmt::{Debug, Display, Formatter, LowerExp, Result, UpperExp};
use std::{char, fmt::Alignment};

const TEN: Double = Double(10.0, 0.0);
const MAX_REAL_PRECISION: usize = 31;

impl Display for Double {
    /// Formats a `Double` for display.
    ///
    /// All formatting options that are shown in [`std::fmt`] are supported *except* for
    /// ones that are typically meant only for integers (hexadecimal, binary, octal, and
    /// pointer formats). Because of this, the "alternate" (`#`) flag is only recognized
    /// along with `?`, pretty-printing the `Debug` output.
    ///
    /// By default, `Double`s are printed with 31 digits but drop trailing zeros.
    ///
    /// This function also provides the formatting for [`to_string`], which renders the
    /// `Double` as if formatted with an empty format specifier (`"{}"`).
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(format!("{}", dd!(1.5)) == "1.5");
    ///
    /// // The next digit in π is 0, which is why it's one digit shorter than e
    /// assert!(format!("{}", Double::PI) == "3.14159265358979323846264338328");
    /// assert!(format!("{}", Double::E) == "2.718281828459045235360287471353");
    ///
    /// // to_string renders as if formatted with "{}"
    /// assert!(Double::PI.to_string() == "3.14159265358979323846264338328");
    ///
    /// // debug
    /// assert!(format!("{:?}", Double::PI) ==
    ///     "Double(3.141592653589793e0, 1.2246467991473532e-16)");
    /// assert!(format!("{:#?}", Double::PI) ==
    /// "Double(
    ///     3.141592653589793e0,
    ///     1.2246467991473532e-16
    /// )");
    ///
    /// // precision and exponents
    /// let value = dd!(0.016_777_216);
    /// assert!(format!("{:.0}", value) == "0");
    /// assert!(format!("{:.5}", value) == "0.01678");
    /// assert!(format!("{:.12}", value) == "0.016777216000");
    /// assert!(format!("{:.3e}", value) == "1.678e-2");
    /// assert!(format!("{:.*e}", 3, value) == "1.678e-2");
    /// assert!(format!("{0:.1$E}", value, 4) == "1.6777E-2");
    /// assert!(format!("{:.prec$E}", value, prec = 10) == "1.6777216000E-2");
    ///
    /// // width, alignment, and fill
    /// let value = dd!(123_456);
    /// assert!(format!("{:10}", value) == "    123456"); // right-align is the default
    /// assert!(format!("{:>10}", value) == "    123456");
    /// assert!(format!("{:<10}", value) == "123456    ");
    /// assert!(format!("{:^10}", value) == "  123456  ");
    /// assert!(format!("{:0>10}", value) == "0000123456");
    /// assert!(format!("{:*<10}", value) == "123456****");
    /// assert!(format!("{:'^10}", value) == "''123456''");
    ///
    /// // plus sign and sign-aware zero fill
    /// let value = dd!(123_456);
    /// assert!(format!("{:+}", value) == "+123456");
    /// assert!(format!("{:0>10}", -value) == "000-123456");
    /// assert!(format!("{:010}", -value) == "-000123456");
    /// assert!(format!("{:+012e}", value) == "+001.23456e5");
    /// ```
    ///
    /// [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
    /// [`to_string`]: #tymethod.to_string
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            push_nan(&mut result);
        } else if self.is_infinite() {
            push_inf(&mut result);
        } else if self.is_zero() {
            push_zero(&mut result, f);
        } else {
            push_digits_fixed(&mut result, self, f);
        }
        align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl LowerExp for Double {
    /// Formats a `Double` for display when the "`e`" formatting option is specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            push_nan(&mut result);
        } else if self.is_infinite() {
            push_inf(&mut result);
        } else if self.is_zero() {
            push_zero(&mut result, f);
        } else {
            push_digits_exp(&mut result, self, f);
        }

        if self.is_finite() {
            let exp = if self.is_zero() {
                0
            } else {
                self.0.abs().log10().floor() as i32
            };
            push_exp(&mut result, 'e', exp)
        }

        align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl UpperExp for Double {
    /// Formats a `Double` for display when the "`E`" formatting option is specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = vec![];
        let signed = push_sign(&mut result, self, f);

        if self.is_nan() {
            push_nan(&mut result);
        } else if self.is_infinite() {
            push_inf(&mut result);
        } else if self.is_zero() {
            push_zero(&mut result, f);
        } else {
            push_digits_exp(&mut result, self, f);
        }

        if self.is_finite() {
            let exp = if self.is_zero() {
                0
            } else {
                self.0.abs().log10().floor() as i32
            };
            push_exp(&mut result, 'E', exp)
        }

        align_and_fill(&mut result, signed, f);

        write!(f, "{}", result.into_iter().collect::<String>())
    }
}

impl Debug for Double {
    /// Formats a `Double` for display when the "`?`" formatting option is specified.
    ///
    /// See [`Display::fmt`](#method.fmt-1) for more information.
    fn fmt(&self, f: &mut Formatter) -> Result {
        let alt = f.alternate();
        let mut str = String::from("Double(");
        if alt {
            str.push_str("\n    ");
        }
        str.push_str(format!("{:e},", self.0).as_str());
        if alt {
            str.push_str("\n    ");
        } else {
            str.push(' ');
        }
        str.push_str(format!("{:e}", self.1).as_str());
        if alt {
            str.push('\n');
        }
        str.push(')');
        write!(f, "{}", str)
    }
}

fn push_nan(chars: &mut Vec<char>) {
    chars.append(&mut "NaN".chars().collect());
}

fn push_inf(chars: &mut Vec<char>) {
    chars.append(&mut "inf".chars().collect());
}

fn push_sign(chars: &mut Vec<char>, value: &Double, f: &Formatter) -> bool {
    if value.is_sign_negative() {
        chars.push('-');
        true
    } else if f.sign_plus() {
        chars.push('+');
        true
    } else {
        false
    }
}

fn push_zero(chars: &mut Vec<char>, f: &Formatter) {
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

fn push_exp(chars: &mut Vec<char>, marker: char, exp: i32) {
    chars.push(marker);
    chars.append(&mut exp.to_string().chars().collect());
}

fn push_digits_fixed(chars: &mut Vec<char>, value: &Double, f: &mut Formatter) {
    let value = value.abs();
    let exp = value.0.log10().floor() as i32;
    let prec = f.precision();

    let mut digits = extract_digits(&value, exp);
    adjust_zeros(&mut digits, exp);
    adjust_prec(&mut digits, exp, prec);

    chars.append(&mut place_decimal(digits, exp));
}

fn push_digits_exp(chars: &mut Vec<char>, value: &Double, f: &mut Formatter) {
    let value = value.abs();
    let exp = value.0.log10().floor() as i32;
    let prec = f.precision();

    let mut digits = extract_digits(&value, exp);
    adjust_zeros(&mut digits, 0);
    adjust_prec(&mut digits, 0, prec);

    chars.append(&mut place_decimal(digits, 0));
}

// Rounds the digits in a vector to a certain index and then truncates the vector at that
// index.
fn round_and_trunc(digits: &mut Vec<u8>, len: usize) {
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

// Extracts the decimal digits of `value` into an array of unsigned integers.
//
// This function assumes that `value` is positive. Zero and non-finite values are handled
// before we get to this function, and the sign is already pushed to the output vector. With
// that assumption, this function will return a vector of numbers from 0-9 - digits.
fn extract_digits(value: &Double, exp: i32) -> Vec<u8> {
    // Normalize the number to have an exponent of 0 (i.e., one digit before the decimal
    // point). We don't actually otherwise need the exponent in this function, as all we're
    // doing is parsing digits from the mantissa. This normalization makes the math involved
    // much faster. It also ensures that really large numbers don't overflow on
    // multiplication by ten.
    let divisor = TEN.powi(exp);

    let mut value = value / divisor;
    let mut digits = vec![];

    for _ in 0..(MAX_REAL_PRECISION + 1) {
        let digit = value.trunc();

        value -= digit;
        value *= TEN;

        digits.push(digit.0 as u8);
    }

    // We will not record digits after the 323rd (308 for the largest negative exponent,
    // plus 15 digits for the `f64` accurate width) decimal place; since the `f64`s that
    // make up `Double`s can't do that, it's just garbage after that point.
    //
    // The 324 is the 323 digits plus the one needed because the first digit of the number
    // is at position exp + 1 in a negative exponent number.
    //
    // If this isn't an issue, we still truncate by one because we produced an extra digit
    // for rounding.
    round_and_trunc(
        &mut digits,
        (324 + exp).min(MAX_REAL_PRECISION as i32) as usize,
    );

    digits
}

// Adds or removes zeros to the vector depending on the exponent. If the exponent is
// positive, this will only add zeros to the end if zeros need to be added in order for the
// number to reach the decimal point position. If the exponent is negative, zeros will be
// added to the beginning of the vector.
//
// The exponent parameter is used *only* for determining the number of zeros that need to be
// added or removed. If this is being used for a number that is to be in exponential form,
// use 0 for the exponent.
fn adjust_zeros(digits: &mut Vec<u8>, exp: i32) {
    let absexp = exp.abs() as usize;

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
        let zero_length = 0.max(absexp as isize - MAX_REAL_PRECISION as isize + 1) as usize;
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
fn adjust_prec(digits: &mut Vec<u8>, exp: i32, prec: Option<usize>) {
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
fn place_decimal(digits: Vec<u8>, exp: i32) -> Vec<char> {
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
fn align_and_fill(chars: &mut Vec<char>, signed: bool, f: &mut Formatter) {
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

#[cfg(test)]
mod tests {
    use super::*;

    // debug tests
    test_all_eq!(
        debug_zero:
            "Double(0e0, 0e0)",
            format!("{:?}", Double::ZERO);
        debug_pi:
            "Double(3.141592653589793e0, 1.2246467991473532e-16)",
            format!("{:?}", Double::PI);
        debug_alt_zero:
            "Double(\n    0e0,\n    0e0\n)",
            format!("{:#?}", Double::ZERO);
        debug_alt_pi:
            "Double(\n    3.141592653589793e0,\n    1.2246467991473532e-16\n)",
            format!("{:#?}", Double::PI);
    );

    // special number tests
    test_all_eq!(
        nan:
            "NaN",
            format!("{}", Double::NAN);
        neg_nan:
            "-NaN",
            format!("{}", -Double::NAN);
        plus_nan:
            "+NaN",
            format!("{:+}", Double::NAN);
        plus_neg_nan:
            "-NaN",
            format!("{:+}", -Double::NAN);
        inf:
            "inf",
            format!("{}", Double::INFINITY);
        neg_inf:
            "-inf",
            format!("{}", Double::NEG_INFINITY);
        plus_inf:
            "+inf",
            format!("{:+}", Double::INFINITY);
        plus_neg_inf:
            "-inf",
            format!("{:+}", Double::NEG_INFINITY);
        zero:
            "0",
            format!("{}", Double::ZERO);
        neg_zero:
            "-0",
            format!("{}", Double::NEG_ZERO);
        plus_zero:
            "+0",
            format!("{:+}", Double::ZERO);
        plus_neg_zero:
            "-0",
            format!("{:+}", Double::NEG_ZERO);
    );

    // Basic fixed number tests
    test_all_eq!(
        pi:
            "3.14159265358979323846264338328",
            format!("{}", Double::PI);
        neg_pi:
            "-3.14159265358979323846264338328",
            format!("{}", -Double::PI);
        plus_pi:
            "+3.14159265358979323846264338328",
            format!("{:+}", Double::PI);
        plus_neg_pi:
            "-3.14159265358979323846264338328",
            format!("{:+}", -Double::PI);

        ln2:
            "0.6931471805599453094172321214582",
            format!("{}", Double::LN_2);
        neg_ln2:
            "-0.6931471805599453094172321214582",
            format!("{}", -Double::LN_2);
        plus_ln2:
            "+0.6931471805599453094172321214582",
            format!("{:+}", Double::LN_2);
        plus_neg_ln2:
            "-0.6931471805599453094172321214582",
            format!("{:+}", -Double::LN_2);

        ln2_100:
            "0.006931471805599453094172321214582",
            format!("{}", Double::LN_2 / dd!(100));
        neg_ln2_100:
            "-0.006931471805599453094172321214582",
            format!("{}", -Double::LN_2 / dd!(100));
        plus_ln2_100:
            "+0.006931471805599453094172321214582",
            format!("{:+}", Double::LN_2 / dd!(100));
        plus_neg_ln2_100:
            "-0.006931471805599453094172321214582",
            format!("{:+}", -Double::LN_2 / dd!(100));

        large:
            format!("1234567890123456789{}", "0".repeat(290)),
            format!("{}", dd!("1.234567890123456789e308"));
        neg_large:
            format!("-1234567890123456789{}", "0".repeat(290)),
            format!("{}", dd!("-1.234567890123456789e308"));
        plus_large:
            format!("+1234567890123456789{}", "0".repeat(290)),
            format!("{:+}", dd!("1.234567890123456789e308"));

        small:
            format!("0.{}1234567890123457", "0".repeat(307)),
            format!("{}", dd!("1.234567890123456789e-308"));
        neg_small:
            format!("-0.{}1234567890123457", "0".repeat(307)),
            format!("{}", dd!("-1.234567890123456789e-308"));
        plus_small:
            format!("+0.{}1234567890123457", "0".repeat(307)),
            format!("{:+}", dd!("1.234567890123456789e-308"));
    );

    // precision tests
    test_all_eq!(
        prec_zero:
            "0.0000",
            format!("{:.4}", Double::ZERO);
        prec_neg_zero:
            "-0.0",
            format!("{:.1}", Double::NEG_ZERO);
        prec_plus_zero:
            "+0.000000000000",
            format!("{:+.12}", Double::ZERO);
        prec_plus_neg_zero:
            "-0.000000",
            format!("{:+.6}", Double::NEG_ZERO);

        pi_prec_5:
            "3.14159",
            format!("{:.5}", Double::PI);
        pi_prec_7:
            "3.1415927",
            format!("{:.7}", Double::PI);
        pi_prec_20:
            "3.14159265358979323846",
            format!("{:.20}", Double::PI);
        pi_prec_40:
            "3.1415926535897932384626433832800000000000",
            format!("{:.40}", Double::PI);
        pi_prec_0:
            "3",
            format!("{:.0}", Double::PI);
        neg_pi_prec_40:
            "-3.1415926535897932384626433832800000000000",
            format!("{:.40}", -Double::PI);
        neg_pi_prec_0:
            "-3",
            format!("{:.0}", -Double::PI);
        plus_pi_prec_40:
            "+3.1415926535897932384626433832800000000000",
            format!("{:+.40}", Double::PI);
        plus_pi_prec_0:
            "+3",
            format!("{:+.0}", Double::PI);


        ln2_prec_5:
            "0.69315",
            format!("{:.5}", Double::LN_2);
        ln2_prec_7:
            "0.6931472",
            format!("{:.7}", Double::LN_2);
        ln2_prec_20:
            "0.69314718055994530942",
            format!("{:.20}", Double::LN_2);
        ln2_prec_40:
            "0.6931471805599453094172321214582000000000",
            format!("{:.40}", Double::LN_2);
        ln2_prec_0:
            "1",
            format!("{:.0}", Double::LN_2);
        neg_ln2_prec_40:
            "-0.6931471805599453094172321214582000000000",
            format!("{:.40}", -Double::LN_2);
        neg_ln2_prec_0:
            "-1",
            format!("{:.0}", -Double::LN_2);
        plus_ln2_prec_40:
            "+0.6931471805599453094172321214582000000000",
            format!("{:+.40}", Double::LN_2);
        plus_ln2_prec_0:
            "+1",
            format!("{:+.0}", Double::LN_2);

        ln2_100_prec_5:
            "0.00693",
            format!("{:.5}", Double::LN_2 / dd!(100));
        ln2_100_prec_7:
            "0.0069315",
            format!("{:.7}", Double::LN_2 / dd!(100));
        ln2_100_prec_20:
            "0.00693147180559945309",
            format!("{:.20}", Double::LN_2 / dd!(100));
        ln2_100_prec_40:
            "0.0069314718055994530941723212145820000000",
            format!("{:.40}", Double::LN_2 / dd!(100));
        ln2_100_prec_0:
            "0",
            format!("{:.0}", Double::LN_2 / dd!(100));
        neg_ln2_100_prec_40:
            "-0.0069314718055994530941723212145820000000",
            format!("{:.40}", -Double::LN_2 / dd!(100));
        neg_ln2_100_prec_0:
            "-0",
            format!("{:.0}", -Double::LN_2 / dd!(100));
        plus_ln2_100_prec_40:
            "+0.0069314718055994530941723212145820000000",
            format!("{:+.40}", Double::LN_2 / dd!(100));
        plus_ln2_100_prec_0:
            "+0",
            format!("{:+.0}", Double::LN_2 / dd!(100));


        large_prec_0:
            format!("1234567890123456789{}", "0".repeat(290)),
            format!("{:.0}", dd!("1.234567890123456789e308"));
        large_prec_8:
            format!("1234567890123456789{}.00000000", "0".repeat(290)),
            format!("{:.8}", dd!("1.234567890123456789e308"));


        small_prec_0:
            "0",
            format!("{:.0}", dd!("1.234567890123456789e-308"));
        small_prec_8:
            "0.00000000",
            format!("{:.8}", dd!("1.234567890123456789e-308"));
        small_prec_316:
            format!("0.{}123456789", "0".repeat(307)),
            format!("{:.316}", dd!("1.234567890123456789e-308"));
    );

    // width tests (default right align)
    test_all_eq!(
        zero_width:
            "         0",
            format!("{:10}", Double::ZERO);
        neg_zero_width:
            "        -0",
            format!("{:10}", Double::NEG_ZERO);
        inf_width:
            "       inf",
            format!("{:10}", Double::INFINITY);
        neg_inf_width:
            "      -inf",
            format!("{:10}", Double::NEG_INFINITY);
        nan_width:
            "       NaN",
            format!("{:10}", Double::NAN);
        one_width:
            "         1",
            format!("{:10}", Double::ONE);
        neg_one_width:
            "        -1",
            format!("{:10}", Double::NEG_ONE);
        plus_one_width:
            "        +1",
            format!("{:+10}", Double::ONE);
        pi_width:
            "3.14159265358979323846264338328",
            format!("{:10}", Double::PI);
        pi_width_prec:
            "   3.14159",
            format!("{:10.5}", Double::PI);
    );

    // right-align tests
    test_all_eq!(
        zero_right:
            "         0",
            format!("{:>10}", Double::ZERO);
        neg_zero_right:
            "        -0",
            format!("{:>10}", Double::NEG_ZERO);
        inf_right:
            "       inf",
            format!("{:>10}", Double::INFINITY);
        neg_inf_right:
            "      -inf",
            format!("{:>10}", Double::NEG_INFINITY);
        nan_right:
            "       NaN",
            format!("{:>10}", Double::NAN);
        one_right:
            "         1",
            format!("{:>10}", Double::ONE);
        neg_one_right:
            "        -1",
            format!("{:>10}", Double::NEG_ONE);
        plus_one_right:
            "        +1",
            format!("{:>+10}", Double::ONE);
        pi_right:
            "3.14159265358979323846264338328",
            format!("{:>10}", Double::PI);
        pi_right_prec:
            "   3.14159",
            format!("{:>10.5}", Double::PI);
    );

    // left-align tests
    test_all_eq!(
        zero_left:
            "0         ",
            format!("{:<10}", Double::ZERO);
        neg_zero_left:
            "-0        ",
            format!("{:<10}", Double::NEG_ZERO);
        inf_left:
            "inf       ",
            format!("{:<10}", Double::INFINITY);
        neg_inf_left:
            "-inf      ",
            format!("{:<10}", Double::NEG_INFINITY);
        nan_left:
            "NaN       ",
            format!("{:<10}", Double::NAN);
        one_left:
            "1         ",
            format!("{:<10}", Double::ONE);
        neg_one_left:
            "-1        ",
            format!("{:<10}", Double::NEG_ONE);
        plus_one_left:
            "+1        ",
            format!("{:<+10}", Double::ONE);
        pi_left:
            "3.14159265358979323846264338328",
            format!("{:<10}", Double::PI);
        pi_left_prec:
            "3.14159   ",
            format!("{:<10.5}", Double::PI);
    );

    // center-align tests
    test_all_eq!(
        zero_center:
            "    0     ",
            format!("{:^10}", Double::ZERO);
        neg_zero_center:
            "    -0    ",
            format!("{:^10}", Double::NEG_ZERO);
        inf_center:
            "   inf    ",
            format!("{:^10}", Double::INFINITY);
        neg_inf_center:
            "   -inf   ",
            format!("{:^10}", Double::NEG_INFINITY);
        nan_center:
            "   NaN    ",
            format!("{:^10}", Double::NAN);
        one_center:
            "    1     ",
            format!("{:^10}", Double::ONE);
        neg_one_center:
            "    -1    ",
            format!("{:^10}", Double::NEG_ONE);
        plus_one_center:
            "    +1    ",
            format!("{:^+10}", Double::ONE);
        pi_center:
            "3.14159265358979323846264338328",
            format!("{:^10}", Double::PI);
        pi_center_prec:
            " 3.14159  ",
            format!("{:^10.5}", Double::PI);
    );

    // fill tests
    test_all_eq!(
        zero_fill:
            "_________0",
            format!("{:_>10}", Double::ZERO);
        neg_zero_fill:
            "-0________",
            format!("{:_<10}", Double::NEG_ZERO);
        inf_fill:
            "___inf____",
            format!("{:_^10}", Double::INFINITY);
        neg_inf_fill:
            "______-inf",
            format!("{:_>10}", Double::NEG_INFINITY);
        nan_fill:
            "NaN_______",
            format!("{:_<10}", Double::NAN);
        one_fill:
            "____1_____",
            format!("{:_^10}", Double::ONE);
        neg_one_fill:
            "________-1",
            format!("{:_>10}", Double::NEG_ONE);
        plus_one_fill:
            "+1________",
            format!("{:_<+10}", Double::ONE);
        pi_fill:
            "3.14159265358979323846264338328",
            format!("{:_^10}", Double::PI);
        pi_fill_prec:
            "___3.14159",
            format!("{:_>10.5}", Double::PI);
    );

    // sign-aware zero-poading tests
    test_all_eq!(
        zero_zero_pad:
            "0000000000",
            format!("{:010}", Double::ZERO);
        neg_zero_zero_pad:
            "-000000000",
            format!("{:010}", Double::NEG_ZERO);
        inf_zero_pad:
            "0000000inf",
            format!("{:010}", Double::INFINITY);
        neg_inf_zero_pad:
            "-000000inf",
            format!("{:010}", Double::NEG_INFINITY);
        nan_zero_pad:
            "0000000NaN",
            format!("{:010}", Double::NAN);
        one_zero_pad:
            "0000000001",
            format!("{:010}", Double::ONE);
        neg_one_zero_pad:
            "-000000001",
            format!("{:010}", Double::NEG_ONE);
        plus_one_zero_pad:
            "+000000001",
            format!("{:+010}", Double::ONE);
        pi_zero_pad:
            "3.14159265358979323846264338328",
            format!("{:010}", Double::PI);
        pi_width_zero_pad_prec:
            "0003.14159",
            format!("{:010.5}", Double::PI);
    );

    // special number exp tests
    test_all_eq!(
        nan_lexp:
            "NaN",
            format!("{:e}", Double::NAN);
        neg_nan_lexp:
            "-NaN",
            format!("{:e}", -Double::NAN);
        plus_nan_lexp:
            "+NaN",
            format!("{:+e}", Double::NAN);
        plus_neg_nan_lexp:
            "-NaN",
            format!("{:+e}", -Double::NAN);
        inf_lexp:
            "inf",
            format!("{:e}", Double::INFINITY);
        neg_inf_lexp:
            "-inf",
            format!("{:e}", Double::NEG_INFINITY);
        plus_inf_lexp:
            "+inf",
            format!("{:+e}", Double::INFINITY);
        plus_neg_inf_lexp:
            "-inf",
            format!("{:+e}", Double::NEG_INFINITY);
        zero_lexp:
            "0e0",
            format!("{:e}", Double::ZERO);
        neg_zero_lexp:
            "-0e0",
            format!("{:e}", Double::NEG_ZERO);
        plus_zero_lexp:
            "+0e0",
            format!("{:+e}", Double::ZERO);
        plus_neg_zero_lexp:
            "-0e0",
            format!("{:+e}", Double::NEG_ZERO);
    );

    // special number Exp tests
    test_all_eq!(
        nan_uexp:
            "NaN",
            format!("{:E}", Double::NAN);
        neg_nan_uexp:
            "-NaN",
            format!("{:E}", -Double::NAN);
        plus_nan_uexp:
            "+NaN",
            format!("{:+E}", Double::NAN);
        plus_neg_nan_uexp:
            "-NaN",
            format!("{:+E}", -Double::NAN);
        inf_uexp:
            "inf",
            format!("{:E}", Double::INFINITY);
        neg_inf_uexp:
            "-inf",
            format!("{:E}", Double::NEG_INFINITY);
        plus_inf_uexp:
            "+inf",
            format!("{:+E}", Double::INFINITY);
        plus_neg_inf_uexp:
            "-inf",
            format!("{:+E}", Double::NEG_INFINITY);
        zero_uexp:
            "0E0",
            format!("{:E}", Double::ZERO);
        neg_zero_uexp:
            "-0E0",
            format!("{:E}", Double::NEG_ZERO);
        plus_zero_uexp:
            "+0E0",
            format!("{:+E}", Double::ZERO);
        plus_neg_zero_uexp:
            "-0E0",
            format!("{:+E}", Double::NEG_ZERO);
    );

    // Basic exp tests
    test_all_eq!(
        pi_lexp:
            "3.14159265358979323846264338328e0",
            format!("{:e}", Double::PI);
        neg_pi_lexp:
            "-3.14159265358979323846264338328e0",
            format!("{:e}", -Double::PI);
        plus_pi_lexp:
            "+3.14159265358979323846264338328e0",
            format!("{:+e}", Double::PI);
        plus_neg_pi_lexp:
            "-3.14159265358979323846264338328e0",
            format!("{:+e}", -Double::PI);

        ln2_lexp:
            "6.931471805599453094172321214582e-1",
            format!("{:e}", Double::LN_2);
        neg_ln2_lexp:
            "-6.931471805599453094172321214582e-1",
            format!("{:e}", -Double::LN_2);
        plus_ln2_lexp:
            "+6.931471805599453094172321214582e-1",
            format!("{:+e}", Double::LN_2);
        plus_neg_ln2_lexp:
            "-6.931471805599453094172321214582e-1",
            format!("{:+e}", -Double::LN_2);

        ln2_100_lexp:
            "6.931471805599453094172321214582e-3",
            format!("{:e}", Double::LN_2 / dd!(100));
        neg_ln2_100_lexp:
            "-6.931471805599453094172321214582e-3",
            format!("{:e}", -Double::LN_2 / dd!(100));
        plus_ln2_100_lexp:
            "+6.931471805599453094172321214582e-3",
            format!("{:+e}", Double::LN_2 / dd!(100));
        plus_neg_ln2_100_lexp:
            "-6.931471805599453094172321214582e-3",
            format!("{:+e}", -Double::LN_2 / dd!(100));

        large_lexp:
            "1.234567890123456789e308",
            format!("{:e}", dd!("1.234567890123456789e308"));
        neg_large_lexp:
            "-1.234567890123456789e308",
            format!("{:e}", dd!("-1.234567890123456789e308"));
        plus_large_lexp:
            "+1.234567890123456789e308",
            format!("{:+e}", dd!("1.234567890123456789e308"));

        small_lexp:
            "1.234567890123457e-308",
            format!("{:e}", dd!("1.234567890123456789e-308"));
        neg_small_lexp:
            "-1.234567890123457e-308",
            format!("{:e}", dd!("-1.234567890123456789e-308"));
        plus_small_lexp:
            "+1.234567890123457e-308",
            format!("{:+e}", dd!("1.234567890123456789e-308"));
    );

    // Basic EXP tests
    test_all_eq!(
        pi_uexp:
            "3.14159265358979323846264338328E0",
            format!("{:E}", Double::PI);
        neg_pi_uexp:
            "-3.14159265358979323846264338328E0",
            format!("{:E}", -Double::PI);
        plus_pi_uexp:
            "+3.14159265358979323846264338328E0",
            format!("{:+E}", Double::PI);
        plus_neg_pi_uexp:
            "-3.14159265358979323846264338328E0",
            format!("{:+E}", -Double::PI);

        ln2_uexp:
            "6.931471805599453094172321214582E-1",
            format!("{:E}", Double::LN_2);
        neg_ln2_uexp:
            "-6.931471805599453094172321214582E-1",
            format!("{:E}", -Double::LN_2);
        plus_ln2_uexp:
            "+6.931471805599453094172321214582E-1",
            format!("{:+E}", Double::LN_2);
        plus_neg_ln2_uexp:
            "-6.931471805599453094172321214582E-1",
            format!("{:+E}", -Double::LN_2);

        ln2_100_uexp:
            "6.931471805599453094172321214582E-3",
            format!("{:E}", Double::LN_2 / dd!(100));
        neg_ln2_100_uexp:
            "-6.931471805599453094172321214582E-3",
            format!("{:E}", -Double::LN_2 / dd!(100));
        plus_ln2_100_uexp:
            "+6.931471805599453094172321214582E-3",
            format!("{:+E}", Double::LN_2 / dd!(100));
        plus_neg_ln2_100_uexp:
            "-6.931471805599453094172321214582E-3",
            format!("{:+E}", -Double::LN_2 / dd!(100));

        large_uexp:
            "1.234567890123456789E308",
            format!("{:E}", dd!("1.234567890123456789e308"));
        neg_large_uexp:
            "-1.234567890123456789E308",
            format!("{:E}", dd!("-1.234567890123456789e308"));
        plus_large_uexp:
            "+1.234567890123456789E308",
            format!("{:+E}", dd!("1.234567890123456789e308"));

        small_uexp:
            "1.234567890123457E-308",
            format!("{:E}", dd!("1.234567890123456789e-308"));
        neg_small_uexp:
            "-1.234567890123457E-308",
            format!("{:E}", dd!("-1.234567890123456789e-308"));
        plus_small_uexp:
            "+1.234567890123457E-308",
            format!("{:+E}", dd!("1.234567890123456789e-308"));
    );

    // NOTE
    //
    // The code is so similar between UpperExp and LowerExp that after those tests there's
    // not much need to continue to test them both. Further tests are being done solely with
    // LowerExp.

    // precision exp tests
    test_all_eq!(
        prec_zero_exp:
            "0.0000e0",
            format!("{:.4e}", Double::ZERO);
        prec_neg_zero_exp:
            "-0.0e0",
            format!("{:.1e}", Double::NEG_ZERO);
        prec_plus_zero_exp:
            "+0.000000000000e0",
            format!("{:+.12e}", Double::ZERO);
        prec_plus_neg_zero_exp:
            "-0.000000e0",
            format!("{:+.6e}", Double::NEG_ZERO);

        pi_prec_5_exp:
            "3.14159e0",
            format!("{:.5e}", Double::PI);
        pi_prec_7_exp:
            "3.1415927e0",
            format!("{:.7e}", Double::PI);
        pi_prec_20_exp:
            "3.14159265358979323846e0",
            format!("{:.20e}", Double::PI);
        pi_prec_40_exp:
            "3.1415926535897932384626433832800000000000e0",
            format!("{:.40e}", Double::PI);
        pi_prec_0_exp:
            "3e0",
            format!("{:.0e}", Double::PI);
        neg_pi_prec_40_exp:
            "-3.1415926535897932384626433832800000000000e0",
            format!("{:.40e}", -Double::PI);
        neg_pi_prec_0_exp:
            "-3e0",
            format!("{:.0e}", -Double::PI);
        plus_pi_prec_40_exp:
            "+3.1415926535897932384626433832800000000000e0",
            format!("{:+.40e}", Double::PI);
        plus_pi_prec_0_exp:
            "+3e0",
            format!("{:+.0e}", Double::PI);


        ln2_prec_5_exp:
            "6.93147e-1",
            format!("{:.5e}", Double::LN_2);
        ln2_prec_7_exp:
            "6.9314718e-1",
            format!("{:.7e}", Double::LN_2);
        ln2_prec_20_exp:
            "6.93147180559945309417e-1",
            format!("{:.20e}", Double::LN_2);
        ln2_prec_40_exp:
            "6.9314718055994530941723212145820000000000e-1",
            format!("{:.40e}", Double::LN_2);
        ln2_prec_0_exp:
            "7e-1",
            format!("{:.0e}", Double::LN_2);
        neg_ln2_prec_40_exp:
            "-6.9314718055994530941723212145820000000000e-1",
            format!("{:.40e}", -Double::LN_2);
        neg_ln2_prec_0_exp:
            "-7e-1",
            format!("{:.0e}", -Double::LN_2);
        plus_ln2_prec_40_exp:
            "+6.9314718055994530941723212145820000000000e-1",
            format!("{:+.40e}", Double::LN_2);
        plus_ln2_prec_0_exp:
            "+7e-1",
            format!("{:+.0e}", Double::LN_2);

        ln2_100_prec_5_exp:
            "6.93147e-3",
            format!("{:.5e}", Double::LN_2 / dd!(100));
        ln2_100_prec_7_exp:
            "6.9314718e-3",
            format!("{:.7e}", Double::LN_2 / dd!(100));
        ln2_100_prec_20_exp:
            "6.93147180559945309417e-3",
            format!("{:.20e}", Double::LN_2 / dd!(100));
        ln2_100_prec_40_exp:
            "6.9314718055994530941723212145820000000000e-3",
            format!("{:.40e}", Double::LN_2 / dd!(100));
        ln2_100_prec_0_exp:
            "7e-3",
            format!("{:.0e}", Double::LN_2 / dd!(100));
        neg_ln2_100_prec_40_exp:
            "-6.9314718055994530941723212145820000000000e-3",
            format!("{:.40e}", -Double::LN_2 / dd!(100));
        neg_ln2_100_prec_0_exp:
            "-7e-3",
            format!("{:.0e}", -Double::LN_2 / dd!(100));
        plus_ln2_100_prec_40_exp:
            "+6.9314718055994530941723212145820000000000e-3",
            format!("{:+.40e}", Double::LN_2 / dd!(100));
        plus_ln2_100_prec_0_exp:
            "+7e-3",
            format!("{:+.0e}", Double::LN_2 / dd!(100));


        large_prec_0_exp:
            "1e308",
            format!("{:.0e}", dd!("1.234567890123456789e308"));
        large_prec_8_exp:
            "1.23456789e308",
            format!("{:.8e}", dd!("1.234567890123456789e308"));


        small_prec_0_exp:
            "1e-308",
            format!("{:.0e}", dd!("1.234567890123456789e-308"));
        small_prec_8_exp:
            "1.23456789e-308",
            format!("{:.8e}", dd!("1.234567890123456789e-308"));
    );

    // width tests (default right align)
    test_all_eq!(
        zero_width_exp:
            "       0e0",
            format!("{:10e}", Double::ZERO);
        neg_zero_width_exp:
            "      -0e0",
            format!("{:10e}", Double::NEG_ZERO);
        inf_width_exp:
            "       inf",
            format!("{:10e}", Double::INFINITY);
        neg_inf_width_exp:
            "      -inf",
            format!("{:10e}", Double::NEG_INFINITY);
        nan_width_exp:
            "       NaN",
            format!("{:10e}", Double::NAN);
        one_width_exp:
            "       1e0",
            format!("{:10e}", Double::ONE);
        neg_one_width_exp:
            "      -1e0",
            format!("{:10e}", Double::NEG_ONE);
        plus_one_width_exp:
            "      +1e0",
            format!("{:+10e}", Double::ONE);
        pi_width_exp:
            "3.14159265358979323846264338328e0",
            format!("{:10e}", Double::PI);
        pi_width_prec_exp:
            " 3.14159e0",
            format!("{:10.5e}", Double::PI);
    );

    // right-align tests
    test_all_eq!(
        zero_right_exp:
            "       0e0",
            format!("{:>10e}", Double::ZERO);
        neg_zero_right_exp:
            "      -0e0",
            format!("{:>10e}", Double::NEG_ZERO);
        inf_right_exp:
            "       inf",
            format!("{:>10e}", Double::INFINITY);
        neg_inf_right_exp:
            "      -inf",
            format!("{:>10e}", Double::NEG_INFINITY);
        nan_right_exp:
            "       NaN",
            format!("{:>10e}", Double::NAN);
        one_right_exp:
            "       1e0",
            format!("{:>10e}", Double::ONE);
        neg_one_right_exp:
            "      -1e0",
            format!("{:>10e}", Double::NEG_ONE);
        plus_one_right_exp:
            "      +1e0",
            format!("{:>+10e}", Double::ONE);
        pi_right_exp:
            "3.14159265358979323846264338328e0",
            format!("{:>10e}", Double::PI);
        pi_right_prec_exp:
            " 3.14159e0",
            format!("{:>10.5e}", Double::PI);
    );

    // left-align tests
    test_all_eq!(
        zero_left_exp:
            "0e0       ",
            format!("{:<10e}", Double::ZERO);
        neg_zero_left_exp:
            "-0e0      ",
            format!("{:<10e}", Double::NEG_ZERO);
        inf_left_exp:
            "inf       ",
            format!("{:<10e}", Double::INFINITY);
        neg_inf_left_exp:
            "-inf      ",
            format!("{:<10e}", Double::NEG_INFINITY);
        nan_left_exp:
            "NaN       ",
            format!("{:<10e}", Double::NAN);
        one_left_exp:
            "1e0       ",
            format!("{:<10e}", Double::ONE);
        neg_one_left_exp:
            "-1e0      ",
            format!("{:<10e}", Double::NEG_ONE);
        plus_one_left_exp:
            "+1e0      ",
            format!("{:<+10e}", Double::ONE);
        pi_left_exp:
            "3.14159265358979323846264338328e0",
            format!("{:<10e}", Double::PI);
        pi_left_prec_exp:
            "3.14159e0 ",
            format!("{:<10.5e}", Double::PI);
    );

    // center-align tests
    test_all_eq!(
        zero_center_exp:
            "   0e0    ",
            format!("{:^10e}", Double::ZERO);
        neg_zero_center_exp:
            "   -0e0   ",
            format!("{:^10e}", Double::NEG_ZERO);
        inf_center_exp:
            "   inf    ",
            format!("{:^10e}", Double::INFINITY);
        neg_inf_center_exp:
            "   -inf   ",
            format!("{:^10e}", Double::NEG_INFINITY);
        nan_center_exp:
            "   NaN    ",
            format!("{:^10e}", Double::NAN);
        one_center_exp:
            "   1e0    ",
            format!("{:^10e}", Double::ONE);
        neg_one_center_exp:
            "   -1e0   ",
            format!("{:^10e}", Double::NEG_ONE);
        plus_one_center_exp:
            "   +1e0   ",
            format!("{:^+10e}", Double::ONE);
        pi_center_exp:
            "3.14159265358979323846264338328e0",
            format!("{:^10e}", Double::PI);
        pi_center_prec_exp:
            "3.14159e0 ",
            format!("{:^10.5e}", Double::PI);
    );

    // fill tests
    test_all_eq!(
        zero_fill_exp:
            "_______0e0",
            format!("{:_>10e}", Double::ZERO);
        neg_zero_fill_exp:
            "-0e0______",
            format!("{:_<10e}", Double::NEG_ZERO);
        inf_fill_exp:
            "___inf____",
            format!("{:_^10e}", Double::INFINITY);
        neg_inf_fill_exp:
            "______-inf",
            format!("{:_>10e}", Double::NEG_INFINITY);
        nan_fill_exp:
            "NaN_______",
            format!("{:_<10e}", Double::NAN);
        one_fill_exp:
            "___1e0____",
            format!("{:_^10e}", Double::ONE);
        neg_one_fill_exp:
            "______-1e0",
            format!("{:_>10e}", Double::NEG_ONE);
        plus_one_fill_exp:
            "+1e0______",
            format!("{:_<+10e}", Double::ONE);
        pi_fill_exp:
            "3.14159265358979323846264338328e0",
            format!("{:_^10e}", Double::PI);
        pi_fill_prec_exp:
            "_3.14159e0",
            format!("{:_>10.5e}", Double::PI);
    );

    // sign-aware zero-poading tests
    test_all_eq!(
        zero_zero_pad_exp:
            "00000000e0",
            format!("{:010e}", Double::ZERO);
        neg_zero_zero_pad_exp:
            "-0000000e0",
            format!("{:010e}", Double::NEG_ZERO);
        inf_zero_pad_exp:
            "0000000inf",
            format!("{:010e}", Double::INFINITY);
        neg_inf_zero_pad_exp:
            "-000000inf",
            format!("{:010e}", Double::NEG_INFINITY);
        nan_zero_pad_exp:
            "0000000NaN",
            format!("{:010e}", Double::NAN);
        one_zero_pad_exp:
            "00000001e0",
            format!("{:010e}", Double::ONE);
        neg_one_zero_pad_exp:
            "-0000001e0",
            format!("{:010e}", Double::NEG_ONE);
        plus_one_zero_pad_exp:
            "+0000001e0",
            format!("{:+010e}", Double::ONE);
        pi_zero_pad_exp:
            "3.14159265358979323846264338328e0",
            format!("{:010e}", Double::PI);
        pi_width_zero_pad_prec_exp:
            "03.14159e0",
            format!("{:010.5e}", Double::PI);
    );
}
