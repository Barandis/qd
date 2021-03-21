// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::double::Double;

impl Double {
    /// Calculates x · 2<sup>n</sup>, where *x* is the `Double` and *n* is an integer.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced
    /// mathematical calculations (including several within this library). Therefore an
    /// implementation that is much more efficient than calculating it through
    /// multiplication and [`powi`] is offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(5);
    /// assert!(x.ldexp(3) == dd!(40)); // 5 * 2^3
    /// # }
    /// ```
    ///
    /// [`powi`]: #method.powi
    #[inline]
    pub fn ldexp(self, n: i32) -> Double {
        let factor = 2f64.powi(n);
        Double(self.0 * factor, self.1 * factor)
    }

    /// Mutiplies the `Double` by an `f64` that must be a power of 2.
    ///
    /// Like [`ldexp`], this function is not present in the `f64` API. However, it executes
    /// about half of the floating-point operations as regular multiplication between two
    /// `Double`s does, and that potential increase in efficiency makes it worth including.
    ///
    /// The caveat is that this function will work *only* when `n` is a power of two
    /// (including negative powers, so 0.5, 0.25, etc. will also work). If this function is
    /// used with a number that is *not* a power of two, **it will still return a result but
    /// that result will be wrong.** Use this function *only* when you are 100% certain that
    /// the argument will be a power of two.
    ///
    /// `mul_pwr2` is primarily useful for implementing mathematical algorithms. It is used
    /// extensively in this library.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI.mul_pwr2(0.5); // faster than Double::PI / 2
    /// let expected = dd!("1.5707963267948966192313216916398");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`ldexp`]: #methods.ldexp
    #[inline]
    pub fn mul_pwr2(self, n: f64) -> Double {
        Double(self.0 * n, self.1 * n)
    }

