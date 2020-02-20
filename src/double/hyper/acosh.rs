// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the inverse hyperbolic cosine of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).acosh();
    /// let expected = dd!("0.96242365011920689499551782684874");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn acosh(self) -> Double {
        if self < Double::ONE {
            Double::NAN
        } else if self.is_infinite() {
            Double::INFINITY
        } else {
            (self + (self.sqr() - Double::ONE).sqrt()).ln()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acosh() {
        assert_close!(
            dd!("1.81152627246085310702185204930542"),
            Double::PI.acosh()
        );
        assert_close!(
            dd!("1.65745445415307727259382874228053"),
            Double::E.acosh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::ZERO.acosh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.acosh());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.acosh());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.acosh());
    }
}
