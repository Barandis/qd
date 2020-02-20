// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(7).ln();
    /// let expected = dd!("1.9459101490553133051053527434432");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn ln(self) -> Double {
        // Strategy:
        //
        // The Taylor series for logarithms converges much more slowly than that
        // of exp because of the lack of a factorial term in the denominator.
        // Hence this routine instead tries to determine the root of the
        // function
        //
        //      f(x) = exp(x) - a
        //
        // using Newton's iteration. This iteration is given by
        //
        //      x' = x - f(x)/f'(x)
        //         = x - (1 - a * exp(-x))
        //         = x + a * exp(-x) - 1
        //
        // Testing has shown that it requires two iterations to get the required
        // precision.
        if self == Double::ONE {
            Double::ZERO
        } else if self.is_zero() || self.is_sign_negative() {
            Double::NAN
        } else if self.is_infinite() {
            Double::INFINITY
        } else {
            let mut x = Double::from(self.0.ln()); // initial approximation
            let mut i = 0;
            loop {
                let next = x + self * (-x).exp() - Double::ONE;
                if (x - next).abs() < Double::EPSILON || i >= 5 {
                    return next;
                }
                x = next;
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ln() {
        assert_close!(dd!("2.30258509299404568401799145468436"), dd!(10).ln());
        assert_close!(dd!("5.49306144334054845697622618461263"), dd!(243).ln());
        assert_close!(Double::ONE, Double::E.ln());
        assert_exact!(Double::ZERO, Double::ONE.ln());
    }

    #[test]
    fn zero() {
        assert_exact!(Double::NAN, Double::ZERO.ln());
        assert_exact!(Double::NAN, Double::NEG_ZERO.ln());
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::INFINITY, Double::INFINITY.ln());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.ln());
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.ln());
    }

    #[test]
    fn negative() {
        assert_exact!(Double::NAN, dd!(-1).ln());
    }
}
