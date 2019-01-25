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
    fn basic() {
        assert_close!(
            dd!("1.811526272460853107021852049305420510220702081057922474861595623"),
            Double::PI.acosh()
        );
        assert_close!(
            dd!("1.657454454153077272593828742280534739158392762033676825848582209"),
            Double::E.acosh()
        );
    }

    #[test]
    fn special() {
        assert_exact!(Double::NAN, dd!(0.0).acosh());
        assert_exact!(Double::NAN, Double::NAN.acosh());
        assert_exact!(Double::INFINITY, Double::INFINITY.acosh());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.acosh());
    }
}
