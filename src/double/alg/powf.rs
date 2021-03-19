// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the number raised to a double-double power.
    ///
    /// This function is implemented using the logarithm of the number being raised, which
    /// means it will not work for negatives even though raising a negative number to a
    /// non-integer power is defined. It has been extended to handle zero in accordance with
    /// the IEEE 754 specification.
    ///
    /// It is possible that a new algorithm will eventually remove this restriction, though
    /// this is a surprisingly hard problem (see [this libm implementation][1], for
    /// example).
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
    ///
    /// [1]: http://www.netlib.org/fdlibm/e_pow.c
    #[inline]
    pub fn powf(self, n: Double) -> Double {
        if self.is_zero() {
            if n.is_zero() {
                Double::NAN
            } else if n.is_sign_positive() {
                Double::ZERO
            } else {
                Double::INFINITY
            }
        } else if n.is_infinite() {
            if self == Double::ONE {
                Double::NAN
            } else if n.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::ZERO
            }
        } else {
            (n * self.ln()).exp()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powf() {
        assert_close!(
            dd!("24567.24805421478199532529771567617705237"),
            dd!(11.1).powf(dd!(4.2))
        );
        assert_close!(
            dd!("1.4097592790750537168360032434417"),
            Double::PI.powf(dd!(0.3))
        );
        assert_close!(
            dd!("0.0068107193801662768268461273817212"),
            dd!(0.2).powf(dd!(3.1))
        );
        assert_close!(
            dd!("146.82736788600237573930795821149"),
            dd!(0.2).powf(dd!(-3.1))
        );
    }

    #[test]
    fn zero() {
        assert_exact!(Double::ZERO, Double::ZERO.powf(dd!(3)));
        assert_exact!(Double::ZERO, Double::NEG_ZERO.powf(dd!(3)));
        assert_exact!(Double::ZERO, Double::ZERO.powf(Double::INFINITY));
        assert_exact!(Double::ZERO, Double::NEG_ZERO.powf(Double::INFINITY));
        assert_exact!(Double::INFINITY, Double::ZERO.powf(dd!(-2)));
        assert_exact!(Double::INFINITY, Double::NEG_ZERO.powf(dd!(-2)));
        assert_exact!(Double::INFINITY, Double::ZERO.powf(Double::NEG_INFINITY));
        assert_exact!(
            Double::INFINITY,
            Double::NEG_ZERO.powf(Double::NEG_INFINITY)
        );
    }

    #[test]
    fn zero_exponent() {
        assert_exact!(Double::ONE, dd!(2).powf(Double::ZERO));
        assert_exact!(Double::ONE, dd!(2).powf(Double::NEG_ZERO));
        assert_exact!(Double::NAN, Double::ZERO.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::NEG_ZERO.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::ZERO.powf(Double::NEG_ZERO));
        assert_exact!(Double::NAN, Double::NEG_ZERO.powf(Double::NEG_ZERO));
    }

    #[test]
    fn infinity() {
        assert_exact!(Double::NAN, Double::INFINITY.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::INFINITY.powf(Double::NEG_ZERO));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.powf(Double::NEG_ZERO));
    }

    #[test]
    fn infinite_exponent() {
        assert_exact!(Double::INFINITY, dd!(2).powf(Double::INFINITY));
        assert_exact!(Double::ZERO, dd!(2).powf(Double::NEG_INFINITY));
        assert_exact!(Double::NAN, dd!(1).powf(Double::INFINITY));
        assert_exact!(Double::NAN, dd!(1).powf(Double::NEG_INFINITY));
    }

    #[test]
    fn nan() {
        assert_exact!(Double::NAN, Double::NAN.powf(dd!(3)));
        assert_exact!(Double::NAN, dd!(3).powf(Double::NAN));
    }

    #[test]
    fn negative() {
        assert_exact!(Double::NAN, dd!(-1).powf(dd!(1)));
    }
}
