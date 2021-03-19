// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common;
use super::tables;
use crate::double::Double;

impl Double {
    /// Computes the cosine of the number.
    /// 
    /// The domain of this function is (-∞, ∞), and the range is [-1, 1].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = (Double::PI / dd!(2)).cos();
    /// let expected = dd!(0);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn cos(self) -> Double {
        if self.is_zero() {
            Double::ONE
        } else if !self.is_finite() {
            Double::NAN
        } else {
            let (j, k, t) = common::reduce(self);
            let abs_k = k.abs() as usize;

            if k == 0 {
                match j {
                    0 => common::cos_taylor(t),
                    1 => -common::sin_taylor(t),
                    -1 => common::sin_taylor(t),
                    _ => -common::cos_taylor(t),
                }
            } else {
                let u = tables::COSINES[abs_k - 1];
                let v = tables::SINES[abs_k - 1];
                let (sin_t, cos_t) = common::sincos_taylor(t);

                if k > 0 {
                    match j {
                        0 => u * cos_t - v * sin_t,
                        1 => -u * sin_t - v * cos_t,
                        -1 => u * sin_t + v * cos_t,
                        _ => -u * cos_t + v * sin_t,
                    }
                } else {
                    match j {
                        0 => u * cos_t + v * sin_t,
                        1 => v * cos_t - u * sin_t,
                        -1 => u * sin_t - v * cos_t,
                        _ => -u * cos_t - v * sin_t,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cos() {
        assert_close!(dd!("0.54030230586813971740093660744298"), dd!(1).cos());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / dd!(4)).cos()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_3.cos());
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.cos());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ONE, Double::ZERO.cos());
        assert_exact!(Double::ONE, Double::NEG_ZERO.cos());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.cos());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.cos());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.cos());
    }
}
