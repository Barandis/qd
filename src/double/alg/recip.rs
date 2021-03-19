// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the reciprocal of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI.recip();
    /// let expected = dd!("0.31830988618379067153776752674503");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Double {
        Double::ONE / self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recip() {
        assert_close!(
            dd!("0.31830988618379067153776752674503"),
            Double::PI.recip()
        );
        assert_close!(dd!("0.36787944117144232159552377016146"), Double::E.recip());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::INFINITY, Double::ZERO.recip());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ZERO.recip());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::ZERO, Double::INFINITY.recip());
        assert_exact!(Double::NEG_ZERO, Double::NEG_INFINITY.recip());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.recip());
    }
}
