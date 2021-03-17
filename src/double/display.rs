// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::display;
use crate::double::Double;
use std::fmt;

const DEFAULT_PRECISION: usize = 31;
const TEN: Double = Double(10.0, 0.0);

// Calculates the exponent of the supplied double-double, adjusting the double-double to
// fall somewhere in the range [1, 10) (i.e., to have a single non-zero digit before the
// decimal point).
#[inline]
fn calculate_exponent(r: &mut Double) -> i32 {
    // Quick calculation of exponent based on the first component of `r`. This could turn
    // out to be off by 1 either direction depending on the second component.
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

    // If `r` is outside the range [1, 10), then the exponent was off by 1. Adjust both it
    // and `r`.
    if *r >= TEN {
        *r /= TEN;
        exp += 1;
    } else if *r < Double::ONE {
        *r *= TEN;
        exp -= 1;
    }

    exp
}

// Extracts the digits of `r` into a vector of integers. These integers will fall in the
// range [-9, 9]. Even if `r` is always positive as a whole, its second component can be
// negative which will generate negative 'digits'.
//
// `r` is modified throughout to extract the digits and contains nothing of value when this
// function is complete.
#[inline]
fn extract_digits(r: &mut Double, precision: usize) -> Vec<i32> {
    let mut digits = Vec::with_capacity(precision);
    for _ in 0..precision {
        let digit = r.0 as i32;
        *r -= Double::from(digit);
        *r *= TEN;
        digits.push(digit);
    }
    digits
}

// Turns a double-double into a vector of digits and an exponent. Sign is ignored, and no
// decimal appears in the vector; the exponent is calculated based on having the decimal
// point after the first digit.
//
// This function returns a vector of signed integers even though unsigned would make more
// logical sense. That's because internally (with the call to `extract_digits`) the vector
// has to deal with signed integers, and it's more efficient to let the caller cast them to
// unsigned as needed than it is to create a new vector of unsigned integers and copy them
// over.
fn to_digits(r: &Double, precision: usize) -> (Vec<i32>, i32) {
    let mut r = r.abs();

    if r.is_zero() {
        return (vec![0; precision], 0);
    }

    let mut exp = calculate_exponent(&mut r);
    // We pass one more than the actual precision to leave an extra digit at the end to do
    // rounding
    let mut digits = extract_digits(&mut r, precision + 1);
    display::correct_range(&mut digits);
    display::round_vec(&mut digits, &mut exp);

    (digits, exp)
}

// Potentially pushes a sign character to the supplied vector. Returns whether or not a
// character was actually added, information that is used later in formatting.
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

// Formats `value` as a fixed-point number, with the format defined by `f`.
#[inline]
fn format_fixed(value: &Double, f: &mut fmt::Formatter) -> fmt::Result {
    let mut result = Vec::new();
    let mut sign = true;
    let precision = f.precision().unwrap_or(DEFAULT_PRECISION);

    if value.is_nan() {
        display::push_nan(&mut result);
    } else {
        sign = push_sign(&mut result, value, f);

        if value.is_infinite() {
            display::push_inf(&mut result);
        } else if value.is_zero() {
            display::push_zero(&mut result, f);
        } else {
            let width = precision as i32 + f64::from(value.abs().log10().floor()) as i32 + 1;
            // Higher than the max-length number + max precision so that users can do their
            // format!("{:.30}", Double::from_str("999999999999999999999999999999")) in
            // peace
            let extra = width.max(65);

            // Special case: zero precision, |value| < 1.0
            // In this case a number greater than 0.5 prints 0 and should print 1
            if precision == 0 && f64::from(value.abs()) < 1.0 {
                result.push(if f64::from(value.abs()) >= 0.5 {
                    '1'
                } else {
                    '0'
                });
            } else if width < 0 {
                display::push_zero(&mut result, f);
            } else {
                let (mut digits, exp) = to_digits(value, extra as usize);
                display::push_fixed_digits(
                    &mut result,
                    &mut digits,
                    exp,
                    f.precision(),
                    DEFAULT_PRECISION,
                );
            }
        }

        if !value.is_infinite() {
            display::drop_trailing_zeros(&mut result, f);
        }
    }
    display::align_and_fill(&mut result, f, sign);

    write!(f, "{}", result.into_iter().collect::<String>())
}

