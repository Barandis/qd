// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::double::common as c;
use crate::double::Double;

impl Double {
    /// Calculates $x\sdot2^n$, where $x$ is `self` and $n$ is the argument.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced
    /// mathematical calculations (including several within this library). Therefore an
    /// implementation that is much more efficient than calculating it through
    /// multiplication and [`powi`] is offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(5);
    /// assert!(x.ldexp(3) == dd!(40)); // 5 * 2^3
    /// ```
    ///
    /// [`powi`]: #method.powi
    #[inline]
    pub fn ldexp(self, n: i32) -> Double {
        let factor = 2f64.powi(n);
        Double(self.0 * factor, self.1 * factor)
    }

    /// Calculates the arithmetic-geometric mean of $x$ and $y$ ($M(x, y)$ or $\text{agm}(x,
    /// y)$), where $x$ and $y$ are `self` and the argument.
    ///
    /// The AGM is an iterative calculation. $x$ and $y$ are assigned as the two inputs to
    /// the first iteration.
    ///
    /// $$a_0 = x \newline g_0 = y$$
    ///
    /// Then the iterations are performed as two interdependent sequences, one calculating
    /// the arithmetic mean and one the geometric mean. The results from each iteration are
    /// fed back into the next iteration as inputs.
    ///
    /// $$a_{n + 1} = \frac{a_n + g_n}{2}$$
    /// $$g_{n + 1} = \sqrt{a_{n}g_{n}}$$
    ///
    /// These numbers both converge towards the same number, and once they reach that
    /// number, it's returned as the AGM.
    ///
    /// These sequences converge very quickly, doubling the accuracy with each iteration.
    /// The AGM can be used in fast algorithms for transcendental functions and for
    /// computing some mathematical constants (most notably $\pi$). The speed advantage from
    /// its fast convergence is realized at around 400 decimal digits, so it isn't currently
    /// used for these purposes in this library.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::E.agm(Double::PI);
    /// let expected = dd!("2.9261085515723046966658957101705574");
    ///
    /// let delta = (x - expected).abs();
    /// assert!(delta < dd!(1e-30));
    /// ```
    pub fn agm(self, other: Double) -> Double {
        match self.pre_agm(&other) {
            Some(r) => r,
            None => {
                let mut a = c::mul_pwr2(self + other, 0.5);
                let mut g = (self * other).sqrt();

                let k = a.0.log2().floor() as i32;
                let eps = c::mul_pwr2(Double::EPSILON, 2f64.powi(k + 2));

                println!("Eps: {}", eps);

                while (a - g).abs() > eps {
                    let am = c::mul_pwr2(a + g, 0.5);
                    let gm = (a * g).sqrt();
                    a = am;
                    g = gm;
                }
                a
            }
        }
    }

    /// Calculates the square of $x$, $x^2$, where $x$ is `self`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available
    /// when the two numbers being multiplied are the same, so it is more efficient than
    /// bare multiplication.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(3);
    /// assert!(x.sqr() == x * x); // The left side is faster though
    /// ```
    #[inline]
    pub fn sqr(self) -> Double {
        match self.pre_sqr() {
            Some(r) => r,
            None => {
                let (p, e) = p::two_sqr(self.0);
                let (a, b) = u::renorm2(p, e + 2.0 * self.0 * self.1 + self.1 * self.1);
                Double(a, b)
            }
        }
    }

