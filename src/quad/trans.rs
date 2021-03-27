// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::{tables, Quad};

const INV_K: Quad = Quad(1.52587890625e-05, 0.0, 0.0, 0.0); //   1/65536, used for exp

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
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(2.3).exp();
    /// let expected = qd!("9.974182454814720739957615156908858001478701193684029563691421917");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-59));
    /// # }
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

                // k = 65536 is chosen; INV_K is defined above as that reciprocal
                let threshold = Quad::EPSILON.mul_pwr2(INV_K.0);
                // m doesn't need to be *that* accurate, so we calculate it with f64
                // arithmetic instead of the more expensive Quad arithmetic
                let m = (self.0 / Quad::LN_2.0 + 0.5).floor();

                // solving for x in exp(kx + m * ln(2)). INV_K is a power of 2 so we could
                // use mul_exp2, but on larger numbers that causes a loss of precision when
                // used with negative powers of two because bits are being shifted to the
                // right without accounting for the ones that are lost off the right.
                let x = (self - Quad::LN_2 * Quad(m, 0.0, 0.0, 0.0)) * INV_K;

                // This is the "x + x^2/2! + x^3/3!" part of the Taylor series.
                let mut p = x.sqr();
                let mut r = x + p.mul_pwr2(0.5);
                p *= x;
                let mut t = p * tables::INV_FACTS[0];
                let mut i = 0;

                // This is the rest of the Taylor series. We perform it as many times as
                // we need to reach our desired precision.
                loop {
                    r += t;
                    p *= x;
                    i += 1;
                    t = p * tables::INV_FACTS[i];
                    if i >= 9 || t.abs() <= threshold {
                        break;
                    }
                }

                // Add the Taylor series parts together, then expand by the same number of
                // times that we reduced earlier.
                r += t;

                // mul_pwr2 can be used here because multiplication doesn't lose precision
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();
                r = r.mul_pwr2(2.0) + r.sqr();

                // Finally, add the "1 +" part of the Taylor series.
                r += Quad::ONE;

                // Final step of expansion, this is the "* 2^m" part
                r.ldexp(m as i32)
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
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(7).ln();
    /// let expected = qd!("1.945910149055313305105352743443179729637084729581861188459390150");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
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
                let eps = Quad::EPSILON.mul_pwr2(2f64.powi(k + 2));

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
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = Quad::E.log10();
    /// let expected = qd!("0.4342944819032518276511289189166050822943970058036665661144537832");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
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
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(10).log2();
    /// let expected = qd!("3.321928094887362347870319429489390175864831393024580612054756396");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-59));
    /// # }
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
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(10).log(qd!(7.3));
    /// let expected = qd!("1.158315209978887965104764376269736420106652944692834002126233653");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-59));
    /// # }
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
        if self.0 <= -470.0 {
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

    #[test]
    fn exp() {
        assert_all_close!(
            qd!("1.0644944589178594295633905946428896731007254436493533015193075106"),
            qd!(0.0625).exp();

            qd!("1.1331484530668263168290072278117938725655031317451816259128200361"),
            qd!(0.125).exp();

            qd!("1.2062302494209807106555860104464335480403936461999703807388699348"),
            qd!(0.1875).exp();

            qd!("1.2840254166877414840734205680624364583362808652814630892175072969"),
            qd!(0.25).exp();


            qd!("0.93941306281347578611971082462230508452468089054944182200949266205"),
            qd!(-0.0625).exp();

            qd!("0.88249690258459540286489214322905073622200482499065074177030931921"),
            qd!(-0.125).exp();

            qd!("0.82902911818040034301464550934308186242538840928345113275699093884"),
            qd!(-0.1875).exp();

            qd!("0.77880078307140486824517026697832064729677229042614147424131736627"),
            qd!(-0.25).exp();


            qd!("23.14069263277926900572908636794854738026610624260021199344504641"),
            Quad::PI.exp();

            qd!("15.154262241479264189760430272629911905528548536856139769140746406"),
            Quad::E.exp();

            qd!("0.043213918263772249774417737171728011275728109810633082980719687401"),
            (-Quad::PI).exp();

            qd!("0.065988035845312537076790187596846424938577048252796436402473541567"),
            (-Quad::E).exp();

            qd!("535.49165552476473650304932958904718147780579760329491550720525504"),
            Quad::MUL_2_PI.exp();

            qd!("4.8104773809653516554730356667038331263901708746645349400208154892"),
            Quad::FRAC_PI_2.exp();

            qd!("4.1132503787829275171735818151403045024016639431511096100683647098"),
            Quad::SQRT_2.exp();

            qd!("2.0281149816474724511081261127463511751743250925426135206177759721"),
            Quad::FRAC_1_SQRT_2.exp();

            qd!("1.3937095806663796973183419371414574774736900614021843823375644484e+65"),
            qd!(150).exp();

            qd!("1.5804200602736129648293184125529729370695576094760652601359869282e-61"),
            qd!(-140).exp();


            Quad::E, Quad::E.ln().exp();
            Quad::ONE, Quad::ONE.ln().exp();
            Quad::E, Quad::ONE.exp();

            qd!("2.7182818284590452353602874713526624977572742765182441654193212306"),
            qd!("1.00000000000000000000000000000000000000001").exp();

            qd!("2.7182818284590452353602874713526624977572199108816749845146140248"),
            qd!("0.99999999999999999999999999999999999999999").exp();

            qd!("22026.465794806716516957900645284244366353512618556781074235426355"),
            qd!(10).exp();

            qd!("0.00012340980408667954949763669073003382607215283228893905253448204515"),
            qd!(-9).exp();
        );
        assert_precision_all!(
            qd!("1.6770203186015345280337085962223442506480949349167199078439535946e-200"),
            qd!(-460).exp(), 59;

            qd!("1.0142320547350045094553295952312676152046795722430733487805362812e+304"),
            qd!(700).exp(), 59;

            qd!("3.0233831442760550147756219850967309958990319946798820666918417986e+307"),
            qd!(708).exp(), 59;
        );
        assert_all_exact!(
            Quad::ZERO, qd!(-710).exp();
            Quad::INFINITY, qd!(710).exp();
            Quad::ONE, Quad::ZERO.exp();
            Quad::ONE, Quad::NEG_ZERO.exp();
            Quad::INFINITY, Quad::INFINITY.exp();
            Quad::ZERO, Quad::NEG_INFINITY.exp();
            Quad::NAN, Quad::NAN.exp();
        );
    }

    #[test]
    fn ln() {
        assert_all_close!(
            qd!("1.1447298858494001741434273513530587116472948129153115715136230715"),
            Quad::PI.ln();

            Quad::ONE,
            Quad::E.ln();

            qd!("1.837877066409345483560659472811235279722794947275566825634303081"),
            Quad::MUL_2_PI.ln();

            qd!("0.45158270528945486472619522989488214357179467855505631739294306198"),
            Quad::FRAC_PI_2.ln();

            qd!("0.34657359027997265470861606072908828403775006718012762706034000475"),
            Quad::SQRT_2.ln();

            qd!("-0.34657359027997265470861606072908828403775006718012762706034000475"),
            Quad::FRAC_1_SQRT_2.ln();

            qd!("69.077552789821370520539743640530926228033044658863189280999837029"),
            qd!("1e30").ln();

            qd!("-69.077552789821370520539743640530926228033044658863189280999837029"),
            qd!("1e-30").ln();

            qd!("460.51701859880913680359829093687284152022029772575459520666558019"),
            qd!("1e200").ln();

            qd!("-598.67212417845187784467777821793469397628638704348097376866525425"),
            qd!("1e-260").ln();

        );
        assert_all_exact!(
            Quad::NAN, (-Quad::PI).ln();
            Quad::NAN, (-Quad::E).ln();
            Quad::ZERO, Quad::ONE.ln();
            Quad::NEG_INFINITY, Quad::ZERO.ln();
            Quad::NAN, Quad::NEG_ZERO.ln();
            Quad::INFINITY, Quad::INFINITY.ln();
            Quad::NAN, Quad::NEG_INFINITY.ln();
            Quad::NAN, Quad::NAN.ln();
        );
    }

    #[test]
    fn log10() {
        assert_all_close!(
            qd!("0.49714987269413385435126828829089887365167832438044244613405349992"),
            Quad::PI.log10();

            qd!("0.43429448190325182765112891891660508229439700580366656611445378317"),
            Quad::E.log10();

            qd!("0.79817986835811504956500718301539190041986820584255098744448096105"),
            Quad::MUL_2_PI.log10();

            qd!("0.1961198770301526591375293935664058468834884429183339048236260388"),
            Quad::FRAC_PI_2.log10();

            qd!("0.15051499783199059760686944736224651338409494073105427065521373056"),
            Quad::SQRT_2.log10();

            qd!("-0.15051499783199059760686944736224651338409494073105427065521373056"),
            Quad::FRAC_1_SQRT_2.log10();

            qd!("30.0"),
            qd!("1e30").log10();

            qd!("-30.0"),
            qd!("1e-30").log10();

            qd!("200.20411998265592478085495557889797210707275952584843416524170984"),
            qd!("1.6e200").log10();

            qd!("-260.0"),
            qd!("1e-260").log10();
        );
        assert_all_exact!(
            Quad::NAN, (-Quad::PI).log10();
            Quad::NAN, (-Quad::E).log10();
            Quad::ZERO, Quad::ONE.log10();
            Quad::NEG_INFINITY, Quad::ZERO.log10();
            Quad::NAN, Quad::NEG_ZERO.log10();
            Quad::INFINITY, Quad::INFINITY.log10();
            Quad::NAN, Quad::NEG_INFINITY.log10();
            Quad::NAN, Quad::NAN.log10();
        );
    }

    #[test]
    fn log2() {
        assert_all_close!(
            qd!("1.6514961294723187980432792951080073350184769267630415294067885155"),
            Quad::PI.log2();

            qd!("1.4426950408889634073599246810018921374266459541529859341354494069"),
            Quad::E.log2();

            qd!("2.6514961294723187980432792951080073350184769267630415294067885155"),
            Quad::MUL_2_PI.log2();

            qd!("0.65149612947231879804327929510800733501847692676304152940678851549"),
            Quad::FRAC_PI_2.log2();

            qd!("0.5"),
            Quad::SQRT_2.log2();

            qd!("-0.5"),
            Quad::FRAC_1_SQRT_2.log2();

            qd!("99.657842846620870436109582884681705275944941790737418361642691875"),
            qd!("1e30").log2();

            qd!("-99.657842846620870436109582884681705275944941790737418361642691875"),
            qd!("1e-30").log2();

            qd!("599.32556870297895242918399053285753621343325575319096692733985838"),
            qd!("2.6e180").log2();

            qd!("-707.57068421100818009637803848124010745920908671423567036766311231"),
            qd!("1e-213").log2();

        );
        assert_all_exact!(
            Quad::NAN, (-Quad::PI).log2();
            Quad::NAN, (-Quad::E).log2();
            Quad::ZERO, Quad::ONE.log2();
            Quad::NEG_INFINITY, Quad::ZERO.log2();
            Quad::NAN, Quad::NEG_ZERO.log2();
            Quad::INFINITY, Quad::INFINITY.log2();
            Quad::NAN, Quad::NEG_INFINITY.log2();
            Quad::NAN, Quad::NAN.log2();
        );
    }

    #[test]
    fn log() {
        assert_all_close!(
            qd!("1.6514961294723187980432792951080073350184769267630415294067885154872"),
            Quad::PI.log(qd!(2.0));

            qd!("0.87356852683023186835397746476334273882072986617613914765231984243114"),
            Quad::E.log(Quad::PI);

            qd!("1.8378770664093454835606594728112352797227949472755668256343030809653"),
            Quad::MUL_2_PI.log(Quad::E);

            qd!("0.19611987703015265913752939356640584688348844291833390482362603879769"),
            Quad::FRAC_PI_2.log(qd!(10.0));

            qd!("0.12159929443072307483899992782618244339949906038928361898477291551813"),
            Quad::SQRT_2.log(qd!(17.29));

            qd!("0.075257498915995298803434723681123256692047470365527135327606865281672"),
            Quad::FRAC_1_SQRT_2.log(qd!(0.01));

            qd!("99.657842846620870436109582884681705275944941790737418361642691874514"),
            qd!("1e30").log(qd!(2.0));

            qd!("-60.343976027641828162941661418661260900837490622117196618434822573006"),
            qd!("1e-30").log(Quad::PI);

            qd!("415.42082818395565948469118995152468833347793454367082508050541979348"),
            qd!("2.6e180").log(Quad::E);

            qd!("-213.0"),
            qd!("1e-213").log(qd!(10.0));

        );
        assert_all_exact!(
            Quad::NAN, (-Quad::PI).log(qd!(2.0));
            Quad::NAN, (-Quad::E).log(Quad::PI);
            Quad::ZERO, Quad::ONE.log(Quad::E);
            Quad::NEG_INFINITY, Quad::ZERO.log(qd!(10.0));
            Quad::NAN, Quad::NEG_ZERO.log(qd!(3.2));
            Quad::INFINITY, Quad::INFINITY.log(qd!(7.1));
            Quad::NAN, Quad::NEG_INFINITY.log(qd!(3.0));
            Quad::NAN, Quad::NAN.log(qd!(5.0));
            Quad::NAN, Quad::PI.log(Quad::NAN);
        );
    }
}
