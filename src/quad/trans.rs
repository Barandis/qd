// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common as c;
use crate::quad::Quad;

const FRAC_1_65536: f64 = 1.52587890625e-05; //   1/65536, used for exp

impl Quad {
    /// Computes the exponential function, *e*<sup>x</sup>, where *x* is this `Quad`.
    ///
    /// The result of this function grows rapidly. Once *x* exceeds 708, the result is too
    /// large to represent with a `Quad`; at that point the function begins to return
    /// [`INFINITY`]. The limit on the low end is less due to the fact that the second,
    /// third, and fourth components need to fit in an `f64` rather than the first, along
    /// with extra bits used in argument reduction; this function begins to return 0 at
    /// -460.
    ///
    /// As *x* grows this function does lose a bit of precision. It's precise to at least 60
    /// digits up to values of -140 <= x <= 150, and from then until the limits, it's
    /// precise to at least 59 digits.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(2.3).exp();
    /// let expected = qd!("9.974182454814720739957615156908858001478701193684029563691421917");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-59));
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    #[allow(clippy::many_single_char_names)]
    pub fn exp(self) -> Quad {
        match self.pre_exp() {
            Some(r) => r,
            None => {
                // Strategy:
                //
                // We use the Taylor series, which for e^x is defined as
                //
                //      e^x = 1 + x + x^2/2! + x^3/3! + x^4/4! ...
                //
                // This works well for the exponential function, as the factorials in the
                // denominator mean the series converges relatively quickly.
                //
                // But it converges much *more* quickly for small numbers. We can use this,
                // along with mathematical identities, to make the number we use to do the
                // calculation much smaller.
                //
                //     (1) e^(a + b) = e^a · e^b
                //
                // We can then choose a and b just so in order to make e^a very easy to
                // calculate and to make b small.
                //
                // In fact, it so happens that if we choose a = k ln 2 and b = r, then we
                // have the following identities, based on (1) above.
                //
                //     (2) e^x = e^(k ln 2 + r) = e^(k ln 2) · e^r = 2^k · e^r
                //
                // This is one reason we made that particular choice of k ln 2; because
                // raising e to that power results in 2^k, which is really easy to compute,
                // especially when k is in integer. The other reason is that if we figure
                // out the integer k that gets k ln 2 as close as possible to x, then |r| <=
                // (ln 2) / 2, which qualifies as a "small number" (it's < 0.347).
                //
                // We can find k by ignoring r for the moment and solving x = k ln 2 for
                // kbeing an integer by rounding:
                //
                //     (3) k = round(x / ln 2) = floor((x / ln 2) + 1/2)
                //
                // Then we can solve x = k ln 2 + r for r, using this now-known value for k.
                //
                //     (4) r = x - k ln 2
                //
                // We can now use the Taylor series to compute the much smaller (and faster)
                // e^r, and after we have that answer, multiply it by 2^k (from (2) above)
                // for the final answer.

                // The implementation of equation (3). Since k is going to be an integer
                // anyway and doesn't therefore require Quad precision, we use regular f64
                // arithmetic.
                let k = (self.0 / Quad::LN_2.0 + 0.5).floor();

                // The implementation of equation (4). We actually go further here by
                // halving the answer 16 more times (FRAC_1_65536 is (1/2)^16), using the
                // identity
                //
                //     (5) exp(2x) = exp(x)^2
                //
                // We'll expand this later.
                let r = c::mul_pwr2(self - Quad::LN_2 * Quad(k, 0.0, 0.0, 0.0), FRAC_1_65536);

                // This is the "x + x^2/2! + x^3/3!" part of the Taylor series.
                let mut p = r.sqr();
                let mut s = r + c::mul_pwr2(p, 0.5);
                p *= r;
                let mut t = p * c::INV_FACTS[0];
                let mut i = 0;

                // This is the rest of the Taylor series. We perform it as many times as
                // we need to reach our desired precision.
                loop {
                    s += t;
                    p *= r;
                    i += 1;
                    t = p * c::INV_FACTS[i];
                    if i >= 9 || t.abs() <= Quad::EPSILON {
                        break;
                    }
                }
                s += t;

                // This is the expansion based on equation (5). We do it 16 times because
                // halving was done 16 times (same as multiplying by (1/2)^16).
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();
                s = c::mul_pwr2(s, 2.0) + s.sqr();

                // Finally, add the "1 +" part of the Taylor series.
                s += Quad::ONE;

                // Multiply by 2^k. This is the implementation of the expansion in equation
                // (2). The ldexp function is defined as multiplying the number (s in this
                // case) by 2 raised to the power of its argument (k in this case).
                s.ldexp(k as i32)
            }
        }
    }

