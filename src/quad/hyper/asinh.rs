// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the inverse hyperbolic sine (sinh<sup>-1</sup>) of the `Quad`.
    /// 
    /// The domain and range of this function are both (-∞, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1.5).asinh();
    /// let expected = qd!("1.194763217287109304111930828519090523536162075153005429270680299");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn asinh(self) -> Quad {
        if self.is_infinite() {
            if self.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::NEG_INFINITY
            }
        } else {
            (self + (self.sqr() + Quad::ONE).sqrt()).ln()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asinh() {
        assert_close!(
            qd!("1.862295743310848219888361325182620574902674184961554765612879514"),
            Quad::PI.asinh()
        );
        assert_close!(
            qd!("1.725382558852315093945097970404888756274557274672938668814211557"),
            Quad::E.asinh()
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.asinh());
    }

    #[test]
    fn infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.asinh());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.asinh());
    }

    #[test]
    fn nan() {
        assert_exact!(Quad::NAN, Quad::NAN.asinh());
    }
}
