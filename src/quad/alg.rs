// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::quad::common as c;
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
    /// # use qd::qd;
    /// let x = qd!(3);
    /// assert!(x.ldexp(5) == qd!(96)); // 3 * 2^5
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

    /// Calculates the square of the `Quad`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available
    /// when the two numbers being multiplied are the same, so it is more efficient than
    /// bare multiplication.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(3);
    /// assert!(x.sqr() == x * x); // The left side is faster though
    /// ```
    #[inline]
    pub fn sqr(self) -> Quad {
        match self.pre_sqr() {
            Some(r) => r,
            None => {
                // A considerable simplification over simply multiplying the number by
                // itself, with the simplifications possible because the two numbers being
                // multiplied are in fact equal.
                //
                // The result is a simpler calculation:
                //
                //      a0² + 2a0a1 + 2a0a2 + a1² + 2a0a3 + 2a1a2
                //
                // where any further terms, including the low words of the final two terms,
                // are unnecessary to achieve the desired accuracy.

                let (h0, l0) = p::two_sqr(self.0);
                let (h1, l1) = p::two_prod(2.0 * self.0, self.1);
                let (h2, l2) = p::two_prod(2.0 * self.0, self.2);
                let (h3, l3) = p::two_sqr(self.1);
                let h4 = 2.0 * self.0 * self.3;
                let h5 = 2.0 * self.1 * self.2;

                // Less primitive functions are not used here because there are steps in
                // them that can be skipped.

                let r0 = h0;

                let (r1, a1) = p::two_sum(h1, l0);

                let (b0, b1) = p::two_sum(a1, l1);
                let (c0, c1) = p::two_sum(h2, h3);
                let (d0, d1) = p::two_sum(b0, c0);
                let (e0, e1) = p::two_sum(b1, c1);
                let (f0, f1) = p::two_sum(d1, e0);
                let (i0, i1) = p::quick_two_sum(f0, e1 + f1);
                let (r2, j1) = p::quick_two_sum(d0, i0);

                let (k0, k1) = p::quick_two_sum(i1, j1);
                let (m0, m1) = p::two_sum(h4, h5);
                let (n0, n1) = p::two_sum(l2, l3);
                let (o0, o1) = p::two_sum(m0, n0);
                let (r3, q1) = p::two_sum(k0, o0);

                let r4 = m1 + n1 + o1 + k1 + q1;

                let (a, b, c, d) = u::renorm5(r0, r1, r2, r3, r4);
                Quad(a, b, c, d)
            }
        }
    }

    /// Calculates the square root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(2).sqrt();
    /// let expected = qd!("1.414213562373095048801688724209698078569671875376948073176679738");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    pub fn sqrt(self) -> Quad {
        match self.pre_sqrt() {
            Some(r) => r,
            None => {
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
                let h = c::mul_pwr2(self, 0.5);
                let k = Quad(0.5, 0.0, 0.0, 0.0);

                r += (k - h * r.sqr()) * r;
                r += (k - h * r.sqr()) * r;
                r += (k - h * r.sqr()) * r;

                r *= self;
                r
            }
        }
    }

    /// Calculates the *n*th root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(2).nroot(4);
    /// let expected = qd!("1.189207115002721066717499970560475915292972092463817413019002225");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    pub fn nroot(self, n: i32) -> Quad {
        match self.pre_nroot(n) {
            Some(r) => r,
            None => {
                // Strategy: the traditional way of finding roots is using Newton's
                // iteration for the function
                //
                //      f(x) = x^(-n) - a
                //
                // to find its root a^(-1/n). The iteration is therefore
                //
                //      x' = x + x * (1 - a * x^n) / n
                //
                // This converges quadratically, which is pretty fast. After performing a
                // small number of iterations, we can then find a^(1/n) by taking the
                // reciprocal.

                let r = self.abs();
                // a^(-1/n) = exp(-ln(a) / n)
                let mut x = Quad::from((-(r.0.ln()) / n as f64).exp());

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
    }

    /// Calculates the cube root of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(2).cbrt();
    /// let expected = qd!("1.259921049894873164767210607278228350570251464701507980081975112");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
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
    /// # use qd::qd;
    /// let x = qd!(3);
    /// assert!(x.powi(3) == qd!(27));
    /// ```
    pub fn powi(self, n: i32) -> Quad {
        match self.pre_powi(n) {
            Some(r) => r,
            None => {
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

    /// Calculates the `Quad` raised to a `Quad` power.
    ///
    /// In general, x<sup>n</sup> is equal to *e*<sup>n ln x</sup>. This precludes raising
    /// a negative `Quad` to a fractional or irrational power because *ln x* is undefined
    /// when *x* is negative. In that case, this function returns [`NAN`].
    ///
    /// It's actually more complex than that; if the exponent can be expressed as a fraction
    /// with an odd denominator, then there is an answer (a cube root, which is defined for
    /// negative numbers, is the same as a power of 1/3). Therefore, something like
    /// `qd!(-4).powf(qd!(0.2))` should work, as 0.2 is a fraction with an odd denominator
    /// (1/5). However, it's impossible in general to tell whether a number is a fraction
    /// while using floating-point numbers, so no attempt is made to make this work. If you
    /// need a fifth root of -4, use `qd!(-4).nroot(5)`.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(3).powf(qd!(3.3));
    /// let expected = qd!("37.54050759852955219310186595463382927684873090166843452920390518");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    pub fn powf(self, n: Quad) -> Quad {
        match self.pre_powf(&n) {
            Some(r) => r,
            None => (n * self.ln()).exp(),
        }
    }

    /// Calculates the reciprocal of the `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::PI.recip();
    /// let expected = qd!("0.3183098861837906715377675267450287240689192914809128974953346881");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    #[inline]
    pub fn recip(self) -> Quad {
        Quad::ONE / self
    }

    // PRecalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_sqr(&self) -> Option<Quad> {
        if self.is_infinite() {
            Some(Quad::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_sqrt(&self) -> Option<Quad> {
        if self.is_zero() {
            Some(Quad::ZERO)
        } else if self.is_sign_negative() {
            Some(Quad::NAN)
        } else if self.is_infinite() {
            Some(Quad::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_nroot(&self, n: i32) -> Option<Quad> {
        if n == 0 {
            Some(Quad::NAN)
        } else if n == 1 {
            Some(*self)
        } else if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Some(Quad::ZERO)
                } else {
                    Some(Quad::INFINITY)
                }
            } else if n > 0 {
                Some(Quad::NEG_ZERO)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                if n > 0 {
                    Some(Quad::INFINITY)
                } else {
                    Some(Quad::ZERO)
                }
            } else if n % 2 == 0 {
                Some(Quad::NAN)
            } else if n > 0 {
                Some(Quad::NEG_INFINITY)
            } else {
                Some(Quad::NEG_ZERO)
            }
        } else if self.is_sign_negative() && n % 2 == 0 {
            Some(Quad::NAN)
        } else if n == 2 {
            Some(self.sqrt()) // use the more specialized method in sqrt
        } else {
            None
        }
    }

    #[inline]
    fn pre_powi(&self, n: i32) -> Option<Quad> {
        if n == 0 {
            Some(Quad::ONE)
        } else if self.is_nan() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Some(Quad::ZERO)
                } else {
                    Some(Quad::INFINITY)
                }
            } else if n > 0 {
                Some(Quad::NEG_ZERO)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else if self.is_infinite() {
            if n % 2 == 0 || self.is_sign_positive() {
                if n > 0 {
                    Some(Quad::INFINITY)
                } else {
                    Some(Quad::ZERO)
                }
            } else if n > 0 {
                Some(Quad::NEG_INFINITY)
            } else {
                Some(Quad::NEG_ZERO)
            }
        } else {
            None
        }
    }

    #[inline]
    fn pre_powf(&self, n: &Quad) -> Option<Quad> {
        if self.is_zero() {
            if n.is_zero() {
                Some(Quad::NAN)
            } else if n.is_sign_positive() {
                Some(Quad::ZERO)
            } else {
                Some(Quad::INFINITY)
            }
        } else if n.is_infinite() {
            if *self == Quad::ONE {
                Some(Quad::NAN)
            } else if n.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::ZERO)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ldexp_tests
    test_all_near!(
        ldexp_pi:
            qd!("12.566370614359172953850573533118011536788677597500423283899778369228"),
            Quad::PI.ldexp(2);
        ldexp_e:
            qd!("86.985018510689447531529199083285199928231906998398706398942964087165"),
            Quad::E.ldexp(5);
        ldexp_neg_pi:
            qd!("-1.5707963267948966192313216916397514420985846996875529104874722961534"),
            (-Quad::PI).ldexp(-1);
        ldexp_neg_e:
            qd!("-44536.329477472997136142949930642022363254736383180137676258797612629"),
            (-Quad::E).ldexp(14);
        ldexp_2_pi:
            qd!("0.00000074901405658478575669828495580661365609101519569757123493550886924431"),
            Quad::TAU.ldexp(-23);
        ldexp_pi_2:
            qd!("205887.41614566068967588779676660550101874569375744693508341396880142"),
            Quad::FRAC_PI_2.ldexp(17);
        ldexp_sqrt_2:
            qd!("759250124.99401242311284713250657708668634739527437245582300678806679"),
            Quad::SQRT_2.ldexp(29);
        ldexp_1_sqrt_2:
            qd!("2.828427124746190097603377448419396157139343750753896146353359475983"),
            Quad::FRAC_1_SQRT_2.ldexp(2);
        ldexp_150:
            qd!("18.75"),
            qd!(150).ldexp(-3);
        ldexp_neg_140:
            qd!("-2240"),
            qd!(-140).ldexp(4);
    );
    test_all_exact!(
        ldexp_zero:
            Quad::ZERO,
            Quad::ZERO.ldexp(2);
        ldexp_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.ldexp(2);
        ldexp_zero_exp:
            Quad::ONE,
            Quad::ONE.ldexp(0);

        ldexp_inf:
            Quad::INFINITY,
            Quad::INFINITY.ldexp(4);
        ldexp_inf_neg_exp:
            Quad::INFINITY,
            Quad::INFINITY.ldexp(-4);
        ldexp_inf_zero_exp:
            Quad::INFINITY,
            Quad::INFINITY.ldexp(0);
        ldexp_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.ldexp(3);
        ldexp_neg_inf_neg_exp:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.ldexp(-3);
        ldexp_neg_inf_zero_exp:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.ldexp(0);

        ldexp_nan:
            Quad::NAN,
            Quad::NAN.ldexp(5);
    );

    // sqr tests
    test_all_near!(
        sqr_pi:
            qd!("9.8696044010893586188344909998761511353136994072407906264133493762187"),
            Quad::PI.sqr();
        sqr_e:
            qd!("7.3890560989306502272304274605750078131803155705518473240871278225229"),
            Quad::E.sqr();
        sqr_neg_pi:
            qd!("9.8696044010893586188344909998761511353136994072407906264133493762187"),
            (-Quad::PI).sqr();
        sqr_neg_e:
            qd!("7.3890560989306502272304274605750078131803155705518473240871278225229"),
            (-Quad::E).sqr();
        sqr_2_pi:
            qd!("39.478417604357434475337963999504604541254797628963162505653397504875"),
            Quad::TAU.sqr();
        sqr_pi_2:
            qd!("2.4674011002723396547086227499690377838284248518101976566033373440547"),
            Quad::FRAC_PI_2.sqr();
        sqr_sqrt_2:
            qd!("2.0"),
            Quad::SQRT_2.sqr();
        sqr_1_sqrt_2:
            qd!("0.5"),
            Quad::FRAC_1_SQRT_2.sqr();
        sqr_150:
            qd!("22500.0"),
            qd!(150).sqr();
        sqr_neg_140:
            qd!("19600.0"),
            qd!(-140).sqr();
    );
    test_all_exact!(
        sqr_zero:
            Quad::ZERO,
            Quad::ZERO.sqr();
        sqr_neg_zero:
            Quad::ZERO,
            Quad::NEG_ZERO.sqr();

        sqr_inf:
            Quad::INFINITY,
            Quad::INFINITY.sqr();
        sqr_neg_inf:
            Quad::INFINITY,
            Quad::NEG_INFINITY.sqr();

        sqr_nan:
            Quad::NAN,
            Quad::NAN.sqr();
    );

    // sqrt tests
    test_all_near!(
        sqrt_pi:
            qd!("1.7724538509055160272981674833411451827975494561223871282138077898522"),
            Quad::PI.sqrt();
        sqrt_e:
            qd!("1.6487212707001281468486507878141635716537761007101480115750793116401"),
            Quad::E.sqrt();
        sqrt_2_pi:
            qd!("2.506628274631000502415765284811045253006986740609938316629923576343"),
            Quad::TAU.sqrt();
        sqrt_pi_2:
            qd!("1.2533141373155002512078826424055226265034933703049691583149617881715"),
            Quad::FRAC_PI_2.sqrt();
        sqrt_sqrt_2:
            qd!("1.1892071150027210667174999705604759152929720924638174130190022247198"),
            Quad::SQRT_2.sqrt();
        sqrt_1_sqrt_2:
            qd!("0.84089641525371454303112547623321489504003426235678451081322608597515"),
            Quad::FRAC_1_SQRT_2.sqrt();
        sqrt_150:
            qd!("12.247448713915890490986420373529456959829737403283350642163462836256"),
            qd!(150).sqrt();
    );
    test_all_exact!(
        sqrt_neg_pi:
            Quad::NAN,
            (-Quad::PI).sqrt();
        sqrt_neg_e:
            Quad::NAN,
            (-Quad::E).sqrt();
        sqrt_zero:
            Quad::ZERO,
            Quad::ZERO.sqrt();
        sqrt_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.sqrt();
        sqrt_inf:
            Quad::INFINITY,
            Quad::INFINITY.sqrt();
        sqrt_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.sqrt();
        sqrt_nan:
            Quad::NAN,
            Quad::NAN.sqrt();
    );

    // nroot tests
    test_all_near!(
        nroot_pi_one:
            Quad::PI,
            Quad::PI.nroot(1);
        nroot_pi_even:
            qd!("1.7724538509055160272981674833411451827975494561223871282138077898522"),
            Quad::PI.nroot(2);
        nroot_pi_odd:
            qd!("1.4645918875615232630201425272637903917385968556279371743572559371381"),
            Quad::PI.nroot(3);
        nroot_e_even:
            qd!("1.2840254166877414840734205680624364583362808652814630892175072968728"),
            Quad::E.nroot(4);
        nroot_e_odd:
            qd!("1.2214027581601698339210719946396741703075809415205036412734250985987"),
            Quad::E.nroot(5);
        nroot_neg_pi_odd:
            qd!("-1.177664030023197396684700855837046410967631450033500087569918027235"),
            (-Quad::PI).nroot(7);
        nroot_neg_e_odd:
            qd!("-1.1175190687418636486220597164816527772611027132027551083452265292597"),
            (-Quad::E).nroot(9);
        nroot_2_pi_even:
            qd!("1.201760670204157077572973120010031930912333043448441348680050608147"),
            Quad::TAU.nroot(10);
        nroot_2_pi_odd:
            qd!("1.181848494244013415633422793808887363527673194263035551523363315672"),
            Quad::TAU.nroot(11);
        nroot_pi_2_even:
            qd!("0.79788456080286535587989211986876373695171726232986931533185165934132"),
            Quad::FRAC_PI_2.nroot(-2);
        nroot_pi_2_odd:
            qd!("0.86025401382809962533698050905740949744348754456697746051876515772429"),
            Quad::FRAC_PI_2.nroot(-3);
        nroot_sqrt_2_even:
            qd!("0.91700404320467123174354159479414442803865516643683974979166206935351"),
            Quad::SQRT_2.nroot(-4);
        nroot_sqrt_2_odd:
            qd!("0.93303299153680741598134326614994216702722996435149403890049738548566"),
            Quad::SQRT_2.nroot(-5);
        nroot_1_sqrt_2_even:
            qd!("1.0594630943592952645618252949463417007792043174941856285592084314585"),
            Quad::FRAC_1_SQRT_2.nroot(-6);
        nroot_1_sqrt_2_odd:
            qd!("1.0507566386532194247355350853236871653483930556086861784037896755509"),
            Quad::FRAC_1_SQRT_2.nroot(-7);
        nroot_150_even:
            qd!("0.53455031846392155806266225608310095730016744201038647195444926340838"),
            qd!(150).nroot(-8);
        nroot_150_odd:
            qd!("0.57307581713722947566164111673679531838299720831267310026356742661948"),
            qd!(150).nroot(-9);
        nroot_neg_140_odd:
            qd!("-0.63811279267447920124399988241494030594315682697915110323602465090342"),
            qd!(-140).nroot(-11);
    );
    test_all_exact!(
        nroot_neg_pi_even:
            Quad::NAN,
            (-Quad::PI).nroot(6);
        nroot_neg_e_even:
            Quad::NAN,
            (-Quad::E).nroot(8);
        nroot_neg_140_even:
            Quad::NAN,
            qd!(-140).nroot(-10);

        nroot_zero_even:
            Quad::ZERO,
            Quad::ZERO.nroot(4);
        nroot_neg_zero_even:
            Quad::ZERO,
            Quad::NEG_ZERO.nroot(4);
        nroot_zero_odd:
            Quad::ZERO,
            Quad::ZERO.nroot(5);
        nroot_neg_zero_odd:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.nroot(4);
        nroot_zero_neg_even:
            Quad::INFINITY,
            Quad::ZERO.nroot(-2);
        nroot_neg_zero_neg_even:
            Quad::INFINITY,
            Quad::NEG_ZERO.nroot(-2);
        nroot_zero_neg_odd:
            Quad::INFINITY,
            Quad::ZERO.nroot(-3);
        nroot_neg_zero_neg_odd:
            Quad::NEG_INFINITY,
            Quad::NEG_ZERO.nroot(-3);
        nroot_zero_root:
            Quad::NAN,
            Quad::PI.nroot(0);

        nroot_inf_even:
            Quad::INFINITY,
            Quad::INFINITY.nroot(4);
        nroot_neg_inf_even:
            Quad::NAN,
            Quad::NEG_INFINITY.nroot(4);
        nroot_inf_odd:
            Quad::INFINITY,
            Quad::INFINITY.nroot(3);
        nroot_neg_inf_odd:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.nroot(3);
        nroot_inf_neg_even:
            Quad::ZERO,
            Quad::INFINITY.nroot(-4);
        nroot_neg_inf_neg_even:
            Quad::NAN,
            Quad::NEG_INFINITY.nroot(-4);
        nroot_inf_neg_odd:
            Quad::ZERO,
            Quad::INFINITY.nroot(-3);
        nroot_neg_inf_neg_odd:
            Quad::NEG_ZERO,
            Quad::NEG_INFINITY.nroot(-3);

        nroot_nan:
            Quad::NAN,
            Quad::NAN.nroot(4);
    );

    // cbrt tests
    test_all_near!(
        cbrt_pi:
            qd!("1.4645918875615232630201425272637903917385968556279371743572559371381"),
            Quad::PI.cbrt();
        cbrt_e:
            qd!("1.3956124250860895286281253196025868375979065151994069826175167060318"),
            Quad::E.cbrt();
        cbrt_neg_pi:
            qd!("-1.4645918875615232630201425272637903917385968556279371743572559371381"),
            (-Quad::PI).cbrt();
        cbrt_neg_e:
            qd!("-1.3956124250860895286281253196025868375979065151994069826175167060318"),
            (-Quad::E).cbrt();
        cbrt_2_pi:
            qd!("1.8452701486440284190968038795889880267800362662159516627255089733343"),
            Quad::TAU.cbrt();
        cbrt_pi_2:
            qd!("1.1624473515096264755708998144778370668593380927618370343897577226871"),
            Quad::FRAC_PI_2.cbrt();
        cbrt_sqrt_2:
            qd!("1.1224620483093729814335330496791795162324111106139867534404095458825"),
            Quad::SQRT_2.cbrt();
        cbrt_1_sqrt_2:
            qd!("0.8908987181403393047402262055905125079872126158781604033837569922518"),
            Quad::FRAC_1_SQRT_2.cbrt();
        cbrt_150:
            qd!("5.3132928459130553302387111108273152536671755781436662378582302628134"),
            qd!(150).cbrt();
        cbrt_neg_140:
            qd!("-5.1924941018511040261944552486014440955378002781346967593066085688115"),
            qd!(-140).cbrt();
    );
    test_all_exact!(
        cbrt_zero:
            Quad::ZERO,
            Quad::ZERO.cbrt();
        cbrt_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.cbrt();
        cbrt_inf:
            Quad::INFINITY,
            Quad::INFINITY.cbrt();
        cbrt_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.cbrt();
        cbrt_nan:
            Quad::NAN,
            Quad::NAN.cbrt();
    );

    // powi_tests
    test_all_near!(
        powi_pi:
            qd!("9.8696044010893586188344909998761511353136994072407906264133493762187"),
            Quad::PI.powi(2);
        powi_e:
            qd!("148.4131591025766034211155800405522796234876675938789890467528451108"),
            Quad::E.powi(5);
        powi_neg_pi:
            qd!("-0.31830988618379067153776752674502872406891929148091289749533468811787"),
            (-Quad::PI).powi(-1);
        powi_neg_e:
            qd!("1202604.2841647767777492367707678594494124865433761022403132906331962"),
            (-Quad::E).powi(14);
        powi_2_pi:
            qd!("4.3839241128549052526314338930793281030215689867488119977629459317994e-19"),
            Quad::TAU.powi(-23);
        powi_pi_2:
            qd!("2157.9327666208881618822522078236980140245618172124426850663668765245"),
            Quad::FRAC_PI_2.powi(17);
        powi_sqrt_2:
            qd!("23170.475005920789279566868057451693319285504006175917230926720827039"),
            Quad::SQRT_2.powi(29);
        powi_1_sqrt_2:
            qd!("0.5"),
            Quad::FRAC_1_SQRT_2.powi(2);
        powi_150:
            qd!("0.0000002962962962962962962962962962962962962962962962962962962962962962964"),
            qd!(150).powi(-3);
        powi_neg_140:
            qd!("384160000.0"),
            qd!(-140).powi(4);
    );
    test_all_exact!(
        powi_zero_odd:
            Quad::ZERO,
            Quad::ZERO.powi(3);
        powi_neg_zero_odd:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.powi(3);
        powi_zero_even:
            Quad::ZERO,
            Quad::ZERO.powi(4);
        powi_neg_zero_even:
            Quad::ZERO,
            Quad::NEG_ZERO.powi(4);
        powi_inf_neg_odd:
            Quad::INFINITY,
            Quad::ZERO.powi(-1);
        powi_neg_inf_neg_odd:
            Quad::NEG_INFINITY,
            Quad::NEG_ZERO.powi(-1);
        powi_inf_neg_even:
            Quad::INFINITY,
            Quad::ZERO.powi(-2);
        powi_neg_inf_neg_even:
            Quad::INFINITY,
            Quad::NEG_ZERO.powi(-2);

        powi_zero_zero:
            Quad::ONE,
            Quad::ZERO.powi(0);
        powi_one_zero:
            Quad::ONE,
            Quad::ONE.powi(0);
        powi_2317_zero:
            Quad::ONE,
            qd!(2317).powi(0);
        powi_inf_zero:
            Quad::ONE,
            Quad::INFINITY.powi(0);
        powi_neg_inf_zero:
            Quad::ONE,
            Quad::NEG_INFINITY.powi(0);
        powi_nan_zero:
            Quad::ONE,
            Quad::NAN.powi(0);

        powi_inf_even:
            Quad::INFINITY,
            Quad::INFINITY.powi(2);
        powi_inf_odd:
            Quad::INFINITY,
            Quad::INFINITY.powi(3);
        powi_neg_inf_even:
            Quad::INFINITY,
            Quad::NEG_INFINITY.powi(2);
        powi_neg_inf_odd:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.powi(3);

        powi_nan:
            Quad::NAN,
            Quad::NAN.powi(2);
    );

    // powf_tests
    test_all_near!(
        powf_pi:
            qd!("93648.047476083020973716690184919345635998157275514694127052449392906"),
            Quad::PI.powf(qd!(10.0));
        powf_e:
            qd!("7.3890560989306502272304274605750078131803155705518473240871278225229"),
            Quad::E.powf(qd!(2.0));
        powf_2_pi:
            qd!("0.074336687544542155208242592466630548087787623610561811515214014892929"),
            Quad::TAU.powf(-Quad::SQRT_2);
        powf_pi_2:
            qd!("0.86611010235488054964635563326801753095024536815755494441900408376814"),
            Quad::FRAC_PI_2.powf(-Quad::FRAC_1_PI);
        powf_sqrt_2:
            qd!("1.2777037682648325212982228274557087924939442033176200268009327763854"),
            Quad::SQRT_2.powf(Quad::FRAC_1_SQRT_2);
        powf_1_sqrt_2:
            qd!("1.2715371297141403818925670498911252946895411664169044422389164132313"),
            Quad::FRAC_1_SQRT_2.powf(-Quad::LN_2);
        powf_150:
            qd!("0.0000097580874511463571772513415019480387518681397873542312835791633473171"),
            qd!(150).powf(-Quad::LN_10);
    );
    test_all_exact!(
        powf_zero:
            Quad::ZERO,
            Quad::ZERO.powf(qd!(3));
        powf_neg_zero:
            Quad::ZERO,
            Quad::NEG_ZERO.powf(qd!(3));
        powf_zero_inf:
            Quad::ZERO,
            Quad::ZERO.powf(Quad::INFINITY);
        powf_neg_zero_inf:
            Quad::ZERO,
            Quad::NEG_ZERO.powf(Quad::INFINITY);
        powf_zero_neg:
            Quad::INFINITY,
            Quad::ZERO.powf(qd!(-2));
        powf_neg_zero_neg:
            Quad::INFINITY,
            Quad::NEG_ZERO.powf(qd!(-2));
        powf_zero_neg_inf:
            Quad::INFINITY,
            Quad::ZERO.powf(Quad::NEG_INFINITY);
        powf_neg_zero_neg_inf:
            Quad::INFINITY,
            Quad::NEG_ZERO.powf(Quad::NEG_INFINITY);

        powf_exp_zero:
            Quad::ONE,
            qd!(2).powf(Quad::ZERO);
        powf_exp_neg_zero:
            Quad::ONE,
            qd!(2).powf(Quad::NEG_ZERO);
        powf_zero_zero:
            Quad::NAN,
            Quad::ZERO.powf(Quad::ZERO);
        powf_neg_zero_zero:
            Quad::NAN,
            Quad::NEG_ZERO.powf(Quad::ZERO);
        powf_zero_neg_zero:
            Quad::NAN,
            Quad::ZERO.powf(Quad::NEG_ZERO);
        powf_neg_zero_neg_zero:
            Quad::NAN,
            Quad::NEG_ZERO.powf(Quad::NEG_ZERO);

        powf_inf_zero:
            Quad::NAN,
            Quad::INFINITY.powf(Quad::ZERO);
        powf_inf_neg_zero:
            Quad::NAN,
            Quad::INFINITY.powf(Quad::NEG_ZERO);
        powf_neg_inf_zero:
            Quad::NAN,
            Quad::NEG_INFINITY.powf(Quad::ZERO);
        powf_neg_inf_neg_zero:
            Quad::NAN,
            Quad::NEG_INFINITY.powf(Quad::NEG_ZERO);

        powf_exp_inf:
            Quad::INFINITY,
            qd!(2).powf(Quad::INFINITY);
        powf_exp_neg_inf:
            Quad::ZERO,
            qd!(2).powf(Quad::NEG_INFINITY);
        powf_one_inf:
            Quad::NAN,
            qd!(1).powf(Quad::INFINITY);
        powf_one_neg_inf:
            Quad::NAN,
            qd!(1).powf(Quad::NEG_INFINITY);

        powf_nan:
            Quad::NAN,
            Quad::NAN.powf(qd!(3));
        powf_exp_nan:
            Quad::NAN,
            qd!(3).powf(Quad::NAN);
        powf_neg:
            Quad::NAN,
            qd!(-1).powf(qd!(1));
    );

    // recip tests
    test_all_near!(
        recip_pi:
            qd!("0.31830988618379067153776752674502872406891929148091289749533468811787"),
            Quad::PI.recip();
        recip_e:
            qd!("0.36787944117144232159552377016146086744581113103176783450783680169744"),
            Quad::E.recip();
        recip_neg_pi:
            qd!("-0.31830988618379067153776752674502872406891929148091289749533468811787"),
            (-Quad::PI).recip();
        recip_neg_e:
            qd!("-0.36787944117144232159552377016146086744581113103176783450783680169744"),
            (-Quad::E).recip();
        recip_2_pi:
            qd!("0.15915494309189533576888376337251436203445964574045644874766734405894"),
            Quad::TAU.recip();
        recip_pi_2:
            qd!("0.63661977236758134307553505349005744813783858296182579499066937623574"),
            Quad::FRAC_PI_2.recip();
        recip_sqrt_2:
            qd!("0.70710678118654752440084436210484903928483593768847403658833986899576"),
            Quad::SQRT_2.recip();
        recip_1_sqrt_2:
            qd!("1.4142135623730950488016887242096980785696718753769480731766797379903"),
            Quad::FRAC_1_SQRT_2.recip();
        recip_150:
            qd!("0.0066666666666666666666666666666666666666666666666666666666666666666673"),
            qd!(150).recip();
        recip_neg_140:
            qd!("-0.0071428571428571428571428571428571428571428571428571428571428571428589"),
            qd!(-140).recip();
    );
    test_all_exact!(
        recip_zero:
            Quad::INFINITY,
            Quad::ZERO.recip();
        recip_neg_zero:
            Quad::NEG_INFINITY,
            Quad::NEG_ZERO.recip();
        recip_inf:
            Quad::ZERO,
            Quad::INFINITY.recip();
        recip_neg_inf:
            Quad::NEG_ZERO,
            Quad::NEG_INFINITY.recip();
        recip_nan:
            Quad::NAN,
            Quad::NAN.recip();
    );
}
