// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the number raised to a quad-double power.
    ///
    /// This function only works for positive values of the number, as it uses a simplified
    /// logarithm-based algorithm. Full algorithms are much more difficult (see [this libm
    /// implementation][1] if you're curious) and it will take some time before there is such an
    /// implementation here.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3).powf(qd!(3.3));
    /// let expected = qd!("37.54050759852955219310186595463382927684873090166843452920390518");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn powf(self, n: Quad) -> Quad {
        // a^b = exp(b ln(a)), but since ln(a) is not defined for negative values, this works
        // ONLY FOR POSITIVE VALUES OF A (self in this case). Other solutions to powf are more
        // general but also much more complex and I am not yet ready to try one.
        (n * self.ln()).exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc() {
        assert_close!(
            qd!("24567.24805421478199532529771567617705237167216222778116359595012"),
            qd!(11.1).powf(qd!(4.2))
        );
        assert_close!(
            qd!("1.409759279075053716836003243441716711042960485535248677014414790"),
            Quad::PI.powf(qd!(0.3))
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Quad::NAN, Quad::NAN.powf(qd!(3.6)));
        assert_exact!(Quad::NAN, qd!(0).powf(qd!(3.2))); // Sigh
        assert_exact!(Quad::NAN, qd!(0).powf(qd!(0)));
        assert_exact!(Quad::NAN, qd!(-1).powf(qd!(1))); // Also sigh
    }
}
