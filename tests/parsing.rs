use qd::DoubleDouble;
use qd::error::QdFloatErrorKind;

// Checks down to the length of the expected - 1, thus ignoring rounding error.
// Pretty much equivalent to testing (a - b).abs() against EPSILON, except for
// strings
fn eq_to_precision(expected: &str, actual: &str) -> bool {
    let len = expected.len() - 1;
    &expected[0..len] == &actual[0..len]
}

#[test]
fn parse_plain_integer() {
    let dd: DoubleDouble = "1".parse().unwrap();
    assert_eq!(format!("{}", dd), "1");
}

#[test]
fn parse_plain_zero() {
    let dd = "0".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{}", dd), "0");
}

#[test]
fn parse_plain_float() {
    let expected = "12.3456789";
    let dd = expected.parse::<DoubleDouble>().unwrap();
    assert!(eq_to_precision(expected, format!("{}", dd).as_str()));
}

#[test]
fn parse_plain_long_float() {
    let text = "1.4142135623730950488016887242096980785696718753769";
    let expected = &text[0..31];
    let dd = text.parse::<DoubleDouble>().unwrap();
    assert!(eq_to_precision(expected, format!("{}", dd).as_str()))
}

#[test]
fn parse_nan() {
    let dd = "nan".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{}", dd), "NaN");
}

#[test]
fn parse_inf() {
    let dd = "inf".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{}", dd), "inf");
}

#[test]
fn parse_neg_inf() {
    let dd = "-inf".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{}", dd), "-inf");
}

#[test]
fn parse_error_empty() {
    let err = "".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Empty);
}

#[test]
fn parse_error_double_point() {
    let err = "0.1234.567".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn parse_error_double_negative() {
    let err = "-0.123-456".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn parse_error_misplaced_negative() {
    let err = "0.123-456".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn parse_error_double_positive() {
    let err = "+0.123+456".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn parse_error_misplaced_positive() {
    let err = "0.123+456".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn parse_error_bad_exponent() {
    let err = "1.23456e3a".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn parse_error_bad_character() {
    let err = "1.23d567".parse::<DoubleDouble>().unwrap_err();
    assert_eq!(err.kind, QdFloatErrorKind::Invalid);
}

#[test]
fn format_width() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:10}", dd), "    1.2345");
    let dn = "-1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:10}", dn), "   -1.2345");
}

#[test]
fn format_align_right() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:>10}", dd), "    1.2345");
}

#[test]
fn format_align_left() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:<10}", dd), "1.2345    ");
}

#[test]
fn format_align_center() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:^10}", dd), "  1.2345  ");
    let dn = "-1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:^10}", dn), " -1.2345  ");
}

#[test]
fn format_fill() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:.<10}", dd), "1.2345....");
}

#[test]
fn format_positive() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:+}", dd), "+1.2345");
}

#[test]
fn format_sign_aware_zero() {
    let dd = "1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:010}", dd), "00001.2345");
    assert_eq!(format!("{:+010}", dd), "+0001.2345");
    let dn = "-1.2345".parse::<DoubleDouble>().unwrap();
    assert_eq!(format!("{:010}", dn), "-0001.2345");
}
