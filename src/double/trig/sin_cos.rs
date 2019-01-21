// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common::{reduce, sincos_taylor};
use super::tables::{COSINES, SINES};
use crate::double::Double;

impl Double {
    /// Simultaneously computes the sine and the cosine of the number. This is more efficient if you
    /// need both numbers. Returns `(sin(x), cos(x))`.
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
    pub fn sin_cos(self) -> (Double, Double) {
        if self.is_zero() {
            (Double::ZERO, Double::ONE)
        } else if !self.is_finite() {
            (Double::NAN, Double::NAN)
        } else {
            let (j, k, t) = reduce(self);
            let abs_k = k.abs() as usize;

            let (sin_t, cos_t) = sincos_taylor(t);

            let (s, c) = if k == 0 {
                (sin_t, cos_t)
            } else {
                let u = COSINES[abs_k - 1];
                let v = SINES[abs_k - 1];
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
    fn calc() {
        let (s, c) = dd!(1).sin_cos();
        assert_close!(dd!("0.84147098480789650665250232163030"), s);
        assert_close!(dd!("0.54030230586813971740093660744298"), c);
        let (s, c) = dd!(Double::PI / dd!(4)).sin_cos();
        assert_close!(dd!("0.70710678118654752440084436210485"), s);
        assert_close!(dd!("0.70710678118654752440084436210485"), c);
        assert_close!(dd!(0.5), Double::FRAC_PI_6.sin());
    }

    #[test]
    fn edge() {
        assert_exact!(Double::ZERO, Double::ZERO.sin_cos().0);
        assert_exact!(Double::ONE, Double::ZERO.sin_cos().1);

        assert_exact!(Double::ONE, Double::FRAC_PI_2.sin_cos().0);
        assert_exact!(Double::ZERO, Double::FRAC_PI_2.sin_cos().1);

        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::INFINITY.sin_cos().1);

        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().0);
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sin_cos().1);

        assert_exact!(Double::NAN, Double::NAN.sin_cos().0);
        assert_exact!(Double::NAN, Double::NAN.sin_cos().1);
    }
}