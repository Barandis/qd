// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the inverse hyperbolic cosine (cosh<sup>-1</sup>) of the number.
    /// 
    /// The domain of the function is [1, ∞) and the range is [0, ∞). Any argument outside
    /// the range will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1.5).acosh();
    /// let expected = qd!("0.9624236501192068949955178268487368462703686687713210393220363377");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-30));
    /// # }
    /// ```
    /// 
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acosh(self) -> Quad {
        if self < Quad::ONE {
            Quad::NAN
        } else if self.is_infinite() {
            Quad::INFINITY
        } else {
            (self + (self.sqr() - Quad::ONE).sqrt()).ln()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acosh() {
        assert_close!(
            qd!("1.811526272460853107021852049305420510220702081057922474861595623"),
            Quad::PI.acosh()
        );
        assert_close!(
            qd!("1.657454454153077272593828742280534739158392762033676825848582209"),
            Quad::E.acosh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.acosh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.acosh());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.acosh());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.acosh());
    }
}
