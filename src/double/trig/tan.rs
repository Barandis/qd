// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the tangent (tan) of the `Double`.
    /// 
    /// The domain and range of this function are both (-∞, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = (Double::PI / dd!(4)).tan();
    /// let expected = dd!(1);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn tan(self) -> Double {
        let (s, c) = self.sin_cos();
        s / c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(dd!("1.5574077246549022305069748074584"), dd!(1).tan());
        assert_close!(dd!(1), Double::FRAC_PI_4.tan());
        assert!(Double::FRAC_PI_2.tan().is_infinite());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.tan());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.tan());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.tan());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.tan());
    }
}
