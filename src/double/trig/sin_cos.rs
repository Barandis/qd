// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common;
use super::tables;
use crate::double::Double;

impl Double {
    /// Simultaneously computes the sine (sin) and the cosine (cos) of the `Double`. This is
    /// more efficient than calling the separate [`sin`] and [`cos`] functions if you need
    /// both numbers.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI / dd!(4);
    /// let (sin_x, cos_x) = x.sin_cos();
    ///
    /// let diff_sin = (sin_x - x.sin()).abs();
    /// let diff_cos = (cos_x - x.cos()).abs();
    ///
    /// assert!(diff_sin < dd!(1e-30));
    /// assert!(diff_cos < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`sin`]: #method.sin
    /// [`cos`]: #method.cos
    #[allow(clippy::many_single_char_names)]
    pub fn sin_cos(self) -> (Double, Double) {
        if self.is_zero() {
            (Double::ZERO, Double::ONE)
        } else if !self.is_finite() {
            (Double::NAN, Double::NAN)
        } else {
            let (j, k, t) = common::reduce(self);
            let abs_k = k.abs() as usize;

            let (sin_t, cos_t) = common::sincos_taylor(t);

            let (s, c) = if k == 0 {
                (sin_t, cos_t)
            } else {
                let u = tables::COSINES[abs_k - 1];
                let v = tables::SINES[abs_k - 1];
                if k > 0 {
                    (u * sin_t + v * cos_t, u * cos_t - v * sin_t)
                } else {
                    (u * sin_t - v * cos_t, u * cos_t + v * sin_t)
                }
            };

            match j {
                0 => (s, c),
                1 => (c, -s),
                -1 => (-c, s),
                _ => (-s, -c),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sin_cos() {
        let (s, c) = dd!(1).sin_cos();
        assert_close!(dd!("0.84147098480789650665250232163030"), s);
        assert_close!(dd!("0.54030230586813971740093660744298"), c);

        let (s, c) = dd!(Double::PI / dd!(4)).sin_cos();
        assert_close!(dd!("0.70710678118654752440084436210485"), s);
        assert_close!(dd!("0.70710678118654752440084436210485"), c);
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());

        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin_cos().0);
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.sin_cos().1);
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sin_cos().0);
        assert_exact!(Double::ONE, Double::ZERO.sin_cos().1);
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().1);

        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().1);
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.sin_cos().0);
        assert_exact!(Double::NAN, Double::NAN.sin_cos().1);
    }
}
