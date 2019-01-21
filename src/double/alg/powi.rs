// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the number raised to an integral power.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(3);
    /// assert!(x.powi(3) == dd!(27));
    /// # }
    /// ```
    pub fn powi(self, n: i32) -> Double {
        if n == 0 {
            if self.is_zero() {
                Double::NAN
            } else {
                Double::ONE
            }
        } else {
            let mut r = self.clone();
            let mut s = Double::ONE;
            let mut i = n.abs();

            if i > 1 {
                while i > 0 {
                    if i % 2 == 1 {
                        s *= r;
                    }
                    i /= 2;
                    if i > 0 {
                        r = r.sqr();
                    }
                }
            } else {
                s = r;
            }

            if n < 0 {
                s.recip()
            } else {
                s
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
            dd!("-6.2092132305915517444784571346965e-6"),
            dd!(-11).powi(-5)
        );
        assert_close!(dd!("97.409091034002437236440332688705"), Double::PI.powi(4));
    }

    #[test]
    fn edge() {
        assert_exact!(Double::NAN, Double::NAN.powi(3));
        assert_exact!(Double::ZERO, dd!(0).powi(3));
        assert_exact!(Double::NAN, dd!(0).powi(0));
    }
}
