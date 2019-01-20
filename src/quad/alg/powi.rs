// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;

impl Quad {
    /// Calculates the number raised to an integral power.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.powi(3) == qd!(27));
    /// # }
    /// ```
    pub fn powi(self, n: i32) -> Quad {
        if n == 0 {
            if self.is_zero() {
                Quad::NAN
            } else {
                Quad::ONE
            }
        } else {
            let mut r = self;
            let mut s = Quad::ONE;
            let mut k = n.abs();

            if k > 1 {
                while k > 0 {
                    if k % 2 == 1 {
                        s *= r;
                    }
                    k /= 2;
                    if k > 0 {
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
    fn quad_alg_powi() {
        assert_exact!(Quad::NAN, Quad::NAN.powi(3));
        assert_exact!(Quad::ZERO, qd!(0).powi(3));
        assert_exact!(Quad::NAN, qd!(0).powi(0));
        assert_close!(
            qd!("-6.209213230591551744478457134696462611222531992971170622970363425e-6"),
            qd!(-11).powi(-5)
        );
        assert_close!(
            qd!("97.40909103400243723644033268870511124972758567268542169146785939"),
            Quad::PI.powi(4)
        );
    }
}
