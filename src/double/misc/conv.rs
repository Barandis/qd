// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::two_sum;
use crate::double::Double;

impl Double {
    /// Converts the double-double into an `f64`.
    ///
    /// There *will* be accuracy loss if the quad-double was more accurate than an `f64` to
    /// begin with.
    #[inline]
    pub fn as_float(self) -> f64 {
        self.0
    }

    /// Converts the double-double into an `i64`.
    ///
    /// While it is possible for a `Double` to be created from a `u64`, whether or not the
    /// original is signed is not recorded (since `Double` itself is signed). The return
    /// value of this function can be cast to `u64` if necessary.
    #[inline]
    pub fn as_int(self) -> i64 {
        self.0 as i64 + self.1 as i64
    }

    /// Converts the double-double into a 2-tuple of `f64`s.
    ///
    /// The components of the returned tuples are the same numbers used to represent the
    /// double-double internally.
    #[inline]
    pub fn as_tuple(self) -> (f64, f64) {
        (self.0, self.1)
    }

    /// Assigns the components of a tuple to the components of the double-double.
    #[inline]
    pub fn assign(&mut self, (a, b): (f64, f64)) {
        let (s, e) = two_sum(a, b);
        self.0 = s;
        self.1 = e;
    }
}
