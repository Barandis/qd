// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(7).ln();
    /// let expected = qd!("1.945910149055313305105352743443179729637084729581861188459390150");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn ln(self) -> Quad {
        // Strategy:
        //
        // The Taylor series for logarithms converges much more slowly than that of exp because of
        // the lack of a factorial term in the denominator. Hence this routine instead tries to
        // determine the root of the function
        //
        //      f(x) = exp(x) - a
        //
        // using Newton's iteration. This iteration is given by
        //
        //      x' = x - f(x)/f'(x)
        //         = x - (1 - a * exp(-x))
        //         = x + a * exp(-x) - 1
        //
        // Testing has shown that it requires two iterations to get the required precision.
        if self == Quad::ONE {
            Quad::ZERO
        } else if self.is_zero() {
            Quad::NAN
        } else if self.is_sign_negative() {
            Quad::NAN
        } else {
            let mut x = Quad::from(self.0.ln()); // initial approximation
            let mut i = 0;
            loop {
                let next = x + self * (-x).exp() - Quad::ONE;
                if (x - next).abs() < Quad::EPSILON || i >= 5 {
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
    fn calc() {
        assert_close!(
            qd!("2.302585092994045684017991454684364207601101488628772976033327901"),
            qd!(10).ln()
        );
        assert_close!(
            qd!("5.493061443340548456976226184612628523237452789113747258673471668"),
            qd!(243).ln()
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::ZERO, qd!(1).ln());
        assert_exact!(Quad::NAN, qd!(0).ln());
        assert_exact!(Quad::NAN, qd!(-1).ln());
        assert_close!(Quad::ONE, qd!(Quad::E).ln());
    }
}
