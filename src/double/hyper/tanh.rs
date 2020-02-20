// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Computes the hyperbolic tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).tanh();
    /// let expected = dd!("0.76159415595576488811945828260479");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn tanh(self) -> Double {
        if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ZERO
        } else if self.is_infinite() {
            self.signum() * Double::ONE
        } else if self.abs().as_float() > 0.05 {
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
            dd!("0.99627207622074994426469058001254"),
            Double::PI.tanh()
        );
        assert_close!(
            dd!("0.99132891580059983779555761569968"),
            Double::E.tanh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.tanh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::ONE, Double::INFINITY.tanh());
        assert_exact!(-Double::ONE, Double::NEG_INFINITY.tanh());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.tanh());
    }
}
