// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Computes the hyperbolic tangent (tanh) of the `Quad`.
    /// 
    /// The domain of this function is (-∞, ∞), and the range is (-1, 1).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).tanh();
    /// let expected = qd!("0.7615941559557648881194582826047935904127685972579365515968105001");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn tanh(self) -> Quad {
        if self.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
            Quad::ZERO
        } else if self.is_infinite() {
            self.signum() * Quad::ONE
        } else if self.abs().0 > 0.05 {
            let a = self.exp();
            let inv_a = a.recip();
            (a - inv_a) / (a + inv_a)
        } else {
            let (s, c) = self.sinh_cosh();
            s / c
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tanh() {
        assert_close!(
            qd!("0.9962720762207499442646905800125367118968991908045876143626124160"),
            Quad::PI.tanh()
        );
        assert_close!(
            qd!("0.9913289158005998377955576156996843829216586979874637167678292814"),
            Quad::E.tanh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.tanh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::ONE, Quad::INFINITY.tanh());
        assert_exact!(Quad::NEG_ONE, Quad::NEG_INFINITY.tanh());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.tanh());
    }
}