    /// Calculates the square of the `Double`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available
    /// when the two numbers being multiplied are the same, so it is more efficient than
    /// bare multiplication.
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
        match self.pre_sqr() {
            Some(r) => r,
            None => {
                let (p, e) = p::two_sqr(self.0);
                let (a, b) = p::renorm2(p, e + 2.0 * self.0 * self.1 + self.1 * self.1);
                Double(a, b)
            }
        }
    }

    /// Calculates the square root of the `Double`.
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
        match self.pre_sqrt() {
            Some(r) => r,
            None => {
                // Strategy: use a method developed by Alan Karp and Peter Markstein at HP
                // https://cr.yp.to/bib/1997/karp.pdf
                //
                // If x is an approximation of sqrt(a), then
                //
                //      sqrt(a) ≈ ax + (a - (ax)^2)x / 2
                //
                // The approximation is accurate to twice the accuracy of x. This can be
                // repeated an arbitrary number of times, but this method when used on
                // double-doubles only requires one iteration.
                let x = Double::from_div(1.0, self.0.sqrt());
                let ax = self * x;
                ax + (self - ax.sqr()) * x.mul_pwr2(0.5)
            }
        }
    }

    /// Calculates the *n*th root of the `Double`.
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
        match self.pre_nroot(n) {
            Some(r) => r,
            None => {
                // Strategy: the square root method is specialized for square roots, but the
                // traditional way of finding roots is using Newton's iteration for the
                // function
                //
                //      f(x) = x^(-n) - a
                //
                // to find its root a^(-1/n). The iteration is therefore
                //
                //      x' = x + x * (1 - a * x^n) / n
                //
                // This converges quadratically, which is pretty fast. We can then find
                // a^(1/n) by taking the reciprocal.

                let r = self.abs();
                // a^(-1/n) = exp(-ln(a) / n)
                let mut x = Double::from((-(r.0.ln()) / n as f64).exp());

                x += x * (Double::ONE - r * x.powi(n)) / Double(n.into(), 0.0);
                if self.is_sign_negative() {
                    x = -x;
                }
                x.recip()
            }
        }
    }

    /// Calculates the cube root of the `Double`.
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

    /// Calculates the `Double` raised to an integer power.
    ///
    /// This function correctly handles the special inputs defined in IEEE 754. In
    /// particular:
    ///
    /// * `x.powi(0)` is 1 for any `x` (including 0, `NaN`, or infinity)
    /// * `x.powi(n)` is ±∞ for `x` = ±0 and any odd negative `n`
    /// * `x.powi(n)` is +∞ for `x` = ±0 and any even negative `n`
    /// * `x.powi(n)` is ±0 for `x` = ±0 and any odd positive `n`
    /// * `x.powi(n)` is +0 for `x` = ±0 and any even positive `n`
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
        match self.pre_powi(n) {
            Some(r) => r,
            None => {
                let mut r = self;
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

    /// Calculates the `Double` raised to a `Quad` power.
    ///
    /// In general, x<sup>n</sup> is equal to *e*<sup>n ln x</sup>. This precludes raising
    /// a negative `Double` to a fractional or irrational power because *ln x* is undefined
    /// when *x* is negative. In that case, this function returns [`NAN`].
    /// 
    /// It's actually more complex than that; if the exponent can be expressed as a fraction
    /// with an odd denominator, then there is an answer (a cube root, which is defined for
    /// negative numbers, is the same as a power of 1/3). Therefore, something like
    /// `dd!(-4).powf(dd!(0.2))` should work, as 0.2 is a fraction with an odd denominator
    /// (1/5). However, it's impossible in general to tell whether a number is a fraction
    /// while using floating-point numbers, so no attempt is made to make this work. If you
    /// need a fifth root of -4, use `dd!(-4).nroot(5)`.
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
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    pub fn powf(self, n: Double) -> Double {
        match self.pre_powf(&n) {
            Some(r) => r,
            None => (n * self.ln()).exp(),
        }
    }

    /// Calculates the reciprocal of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI.recip();
    /// let expected = dd!("0.31830988618379067153776752674503");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Double {
        Double::ONE / self
    }

    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_sqr(&self) -> Option<Double> {
        if self.is_infinite() {
            Some(Double::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_sqrt(&self) -> Option<Double> {
        if self.is_zero() {
            Some(Double::ZERO)
        } else if self.is_sign_negative() {
            Some(Double::NAN)
        } else if self.is_infinite() {
            Some(Double::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_nroot(&self, n: i32) -> Option<Double> {
        if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Some(Double::ZERO)
                } else {
                    Some(Double::INFINITY)
                }
            } else if n > 0 {
                Some(Double::NEG_ZERO)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if self.is_sign_negative() && n % 2 == 0 {
            Some(Double::NAN)
        } else if n <= 0 {
            Some(Double::NAN)
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if n == 1 {
            Some(*self)
        } else if n == 2 {
            Some(self.sqrt()) // use the more specialized method in sqrt
        } else {
            None
        }
    }

    #[inline]
    fn pre_powi(&self, n: i32) -> Option<Double> {
        if n == 0 {
            Some(Double::ONE)
        } else if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Some(Double::ZERO)
                } else {
                    Some(Double::INFINITY)
                }
            } else if n > 0 {
                Some(Double::NEG_ZERO)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if self.is_infinite() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Some(Double::INFINITY)
                } else {
                    Some(Double::ZERO)
                }
            } else if n > 0 {
                Some(Double::NEG_INFINITY)
            } else {
                Some(Double::NEG_ZERO)
            }
        } else {
            None
        }
    }

    #[inline]
    fn pre_powf(&self, n: &Double) -> Option<Double> {
        if self.is_zero() {
            if n.is_zero() {
                Some(Double::NAN)
            } else if n.is_sign_positive() {
                Some(Double::ZERO)
            } else {
                Some(Double::INFINITY)
            }
        } else if n.is_infinite() {
            if *self == Double::ONE {
                Some(Double::NAN)
            } else if n.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::ZERO)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldexp() {
        assert_exact!(dd!(48), dd!(3).ldexp(4));
        assert_close!(dd!(0.078_125), dd!(5).ldexp(-6));
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
    fn ldexp_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.ldexp(2));
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.ldexp(2));
        assert_exact!(Double::ONE, Double::ONE.ldexp(0));
    }

    #[test]
    fn ldexp_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.ldexp(4));
        assert_exact!(Double::INFINITY, Double::INFINITY.ldexp(-4));
        assert_exact!(Double::INFINITY, Double::INFINITY.ldexp(0));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.ldexp(3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.ldexp(-3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.ldexp(0));
    }

    #[test]
    fn ldexp_nan() {
        assert_exact!(Double::NAN, Double::NAN.ldexp(5));
    }

    #[test]
    fn sqr() {
        assert_exact!(dd!(121), dd!(-11).sqr());
        assert_close!(dd!("9.869604401089358618834490999876"), Double::PI.sqr());
    }

    #[test]
    fn sqr_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sqr());
        assert_exact!(Double::ZERO, Double::NEG_ZERO.sqr());
    }

    #[test]
    fn sqr_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sqr());
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.sqr());
    }

    #[test]
    fn sqr_nan() {
        assert_exact!(Double::NAN, Double::NAN.sqr());
    }

    #[test]
    fn sqrt() {
        assert_close!(dd!("1.7724538509055160272981674833411"), Double::PI.sqrt());
        assert_close!(dd!("48.135226186234961951944911890074"), dd!(2317).sqrt());
    }

    #[test]
    fn sqrt_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sqrt());
    }

    #[test]
    fn sqrt_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sqrt());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.sqrt());
    }

    #[test]
    fn sqrt_nan() {
        assert_exact!(Double::NAN, Double::NAN.sqrt());
        assert_exact!(Double::NAN, dd!(-3).sqrt());
    }

    #[test]
    fn nroot() {
        assert_close!(Double::PI, Double::PI.nroot(1));
        assert_close!(
            dd!("1.77245385090551602729816748334115"),
            Double::PI.nroot(2)
        );
        assert_close!(
            dd!("1.28402541668774148407342056806244"),
            Double::E.nroot(4)
        );
    }

    #[test]
    fn nroot_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.nroot(4));
        assert_exact!(Double::ZERO, Double::NEG_ZERO.nroot(4));
        assert_exact!(Double::ZERO, Double::ZERO.nroot(5));
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.nroot(4));
        assert_exact!(Double::INFINITY, Double::ZERO.nroot(-2));
        assert_exact!(Double::INFINITY, Double::NEG_ZERO.nroot(-2));
        assert_exact!(Double::INFINITY, Double::ZERO.nroot(-3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ZERO.nroot(-3));
    }

    #[test]
    fn nroot_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.nroot(4));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.nroot(4));
        assert_exact!(Double::INFINITY, Double::INFINITY.nroot(3));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.nroot(3));
    }

    #[test]
    fn nroot_nan() {
        assert_exact!(Double::NAN, dd!(2).nroot(-2));
        assert_exact!(Double::NAN, Double::NAN.nroot(3));
    }

    #[test]
    fn nroot_neg() {
        assert_close!(dd!("-1.4645918875615232630201425272638"), (-Double::PI).nroot(3));
        assert_exact!(Double::NAN, (-Double::PI).nroot(4));
    }

    #[test]
    fn cbrt() {
        assert_close!(dd!("1.4645918875615232630201425272638"), Double::PI.cbrt());
        assert_close!(
            dd!("-1.3956124250860895286281253196026"),
            (-Double::E).cbrt()
        );
    }

    #[test]
    fn cbrt_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.cbrt());
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.cbrt());
    }

    #[test]
    fn cbrt_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.cbrt());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.cbrt());
    }

    #[test]
    fn cbrt_nan() {
        assert_exact!(Double::NAN, Double::NAN.cbrt());
    }

    #[test]
    fn powi() {
        assert_close!(
            dd!("-6.2092132305915517444784571346965e-6"),
            dd!(-11).powi(-5)
        );
        assert_close!(dd!("97.409091034002437236440332688705"), Double::PI.powi(4));
    }

    #[test]
    fn powi_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.powi(3));
        assert_exact!(Double::NEG_ZERO, Double::NEG_ZERO.powi(3));
        assert_exact!(Double::ZERO, Double::ZERO.powi(4));
        assert_exact!(Double::ZERO, Double::NEG_ZERO.powi(4));
        assert_exact!(Double::INFINITY, Double::ZERO.powi(-1));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ZERO.powi(-1));
        assert_exact!(Double::INFINITY, Double::ZERO.powi(-2));
        assert_exact!(Double::INFINITY, Double::NEG_ZERO.powi(-2));
    }

    #[test]
    fn powi_zero_exp() {
        assert_exact!(Double::ONE, Double::ZERO.powi(0));
        assert_exact!(Double::ONE, Double::ONE.powi(0));
        assert_exact!(Double::ONE, dd!(2317).powi(0));
        assert_exact!(Double::ONE, Double::INFINITY.powi(0));
        assert_exact!(Double::ONE, Double::NEG_INFINITY.powi(0));
        assert_exact!(Double::ONE, Double::NAN.powi(0));
    }

    #[test]
    fn powi_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.powi(2));
        assert_exact!(Double::INFINITY, Double::INFINITY.powi(3));
        assert_exact!(Double::ZERO, Double::INFINITY.powi(-2));
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.powi(2));
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.powi(3));
        assert_exact!(Double::NEG_ZERO, Double::NEG_INFINITY.powi(-3));
    }

    #[test]
    fn powi_nan() {
        assert_exact!(Double::NAN, Double::NAN.powi(2));
    }

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
    fn powf_zero() {
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
    fn powf_zero_exp() {
        assert_exact!(Double::ONE, dd!(2).powf(Double::ZERO));
        assert_exact!(Double::ONE, dd!(2).powf(Double::NEG_ZERO));
        assert_exact!(Double::NAN, Double::ZERO.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::NEG_ZERO.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::ZERO.powf(Double::NEG_ZERO));
        assert_exact!(Double::NAN, Double::NEG_ZERO.powf(Double::NEG_ZERO));
    }

    #[test]
    fn powf_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::INFINITY.powf(Double::NEG_ZERO));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.powf(Double::ZERO));
        assert_exact!(Double::NAN, Double::NEG_INFINITY.powf(Double::NEG_ZERO));
    }

    #[test]
    fn powf_inf_exp() {
        assert_exact!(Double::INFINITY, dd!(2).powf(Double::INFINITY));
        assert_exact!(Double::ZERO, dd!(2).powf(Double::NEG_INFINITY));
        assert_exact!(Double::NAN, dd!(1).powf(Double::INFINITY));
        assert_exact!(Double::NAN, dd!(1).powf(Double::NEG_INFINITY));
    }

    #[test]
    fn powf_nan() {
        assert_exact!(Double::NAN, Double::NAN.powf(dd!(3)));
        assert_exact!(Double::NAN, dd!(3).powf(Double::NAN));
        assert_exact!(Double::NAN, dd!(-1).powf(dd!(1)));
    }

    #[test]
    fn recip() {
        assert_close!(
            dd!("0.31830988618379067153776752674503"),
            Double::PI.recip()
        );
        assert_close!(dd!("0.36787944117144232159552377016146"), Double::E.recip());
    }

    #[test]
    fn recip_zero() {
        assert_exact!(Double::INFINITY, Double::ZERO.recip());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_ZERO.recip());
    }

    #[test]
    fn recip_inf() {
        assert_exact!(Double::ZERO, Double::INFINITY.recip());
        assert_exact!(Double::NEG_ZERO, Double::NEG_INFINITY.recip());
    }

    #[test]
    fn recip_nan() {
        assert_exact!(Double::NAN, Double::NAN.recip());
    }
}
