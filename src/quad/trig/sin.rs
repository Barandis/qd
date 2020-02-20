// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common::{cos_taylor, reduce, sin_taylor, sincos_taylor};
use super::tables::{COSINES, SINES};
use crate::quad::Quad;

impl Quad {
    /// Computes the sine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).sin();
    /// let expected = qd!("0.8414709848078965066525023216302989996225630607983710656727517099884");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sin(self) -> Quad {
        // Strategy:
        //
        // We choose integers a and b so that
        //
        //      x = s + aπ/2 + bπ/1024
        //
        // where |s| <= π/2048. Using a precomputed table of sin (kπ/1024) and
        // cos (kπ/1024), we can compute sin x from sin s and cos s. This
        // greatly increases the convergence of the Taylor series for sine and
        // cosine.
        if self.is_zero() {
            Quad::ZERO
        } else if !self.is_finite() {
            Quad::NAN
        } else {
            let (j, k, t) = reduce(self);
            let abs_k = k.abs() as usize;

            if k == 0 {
                match j {
                    0 => sin_taylor(t),
                    1 => cos_taylor(t),
                    -1 => -cos_taylor(t),
                    _ => -sin_taylor(t),
                }
            } else {
                let u = COSINES[abs_k - 1];
                let v = SINES[abs_k - 1];
                let (sin_t, cos_t) = sincos_taylor(t);

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
    fn basic() {
        assert_close!(
            qd!("0.8414709848078965066525023216302989996225630607983710656727517100"),
            qd!(1).sin()
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            (Quad::PI / qd!(4)).sin()
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_6.sin());
    }

    #[test]
    fn special() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sin());
        assert_exact!(Quad::ONE, Quad::FRAC_PI_2.sin());
        assert_exact!(Quad::NAN, Quad::INFINITY.sin());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sin());
        assert_exact!(Quad::NAN, Quad::NAN.sin());
    }
}
