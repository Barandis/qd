// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use super::common::{reduce, sincos_taylor};
use super::tables::{COSINES, SINES};
use crate::quad::Quad;

impl Quad {
    /// Simultaneously computes the sine and the cosine of the number. This is more efficient if you
    /// need both numbers. Returns `(sin(x), cos(x))`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI / qd!(4);
    /// let (sin_x, cos_x) = x.sin_cos();
    ///
    /// let diff_sin = (sin_x - x.sin()).abs();
    /// let diff_cos = (cos_x - x.cos()).abs();
    ///
    /// assert!(diff_sin < qd!(1e-60));
    /// assert!(diff_cos < qd!(1e-60));
    /// # }
    /// ```
    pub fn sin_cos(self) -> (Quad, Quad) {
        if self.is_zero() {
            (Quad::ZERO, Quad::ONE)
        } else if !self.is_finite() {
            (Quad::NAN, Quad::NAN)
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
    fn quad_trig_sin_cos() {
        let (s, c) = qd!(1).sin_cos();
        assert_close!(
            qd!("0.8414709848078965066525023216302989996225630607983710656727517100"),
            s
        );
        assert_close!(
            qd!("0.5403023058681397174009366074429766037323104206179222276700972554"),
            c
        );
        let (s, c) = qd!(Quad::PI / qd!(4)).sin_cos();
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            s
        );
        assert_close!(
            qd!("0.7071067811865475244008443621048490392848359376884740365883398690"),
            c
        );
        assert_close!(qd!(0.5), Quad::FRAC_PI_6.sin());

        assert_exact!(Quad::ZERO, Quad::ZERO.sin_cos().0);
        assert_exact!(Quad::ONE, Quad::ZERO.sin_cos().1);

        assert_exact!(Quad::ONE, Quad::FRAC_PI_2.sin_cos().0);
        assert_exact!(Quad::ZERO, Quad::FRAC_PI_2.sin_cos().1);

        assert_exact!(Quad::NAN, Quad::INFINITY.sin_cos().0);
        assert_exact!(Quad::NAN, Quad::INFINITY.sin_cos().1);

        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sin_cos().0);
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sin_cos().1);

        assert_exact!(Quad::NAN, Quad::NAN.sin_cos().0);
        assert_exact!(Quad::NAN, Quad::NAN.sin_cos().1);
    }
}
