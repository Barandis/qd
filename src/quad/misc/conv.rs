// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Converts the quad-double into an `f64`.
    ///
    /// There *will* be accuracy loss if the quad-double was more accurate than
    /// an `f64` to begin with.
    #[inline]
    pub fn as_float(self) -> f64 {
        self.0
    }

    /// Converts the quad-double into an `i128`.
    ///
    /// While it is possible for a `Double` to be created from a `u128`, whether
    /// or not the original is signed is not recorded (since `Double` itself is
    /// signed). The return value of this function can be cast to `u128` if
    /// necessary.
    #[inline]
    pub fn as_int(self) -> i128 {
        self.0 as i128 + self.1 as i128 + self.2 as i128 + self.3 as i128
    }

    /// Converts the quad-double into a 2-tuple of `f64`s.
    ///
    /// The components of the returned tuples are the same numbers used to
    /// represent the quad-double internally.
    #[inline]
    pub fn as_tuple(self) -> (f64, f64, f64, f64) {
        (self.0, self.1, self.2, self.3)
    }
}
