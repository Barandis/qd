// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::quad::Quad;

impl Quad {
    /// Calculates x · 2<sup>n</sup>, where *x* is the `Quad` and *n* is an integer.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced
    /// mathematical calculations (including several within this library). Therefore an
    /// implementation that is much more efficient than calculating it through
    /// multiplication and [`powi`] is offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.ldexp(3) == qd!(24)); // 3 * 2^3
    /// # }
    /// ```
    ///
    /// [`powi`]: #method.powi
    #[inline]
    pub fn ldexp(self, n: i32) -> Quad {
        let factor = 2f64.powi(n);
        Quad(
            self.0 * factor,
            self.1 * factor,
            self.2 * factor,
            self.3 * factor,
        )
    }

    /// Mutiplies the `Quad` by an `f64` that must be a power of 2.
    ///
    /// Like [`ldexp`], this function is not present in the `f64` API. However, it is *much*
    /// faster than regular multiplication for its very particular use. A regular
    /// multiplication between two `Quad`s takes around 150 floating-point operations
    /// (depending on the numbers); this function takes 4.
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
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI.mul_pwr2(0.5); // MUCH faster than Quad::PI / 2
    /// let expected = qd!("1.570796326794896619231321691639751442098584699687552910487472296");
    /// 
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    /// 
    /// [`ldexp`]: #methods.ldexp
    #[inline]
    pub fn mul_pwr2(self, n: f64) -> Quad {
        Quad(self.0 * n, self.1 * n, self.2 * n, self.3 * n)
    }

    /// Calculates the square of the `Quad`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available
    /// when the two numbers being multiplied are the same, so it is more efficient than
    /// bare multiplication.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.sqr() == x * x); // The left side is faster though
    /// # }
    /// ```
    #[inline]
    pub fn sqr(self) -> Quad {
        if self.is_infinite() {
            Quad::INFINITY
        } else {
            // A considerable simplification over simply multiplying the number by itself,
            // with the simplifications possible because the two numbers being multiplied
            // are in fact equal.
            //
            // The result is a simpler calculation:
            //
            //      a0² + 2a0a1 + 2a0a2 + a1² + 2a0a3 + 2a1a2
            //
            // where any further terms, including the low words of the final two terms, are
            // unnecessary to achieve the desired accuracy.

            let (h0, l0) = core::two_sqr(self.0);
            let (h1, l1) = core::two_prod(2.0 * self.0, self.1);
            let (h2, l2) = core::two_prod(2.0 * self.0, self.2);
            let (h3, l3) = core::two_sqr(self.1);
            let h4 = 2.0 * self.0 * self.3;
            let h5 = 2.0 * self.1 * self.2;

            // Less primitive functions are not used here because there are steps in them
            // that can be skipped.

            let r0 = h0;

            let (r1, a1) = core::two_sum(h1, l0);

            let (b0, b1) = core::two_sum(a1, l1);
            let (c0, c1) = core::two_sum(h2, h3);
            let (d0, d1) = core::two_sum(b0, c0);
            let (e0, e1) = core::two_sum(b1, c1);
            let (f0, f1) = core::two_sum(d1, e0);
            let (i0, i1) = core::quick_two_sum(f0, e1 + f1);
            let (r2, j1) = core::quick_two_sum(d0, i0);

            let (k0, k1) = core::quick_two_sum(i1, j1);
            let (m0, m1) = core::two_sum(h4, h5);
            let (n0, n1) = core::two_sum(l2, l3);
            let (o0, o1) = core::two_sum(m0, n0);
            let (r3, q1) = core::two_sum(k0, o0);

            let r4 = m1 + n1 + o1 + k1 + q1;

            let (a, b, c, d) = core::renorm5(r0, r1, r2, r3, r4);
            Quad(a, b, c, d)
        }
    }

