// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

// #region From implementations

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
                Quad(a.into(), 0.0, 0.0, 0.0)
            }
        }
    )*);
}

from_impl! { i8 u8 i16 u16 i32 u32 f32 f64 }

// #endregion