    /// Calculates the natural logarithm, log<sub>*e*</sub>, of the `Quad`.
    ///
    /// This calculation relies upon the [`exp`] calculation, in the opposite direction. A
    /// large positive logarithm, for example, will require the calculation of a large
    /// negative exponential.
    ///
    /// For the same reasons that negative values of [`exp`] are limited to -470, the
    /// accurate results of this function are limited to the number whose logarithm is 460,
    /// which is around 2 &times; 10<sup>200</sup>. Take care with this; unlike in
    /// [`exp`], [`INFINITY`] is *not* returned. In that function, exceeding the maximum
    /// refers to actually overflowing an `f64`, which is appropriate to call [`INFINITY`];
    /// here, it means `470`.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(7).ln();
    /// let expected = qd!("1.945910149055313305105352743443179729637084729581861188459390150");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    ///
    /// [`exp`]: #method.exp
    /// [`INFINITY`]: #associatedconstant.INFINITY
    pub fn ln(self) -> Quad {
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
                let mut x = Quad(self.0.ln(), 0.0, 0.0, 0.0); // initial approximation

                let k = x.0.abs().log2().floor() as i32;
                let eps = c::mul_pwr2(Quad::EPSILON, 2f64.powi(k + 2));

                let mut i = 0;
                loop {
                    let r = x + self * (-x).exp() - Quad::ONE;
                    if (x - r).abs() < eps || i > 5 {
                        return r;
                    }
                    x = r;
                    i += 1;
                }
            }
        }
    }

    /// Calculates the base-10 logarithm, log<sub>10</sub>, of the `Quad`.
    ///
    /// As with [`ln`], this has an upper usable range less than the size of the numbers
    /// themselves. In this case, that upper limit is around 10<sup>200</sup>. Over this
    /// number, the output is not reliable, but it does not return [`INFINITY`] because the
    /// number 200 is so plainly not infinite.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::E.log10();
    /// let expected = qd!("0.4342944819032518276511289189166050822943970058036665661144537832");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    /// [`ln`]: #method.ln
    #[inline]
    pub fn log10(self) -> Quad {
        self.ln() / Quad::LN_10
    }

    /// Calculates the base-2 logarithm, log<sub>2</sub>, of the `Quad`.
    ///
    /// Since 2 is smaller than *e*, this function is constrained even more than [`ln`]. It
    /// will start returning [`NEG_INFINITY`] at around 10<sup>-213</sup> and will start
    /// to fail on the positive side at around 2.6 &times; 10<sup>180</sup>.
    ///
    /// # Examples
    /// ```
    /// # use qd::qd;
    /// let x = qd!(10).log2();
    /// let expected = qd!("3.321928094887362347870319429489390175864831393024580612054756396");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-59));
    /// ```
    ///
    /// [`ln`]: #method.ln
    /// [`NEG_INFINITY`]: #associatedconstant.NEG_INFINITY
    #[inline]
    pub fn log2(self) -> Quad {
        self.ln() / Quad::LN_2
    }

    /// Calculates the base `b` logarithm (log<sub>`b`</sub>) of the `Quad`.
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
    /// # use qd::qd;
    /// let x = qd!(10).log(qd!(7.3));
    /// let expected = qd!("1.158315209978887965104764376269736420106652944692834002126233653");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-59));
    /// ```
    ///
    /// [`ln`]: #method.ln
    /// [`log2`]: #method.log2
    /// [`log10`]: #method.log10
    #[inline]
    pub fn log(self, b: Quad) -> Quad {
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
    fn pre_exp(&self) -> Option<Quad> {
        if self.0 < -470.0 {
            Some(Quad::ZERO)
        } else if self.0 >= 709.0 {
            Some(Quad::INFINITY)
        } else if self.is_nan() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            Some(Quad::ONE)
        } else if *self == Quad::ONE {
            Some(Quad::E)
        } else {
            None
        }
    }

    #[inline]
    fn pre_ln(&self) -> Option<Quad> {
        if self.is_nan() {
            Some(Quad::NAN)
        } else if self.is_sign_negative() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            Some(Quad::NEG_INFINITY)
        } else if self.is_infinite() {
            Some(Quad::INFINITY)
        } else if *self == Quad::ONE {
            Some(Quad::ZERO)
        } else {
            None
        }
    }

    #[inline]
    fn pre_log(&self, b: &Quad) -> Option<Quad> {
        if self.is_nan() {
            Some(Quad::NAN)
        } else if b.is_sign_negative() || b.is_zero() {
            Some(Quad::NAN)
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
            qd!("1.064494458917859429563390594642889673100725443649353301519307510635"),
            qd!(0.0625).exp();
        exp_small_2:
            qd!("1.1331484530668263168290072278117938725655031317451816259128200360786"),
            qd!(0.125).exp();
        exp_small_3:
            qd!("1.2062302494209807106555860104464335480403936461999703807388699348397"),
            qd!(0.1875).exp();
        exp_small_4:
            qd!("1.2840254166877414840734205680624364583362808652814630892175072968728"),
            qd!(0.25).exp();

        exp_neg_small_1:
            qd!("0.93941306281347578611971082462230508452468089054944182200949266205356"),
            qd!(-0.0625).exp();
        exp_neg_small_2:
            qd!("0.88249690258459540286489214322905073622200482499065074177030931920782"),
            qd!(-0.125).exp();
        exp_neg_small_3:
            qd!("0.82902911818040034301464550934308186242538840928345113275699093884012"),
            qd!(-0.1875).exp();
        exp_neg_small_4:
            qd!("0.7788007830714048682451702669783206472967722904261414742413173662682"),
            qd!(-0.25).exp();

        exp_pi:
            qd!("23.140692632779269005729086367948547380266106242600211993445046409512"),
            Quad::PI.exp();
        exp_e:
            qd!("15.154262241479264189760430272629911905528548536856139769140746405908"),
            Quad::E.exp();
        exp_neg_pi:
            qd!("0.043213918263772249774417737171728011275728109810633082980719687401078"),
            (-Quad::PI).exp();
        exp_neg_e:
            qd!("0.065988035845312537076790187596846424938577048252796436402473541566735"),
            (-Quad::E).exp();
        exp_2_pi:
            qd!("535.49165552476473650304932958904718147780579760329491550720525503625"),
            Quad::TAU.exp();
        exp_pi_2:
            qd!("4.8104773809653516554730356667038331263901708746645349400208154892392"),
            Quad::FRAC_PI_2.exp();
        exp_sqrt_2:
            qd!("4.1132503787829275171735818151403045024016639431511096100683647098487"),
            Quad::SQRT_2.exp();
        exp_1_sqrt_2:
            qd!("2.0281149816474724511081261127463511751743250925426135206177759721242"),
            Quad::FRAC_1_SQRT_2.exp();
        exp_150:
            qd!("139370958066637969731834193714145747747369006140218438233756444835.63"),
            qd!(150).exp();
        exp_neg_140:
            qd!("1.5804200602736129648293184125529729370695576094760652601359869282413e-61"),
            qd!(-140).exp();

        exp_id_e:
            Quad::E,
            Quad::E.ln().exp();
        exp_id_1:
            Quad::ONE,
            Quad::ONE.ln().exp();
        exp_1:
            Quad::E,
            Quad::ONE.exp();
        exp_1p:
            qd!("2.7182818284590452353602874713526624977572742765182441654193212305986"),
            qd!("1.00000000000000000000000000000000000000001").exp();
        exp_1m:
            qd!("2.7182818284590452353602874713526624977572199108816749845146140248492"),
            qd!("0.99999999999999999999999999999999999999999").exp();
        exp_10:
            qd!("22026.465794806716516957900645284244366353512618556781074235426355234"),
            qd!(10).exp();
        exp_neg_9:
            qd!("0.00012340980408667954949763669073003382607215283228893905253448204514523"),
            qd!(-9).exp();
        exp_700:
            qd!("1.0142320547350045094553295952312676152046795722430733487805362812495e+304"),
            qd!(700).exp();
    );
    test_all_prec!(
        exp_neg_limit:
            qd!("3.6938830684872562187934275245707479984912684809256483768734951290934e-196"),
            qd!(-450).exp(),
            60;
        exp_limit:
            qd!("3.0233831442760550147756219850967309958990319946798820666918417985884e+307"),
            qd!(708).exp(),
            61;
    );
    test_all_exact!(
        exp_below:
            Quad::ZERO,
            qd!(-710).exp();
        exp_above:
            Quad::INFINITY,
            qd!(710).exp();
        exp_0:
            Quad::ONE,
            Quad::ZERO.exp();
        exp_neg_0:
            Quad::ONE,
            Quad::NEG_ZERO.exp();
        exp_inf:
            Quad::INFINITY,
            Quad::INFINITY.exp();
        exp_neg_inf:
            Quad::ZERO,
            Quad::NEG_INFINITY.exp();
        exp_nan:
            Quad::NAN,
            Quad::NAN.exp();
    );

    // ln tests
    test_all_near!(
        ln_pi:
            qd!("1.1447298858494001741434273513530587116472948129153115715136230714717"),
            Quad::PI.ln();
        ln_e:
            Quad::ONE,
            Quad::E.ln();
        ln_2_pi:
            qd!("1.8378770664093454835606594728112352797227949472755668256343030809653"),
            Quad::TAU.ln();
        ln_pi_2:
            qd!("0.45158270528945486472619522989488214357179467855505631739294306197847"),
            Quad::FRAC_PI_2.ln();
        ln_sqrt_2:
            qd!("0.34657359027997265470861606072908828403775006718012762706034000474648"),
            Quad::SQRT_2.ln();
        ln_1_sqrt_2:
            qd!("-0.34657359027997265470861606072908828403775006718012762706034000474619"),
            Quad::FRAC_1_SQRT_2.ln();
        ln_30:
            qd!("69.077552789821370520539743640530926228033044658863189280999837029056"),
            qd!("1e30").ln();
        ln_neg_30:
            qd!("-69.077552789821370520539743640530926228033044658863189280999837029056"),
            qd!("1e-30").ln();
        ln_170:
            qd!("391.4394658089877662830585472963419152921872530668914059256657431644"),
            qd!("1e170").ln();
        ln_neg_250:
            qd!("-575.64627324851142100449786367109105190027537215719324400833197524208"),
            qd!("1e-250").ln();
    );
    test_all_exact!(
        ln_neg_pi:
            Quad::NAN,
            (-Quad::PI).ln();
        ln_neg_e:
            Quad::NAN,
            (-Quad::E).ln();
        ln_1:
            Quad::ZERO,
            Quad::ONE.ln();
        ln_0:
            Quad::NEG_INFINITY,
            Quad::ZERO.ln();
        ln_neg_0:
            Quad::NAN,
            Quad::NEG_ZERO.ln();
        ln_inf:
            Quad::INFINITY,
            Quad::INFINITY.ln();
        ln_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.ln();
        ln_nan:
            Quad::NAN,
            Quad::NAN.ln();
    );

    // log10 tests
    test_all_near!(
        log10_pi:
            qd!("0.49714987269413385435126828829089887365167832438044244613405349992497"),
            Quad::PI.log10();
        log10_e:
            qd!("0.43429448190325182765112891891660508229439700580366656611445378316577"),
            Quad::E.log10();
        log10_2_pi:
            qd!("0.79817986835811504956500718301539190041986820584255098744448096105195"),
            Quad::TAU.log10();
        log10_pi_2:
            qd!("0.19611987703015265913752939356640584688348844291833390482362603879769"),
            Quad::FRAC_PI_2.log10();
        log10_sqrt_2:
            qd!("0.15051499783199059760686944736224651338409494073105427065521373056349"),
            Quad::SQRT_2.log10();
        log10_1_sqrt_2:
            qd!("-0.15051499783199059760686944736224651338409494073105427065521373056334"),
            Quad::FRAC_1_SQRT_2.log10();
        log10_30:
            qd!("30.0"),
            qd!("1e30").log10();
        log10_neg_30:
            qd!("-30.0"),
            qd!("1e-30").log10();
        log10_200:
            qd!("200.41497334797081796442024405266682145759791906984917681311161843622"),
            qd!("2.6e200").log10();
        log10_neg_260:
            qd!("-260.0"),
            qd!("1e-260").log10();
    );
    test_all_exact!(
        log10_neg_pi:
            Quad::NAN,
            (-Quad::PI).log10();
        log10_neg_e:
            Quad::NAN,
            (-Quad::E).log10();
        log10_1:
            Quad::ZERO,
            Quad::ONE.log10();
        log10_0:
            Quad::NEG_INFINITY,
            Quad::ZERO.log10();
        log10_neg_0:
            Quad::NAN,
            Quad::NEG_ZERO.log10();
        log10_inf:
            Quad::INFINITY,
            Quad::INFINITY.log10();
        log10_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.log10();
        log10_nan:
            Quad::NAN,
            Quad::NAN.log10();
    );

    // log2 tests
    test_all_near!(
        log2_pi:
            qd!("1.6514961294723187980432792951080073350184769267630415294067885154872"),
            Quad::PI.log2();
        log2_e:
            qd!("1.4426950408889634073599246810018921374266459541529859341354494069313"),
            Quad::E.log2();
        log2_2_pi:
            qd!("2.6514961294723187980432792951080073350184769267630415294067885154884"),
            Quad::TAU.log2();
        log2_pi_2:
            qd!("0.65149612947231879804327929510800733501847692676304152940678851548782"),
            Quad::FRAC_PI_2.log2();
        log2_sqrt_2:
            qd!("0.5"),
            Quad::SQRT_2.log2();
        log2_1_sqrt_2:
            qd!("-0.5"),
            Quad::FRAC_1_SQRT_2.log2();
        log2_30:
            qd!("99.657842846620870436109582884681705275944941790737418361642691874514"),
            qd!("1e30").log2();
        log2_neg_30:
            qd!("-99.657842846620870436109582884681705275944941790737418361642691874514"),
            qd!("1e-30").log2();
        log2_180:
            qd!("599.32556870297895242918399053285753621343325575319096692733985837663"),
            qd!("2.6e180").log2();
        log2_neg_213:
            qd!("-707.57068421100818009637803848124010745920908671423567036766311230853"),
            qd!("1e-213").log2();
    );
    test_all_exact!(
        log2_neg_pi:
            Quad::NAN,
            (-Quad::PI).log2();
        log2_neg_e:
            Quad::NAN,
            (-Quad::E).log2();
        log2_1:
            Quad::ZERO,
            Quad::ONE.log2();
        log2_0:
            Quad::NEG_INFINITY,
            Quad::ZERO.log2();
        log2_neg_0:
            Quad::NAN,
            Quad::NEG_ZERO.log2();
        log2_inf:
            Quad::INFINITY,
            Quad::INFINITY.log2();
        log2_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.log2();
        log2_nan:
            Quad::NAN,
            Quad::NAN.log2();
    );

    // log tests
    test_all_near!(
        log_pi:
            qd!("1.6514961294723187980432792951080073350184769267630415294067885154872"),
            Quad::PI.log(qd!(2.0));
        log_e:
            qd!("0.87356852683023186835397746476334273882072986617613914765231984243114"),
            Quad::E.log(Quad::PI);
        log_2_pi:
            qd!("1.8378770664093454835606594728112352797227949472755668256343030809653"),
            Quad::TAU.log(Quad::E);
        log_pi_2:
            qd!("0.19611987703015265913752939356640584688348844291833390482362603879769"),
            Quad::FRAC_PI_2.log(qd!(10.0));
        log_sqrt_2:
            qd!("0.12159929443072307483899992782618244339949906038928361898477291551813"),
            Quad::SQRT_2.log(qd!(17.29));
        log_1_sqrt_2:
            qd!("0.075257498915995298803434723681123256692047470365527135327606865281672"),
            Quad::FRAC_1_SQRT_2.log(qd!(0.01));
        log_30:
            qd!("99.657842846620870436109582884681705275944941790737418361642691874514"),
            qd!("1e30").log(qd!(2.0));
        log_neg_30:
            qd!("-60.343976027641828162941661418661260900837490622117196618434822573006"),
            qd!("1e-30").log(Quad::PI);
        log_180:
            qd!("415.42082818395565948469118995152468833347793454367082508050541979348"),
            qd!("2.6e180").log(Quad::E);
        log_neg_213:
            qd!("-213.0"),
            qd!("1e-213").log(qd!(10.0));
    );
    test_all_exact!(
        log_neg_pi:
            Quad::NAN,
            (-Quad::PI).log(qd!(2.0));
        log_neg_e:
            Quad::NAN,
            (-Quad::E).log(Quad::PI);
        log_1:
            Quad::ZERO,
            Quad::ONE.log(Quad::E);
        log_0:
            Quad::NEG_INFINITY,
            Quad::ZERO.log(qd!(10.0));
        log_neg_0:
            Quad::NAN,
            Quad::NEG_ZERO.log(qd!(3.2));
        log_inf:
            Quad::INFINITY,
            Quad::INFINITY.log(qd!(7.1));
        log_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.log(qd!(3.0));
        log_nan:
            Quad::NAN,
            Quad::NAN.log(qd!(5.0));
        log_base_nan:
            Quad::NAN,
            Quad::PI.log(Quad::NAN);
    );
}
