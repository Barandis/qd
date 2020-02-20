// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::renorm4;
use crate::double::Double;
use crate::quad::Quad;

fn from_float(n: f64) -> Quad {
    if n == 0.0 {
        if n.is_sign_negative() {
            Quad::NEG_ZERO
        } else {
            Quad::ZERO
        }
    } else if n.is_nan() {
        Quad::NAN
    } else if n.is_infinite() {
        if n.is_sign_negative() {
            Quad::NEG_INFINITY
        } else {
            Quad::INFINITY
        }
    } else if n.floor() == n {
        Quad(n, 0.0, 0.0, 0.0)
    } else {
        n.to_string().parse().unwrap()
    }
}

#[inline]
fn from_components(a: f64, b: f64, c: f64, d: f64) -> Quad {
    if d != 0.0 {
        Quad(a, b, c, d)
    } else if a == 0.0 {
        if a.is_sign_negative() {
            Quad::NEG_ZERO
        } else {
            Quad::ZERO
        }
    } else if a.is_nan() {
        Quad::NAN
    } else if a.is_infinite() {
        if a.is_sign_negative() {
            Quad::NEG_INFINITY
        } else {
            Quad::INFINITY
        }
    } else if a.floor() == a {
        Quad(a, b, c, d)
    } else {
        Quad(a, b, c, d).to_string().parse().unwrap()
    }
}

fn from_2_tuple(a: f64, b: f64) -> Quad {
    let (a, b, c, d) = renorm4(a, b, 0.0, 0.0);
    from_components(a, b, c, d)
}

fn from_3_tuple(a: f64, b: f64, c: f64) -> Quad {
    let (a, b, c, d) = renorm4(a, b, c, 0.0);
    from_components(a, b, c, d)
}

#[inline]
fn split_u64(a: u64) -> (u32, u32) {
    let x = (a >> 32) as u32;
    let y = a as u32;
    (x, y)
}

#[inline]
fn split_u128(a: u128) -> (u32, u32, u32, u32) {
    let w = (a >> 96) as u32;
    let x = (a >> 64) as u32;
    let y = (a >> 32) as u32;
    let z = a as u32;
    (w, x, y, z)
}

fn from_u64(a: u64) -> Quad {
    let (x, y) = split_u64(a);
    Quad::from(renorm4(x as f64 * 2f64.powi(32), y as f64, 0.0, 0.0))
}

fn from_i64(a: i64) -> Quad {
    let sign = a.signum();
    let a = a.abs() as u64;
    let (x, y) = split_u64(a);
    let d = Quad::from(renorm4(x as f64 * 2f64.powi(32), y as f64, 0.0, 0.0));
    if sign == -1 {
        -d
    } else {
        d
    }
}

fn from_u128(a: u128) -> Quad {
    let (w, x, y, z) = split_u128(a);
    Quad::from(renorm4(
        w as f64 * 2f64.powi(96),
        x as f64 * 2f64.powi(64),
        y as f64 * 2f64.powi(32),
        z as f64,
    ))
}

fn from_i128(a: i128) -> Quad {
    let sign = a.signum();
    let a = a.abs() as u128;
    let (w, x, y, z) = split_u128(a);
    let d = Quad::from(renorm4(
        w as f64 * 2f64.powi(96),
        x as f64 * 2f64.powi(64),
        y as f64 * 2f64.powi(32),
        z as f64,
    ));
    if sign == -1 {
        -d
    } else {
        d
    }
}

macro_rules! from_int_impl {
    ($($t:ident)*) => ($(
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                Quad(a.into(), 0.0, 0.0, 0.0)
            }
        }

        impl From<($t, $t)> for Quad {
            #[inline]
            fn from((a, b): ($t, $t)) -> Quad {
                Quad(a.into(), b.into(), 0.0, 0.0)
            }
        }

        impl From<($t, $t, $t)> for Quad {
            #[inline]
            fn from((a, b, c): ($t, $t, $t)) -> Quad {
                Quad(a.into(), b.into(), c.into(), 0.0)
            }
        }

        impl From<($t, $t, $t, $t)> for Quad {
            #[inline]
            fn from((a, b, c, d): ($t, $t, $t, $t)) -> Quad {
                Quad(a.into(), b.into(), c.into(), d.into())
            }
        }
    )*);
}

macro_rules! from_float_impl {
    ($($t:ident)*) => ($(
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                from_float(a.into())
            }
        }

        impl From<($t, $t)> for Quad {
            #[inline]
            fn from((a, b): ($t, $t)) -> Quad {
                from_2_tuple(a.into(), b.into())
            }
        }

        impl From<($t, $t, $t)> for Quad {
            #[inline]
            fn from((a, b, c): ($t, $t, $t)) -> Quad {
                from_3_tuple(a.into(), b.into(), c.into())
            }
        }

        impl From<($t, $t, $t, $t)> for Quad {
            #[inline]
            fn from((a, b, c, d): ($t, $t, $t, $t)) -> Quad {
                Quad(a.into(), b.into(), c.into(), d.into())
            }
        }
    )*);
}

macro_rules! from_long_int_impl {
    ($($t:ident $f:ident)*) => ($(
        impl From<$t> for Quad {
            fn from(a: $t) -> Quad {
                $f(a)
            }
        }

        impl From<($t, $t)> for Quad {
            fn from((a, b): ($t, $t)) -> Quad {
                $f(a) + $f(b)
            }
        }

        impl From<($t, $t, $t)> for Quad {
            fn from((a, b, c): ($t, $t, $t)) -> Quad {
                $f(a) + $f(b) + $f(c)
            }
        }

        impl From<($t, $t, $t, $t)> for Quad {
            fn from((a, b, c, d): ($t, $t, $t, $t)) -> Quad {
                $f(a) + $f(b) + $f(c) + $f(d)
            }
        }
    )*);
}

from_int_impl! { i8 u8 i16 u16 i32 u32 }
from_long_int_impl! { i64 from_i64 u64 from_u64 i128 from_i128 u128 from_u128 }
from_float_impl! { f32 f64 }

impl From<Double> for Quad {
    fn from(a: Double) -> Quad {
        from_2_tuple(a[0], a[1])
    }
}

impl From<&str> for Quad {
    /// Converts a string representation of a number into a `Quad`.
    ///
    /// `parse` from [`FromStr`] is a safer way to make this conversion, as it
    /// returns a type (`Result`) that allows for error checking. This function
    /// returns `NaN` in the case of a parse error, which is indistinguishable
    /// from a legitimately-returned `NaN`. Take care when using this function.
    ///
    /// [`FromStr`]: #impl-FromStr
    fn from(s: &str) -> Quad {
        s.parse().unwrap_or(Quad::NAN)
    }
}

impl From<Quad> for (f64, f64, f64, f64) {
    #[inline]
    fn from(a: Quad) -> (f64, f64, f64, f64) {
        (a.0, a.1, a.2, a.3)
    }
}
