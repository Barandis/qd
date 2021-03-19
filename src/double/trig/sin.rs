// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common;
use super::tables;
use crate::double::Double;

impl Double {
    /// Computes the sine (sin) of the `Double`.
    /// 
    /// The domain of this function is (-∞, ∞), and the range is [-1, 1].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = (Double::PI / dd!(2)).sin();
    /// let expected = dd!(1);
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn sin(self) -> Double {
        // Strategy:
        //
        // We choose integers a and b so that
        //
        //      x = s + aπ/2 + bπ/16
        //
        // where |s| <= π/32. Using a precomputed table of sin (kπ/16) and cos (kπ/16), we
        // can compute sin x from sin s and cos s. This greatly increases the convergence of
        // the Taylor series for sine and cosine.
        if self.is_zero() {
            Double::ZERO
        } else if !self.is_finite() {
            Double::NAN
        } else {
            let (j, k, t) = common::reduce(self);
            let abs_k = k.abs() as usize;

            if k == 0 {
                match j {
                    0 => common::sin_taylor(t),
                    1 => common::cos_taylor(t),
                    -1 => -common::cos_taylor(t),
                    _ => -common::sin_taylor(t),
                }
            } else {
                let u = tables::COSINES[abs_k - 1];
                let v = tables::SINES[abs_k - 1];
                let (sin_t, cos_t) = common::sincos_taylor(t);

                if k > 0 {
                    match j {
                        0 => u * sin_t + v * cos_t,
                        1 => u * cos_t - v * sin_t,
                        -1 => -u * cos_t + v * sin_t,
                        _ => -u * sin_t - v * cos_t,
                    }
                } else {
                    match j {
                        0 => u * sin_t - v * cos_t,
                        1 => u * cos_t + v * sin_t,
                        -1 => -u * cos_t - v * sin_t,
                        _ => -u * sin_t + v * cos_t,
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
    fn sin() {
        assert_close!(dd!("0.84147098480789650665250232163030"), dd!(1).sin());
        assert_close!(
            dd!("0.70710678118654752440084436210485"),
            (Double::PI / dd!(4)).sin()
        );
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());
        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sin());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.sin());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.sin());
    }
}
