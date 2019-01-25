// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the number times 2<sup>`n`</sup>.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced mathematical
    /// calculations (including several within this library). Therefore an implementation that is
    /// much more efficient than calculating it through multiplication and [`powi`](#method.powi) is
    /// offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(3);
    /// assert!(x.ldexp(3) == dd!(24)); // 3 * 2^3
    /// # }
    /// ```
    #[inline]
    pub fn ldexp(self, n: i32) -> Double {
        let factor = 2f64.powi(n);
        Double(self.0 * factor, self.1 * factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_exact!(dd!(48), dd!(3).ldexp(4));
        assert_close!(dd!(0.078125), dd!(5).ldexp(-6));
        assert_close!(
            dd!("4.2165742826631309245621820777801e8"),
            Double::PI.ldexp(27)
        );
        assert_close!(
            dd!("0.000020738844516441690333254146357366"),
            Double::E.ldexp(-17)
        );
    }

    #[test]
    fn special() {
        assert_exact!(Double::NAN, Double::NAN.ldexp(5));
        assert_exact!(Double::INFINITY, Double::INFINITY.ldexp(4));
        assert_exact!(Double::INFINITY, Double::INFINITY.ldexp(-4));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.ldexp(3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.ldexp(-3));
    }
}