    /// Calculates the square root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).sqrt();
    /// let expected = qd!("1.414213562373095048801688724209698078569671875376948073176679738");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sqrt(self) -> Quad {
        if self.is_zero() {
            Quad::ZERO
        } else if self.is_sign_negative() {
            Quad::NAN
        } else if self.is_infinite() {
            Quad::INFINITY
        } else {
            // Strategy: use Newton's iteration.
            //
            // Perform the following Newton iteration
            //
            //      x' = x + (1 - ax²) * x / 2
            //
            // which converges to 1/√a, starting with a Quad-precision approximation of
            // 1/√a. Newton's iteration more or less quadruples the precision with each
            // pass, so performing it three times should be enough.

            let mut r = Quad::ONE / Quad::from(self.0.sqrt());
            let h = self.mul_pwr2(0.5);
            let k = Quad(0.5, 0.0, 0.0, 0.0);

            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;
            r += (k - h * r.sqr()) * r;

            r *= self;
            r
        }
    }

    /// Calculates the *n*th root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).nroot(4);
    /// let expected = qd!("1.189207115002721066717499970560475915292972092463817413019002225");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn nroot(self, n: i32) -> Quad {
        if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Quad::ZERO
                } else {
                    Quad::INFINITY
                }
            } else if n > 0 {
                Quad::NEG_ZERO
            } else {
                Quad::NEG_INFINITY
            }
        } else if n <= 0 {
            Quad::NAN
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Quad::INFINITY
            } else if n % 2 == 0 {
                Quad::NAN
            } else {
                Quad::NEG_INFINITY
            }
        } else if n == 1 {
            self
        } else if n == 2 {
            self.sqrt() // use the more specialized method in sqrt
        } else {
            // Strategy: the traditional way of finding roots is using Newton's iteration
            // for the function
            //
            //      f(x) = x^(-n) - a
            //
            // to find its root a^(-1/n). The iteration is therefore
            //
            //      x' = x + x * (1 - a * x^n) / n
            //
            // This converges quadratically, which is pretty fast. After performing a small
            // number of iterations, we can then find a^(1/n) by taking the reciprocal.

            let r = self.abs();
            // a^(-1/n) = exp(-ln(a) / n)
            let mut x: Quad = (-(r.0.ln()) / n as f64).exp().into();

            let qd_n = Quad(n.into(), 0.0, 0.0, 0.0);
            x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
            x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
            x += x * (Quad::ONE - r * x.powi(n)) / qd_n;
            if self.0 < 0.0 {
                x = -x
            }
            x.recip()
        }
    }

    /// Calculates the cube root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2).cbrt();
    /// let expected = qd!("1.259921049894873164767210607278228350570251464701507980081975112");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn cbrt(self) -> Quad {
        self.nroot(3)
    }

    /// Calculates the `Quad` raised to an integer power.
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
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3);
    /// assert!(x.powi(3) == qd!(27));
    /// # }
    /// ```
    pub fn powi(self, n: i32) -> Quad {
        if n == 0 {
            Quad::ONE
        } else if self.is_nan() {
            Quad::NAN
        } else if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Quad::ZERO
                } else {
                    Quad::INFINITY
                }
            } else if n > 0 {
                Quad::NEG_ZERO
            } else {
                Quad::NEG_INFINITY
            }
        } else if self.is_infinite() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Quad::INFINITY
                } else {
                    Quad::ZERO
                }
            } else if n > 0 {
                Quad::NEG_INFINITY
            } else {
                Quad::NEG_ZERO
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

    /// Calculates the `Quad` raised to a `Quad` power.
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
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(3).powf(qd!(3.3));
    /// let expected = qd!("37.54050759852955219310186595463382927684873090166843452920390518");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [1]: http://www.netlib.org/fdlibm/e_pow.c
    #[inline]
    pub fn powf(self, n: Quad) -> Quad {
        if self.is_zero() {
            if n.is_zero() {
                Quad::NAN
            } else if n.is_sign_positive() {
                Quad::ZERO
            } else {
                Quad::INFINITY
            }
        } else if n.is_infinite() {
            if self == Quad::ONE {
                Quad::NAN
            } else if n.is_sign_positive() {
                Quad::INFINITY
            } else {
                Quad::ZERO
            }
        } else {
            (n * self.ln()).exp()
        }
    }

    /// Calculates the reciprocal of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::PI.recip();
    /// let expected = qd!("0.3183098861837906715377675267450287240689192914809128974953346881");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Quad {
        Quad::ONE / self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldexp() {
        assert_exact!(qd!(48), qd!(3).ldexp(4));
        assert_close!(qd!(0.078_125), qd!(5).ldexp(-6));
        assert_close!(
            qd!("4.216574282663130924562182077780080660863911808152513230508318081e8"),
            Quad::PI.ldexp(27)
        );
        assert_close!(
            qd!("0.00002073884451644169033325414635736589430051610636672942790959905722"),
            Quad::E.ldexp(-17)
        );
    }

    #[test]
    fn ldexp_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.ldexp(2));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.ldexp(2));
        assert_exact!(Quad::ONE, Quad::ONE.ldexp(0));
    }

    #[test]
    fn ldexp_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.ldexp(4));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.ldexp(-4));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.ldexp(0));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.ldexp(3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.ldexp(-3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.ldexp(0));
    }

    #[test]
    fn ldexp_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.ldexp(5));
    }

    #[test]
    fn sqr() {
        assert_exact!(qd!(121), qd!(-11).sqr());
        assert_close!(
            qd!("9.869604401089358618834490999876151135313699407240790626413349376"),
            Quad::PI.sqr()
        );
    }

    #[test]
    fn sqr_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sqr());
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.sqr());
    }

    #[test]
    fn sqr_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sqr());
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.sqr());
    }

    #[test]
    fn sqr_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sqr());
    }

    #[test]
    fn sqrt() {
        assert_close!(
            qd!("1.772453850905516027298167483341145182797549456122387128213807790"),
            Quad::PI.sqrt()
        );
        assert_close!(
            qd!("48.13522618623496195194491189007433987957200800774184036920112360"),
            qd!(2317).sqrt()
        );
    }

    #[test]
    fn sqrt_neg() {
        assert_exact!(Quad::NAN, qd!(-3).sqrt());
    }

    #[test]
    fn sqrt_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sqrt());
    }

    #[test]
    fn sqrt_infinity() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sqrt());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.sqrt());
    }

    #[test]
    fn sqrt_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sqrt());
    }

    #[test]
    fn nroot() {
        assert_close!(Quad::PI, Quad::PI.nroot(1));
        assert_close!(
            qd!("1.772453850905516027298167483341145182797549456122387128213807790"),
            Quad::PI.nroot(2)
        );
        assert_close!(
            qd!("1.284025416687741484073420568062436458336280865281463089217507297"),
            Quad::E.nroot(4)
        );
    }

    #[test]
    fn nroot_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.nroot(4));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.nroot(4));
        assert_exact!(Quad::ZERO, Quad::ZERO.nroot(5));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.nroot(4));
        assert_exact!(Quad::INFINITY, Quad::ZERO.nroot(-2));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.nroot(-2));
        assert_exact!(Quad::INFINITY, Quad::ZERO.nroot(-3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ZERO.nroot(-3));
    }

    #[test]
    fn nroot_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.nroot(4));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.nroot(4));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.nroot(3));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.nroot(3));
    }

    #[test]
    fn nroot_nan() {
        assert_exact!(Quad::NAN, qd!(2).nroot(-2));
        assert_exact!(Quad::NAN, Quad::NAN.nroot(3));
    }

    #[test]
    fn cbrt() {
        assert_close!(
            qd!("1.464591887561523263020142527263790391738596855627937174357255937"),
            Quad::PI.cbrt()
        );
        assert_close!(
            qd!("-1.395612425086089528628125319602586837597906515199406982617516706"),
            (-Quad::E).cbrt()
        );
    }

    #[test]
    fn cbrt_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.cbrt());
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.cbrt());
    }

    #[test]
    fn cbrt_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.cbrt());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.cbrt());
    }

    #[test]
    fn cbrt_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.cbrt());
    }

    #[test]
    fn powi() {
        assert_close!(
            qd!("-6.209213230591551744478457134696462611222531992971170622970363425e-6"),
            qd!(-11).powi(-5)
        );
        assert_close!(
            qd!("97.40909103400243723644033268870511124972758567268542169146785939"),
            Quad::PI.powi(4)
        );
    }

    #[test]
    fn powi_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.powi(3));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_ZERO.powi(3));
        assert_exact!(Quad::ZERO, Quad::ZERO.powi(4));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.powi(4));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powi(-1));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ZERO.powi(-1));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powi(-2));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.powi(-2));
    }

    #[test]
    fn powi_zero_exp() {
        assert_exact!(Quad::ONE, Quad::ZERO.powi(0));
        assert_exact!(Quad::ONE, Quad::ONE.powi(0));
        assert_exact!(Quad::ONE, qd!(2317).powi(0));
        assert_exact!(Quad::ONE, Quad::INFINITY.powi(0));
        assert_exact!(Quad::ONE, Quad::NEG_INFINITY.powi(0));
        assert_exact!(Quad::ONE, Quad::NAN.powi(0));
    }

    #[test]
    fn powi_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.powi(2));
        assert_exact!(Quad::INFINITY, Quad::INFINITY.powi(3));
        assert_exact!(Quad::ZERO, Quad::INFINITY.powi(-2));
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.powi(2));
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.powi(3));
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_INFINITY.powi(-3));
    }

    #[test]
    fn powi_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.powi(2));
    }

    #[test]
    fn powf() {
        assert_close!(
            qd!("24567.24805421478199532529771567617705237167216222778116359595012"),
            qd!(11.1).powf(qd!(4.2))
        );
        assert_close!(
            qd!("1.409759279075053716836003243441716711042960485535248677014414790"),
            Quad::PI.powf(qd!(0.3))
        );
        assert_close!(
            qd!("0.006810719380166276826846127381721218763394637801309025289387144601"),
            qd!(0.2).powf(qd!(3.1))
        );
        assert_close!(
            qd!("146.8273678860023757393079582114873627092153773446718337101982774"),
            qd!(0.2).powf(qd!(-3.1))
        );
    }

    #[test]
    fn powf_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.powf(qd!(3)));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.powf(qd!(3)));
        assert_exact!(Quad::ZERO, Quad::ZERO.powf(Quad::INFINITY));
        assert_exact!(Quad::ZERO, Quad::NEG_ZERO.powf(Quad::INFINITY));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powf(qd!(-2)));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.powf(qd!(-2)));
        assert_exact!(Quad::INFINITY, Quad::ZERO.powf(Quad::NEG_INFINITY));
        assert_exact!(Quad::INFINITY, Quad::NEG_ZERO.powf(Quad::NEG_INFINITY));
    }

    #[test]
    fn powf_zero_exp() {
        assert_exact!(Quad::ONE, qd!(2).powf(Quad::ZERO));
        assert_exact!(Quad::ONE, qd!(2).powf(Quad::NEG_ZERO));
        assert_exact!(Quad::NAN, Quad::ZERO.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::ZERO.powf(Quad::NEG_ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_ZERO.powf(Quad::NEG_ZERO));
    }

    #[test]
    fn powf_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::INFINITY.powf(Quad::NEG_ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.powf(Quad::ZERO));
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.powf(Quad::NEG_ZERO));
    }

    #[test]
    fn powf_inf_exp() {
        assert_exact!(Quad::INFINITY, qd!(2).powf(Quad::INFINITY));
        assert_exact!(Quad::ZERO, qd!(2).powf(Quad::NEG_INFINITY));
        assert_exact!(Quad::NAN, qd!(1).powf(Quad::INFINITY));
        assert_exact!(Quad::NAN, qd!(1).powf(Quad::NEG_INFINITY));
    }

    #[test]
    fn powf_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.powf(qd!(3)));
        assert_exact!(Quad::NAN, qd!(3).powf(Quad::NAN));
        assert_exact!(Quad::NAN, qd!(-1).powf(qd!(1)));
    }

    #[test]
    fn recip() {
        assert_close!(
            qd!("0.3183098861837906715377675267450287240689192914809128974953346881"),
            Quad::PI.recip()
        );
        assert_close!(
            qd!("0.3678794411714423215955237701614608674458111310317678345078368017"),
            Quad::E.recip()
        );
    }

    #[test]
    fn recip_zero() {
        assert_exact!(Quad::INFINITY, Quad::ZERO.recip());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_ZERO.recip());
    }

    #[test]
    fn recip_inf() {
        assert_exact!(Quad::ZERO, Quad::INFINITY.recip());
        assert_exact!(Quad::NEG_ZERO, Quad::NEG_INFINITY.recip());
    }

    #[test]
    fn recip_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.recip());
    }
}