// Formats `value` as a exponential number, with the format defined by `f`.
#[inline]
fn format_exp(value: &Double, f: &mut fmt::Formatter, upper: bool) -> fmt::Result {
    let mut result = Vec::new();
    let mut sign = true;
    let mut exp = 0;

    if value.is_nan() {
        display::push_nan(&mut result);
    } else {
        sign = push_sign(&mut result, value, f);

        if value.is_infinite() {
            display::push_inf(&mut result);
        } else if value.is_zero() {
            display::push_zero(&mut result, f);
        } else {
            let width = f.precision().unwrap_or(DEFAULT_PRECISION) + 1;
            let (digits, e) = to_digits(value, width);
            exp = e;
            display::push_exp_digits(&mut result, &digits, f.precision(), DEFAULT_PRECISION);
        }

        if !value.is_infinite() {
            display::drop_trailing_zeros(&mut result, f);
            let marker = if upper { 'E' } else { 'e' };
            display::push_exponent(&mut result, marker, exp);
        }
    }
    display::align_and_fill(&mut result, f, sign);

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

impl fmt::Debug for Double {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alt = f.alternate();
        let mut r = String::from("Double(");
        if alt {
            r.push_str("\n    ");
        }
        r.push_str(format!("{:e}, ", self.0).as_str());
        if alt {
            r.push_str("\n    ");
        }
        r.push_str(format!("{:e}", self.1).as_str());
        if alt {
            r.push('\n');
        }
        r.push(')');
        write!(f, "{}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const PI_TIMES_10_20: &str = "314159265358979323846";
    const PI_TIMES_10_20_EXP: &str = "3.14159265358979323846e20";

    // #region Plain formatting

    fn plain(value: Double) -> String {
        format!("{}", value)
    }

    fn close_str(actual: &str, expected: &str) -> bool {
        let len = expected.len() - 1;
        actual[0..len] == expected[0..len]
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
        // Floating point error will keep these from being displayed exactly when no
        // precision is defined, because the default precision will extend into the deep
        // bits of these numbers. So we're checking to see if they're close.
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
        assert_eq!(plain(Double::from(231_700_000E-5)), "2317");
        assert_eq!(plain(Double::from(-42e3)), "-42000");
    }

    #[test]
    fn format_float_exp() {
        assert!(close_str(plain(Double::from(17.29e0)).as_str(), "17.29"));
        assert!(close_str(
            plain(Double::from(1.677_721_6e-1)).as_str(),
            "0.16777216"
        ));
        assert!(close_str(plain(Double::from(2.317e2)).as_str(), "231.7"));
        assert!(close_str(plain(Double::from(-4.2e-4)).as_str(), "-0.00042"));
    }

    // This is a test for an issue that I have seen mentioned nowhere except in the source
    // code of the MIT library source code. It claims that for numbers of the form 10^x - 1,
    // the decimal point can be printed in the wrong place.
    //
    // I have not seen evidence of this, and it's one otherwise-unmentioned block of code in
    // software that was written more than a decade ago. The "fix" has been taken out of the
    // code but I'm leaving in the test just in case.
    #[test]
    fn format_offset_10_x_minus_1() {
        assert_eq!(
            plain(Double::from(10).powi(29) - Double::ONE),
            "99999999999999999999999999999"
        );
        assert_eq!(
            plain(Double::from(10).powi(30) - Double::ONE),
            "999999999999999999999999999999"
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
        ac_parts[0][0..len] == ex_parts[0][0..len] && ac_parts[1] == ex_parts[1]
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
        // Floating point error will keep these from being displayed exactly when no
        // precision is defined, because the default precision will extend into the deep
        // bits of these numbers. So we're checking to see if they're close.
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
        assert_eq!(exp(Double::from(231_700_000E-5)), "2.317e3");
        assert_eq!(exp(Double::from(-42e3)), "-4.2e4");
    }

    #[test]
    fn format_exp_float_exp() {
        assert!(close_exp(exp(Double::from(17.29e0)).as_str(), "1.729e1"));
        assert!(close_exp(
            exp(Double::from(1.677_721_6e-1)).as_str(),
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
        assert_eq!(
            format!("{:.4}", Double::from(0.000_001_677_721_6)),
            "0.0000"
        );
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
        let value = Double::from(123_456);
        assert_eq!(format!("{:3}", value), "123456");
        assert_eq!(format!("{:6}", value), "123456");
        assert_eq!(format!("{:10}", value), "    123456");
        assert_eq!(format!("{:10}", -value), "   -123456");
        assert_eq!(format!("{:10e}", value), " 1.23456e5");
    }

    #[test]
    fn format_width_right_align() {
        let value = Double::from(123_456);
        assert_eq!(format!("{:>3}", value), "123456");
        assert_eq!(format!("{:>6}", value), "123456");
        assert_eq!(format!("{:>10}", value), "    123456");
        assert_eq!(format!("{:>10}", -value), "   -123456");
        assert_eq!(format!("{:>10e}", value), " 1.23456e5");
    }

    #[test]
    fn format_width_left_align() {
        let value = Double::from(123_456);
        assert_eq!(format!("{:<3}", value), "123456");
        assert_eq!(format!("{:<6}", value), "123456");
        assert_eq!(format!("{:<10}", value), "123456    ");
        assert_eq!(format!("{:<10}", -value), "-123456   ");
        assert_eq!(format!("{:<10e}", value), "1.23456e5 ");
    }

    #[test]
    fn format_width_center_align() {
        let value = Double::from(123_456);
        assert_eq!(format!("{:^3}", value), "123456");
        assert_eq!(format!("{:^6}", value), "123456");
        assert_eq!(format!("{:^10}", value), "  123456  ");
        assert_eq!(format!("{:^10}", -value), " -123456  ");
        assert_eq!(format!("{:^11}", value), "  123456   ");
        assert_eq!(format!("{:^11e}", value), " 1.23456e5 ");
    }

    #[test]
    fn format_width_fill() {
        let value = Double::from(123_456);
        assert_eq!(format!("{:*^3}", value), "123456");
        assert_eq!(format!("{:*^10}", value), "**123456**");
        assert_eq!(format!("{:*>10}", value), "****123456");
        assert_eq!(format!("{:*<10}", value), "123456****");
        assert_eq!(format!("{:*>10}", -value), "***-123456");
        assert_eq!(format!("{:*>10e}", value), "*1.23456e5");
    }

    #[test]
    fn format_width_sign_aware_zero_fill() {
        let value = Double::from(123_456);
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
        let value = Double::from(123_456);
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
        // checking that formatting will actually print out 60 digits (and the decimal
        // point)
        assert_eq!(format!("{:.30}", value).len(), 61);
    }

    // #endregion
}
