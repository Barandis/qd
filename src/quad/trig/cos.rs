// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common::{cos_taylor, reduce, sin_taylor, sincos_taylor};
use super::tables::{COSINES, SINES};
use crate::quad::Quad;

impl Quad {
    /// Computes the cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).cos();
    /// let expected = qd!("0.5403023058681397174009366074429766037323104206179222276700972553787");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn cos(self) -> Quad {
        if self.is_zero() {
            Quad::ONE
        } else if !self.is_finite() {
            Quad::NAN
        } else {
            let (j, k, t) = reduce(self);
            let abs_k = k.abs() as usize;

            if k == 0 {
                match j {
                    0 => cos_taylor(t),
                    1 => -sin_taylor(t),
                    -1 => sin_taylor(t),
                    _ => -cos_taylor(t),
                }
            } else {
                let u = COSINES[abs_k - 1];
                let v = SINES[abs_k - 1];
                let (sin_t, cos_t) = sincos_taylor(t);

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
    fn calc() {
        assert_close!(
            qd!("0.5403023058681397174009366074429766037323104206179222276700972554"),
            qd!(1).cos()
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            (Quad::PI / qd!(4)).cos()
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_3.cos());
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::ONE, Quad::ZERO.cos());
        assert_exact!(Quad::ZERO, Quad::FRAC_PI_2.cos());
        assert_exact!(Quad::NAN, Quad::INFINITY.cos());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.cos());
        assert_exact!(Quad::NAN, Quad::NAN.cos());
    }
}
