// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common as c;
use crate::double::Double;

const INV_K: Double = Double(0.001953125, 0.0); //   1/512, used for exp

impl Double {
    /// Computes the exponential function, *e*<sup>x</sup>, where *x* is this `Double`.
    ///
    /// The result of this function grows rapidly. Once *x* exceeds 708, the result is too
    /// large to represent with a `Double`; at that point the function begins to return
    /// [`INFINITY`]. The limit on the low end is less due to the fact that the second
    /// component needs to fit in an `f64` rather than the first, along with extra bits used
    /// in argument reduction; this function begins to return 0 at -600.
    ///
    /// As *x* grows this function does lose a bit of precision. It's precise to at least 30
    /// digits up to values of -140 <= x <= 150, and from then until the limits, it's
    /// precise to at least 29 digits.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(2).exp();
    /// let expected = dd!("7.3890560989306502272304274605750057");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    #[allow(clippy::many_single_char_names)]
    pub fn exp(self) -> Double {
        match self.pre_exp() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // We first reduce the range of the argument to a convenient size to perform
                // the calculation efficiently. This reduction takes advantage of the
                // following identity.
                //
                //      exp(kx) = exp(x)^k
                //
                // We in fact go a little further because it makes the reduction easier.
                //
                //      exp(kx + m * ln(2)) = 2^m * exp(x)^k
                //
                // where m and k are arbitary integers. By choosing m appropriately we can
                // make |kx| <= ln(2) / 2 = 0.347. Then exp(x) is evaluated using a Taylor
                // series, which for exp(x) is pleasantly easy:
                //
                //      exp(x) = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
                //
                // Reducing x substantially speeds up the convergence, so we have to use
                // fewer terms to reach the required precision.
                //
                // Once we have executed the Taylor series to produce an intermediate
                // answer, we expand it to compensate for the earlier reduction.

                // k = 512 is chosen; INV_K is defined above as that reciprocal
                let eps = c::mul_pwr2(Double::EPSILON, INV_K.0);
                // m doesn't need to be *that* accurate, so we calculate it with f64
                // arithmetic instead of the more expensive Double arithmetic
                let m = (self.0 / Double::LN_2.0 + 0.5).floor();

                // solving for x in exp(kx + m * ln(2)). INV_K is a power of 2 so we could
                // use mul_exp2, but on larger numbers that causes a loss of precision when
                // used with negative powers of two because bits are being shifted to the
                // right without accounting for the ones that are lost off the right.
                let x = (self - Double::LN_2 * Double(m, 0.0)) * INV_K;

                // This is the "x + x^2/2! + x^3/3!" part of the Taylor series.
                let mut p = x.sqr();
                let mut r = x + c::mul_pwr2(p, 0.5);
                p *= x;
                let mut t = p * c::INV_FACTS[0];
                let mut i = 0;

                // This is the rest of the Taylor series. We perform it as many times as
                // we need to reach our desired precision.
                loop {
                    r += t;
                    p *= x;
                    i += 1;
                    t = p * c::INV_FACTS[i];
                    if i >= 5 || t.abs() <= eps {
                        break;
                    }
                }

                // Add the Taylor series parts together, then expand by the same number of
                // times that we reduced earlier.
                r += t;

                // mul_pwr2 can be used here because multiplication doesn't lose precision
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();
                r = c::mul_pwr2(r, 2.0) + r.sqr();

                // Finally, add the "1 +" part of the Taylor series.
                r += Double::ONE;

                // Final step of expansion, this is the "* 2^m" part
                r.ldexp(m as i32)
            }
        }
    }

    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the `Double`.
    ///
    /// This calculation relies upon the [`exp`] calculation, in the opposite direction. A
    /// large positive logarithm, for example, will require the calculation of a large
    /// negative exponential.
    ///
    /// For the same reasons that negative values of [`exp`] are limited to -600, the
    /// accurate results of this function are limited to the number whose logarithm is 600,
    /// which is around 2.65 &times; 10<sup>261</sup>. Take care with this; unlike in
    /// [`exp`], [`INFINITY`] is *not* returned. In that function, exceeding the maximum
    /// refers to actually overflowing an `f64`, which is appropriate to call [`INFINITY`];
    /// here, it means `601`.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(7).ln();
    /// let expected = dd!("1.9459101490553133051053527434432");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// ```
    ///
    /// [`exp`]: #method.exp
    /// [`INFINITY`]: #associatedconstant.INFINITY
    pub fn ln(self) -> Double {
        match self.pre_ln() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // The Taylor series for logarithms converges much more slowly than that of
                // exp because of the lack of a factorial term in the denominator. Hence
                // this routine instead tries to determine the root of the function
                //
                //      f(x) = exp(x) - a
                //
                // using Newton's iteration. This iteration is given by
                //
                //      x' = x - f(x)/f'(x)              (general Newton's iteration)
                //         = x - (exp(x) - a) / exp(x)
                //         = x - (1 - a / exp(x))
                //         = x - (1 - a * exp(-x))
                //         = x + a * exp(-x) - 1
                //
                // Because the derivative of exp(x) is exp(x), this is perhaps the simplest
                // of all Newton iterations.
                let mut x = Double(self.0.ln(), 0.0); // initial approximation

                let k = x.0.abs().log2().floor() as i32;
                let eps = c::mul_pwr2(Double::EPSILON, 2f64.powi(k + 2));

                let mut i = 0;
                loop {
                    let r = x + self * (-x).exp() - Double::ONE;
                    if (x - r).abs() < eps || i > 5 {
                        return r;
                    }
                    x = r;
                    i += 1;
                }
            }
        }
    }

    /// Calculates the base-10 logarithm, log<sub>10</sub>, of the `Double`.
    ///
    /// As with [`ln`], this has an upper usable range less than the size of the numbers
    /// themselves. In this case, that upper limit is around 10<sup>261</sup>. Over this
    /// number, the output is not reliable, but it does not return [`INFINITY`] because the
    /// number 261 is so plainly not infinite.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::E.log10();
    /// let expected = dd!("0.434294481903251827651128918916605");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    /// [`ln`]: #method.ln
    #[inline]
    pub fn log10(self) -> Double {
        self.ln() / Double::LN_10
    }

    /// Calculates the base-2 logarithm, log<sub>2</sub>, of the `Double`.
    ///
    /// Since 2 is smaller than *e*, this function is constrained even more than [`ln`]. It
    /// will start returning [`NEG_INFINITY`] at around 10<sup>-213</sup> and will start
    /// to fail on the positive side at around 2.6 &times; 10<sup>180</sup>.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(10).log2();
    /// let expected = dd!("3.32192809488736234787031942948939");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// ```
    ///
    /// [`ln`]: #method.ln
    /// [`NEG_INFINITY`]: #associatedconstant.NEG_INFINITY
    #[inline]
    pub fn log2(self) -> Double {
        self.ln() / Double::LN_2
    }

    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the `Double`.
    ///
    /// This function will have limits at extreme arguments like the other logarithm
    /// functions. The difference is that those limits will depend on the base argument.
    ///
    /// If the goal is to calculate the base *e*, base 2, or base 10 logarithms of `self`,
    /// the specialized functions for those purposes([`ln`], [`log2`], and [`log10`]
    /// respectively) will be more efficient.
    ///
    /// # Examples
    /// ```
    /// # use qd::dd;
    /// let x = dd!(10).log(dd!(7.0));
    /// let expected = dd!("1.18329466245493832681792856164686");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-29));
    /// ```
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    #[inline]
    pub fn log(self, b: Double) -> Double {
        match self.pre_log(&b) {
            Some(r) => r,
            None => self.ln() / b.ln(),
        }
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
    fn pre_exp(&self) -> Option<Double> {
        if self.0 < -600.0 {
            Some(Double::ZERO)
        } else if self.0 > 708.0 {
            Some(Double::INFINITY)
        } else if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(Double::ONE)
        } else if *self == Double::ONE {
            Some(Double::E)
        } else {
            None
        }
    }

    #[inline]
    fn pre_ln(&self) -> Option<Double> {
        if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_sign_negative() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(Double::NEG_INFINITY)
        } else if self.is_infinite() {
            Some(Double::INFINITY)
        } else if *self == Double::ONE {
            Some(Double::ZERO)
        } else {
            None
        }
    }

    #[inline]
    fn pre_log(&self, b: &Double) -> Option<Double> {
        if self.is_nan() {
            Some(Double::NAN)
        } else if b.is_sign_negative() || b.is_zero() {
            Some(Double::NAN)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // exp tests
    test_all_near!(
        exp_small_1:
            dd!("1.0644944589178594295633905946428894"),
            dd!(0.0625).exp();
        exp_small_2:
            dd!("1.1331484530668263168290072278117932"),
            dd!(0.125).exp();
        exp_small_3:
            dd!("1.2062302494209807106555860104464342"),
            dd!(0.1875).exp();
        exp_small_4:
            dd!("1.2840254166877414840734205680624368"),
            dd!(0.25).exp();

        exp_neg_small_1:
            dd!("0.93941306281347578611971082462230501"),
            dd!(-0.0625).exp();
        exp_neg_small_2:
            dd!("0.88249690258459540286489214322905049"),
            dd!(-0.125).exp();
        exp_neg_small_3:
            dd!("0.82902911818040034301464550934308218"),
            dd!(-0.1875).exp();
        exp_neg_small_4:
            dd!("0.77880078307140486824517026697832046"),
            dd!(-0.25).exp();

        exp_pi:
            dd!("23.140692632779269005729086367948552"),
            Double::PI.exp();
        exp_e:
            dd!("15.154262241479264189760430272629902"),
            Double::E.exp();
        exp_neg_pi:
            dd!("0.043213918263772249774417737171728016"),
            (-Double::PI).exp();
        exp_neg_e:
            dd!("0.065988035845312537076790187596846535"),
            (-Double::E).exp();
        exp_pi_2:
            dd!("4.8104773809653516554730356667038329"),
            Double::FRAC_PI_2.exp();
        exp_sqrt_2:
            dd!("4.113250378782927517173581815140309"),
            Double::SQRT_2.exp();
        exp_1_sqrt_2:
            dd!("2.0281149816474724511081261127463503"),
            Double::FRAC_1_SQRT_2.exp();

        exp_id_e:
            Double::E,
            Double::E.ln().exp();
        exp_id_1:
            Double::ONE,
            Double::ONE.ln().exp();
        exp_1:
            Double::E,
            Double::ONE.exp();
        exp_1p:
            dd!("2.7182818284590452353630057531811221"),
            dd!("1.000000000000000000001").exp();
        exp_1m:
            dd!("2.7182818284590452353575691895242041"),
            dd!("0.999999999999999999999").exp();
        exp_10:
            dd!("22026.465794806716516957900645284255"),
            dd!(10).exp();
        exp_neg_9:
            dd!("0.00012340980408667954949763669073003385"),
            dd!(-9).exp();
    );
    test_all_prec!(
        exp_2_pi:
            dd!("535.49165552476473650304932958904745"),
            Double::TAU.exp(),
            30;
        exp_150:
            dd!("1.3937095806663796973183419371414568e+65"),
            dd!(150).exp(),
            30;
        exp_neg_140:
            dd!("1.5804200602736129648293184125529729e-61"),
            dd!(-140).exp(),
            30;
        exp_neg_limit:
            dd!("2.6503965530043108163386794472695841e-261"),
            dd!(-600).exp(),
            29;
        exp_700:
            dd!("1.0142320547350045094553295952312673e+304"),
            dd!(700).exp(),
            29;
        exp_limit:
            dd!("3.0233831442760550147756219850967309e+307"),
            dd!(708).exp(),
            29;
    );
    test_all_exact!(
        exp_below:
            Double::ZERO,
            dd!(-710).exp();
        exp_above:
            Double::INFINITY,
            dd!(710).exp();
        exp_0:
            Double::ONE,
            Double::ZERO.exp();
        exp_neg_0:
            Double::ONE,
            Double::NEG_ZERO.exp();
        exp_inf:
            Double::INFINITY,
            Double::INFINITY.exp();
        exp_neg_inf:
            Double::ZERO,
            Double::NEG_INFINITY.exp();
        exp_nan:
            Double::NAN,
            Double::NAN.exp();
    );

    // ln tests
    test_all_near!(
        ln_pi:
            dd!("1.1447298858494001741434273513531"),
            Double::PI.ln();
        ln_e:
            Double::ONE,
            Double::E.ln();
        ln_2_pi:
            dd!("1.8378770664093454835606594728112"),
            Double::TAU.ln();
        ln_pi_2:
            dd!("0.45158270528945486472619522989488"),
            Double::FRAC_PI_2.ln();
        ln_sqrt_2:
            dd!("0.34657359027997265470861606072909"),
            Double::SQRT_2.ln();
        ln_1_sqrt_2:
            dd!("-0.34657359027997265470861606072909"),
            Double::FRAC_1_SQRT_2.ln();
        ln_30:
            dd!("69.077552789821370520539743640530881"),
            dd!("1e30").ln();
        ln_neg_30:
            dd!("-69.077552789821370520539743640530881"),
            dd!("1e-30").ln();
        ln_250:
            dd!("575.64627324851142100449786367109143"),
            dd!("1e250").ln();
        ln_neg_290:
            dd!("-667.7496769682732483652175218584658"),
            dd!("1e-290").ln();
    );
    test_all_exact!(
        ln_neg_pi:
            Double::NAN,
            (-Double::PI).ln();
        ln_neg_e:
            Double::NAN,
            (-Double::E).ln();
        ln_1:
            Double::ZERO,
            Double::ONE.ln();
        ln_0:
            Double::NEG_INFINITY,
            Double::ZERO.ln();
        ln_neg_0:
            Double::NAN,
            Double::NEG_ZERO.ln();
        ln_inf:
            Double::INFINITY,
            Double::INFINITY.ln();
        ln_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.ln();
        ln_nan:
            Double::NAN,
            Double::NAN.ln();
    );

    // log10 tests
    test_all_near!(
        log10_pi:
            dd!("0.49714987269413385435126828829089873"),
            Double::PI.log10();
        log10_e:
            dd!("0.4342944819032518276511289189166051"),
            Double::E.log10();
        log10_2_pi:
            dd!("0.7981798683581150495650071830153917"),
            Double::TAU.log10();
        log10_pi_2:
            dd!("0.19611987703015265913752939356640576"),
            Double::FRAC_PI_2.log10();
        log10_sqrt_2:
            dd!("0.15051499783199059760686944736224668"),
            Double::SQRT_2.log10();
        log10_1_sqrt_2:
            dd!("-0.15051499783199059760686944736224687"),
            Double::FRAC_1_SQRT_2.log10();
        log10_30:
            dd!("30.0"),
            dd!("1e30").log10();
        log10_neg_30:
            dd!("-30.0"),
            dd!("1e-30").log10();
        log10_260:
            dd!("260.41497334797081796442024405266689"),
            dd!("2.6e260").log10();
        log10_neg_190:
            dd!("-290.0"),
            dd!("1e-290").log10();
    );
    test_all_exact!(
        log10_neg_pi:
            Double::NAN,
            (-Double::PI).log10();
        log10_neg_e:
            Double::NAN,
            (-Double::E).log10();
        log10_1:
            Double::ZERO,
            Double::ONE.log10();
        log10_0:
            Double::NEG_INFINITY,
            Double::ZERO.log10();
        log10_neg_0:
            Double::NAN,
            Double::NEG_ZERO.log10();
        log10_inf:
            Double::INFINITY,
            Double::INFINITY.log10();
        log10_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.log10();
        log10_nan:
            Double::NAN,
            Double::NAN.log10();
    );

    // log2 tests
    test_all_near!(
        log2_pi:
            dd!("1.6514961294723187980432792951080072"),
            Double::PI.log2();
        log2_e:
            dd!("1.4426950408889634073599246810018917"),
            Double::E.log2();
        log2_2_pi:
            dd!("2.6514961294723187980432792951080087"),
            Double::TAU.log2();
        log2_pi_2:
            dd!("0.65149612947231879804327929510800716"),
            Double::FRAC_PI_2.log2();
        log2_sqrt_2:
            dd!("0.5"),
            Double::SQRT_2.log2();
        log2_1_sqrt_2:
            dd!("-0.5"),
            Double::FRAC_1_SQRT_2.log2();
        log2_30:
            dd!("99.657842846620870436109582884681684"),
            dd!("1e30").log2();
        log2_neg_30:
            dd!("-99.657842846620870436109582884681684"),
            dd!("1e-30").log2();
        log2_180:
            dd!("599.32556870297895242918399053285747"),
            dd!("2.6e180").log2();
        log2_neg_213:
            dd!("-707.57068421100818009637803848124024"),
            dd!("1e-213").log2();
    );
    test_all_exact!(
        log2_neg_pi:
            Double::NAN,
            (-Double::PI).log2();
        log2_neg_e:
            Double::NAN,
            (-Double::E).log2();
        log2_1:
            Double::ZERO,
            Double::ONE.log2();
        log2_0:
            Double::NEG_INFINITY,
            Double::ZERO.log2();
        log2_neg_0:
            Double::NAN,
            Double::NEG_ZERO.log2();
        log2_inf:
            Double::INFINITY,
            Double::INFINITY.log2();
        log2_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.log2();
        log2_nan:
            Double::NAN,
            Double::NAN.log2();
    );

    // log tests
    test_all_near!(
        log_pi:
            dd!("1.6514961294723187980432792951080072"),
            Double::PI.log(dd!(2.0));
        log_e:
            dd!("0.87356852683023186835397746476334251"),
            Double::E.log(Double::PI);
        log_2_pi:
            dd!("1.8378770664093454835606594728112364"),
            Double::TAU.log(Double::E);
        log_pi_2:
            dd!("0.19611987703015265913752939356640576"),
            Double::FRAC_PI_2.log(dd!(10.0));
        log_sqrt_2:
            dd!("0.12159929443072307483899992782618255"),
            Double::SQRT_2.log(dd!(17.29));
        log_1_sqrt_2:
            dd!("0.075257498915995298803434723681123436"),
            Double::FRAC_1_SQRT_2.log(dd!(0.01));
        log_30:
            dd!("99.657842846620870436109582884681684"),
            dd!("1e30").log(dd!(2.0));
        log_neg_30:
            dd!("-60.343976027641828162941661418661277"),
            dd!("1e-30").log(Double::PI);
        log_180:
            dd!("415.4208281839556594846911899515249"),
            dd!("2.6e180").log(Double::E);
        log_neg_213:
            dd!("-213.0"),
            dd!("1e-213").log(dd!(10.0));
    );
    test_all_exact!(
        log_neg_pi:
            Double::NAN,
            (-Double::PI).log(dd!(2.0));
        log_neg_e:
            Double::NAN,
            (-Double::E).log(Double::PI);
        log_1:
            Double::ZERO,
            Double::ONE.log(Double::E);
        log_0:
            Double::NEG_INFINITY,
            Double::ZERO.log(dd!(10.0));
        log_neg_0:
            Double::NAN,
            Double::NEG_ZERO.log(dd!(3.2));
        log_inf:
            Double::INFINITY,
            Double::INFINITY.log(dd!(7.1));
        log_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.log(dd!(3.0));
        log_nan:
            Double::NAN,
            Double::NAN.log(dd!(5.0));
        log_base_nan:
            Double::NAN,
            Double::PI.log(Double::NAN);
    );
}
