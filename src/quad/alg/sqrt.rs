// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common;
use crate::quad::Quad;

impl Quad {
    /// Calculates the square root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).sqrt();
    /// let expected = qd!("1.414213562373095048801688724209698078569671875376948073176679738");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sqrt(self) -> Quad {
        if self.is_zero() {
            Quad::ZERO
        } else if self.is_sign_negative() {
            Quad::NAN
        } else if self.is_infinite() {
            Quad::INFINITY
        } else {
            // Strategy: use Newton's iteration.
            //
            // Perform the following Newton iteration
            //
            //      x' = x + (1 - ax²) * x / 2
            //
            // which converges to 1/√a, starting with a Quad-precision approximation of
            // 1/√a. Newton's iteration more or less quadruples the precision with each
            // pass, so performing it three times should be enough.

            let mut r = Quad::ONE / Quad::from(self.0.sqrt());
            let h = common::mul_pwr2(self, 0.5);
            let k = Quad(0.5, 0.0, 0.0, 0.0);

            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;

            r *= self;
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive() {
        assert_close!(
            qd!("1.772453850905516027298167483341145182797549456122387128213807790"),
            Quad::PI.sqrt()
        );
        assert_close!(
            qd!("48.13522618623496195194491189007433987957200800774184036920112360"),
            qd!(2317).sqrt()
        );
    }

    #[test]
    fn negative() {
        assert_exact!(Quad::NAN, qd!(-3).sqrt());
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sqrt());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sqrt());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sqrt());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sqrt());
    }
}
