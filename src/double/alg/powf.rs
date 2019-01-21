// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the number raised to a double-double power.
    ///
    /// This function only works for positive values of the number, as it uses a simplified
    /// logarithm-based algorithm. Full algorithms are much more difficult (see [this libm
    /// implementation][1] if you're curious) and it will take some time before there is such an
    /// implementation here.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(3).powf(dd!(3.3));
    /// let expected = dd!("37.540507598529552193101865954634");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn powf(self, n: Double) -> Double {
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
            dd!("24567.24805421478199532529771567617705237"),
            dd!(11.1).powf(dd!(4.2))
        );
        assert_close!(
            dd!("1.4097592790750537168360032434417"),
            Double::PI.powf(dd!(0.3))
        );
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, Double::NAN.powf(dd!(3.6)));
        assert_exact!(Double::NAN, dd!(0).powf(dd!(3.2))); // Sigh
        assert_exact!(Double::NAN, dd!(0).powf(dd!(0)));
        assert_exact!(Double::NAN, dd!(-1).powf(dd!(1))); // Also sigh
    }
}