    /// Calculates the square root $x$, $\sqrt{x}$, where $x$ is `self`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = dd!(2).sqrt();
    /// let diff = (x - Double::SQRT_2).abs();
    /// assert!(diff < dd!(1e-30));
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
                let x = Double::from(1.0 / self.0.sqrt());
                let ax = self * x;
                ax + (self - ax.sqr()) * c::mul_pwr2(x, 0.5)
            }
        }
    }

    /// Calculates the $n$th root of $x$, $\sqrt\[n\]{x}$, where $x$ is `self` and $n$
    /// is the argument.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(2).nroot(4);
    /// let expected = dd!("1.1892071150027210667174999705605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
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

    /// Calculates the cube root of $x$, $\sqrt\[3\]{x}$, where $x$ is `self`.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(2).cbrt();
    /// let expected = dd!("1.2599210498948731647672106072782");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    pub fn cbrt(self) -> Double {
        self.nroot(3)
    }

    /// Calculates $x$ raised to the integer power $n$, $x^n$, where $x$ is `self` and
    /// $n$ is the argument.
    ///
    /// This function correctly handles the special inputs defined in IEEE 754. In
    /// particular:
    ///
    /// * `x.powi(0)` is $1$ for any `x` (including `0`, `NaN`, or `inf`)
    /// * `x.powi(n)` is $\pm\infin$ for `x` = $\pm0$ and any odd negative `n`
    /// * `x.powi(n)` is $+\infin$ for `x` = $\pm0$ and any even negative `n`
    /// * `x.powi(n)` is $\pm0$ for `x` = $\pm0$ and any odd positive `n`
    /// * `x.powi(n)` is $+0$ for `x` = $\pm0$ and any even positive `n`
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(3);
    /// assert!(x.powi(3) == dd!(27));
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

    /// Calculates $x$ raised to the `Double` power $n$, $x^n$, where $x$ is `self`
    /// and $n$ is the argument.
    ///
    /// In general, $x^n$ is equal to $e^{n \ln x}$. This precludes raising a negative
    /// `Double` to a fractional or irrational power because $\ln x$ is undefined when $x$
    /// is negative. In that case, this function returns [`NAN`].
    ///
    /// It's actually more complex than that; if the exponent can be expressed as a fraction
    /// with an odd denominator, then there is an answer ($\sqrt\[3\]{x}$, which is defined
    /// for negative $x$, is the same as a $x^\frac{1}{3}$). Therefore, something like
    /// `dd!(-4).powf(dd!(0.2))` should work, as 0.2 is a fraction with an odd denominator
    /// ($\frac{1}{5}$), and the entire expression is the same as $\sqrt\[5\]{-4}$, which is
    /// a real thing. However, it's impossible in general to tell whether a number is a
    /// fraction while using floating-point numbers, so no attempt is made to make this
    /// work. If you need a fifth root of 4, use `dd!(-4).nroot(5)`.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(3).powf(dd!(3.3));
    /// let expected = dd!("37.540507598529552193101865954634");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-28));
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

    /// Calculates the reciprocal of $x$, $\frac{1}{x}$, where $x$ is `self`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::PI.recip();
    /// let expected = dd!("0.31830988618379067153776752674503");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
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
    fn pre_agm(&self, other: &Double) -> Option<Double> {
        if self.is_sign_negative() || other.is_sign_negative() {
            Some(Double::NAN)
        } else {
            None
        }
    }

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
            Some(*self) // according to IEEE 754 definition, sqrt(-0) is -0
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
        if n == 0 {
            Some(Double::NAN)
        } else if n == 1 {
            Some(*self)
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
            if self.is_sign_positive() {
                if n > 0 {
                    Some(Double::INFINITY)
                } else {
                    Some(Double::ZERO)
                }
            } else if n % 2 == 0 {
                Some(Double::NAN)
            } else if n > 0 {
                Some(Double::NEG_INFINITY)
            } else {
                Some(Double::NEG_ZERO)
            }
        } else if self.is_sign_negative() && n % 2 == 0 {
            Some(Double::NAN)
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

    // ldexp_tests
    test_all_near!(
        ldexp_pi:
            dd!("12.566370614359172953850573533118011"),
            Double::PI.ldexp(2);
        ldexp_e:
            dd!("86.985018510689447531529199083285169"),
            Double::E.ldexp(5);
        ldexp_neg_pi:
            dd!("-1.5707963267948966192313216916397514"),
            (-Double::PI).ldexp(-1);
        ldexp_neg_e:
            dd!("-44536.329477472997136142949930642007"),
            (-Double::E).ldexp(14);
        ldexp_2_pi:
            dd!("0.00000074901405658478575669828495580661364"),
            Double::TAU.ldexp(-23);
        ldexp_pi_2:
            dd!("205887.4161456606896758877967666055"),
            Double::FRAC_PI_2.ldexp(17);
        ldexp_sqrt_2:
            dd!("759250124.99401242311284713250657735"),
            Double::SQRT_2.ldexp(29);
        ldexp_1_sqrt_2:
            dd!("2.828427124746190097603377448419394"),
            Double::FRAC_1_SQRT_2.ldexp(2);
        ldexp_150:
            dd!("18.75"),
            dd!(150).ldexp(-3);
        ldexp_neg_140:
            dd!("-2240"),
            dd!(-140).ldexp(4);
    );
    test_all_exact!(
        ldexp_zero:
            Double::ZERO,
            Double::ZERO.ldexp(2);
        ldexp_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.ldexp(2);
        ldexp_zero_exp:
            Double::ONE,
            Double::ONE.ldexp(0);

        ldexp_inf:
            Double::INFINITY,
            Double::INFINITY.ldexp(4);
        ldexp_inf_neg_exp:
            Double::INFINITY,
            Double::INFINITY.ldexp(-4);
        ldexp_inf_zero_exp:
            Double::INFINITY,
            Double::INFINITY.ldexp(0);
        ldexp_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.ldexp(3);
        ldexp_neg_inf_neg_exp:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.ldexp(-3);
        ldexp_neg_inf_zero_exp:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.ldexp(0);

        ldexp_nan:
            Double::NAN,
            Double::NAN.ldexp(5);
    );

    // agm tests
    test_all_near!(
        agm_1_pi:
            dd!("1.9187246659776345296603782507498161"),
            Double::ONE.agm(Double::PI);
        agm_pi_1:
            dd!("1.9187246659776345296603782507498161"),
            Double::PI.agm(Double::ONE);
        agm_e:
            dd!("1.7523515580810808267140866648393655"),
            Double::ONE.agm(Double::E);
        agm_pi_e:
            dd!("2.9261085515723046966658957101705574"),
            Double::PI.agm(Double::E);
        agm_2_pi:
            dd!("3.0476368527436049879565709261829655"),
            Double::ONE.agm(Double::TAU);
        agm_pi_2:
            dd!("1.2693054634338164308347564505622511"),
            Double::ONE.agm(Double::FRAC_PI_2);
        agm_150:
            dd!("36.832864037875317975522582670941527"),
            Double::ONE.agm(dd!(150));
        agm_1e150:
            dd!("4.5297400112601563115809746712420527e+147"),
            Double::ONE.agm(dd!(1e150));
        agm_1e100_1e150:
            dd!("1.3481430934587092639510559486907854e+148"),
            dd!(1e100).agm(dd!(1e150));
    );
    test_all_exact!(
        agm_neg_pos:
            Double::NAN,
            Double::NEG_ONE.agm(Double::ONE);
        agm_pos_neg:
            Double::NAN,
            Double::ONE.agm(Double::NEG_ONE);
        agm_neg_neg:
            Double::NAN,
            Double::NEG_ONE.agm(Double::NEG_ONE);
    );

    // sqr tests
    test_all_near!(
        sqr_pi:
            dd!("9.8696044010893586188344909998761508"),
            Double::PI.sqr();
        sqr_e:
            dd!("7.3890560989306502272304274605750057"),
            Double::E.sqr();
        sqr_neg_pi:
            dd!("9.8696044010893586188344909998761508"),
            (-Double::PI).sqr();
        sqr_neg_e:
            dd!("7.3890560989306502272304274605750057"),
            (-Double::E).sqr();
        sqr_2_pi:
            dd!("39.478417604357434475337963999504603"),
            Double::TAU.sqr();
        sqr_pi_2:
            dd!("2.4674011002723396547086227499690377"),
            Double::FRAC_PI_2.sqr();
        sqr_sqrt_2:
            dd!("2.0"),
            Double::SQRT_2.sqr();
        sqr_1_sqrt_2:
            dd!("0.5"),
            Double::FRAC_1_SQRT_2.sqr();
        sqr_150:
            dd!("22500.0"),
            dd!(150).sqr();
        sqr_neg_140:
            dd!("19600.0"),
            dd!(-140).sqr();
    );
    test_all_exact!(
        sqr_zero:
            Double::ZERO,
            Double::ZERO.sqr();
        sqr_neg_zero:
            Double::ZERO,
            Double::NEG_ZERO.sqr();

        sqr_inf:
            Double::INFINITY,
            Double::INFINITY.sqr();
        sqr_neg_inf:
            Double::INFINITY,
            Double::NEG_INFINITY.sqr();

        sqr_nan:
            Double::NAN,
            Double::NAN.sqr();
    );

    // sqrt tests
    test_all_near!(
        sqrt_pi:
            dd!("1.7724538509055160272981674833411449"),
            Double::PI.sqrt();
        sqrt_e:
            dd!("1.6487212707001281468486507878141628"),
            Double::E.sqrt();
        sqrt_2_pi:
            dd!("2.5066282746310005024157652848110464"),
            Double::TAU.sqrt();
        sqrt_pi_2:
            dd!("1.2533141373155002512078826424055232"),
            Double::FRAC_PI_2.sqrt();
        sqrt_sqrt_2:
            dd!("1.1892071150027210667174999705604763"),
            Double::SQRT_2.sqrt();
        sqrt_1_sqrt_2:
            dd!("0.84089641525371454303112547623321465"),
            Double::FRAC_1_SQRT_2.sqrt();
        sqrt_150:
            dd!("12.247448713915890490986420373529453"),
            dd!(150).sqrt();
    );
    test_all_exact!(
        sqrt_neg_pi:
            Double::NAN,
            (-Double::PI).sqrt();
        sqrt_neg_e:
            Double::NAN,
            (-Double::E).sqrt();
        sqrt_zero:
            Double::ZERO,
            Double::ZERO.sqrt();
        sqrt_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.sqrt();
        sqrt_inf:
            Double::INFINITY,
            Double::INFINITY.sqrt();
        sqrt_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.sqrt();
        sqrt_nan:
            Double::NAN,
            Double::NAN.sqrt();
    );

    // nroot tests
    test_all_near!(
        nroot_pi_one:
            Double::PI,
            Double::PI.nroot(1);
        nroot_pi_even:
            dd!("1.7724538509055160272981674833411449"),
            Double::PI.nroot(2);
        nroot_pi_odd:
            dd!("1.4645918875615232630201425272637902"),
            Double::PI.nroot(3);
        nroot_e_even:
            dd!("1.2840254166877414840734205680624368"),
            Double::E.nroot(4);
        nroot_e_odd:
            dd!("1.2214027581601698339210719946396748"),
            Double::E.nroot(5);
        nroot_neg_pi_odd:
            dd!("-1.1776640300231973966847008558370464"),
            (-Double::PI).nroot(7);
        nroot_neg_e_odd:
            dd!("-1.1175190687418636486220597164816534"),
            (-Double::E).nroot(9);
        nroot_2_pi_even:
            dd!("1.2017606702041570775729731200100314"),
            Double::TAU.nroot(10);
        nroot_2_pi_odd:
            dd!("1.1818484942440134156334227938088873"),
            Double::TAU.nroot(11);
        nroot_pi_2_even:
            dd!("0.79788456080286535587989211986876365"),
            Double::FRAC_PI_2.nroot(-2);
        nroot_pi_2_odd:
            dd!("0.8602540138280996253369805090574098"),
            Double::FRAC_PI_2.nroot(-3);
        nroot_sqrt_2_even:
            dd!("0.91700404320467123174354159479414398"),
            Double::SQRT_2.nroot(-4);
        nroot_sqrt_2_odd:
            dd!("0.9330329915368074159813432661499424"),
            Double::SQRT_2.nroot(-5);
        nroot_1_sqrt_2_even:
            dd!("1.0594630943592952645618252949463415"),
            Double::FRAC_1_SQRT_2.nroot(-6);
        nroot_1_sqrt_2_odd:
            dd!("1.0507566386532194247355350853236869"),
            Double::FRAC_1_SQRT_2.nroot(-7);
        nroot_150_even:
            dd!("0.53455031846392155806266225608310132"),
            dd!(150).nroot(-8);
        nroot_150_odd:
            dd!("0.57307581713722947566164111673679532"),
            dd!(150).nroot(-9);
        nroot_neg_140_odd:
            dd!("-0.6381127926744792012439998824149406"),
            dd!(-140).nroot(-11);
    );
    test_all_exact!(
        nroot_neg_pi_even:
            Double::NAN,
            (-Double::PI).nroot(6);
        nroot_neg_e_even:
            Double::NAN,
            (-Double::E).nroot(8);
        nroot_neg_140_even:
            Double::NAN,
            dd!(-140).nroot(-10);

        nroot_zero_even:
            Double::ZERO,
            Double::ZERO.nroot(4);
        nroot_neg_zero_even:
            Double::ZERO,
            Double::NEG_ZERO.nroot(4);
        nroot_zero_odd:
            Double::ZERO,
            Double::ZERO.nroot(5);
        nroot_neg_zero_odd:
            Double::NEG_ZERO,
            Double::NEG_ZERO.nroot(4);
        nroot_zero_neg_even:
            Double::INFINITY,
            Double::ZERO.nroot(-2);
        nroot_neg_zero_neg_even:
            Double::INFINITY,
            Double::NEG_ZERO.nroot(-2);
        nroot_zero_neg_odd:
            Double::INFINITY,
            Double::ZERO.nroot(-3);
        nroot_neg_zero_neg_odd:
            Double::NEG_INFINITY,
            Double::NEG_ZERO.nroot(-3);
        nroot_zero_root:
            Double::NAN,
            Double::PI.nroot(0);

        nroot_inf_even:
            Double::INFINITY,
            Double::INFINITY.nroot(4);
        nroot_neg_inf_even:
            Double::NAN,
            Double::NEG_INFINITY.nroot(4);
        nroot_inf_odd:
            Double::INFINITY,
            Double::INFINITY.nroot(3);
        nroot_neg_inf_odd:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.nroot(3);
        nroot_inf_neg_even:
            Double::ZERO,
            Double::INFINITY.nroot(-4);
        nroot_neg_inf_neg_even:
            Double::NAN,
            Double::NEG_INFINITY.nroot(-4);
        nroot_inf_neg_odd:
            Double::ZERO,
            Double::INFINITY.nroot(-3);
        nroot_neg_inf_neg_odd:
            Double::NEG_ZERO,
            Double::NEG_INFINITY.nroot(-3);

        nroot_nan:
            Double::NAN,
            Double::NAN.nroot(4);
    );

    // cbrt tests
    test_all_near!(
        cbrt_pi:
            dd!("1.4645918875615232630201425272637902"),
            Double::PI.cbrt();
        cbrt_e:
            dd!("1.3956124250860895286281253196025871"),
            Double::E.cbrt();
        cbrt_neg_pi:
            dd!("-1.4645918875615232630201425272637902"),
            (-Double::PI).cbrt();
        cbrt_neg_e:
            dd!("-1.3956124250860895286281253196025871"),
            (-Double::E).cbrt();
        cbrt_2_pi:
            dd!("1.8452701486440284190968038795889887"),
            Double::TAU.cbrt();
        cbrt_pi_2:
            dd!("1.1624473515096264755708998144778371"),
            Double::FRAC_PI_2.cbrt();
        cbrt_sqrt_2:
            dd!("1.122462048309372981433533049679179"),
            Double::SQRT_2.cbrt();
        cbrt_1_sqrt_2:
            dd!("0.89089871814033930474022620559051262"),
            Double::FRAC_1_SQRT_2.cbrt();
        cbrt_150:
            dd!("5.3132928459130553302387111108273125"),
            dd!(150).cbrt();
        cbrt_neg_140:
            dd!("-5.1924941018511040261944552486014427"),
            dd!(-140).cbrt();
    );
    test_all_exact!(
        cbrt_zero:
            Double::ZERO,
            Double::ZERO.cbrt();
        cbrt_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.cbrt();
        cbrt_inf:
            Double::INFINITY,
            Double::INFINITY.cbrt();
        cbrt_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.cbrt();
        cbrt_nan:
            Double::NAN,
            Double::NAN.cbrt();
    );

    // powi_tests
    test_all_near!(
        powi_pi:
            dd!("9.8696044010893586188344909998761508"),
            Double::PI.powi(2);
        powi_e:
            dd!("148.41315910257660342111558004055192"),
            Double::E.powi(5);
        powi_neg_pi:
            dd!("-0.31830988618379067153776752674502864"),
            (-Double::PI).powi(-1);
        powi_neg_e:
            dd!("1202604.284164776777749236770767854"),
            (-Double::E).powi(14);
        powi_sqrt_2:
            dd!("23170.475005920789279566868057451928"),
            Double::SQRT_2.powi(29);
        powi_1_sqrt_2:
            dd!("0.5"),
            Double::FRAC_1_SQRT_2.powi(2);
        powi_150:
            dd!("0.00000029629629629629629629629629629629632"),
            dd!(150).powi(-3);
        powi_neg_140:
            dd!("384160000.0"),
            dd!(-140).powi(4);
    );
    test_all_prec!(
        powi_2_pi:
            dd!("4.3839241128549052526314338930793323e-19"),
            Double::TAU.powi(-23),
            30;
        powi_pi_2:
            dd!("2157.9327666208881618822522078236982"),
            Double::FRAC_PI_2.powi(17),
            30;
    );
    test_all_exact!(
        powi_zero_odd:
            Double::ZERO,
            Double::ZERO.powi(3);
        powi_neg_zero_odd:
            Double::NEG_ZERO,
            Double::NEG_ZERO.powi(3);
        powi_zero_even:
            Double::ZERO,
            Double::ZERO.powi(4);
        powi_neg_zero_even:
            Double::ZERO,
            Double::NEG_ZERO.powi(4);
        powi_inf_neg_odd:
            Double::INFINITY,
            Double::ZERO.powi(-1);
        powi_neg_inf_neg_odd:
            Double::NEG_INFINITY,
            Double::NEG_ZERO.powi(-1);
        powi_inf_neg_even:
            Double::INFINITY,
            Double::ZERO.powi(-2);
        powi_neg_inf_neg_even:
            Double::INFINITY,
            Double::NEG_ZERO.powi(-2);

        powi_zero_zero:
            Double::ONE,
            Double::ZERO.powi(0);
        powi_one_zero:
            Double::ONE,
            Double::ONE.powi(0);
        powi_2317_zero:
            Double::ONE,
            dd!(2317).powi(0);
        powi_inf_zero:
            Double::ONE,
            Double::INFINITY.powi(0);
        powi_neg_inf_zero:
            Double::ONE,
            Double::NEG_INFINITY.powi(0);
        powi_nan_zero:
            Double::ONE,
            Double::NAN.powi(0);

        powi_inf_even:
            Double::INFINITY,
            Double::INFINITY.powi(2);
        powi_inf_odd:
            Double::INFINITY,
            Double::INFINITY.powi(3);
        powi_neg_inf_even:
            Double::INFINITY,
            Double::NEG_INFINITY.powi(2);
        powi_neg_inf_odd:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.powi(3);

        powi_nan:
            Double::NAN,
            Double::NAN.powi(2);
    );

    // powf_tests
    test_all_near!(
        powf_e:
            dd!("7.3890560989306502272304274605750057"),
            Double::E.powf(dd!(2.0));
        powf_2_pi:
            dd!("0.074336687544542155208242592466630482"),
            Double::TAU.powf(-Double::SQRT_2);
        powf_pi_2:
            dd!("0.8661101023548805496463556332680179"),
            Double::FRAC_PI_2.powf(-Double::FRAC_1_PI);
        powf_sqrt_2:
            dd!("1.277703768264832521298222827455709"),
            Double::SQRT_2.powf(Double::FRAC_1_SQRT_2);
        powf_1_sqrt_2:
            dd!("1.2715371297141403818925670498911262"),
            Double::FRAC_1_SQRT_2.powf(-Double::LN_2);
    );
    test_all_prec!(
        powf_pi:
            dd!("93648.047476083020973716690184919311"),
            Double::PI.powf(dd!(10.0)),
            30;
        powf_150:
            dd!("0.000009758087451146357177251341501948003"),
            dd!(150).powf(-Double::LN_10),
            30;
    );
    test_all_exact!(
        powf_zero:
            Double::ZERO,
            Double::ZERO.powf(dd!(3));
        powf_neg_zero:
            Double::ZERO,
            Double::NEG_ZERO.powf(dd!(3));
        powf_zero_inf:
            Double::ZERO,
            Double::ZERO.powf(Double::INFINITY);
        powf_neg_zero_inf:
            Double::ZERO,
            Double::NEG_ZERO.powf(Double::INFINITY);
        powf_zero_neg:
            Double::INFINITY,
            Double::ZERO.powf(dd!(-2));
        powf_neg_zero_neg:
            Double::INFINITY,
            Double::NEG_ZERO.powf(dd!(-2));
        powf_zero_neg_inf:
            Double::INFINITY,
            Double::ZERO.powf(Double::NEG_INFINITY);
        powf_neg_zero_neg_inf:
            Double::INFINITY,
            Double::NEG_ZERO.powf(Double::NEG_INFINITY);

        powf_exp_zero:
            Double::ONE,
            dd!(2).powf(Double::ZERO);
        powf_exp_neg_zero:
            Double::ONE,
            dd!(2).powf(Double::NEG_ZERO);
        powf_zero_zero:
            Double::NAN,
            Double::ZERO.powf(Double::ZERO);
        powf_neg_zero_zero:
            Double::NAN,
            Double::NEG_ZERO.powf(Double::ZERO);
        powf_zero_neg_zero:
            Double::NAN,
            Double::ZERO.powf(Double::NEG_ZERO);
        powf_neg_zero_neg_zero:
            Double::NAN,
            Double::NEG_ZERO.powf(Double::NEG_ZERO);

        powf_inf_zero:
            Double::NAN,
            Double::INFINITY.powf(Double::ZERO);
        powf_inf_neg_zero:
            Double::NAN,
            Double::INFINITY.powf(Double::NEG_ZERO);
        powf_neg_inf_zero:
            Double::NAN,
            Double::NEG_INFINITY.powf(Double::ZERO);
        powf_neg_inf_neg_zero:
            Double::NAN,
            Double::NEG_INFINITY.powf(Double::NEG_ZERO);

        powf_exp_inf:
            Double::INFINITY,
            dd!(2).powf(Double::INFINITY);
        powf_exp_neg_inf:
            Double::ZERO,
            dd!(2).powf(Double::NEG_INFINITY);
        powf_one_inf:
            Double::NAN,
            dd!(1).powf(Double::INFINITY);
        powf_one_neg_inf:
            Double::NAN,
            dd!(1).powf(Double::NEG_INFINITY);

        powf_nan:
            Double::NAN,
            Double::NAN.powf(dd!(3));
        powf_exp_nan:
            Double::NAN,
            dd!(3).powf(Double::NAN);
        powf_neg:
            Double::NAN,
            dd!(-1).powf(dd!(1));
    );

    // recip tests
    test_all_near!(
        recip_pi:
            dd!("0.31830988618379067153776752674502864"),
            Double::PI.recip();
        recip_e:
            dd!("0.36787944117144232159552377016146107"),
            Double::E.recip();
        recip_neg_pi:
            dd!("-0.31830988618379067153776752674502864"),
            (-Double::PI).recip();
        recip_neg_e:
            dd!("-0.36787944117144232159552377016146107"),
            (-Double::E).recip();
        recip_2_pi:
            dd!("0.15915494309189533576888376337251432"),
            Double::TAU.recip();
        recip_pi_2:
            dd!("0.63661977236758134307553505349005728"),
            Double::FRAC_PI_2.recip();
        recip_sqrt_2:
            dd!("0.70710678118654752440084436210484851"),
            Double::SQRT_2.recip();
        recip_1_sqrt_2:
            dd!("1.4142135623730950488016887242096986"),
            Double::FRAC_1_SQRT_2.recip();
        recip_150:
            dd!("0.0066666666666666666666666666666666678"),
            dd!(150).recip();
        recip_neg_140:
            dd!("-0.0071428571428571428571428571428571449"),
            dd!(-140).recip();
    );
    test_all_exact!(
        recip_zero:
            Double::INFINITY,
            Double::ZERO.recip();
        recip_neg_zero:
            Double::NEG_INFINITY,
            Double::NEG_ZERO.recip();
        recip_inf:
            Double::ZERO,
            Double::INFINITY.recip();
        recip_neg_inf:
            Double::NEG_ZERO,
            Double::NEG_INFINITY.recip();
        recip_nan:
            Double::NAN,
            Double::NAN.recip();
    );
}
