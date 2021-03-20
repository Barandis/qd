// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the cube root of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).cbrt();
    /// let expected = dd!("1.2599210498948731647672106072782");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn cbrt(self) -> Double {
        self.nroot(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cbrt() {
        assert_close!(dd!("1.4645918875615232630201425272638"), Double::PI.cbrt());
        assert_close!(
            dd!("-1.3956124250860895286281253196026"),
            (-Double::E).cbrt()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.cbrt());
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.cbrt());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.cbrt());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.cbrt());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.cbrt());
    }
}
