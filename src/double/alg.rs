// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::*;
use crate::double::common::mul_pwr2;
use crate::double::Double;
use std::f64;

// #region Powers

impl Double {
    /// Calculates the square of the number.
    ///
    /// This method takes advantage of optimizations in multiplication that are available when the
    /// two numbers being multiplied are the same, so it is more efficient than bare multiplication.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(3);
    /// assert!(x.sqr() == x * x); // The left side is faster though
    /// # }
    /// ```
    #[inline]
    pub fn sqr(self) -> Double {
        let (p, e) = two_sqr(self.0);
        Double::from(quick_two_sum(
            p,
            e + 2.0 * self.0 * self.1 + self.1 * self.1,
        ))
    }

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
            let mut s = Double::from(1.0);
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

// #endregion

// #region Roots

impl Double {
    /// Calculates the square root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).sqrt();
    /// let diff = (x - Double::SQRT_2).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn sqrt(self) -> Double {
        if self.is_zero() {
            Double::ZERO
        } else if self.is_sign_negative() {
            Double::NAN
        } else {
            // Strategy: use a method developed by Alan Karp and Peter Markstein at HP
            // https://cr.yp.to/bib/1997/karp.pdf
            //
            // If x is an approximation of sqrt(a), then
            //
            //      sqrt(a) ≈ ax + (a - (ax)^2)x / 2
            //
            // The approximation is accurate to twice the accuracy of x. This can be repeated an
            // arbitrary number of times, but this method when used on double-doubles only requires
            // one iteration.
            let x = Double::from_div(1.0, self.0.sqrt());
            let ax = self * x;
            ax + (self - ax.sqr()) * mul_pwr2(x, 0.5)
        }
    }

    /// Calculates the cube root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).cbrt();
    /// let expected = dd!("1.2599210498948731647672106072782");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn cbrt(self) -> Double {
        self.nroot(3)
    }

    /// Calculates the `n`th root of the number.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(2).nroot(4);
    /// let expected = dd!("1.1892071150027210667174999705605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn nroot(self, n: i32) -> Double {
        if n <= 0 {
            return Double::NAN;
        }
        if n % 2 == 0 && self.is_sign_negative() {
            return Double::NAN;
        }
        if n == 1 {
            return self;
        }
        if n == 2 {
            return self.sqrt(); // use the more specialized method in sqrt
        }
        if self.is_zero() {
            return Double::ZERO;
        }

        // Strategy: the square root method is specialized for square roots, but the traditional
        // way of finding roots is using Newton's iteration for the function
        //
        //      f(x) = x^(-n) - a
        //
        // to find its root a^(-1/n). The iteration is therefore
        //
        //      x' = x + x * (1 - a * x^n) / n
        //
        // This converges quadratically, which is pretty fast. We can then find a^(1/n) by taking
        // the reciprocal.

        let r = self.abs();
        let mut x: Double = (-(r.0.ln()) / n as f64).exp().into(); // a^(-1/n) = exp(-ln(a) / n)

        x += x * (Double::ONE - r * x.powi(n)) / Double::from(n);
        if self.is_sign_negative() {
            x = -x;
        }
        x.recip()
    }
}

// #endregion

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_alg_sqr() {
        assert_exact!(Double::NAN, Double::NAN.sqr());
        assert_exact!(Double::ZERO, dd!(0).sqr());
        assert_exact!(dd!(121), dd!(-11).sqr());
        assert_close!(dd!("9.869604401089358618834490999876"), Double::PI.sqr());
    }

    #[test]
    fn double_alg_powi() {
        assert_exact!(Double::NAN, Double::NAN.powi(3));
        assert_exact!(Double::ZERO, dd!(0).powi(3));
        assert_exact!(Double::NAN, dd!(0).powi(0));
        assert_close!(
            dd!("-6.2092132305915517444784571346965e-6"),
            dd!(-11).powi(-5)
        );
        assert_close!(dd!("97.409091034002437236440332688705"), Double::PI.powi(4));
    }

    #[test]
    fn double_alg_powf() {
        assert_exact!(Double::NAN, Double::NAN.powf(dd!(3.6)));
        assert_exact!(Double::NAN, dd!(0).powf(dd!(3.2))); // Sigh
        assert_exact!(Double::NAN, dd!(0).powf(dd!(0)));
        assert_exact!(Double::NAN, dd!(-1).powf(dd!(1))); // Also sigh
        assert_close!(
            dd!("24567.24805421478199532529771567617705237"),
            dd!(11.1).powf(dd!(4.2))
        );
        assert_close!(
            dd!("1.4097592790750537168360032434417"),
            Double::PI.powf(dd!(0.3))
        );
    }
}
