// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

// #region From implementations

fn from_float(n: f64) -> Quad {
    if n == 0.0 {
        return if n.is_sign_negative() {
            Quad::NEG_ZERO
        } else {
            Quad::ZERO
        };
    }
    if n.is_nan() {
        return Quad::NAN;
    }
    if n.is_infinite() {
        return if n.is_sign_negative() {
            Quad::NEG_INFINITY
        } else {
            Quad::INFINITY
        };
    }
    if n.floor() == n {
        return Quad(n, 0.0, 0.0, 0.0);
    }
    n.to_string().parse().unwrap()
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<($t, $t, $t, $t)> for Quad {
            #[inline]
            fn from((a, b, c, d): ($t, $t, $t, $t)) -> Quad {
                Quad(a.into(), b.into(), c.into(), d.into())
            }
        }

        impl From<($t, $t, $t)> for Quad {
            #[inline]
            fn from((a, b, c): ($t, $t, $t)) -> Quad {
                Quad(a.into(), b.into(), c.into(), 0.0)
            }
        }

        impl From<($t, $t)> for Quad {
            #[inline]
            fn from((a, b): ($t, $t)) -> Quad {
                Quad(a.into(), b.into(), 0.0, 0.0)
            }
        }

        impl From<$t> for Quad {
            #[inline]
            fn from(a: $t) -> Quad {
                Quad::from(from_float(a.into()))
            }
        }
    )*);
}

from_impl! { i8 u8 i16 u16 i32 u32 f32 f64 }

// #endregion

impl Quad {
    #[inline]
    pub fn as_float(self) -> f64 {
        self.0
    }

    #[inline]
    pub fn as_int(self) -> i128 {
        self.0 as i128 + self.1 as i128 + self.2 as i128 + self.3 as i128
    }
}
