// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common::mul_pwr2;
use crate::quad::Quad;

impl Quad {
    /// Calculates the inverse hyperbolic tangent of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(0.5).atanh();
    /// let expected = qd!("0.5493061443340548456976226184612628523237452789113747258673471668");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-30));
    /// # }
    /// ```
    pub fn atanh(self) -> Quad {
        if self.abs() >= Quad::ONE {
            Quad::NAN
        } else {
            mul_pwr2(((Quad::ONE + self) / (Quad::ONE - self)).ln(), 0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_close!(
            qd!("0.3297653149566991076178634175552186042701373911406924144029083548"),
            Quad::PI.recip().atanh()
        );
        assert_close!(
            qd!("0.3859684164526523625353195700175926718961289961812712597770308403"),
            Quad::E.recip().atanh()
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::ZERO, qd!(0.0).atanh());
        assert_exact!(Quad::NAN, Quad::NAN.atanh());
        assert_exact!(Quad::NAN, Quad::INFINITY.atanh());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.atanh());
    }
}
