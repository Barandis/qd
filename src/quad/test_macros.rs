// Copyright (c) 2021 Thomas J. Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

macro_rules! prec {
    ($expected:expr, $actual:expr, $digits:expr $(,)?) => {
        let expected = Quad::from($expected);
        let actual = Quad::from($actual);
        let mag = if expected.is_zero() {
            1
        } else {
            expected.0.abs().log10().floor() as i32
        };
        let epsilon = Quad(10.0, 0.0, 0.0, 0.0).powi(mag - $digits);
        let diff = (expected - actual).abs();
        let message = format!(
            concat!(
                "\n",
                "Expected: {0}\n",
                "Actual:   {1}\n",
                "\n",
                "Delta:    {2:e}\n",
                "Epsilon:  {3:e}\n",
                "\n",
                "Components:\n",
                "  Expected: {4:<22e} {5:<22e} {6:<22e} {7:e}\n",
                "  Actual:   {8:<22e} {9:<22e} {10:<22e} {11:e}\n",
            ),
            expected,
            actual,
            diff,
            epsilon,
            expected[0],
            expected[1],
            expected[2],
            expected[3],
            actual[0],
            actual[1],
            actual[2],
            actual[3],
        );
        assert!(diff < epsilon, message);
    };
}

macro_rules! near {
    ($expected:expr, $actual:expr $(,)?) => {
        prec!($expected, $actual, 62);
    };
}

macro_rules! exact {
    ($expected:expr, $actual:expr $(,)?) => {
        let expected = Quad::from($expected);
        let actual = Quad::from($actual);
        let message = format!(
            concat!(
                "\n",
                "Expected: {0}\n",
                "Actual:   {1}\n",
                "\n",
                "Components:\n",
                "  Expected: {2:<22e} {3:<22e} {4:<22e} {5:e}\n",
                "  Actual:   {6:<22e} {7:<22e} {8:<22e} {9:e}\n",
            ),
            expected,
            actual,
            expected[0],
            expected[1],
            expected[2],
            expected[3],
            actual[0],
            actual[1],
            actual[2],
            actual[3],
        );
        if expected.is_nan() {
            assert!(actual.is_nan(), message);
        } else {
            assert!(expected == actual, message);
        }
    };
}

macro_rules! test {
    ($name:ident: { $($tt:tt)* }) => {
        #[test] fn $name() { $($tt)* }
    };
}

macro_rules! test_prec {
    ($name:ident: $expected:expr, $actual:expr, $digits:expr $(,)?) => {
        #[test]
        fn $name() {
            prec!($expected, $actual, $digits);
        }
    };
}

macro_rules! test_near {
    ($name:ident: $expected:expr, $actual:expr $(,)?) => {
        #[test]
        fn $name() {
            near!($expected, $actual);
        }
    };
}

macro_rules! test_exact {
    ($name:ident: $expected:expr, $actual:expr $(,)?) => {
        #[test]
        fn $name() {
            exact!($expected, $actual);
        }
    };
}

macro_rules! test_all {
    ($($name:ident: { $($tt:tt)* })*) => {
        $(#[test] fn $name() { $($tt)* })*
    };
}

macro_rules! test_all_eq {
    ($($name:ident: $expected:expr, $actual:expr);* $(;)?) => {
        $(#[test] fn $name() { assert_eq!($expected, $actual); })*
    };
}

macro_rules! test_all_assert {
    ($($name:ident: $assert:expr);* $(;)?) => {
        $(#[test] fn $name() { assert!($assert); })*
    };
}

macro_rules! test_all_prec {
    ($($name:ident: $expected:expr, $actual:expr, $digits:expr);* $(;)?) => {
        $(test_prec!($name: $expected, $actual, $digits);)*
    };
}

macro_rules! test_all_near {
    ($($name:ident: $expected:expr, $actual:expr);* $(;)?) => {
        $(test_near!($name: $expected, $actual);)*
    }
}

macro_rules! test_all_exact {
    ($($name:ident: $expected:expr, $actual:expr);* $(;)? )=> {
        $(test_exact!($name: $expected, $actual);)*
    };
}

macro_rules! assert_precision {
    ($expected:expr, $actual:expr, $digits:expr) => {
        let expected = Quad::from($expected);
        let actual = Quad::from($actual);
        let mag = expected.0.abs().log10().floor() as i32;
        let epsilon = Quad(10.0, 0.0, 0.0, 0.0).powi(mag - $digits);
        let diff = (expected - actual).abs();
        let message = format!(
            concat!(
                "\n",
                "Expected: {0}\n",
                "Actual:   {1}\n",
                "\n",
                "Delta:    {2:e}\n",
                "Epsilon:  {3:e}\n",
                "\n",
                "Components:\n",
                "  Expected: {4:<22e} {5:<22e} {6:<22e} {7:e}\n",
                "  Actual:   {8:<22e} {9:<22e} {10:<22e} {11:e}\n",
            ),
            expected,
            actual,
            diff,
            epsilon,
            expected[0],
            expected[1],
            expected[2],
            expected[3],
            actual[0],
            actual[1],
            actual[2],
            actual[3],
        );
        assert!(diff < epsilon, message);
    };
}

macro_rules! assert_close {
    ($expected:expr, $actual:expr $(,)*) => {
        assert_precision!($expected, $actual, 60);
    };
}

macro_rules! assert_exact {
    ($expected:expr, $actual:expr) => {
        let expected = Quad::from($expected);
        let actual = Quad::from($actual);
        let message = format!(
            concat!(
                "\n",
                "Expected: {0}\n",
                "Actual:   {1}\n",
                "\n",
                "Components:\n",
                "  Expected: {2:<22e} {3:<22e} {4:<22e} {5:e}\n",
                "  Actual:   {6:<22e} {7:<22e} {8:<22e} {9:e}\n",
            ),
            expected,
            actual,
            expected[0],
            expected[1],
            expected[2],
            expected[3],
            actual[0],
            actual[1],
            actual[2],
            actual[3],
        );
        if expected.is_nan() {
            assert!(actual.is_nan(), message);
        } else {
            assert!(expected == actual, message);
        }
    };
}
