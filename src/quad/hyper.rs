// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::common as c;
use crate::quad::Quad;

impl Quad {
    /// Simultaneously computes the hyperbolic sine and cosine (sinh and cosh) of the
    /// `Quad`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is (-∞, ∞) for the first
    /// component of the answer (the hyperbolic sine) and [1, ∞) for the second (the
    /// hyperbolic cosine).
    ///
    /// This method is more efficient to run than [`sinh`] and [`cosh`] individually and is
    /// useful when both numbers are needed.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let (sin_h, cos_h) = qd!(1).sinh_cosh();
    /// let esin = qd!("1.175201193643801456882381850595600815155717981334095870229565413");
    /// let ecos = qd!("1.543080634815243778477905620757061682601529112365863704737402215");
    ///
    /// let diff1 = (sin_h - esin).abs();
    /// let diff2 = (cos_h - ecos).abs();
    ///
    /// assert!(diff1 < qd!(1e-60));
    /// assert!(diff2 < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [`sinh`]: #method.sinh
    /// [`cosh`]: #method.cosh
    pub fn sinh_cosh(self) -> (Quad, Quad) {
        match self.pre_sinh_cosh() {
            Some(r) => r,
            None => {
                if self.abs().0 <= 0.05 {
                    let s = self.sinh();
                    let c = (Quad::ONE + s.sqr()).sqrt();
                    (s, c)
                } else {
                    let a = self.exp();
                    let inv_a = a.recip();
                    let s = c::mul_pwr2(a - inv_a, 0.5);
                    let c = c::mul_pwr2(a + inv_a, 0.5);
                    (s, c)
                }
            }
        }
    }

    /// Computes the hyperbolic sine (sinh) of the `Quad`.
    ///
    /// The domain and range of this function are both (-∞, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).sinh();
    /// let expected = qd!("1.175201193643801456882381850595600815155717981334095870229565413");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn sinh(self) -> Quad {
        match self.pre_sinh() {
            Some(r) => r,
            None => {
                if self.abs().0 > 0.05 {
                    let a = self.exp();
                    c::mul_pwr2(a - a.recip(), 0.5)
                } else {
                    // The above formula is not accurate enough with very small numbers.
                    // Use a Taylor series instead.
                    let mut s = self;
                    let mut t = self;
                    let r = t.sqr();
                    let mut m = 1.0;
                    let threshold = (self * Quad::EPSILON).abs();

                    loop {
                        m += 2.0;
                        t *= r;
                        t /= Quad::from((m - 1.0) * m);
                        s += t;
                        if t.abs() <= threshold {
                            break;
                        }
                    }
                    s
                }
            }
        }
    }

    /// Computes the hyperbolic cosine (cosh) of the `Quad`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is [1, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).cosh();
    /// let expected = qd!("1.543080634815243778477905620757061682601529112365863704737402215");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn cosh(self) -> Quad {
        match self.pre_cosh() {
            Some(r) => r,
            None => {
                let a = self.exp();
                c::mul_pwr2(a + a.recip(), 0.5)
            }
        }
    }

    /// Computes the hyperbolic tangent (tanh) of the `Quad`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is (-1, 1).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1).tanh();
    /// let expected = qd!("0.7615941559557648881194582826047935904127685972579365515968105001");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn tanh(self) -> Quad {
        match self.pre_tanh() {
            Some(r) => r,
            None => {
                if self.abs().0 > 0.05 {
                    let a = self.exp();
                    let inv_a = a.recip();
                    (a - inv_a) / (a + inv_a)
                } else {
                    let (s, c) = self.sinh_cosh();
                    s / c
                }
            }
        }
    }

    /// Calculates the inverse hyperbolic sine (sinh<sup>-1</sup>) of the `Quad`.
    ///
    /// The domain and range of this function are both (-∞, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1.5).asinh();
    /// let expected = qd!("1.194763217287109304111930828519090523536162075153005429270680299");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    pub fn asinh(self) -> Quad {
        match self.pre_asinh() {
            Some(r) => r,
            None => (self + (self.sqr() + Quad::ONE).sqrt()).ln(),
        }
    }

    /// Calculates the inverse hyperbolic cosine (cosh<sup>-1</sup>) of the `Quad`.
    ///
    /// The domain of the function is [1, ∞) and the range is [0, ∞). Any argument outside
    /// the range will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(1.5).acosh();
    /// let expected = qd!("0.9624236501192068949955178268487368462703686687713210393220363377");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acosh(self) -> Quad {
        match self.pre_acosh() {
            Some(r) => r,
            None => (self + (self.sqr() - Quad::ONE).sqrt()).ln(),
        }
    }

    /// Calculates the inverse hyperbolic tangent (tanh<sup>-1</sup>) of the `Quad`.
    ///
    /// The domain of the function is (-1, 1) and the range is (-∞, ∞). Any argument whose
    /// absolute value is greater than or equal to 1 will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let x = qd!(0.5).atanh();
    /// let expected = qd!("0.5493061443340548456976226184612628523237452789113747258673471668");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < qd!(1e-60));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn atanh(self) -> Quad {
        match self.pre_atanh() {
            Some(r) => r,
            None => c::mul_pwr2(((Quad::ONE + self) / (Quad::ONE - self)).ln(), 0.5),
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
    fn pre_sinh_cosh(&self) -> Option<(Quad, Quad)> {
        if self.is_nan() {
            Some((Quad::NAN, Quad::NAN))
        } else if self.is_zero() {
            Some((Quad::ZERO, Quad::ONE))
        } else {
            None
        }
    }

    #[inline]
    fn pre_sinh(&self) -> Option<Quad> {
        if self.is_nan() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            Some(Quad::ZERO)
        } else {
            None
        }
    }

    #[inline]
    fn pre_cosh(&self) -> Option<Quad> {
        if self.is_nan() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            Some(Quad::ONE)
        } else {
            None
        }
    }

    #[inline]
    fn pre_tanh(&self) -> Option<Quad> {
        if self.is_nan() {
            Some(Quad::NAN)
        } else if self.is_zero() {
            Some(Quad::ZERO)
        } else if self.is_infinite() {
            Some(self.signum())
        } else {
            None
        }
    }

    #[inline]
    fn pre_asinh(&self) -> Option<Quad> {
        if self.is_infinite() {
            if self.is_sign_positive() {
                Some(Quad::INFINITY)
            } else {
                Some(Quad::NEG_INFINITY)
            }
        } else {
            None
        }
    }

    #[inline]
    fn pre_acosh(&self) -> Option<Quad> {
        if *self < Quad::ONE {
            Some(Quad::NAN)
        } else if self.is_infinite() {
            Some(Quad::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_atanh(&self) -> Option<Quad> {
        if self.abs() >= Quad::ONE {
            Some(Quad::NAN)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // sinh tests
    test_all_near!(
        sinh_pi:
            qd!("11.548739357257748377977334315388409684495189066394789455232163361053"),
            Quad::PI.sinh();
        sinh_e:
            qd!("7.5441371028169758263418200425165327402949857443016716663691364321724"),
            Quad::E.sinh();
        sinh_neg_pi:
            qd!("-11.548739357257748377977334315388409684495189066394789455232163361053"),
            (-Quad::PI).sinh();
        sinh_neg_e:
            qd!("-7.5441371028169758263418200425165327402949857443016716663691364321724"),
            (-Quad::E).sinh();
        sinh_2_pi:
            qd!("267.74489404101651425711744968805617722370618739914622009500293674898"),
            Quad::TAU.sinh();
        sinh_pi_2:
            qd!("2.3012989023072948734630400234344271781781465165163826659728398030928"),
            Quad::FRAC_PI_2.sinh();
        sinh_sqrt_2:
            qd!("1.9350668221743566531843597473201792189703699102294526772575489689003"),
            Quad::SQRT_2.sinh();
        sinh_1_sqrt_2:
            qd!("0.76752314512611633163108436606558684996924366470309367117838183498206"),
            Quad::FRAC_1_SQRT_2.sinh();
        sinh_small:
            qd!("0.000010000000000166666666667500000000001984126984129739858906528078403084"),
            qd!("0.00001").sinh();
        sinh_neg_small:
            qd!("-0.0000000010000000000000000001666666666666666666750000000000000000001984126988"),
            qd!("-0.000000001").sinh();
        sinh_150:
            qd!("69685479033318984865917096857072873873684503070109219116878222417.813"),
            qd!(150).sinh();
        sinh_neg_140:
            qd!("-3163715853577792682171512256175572576556222994111693720356524.5634327"),
            qd!(-140).sinh();
    );
    test_all_exact!(
        sinh_zero:
            Quad::ZERO,
            Quad::ZERO.sinh();
        sinh_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.sinh();
        sinh_inf:
            Quad::INFINITY,
            Quad::INFINITY.sinh();
        sinh_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.sinh();
        sinh_nan:
            Quad::NAN,
            Quad::NAN.sinh();
    );

    // cosh tests
    test_all_near!(
        cosh_pi:
            qd!("11.591953275521520627751752052560137695770917176205422538212883048449"),
            Quad::PI.cosh();
        cosh_e:
            qd!("7.6101251386622883634186102301133791652335627925544681027716099737405"),
            Quad::E.cosh();
        cosh_neg_pi:
            qd!("11.591953275521520627751752052560137695770917176205422538212883048449"),
            (-Quad::PI).cosh();
        cosh_neg_e:
            qd!("7.6101251386622883634186102301133791652335627925544681027716099737405"),
            (-Quad::E).cosh();
        cosh_2_pi:
            qd!("267.74676148374822224593187990099100425409961020414869541220231828758"),
            Quad::TAU.cosh();
        cosh_pi_2:
            qd!("2.5091784786580567820099956432694059482120243581481522740479756861487"),
            Quad::FRAC_PI_2.cosh();
        cosh_sqrt_2:
            qd!("2.1781835566085708639892220678201252834312940329216569328108157409496"),
            Quad::SQRT_2.cosh();
        cosh_1_sqrt_2:
            qd!("1.2605918365213561194770417466807643252050814278395198494393941371403"),
            Quad::FRAC_1_SQRT_2.cosh();
        cosh_small:
            qd!("1.0000000000500000000004166666666680555555555580357142857170414462076"),
            qd!("0.00001").cosh();
        cosh_neg_small:
            qd!("1.0000000000000000005000000000000000000416666666666666666680555555557"),
            qd!("-0.000000001").cosh();
        cosh_150:
            qd!("69685479033318984865917096857072873873684503070109219116878222417.813"),
            qd!(150).cosh();
        cosh_neg_140:
            qd!("3163715853577792682171512256175572576556222994111693720356524.5634327"),
            qd!(-140).cosh();
    );
    test_all_exact!(
        cosh_zero:
            Quad::ONE,
            Quad::ZERO.cosh();
        cosh_neg_zero:
            Quad::ONE,
            Quad::NEG_ZERO.cosh();
        cosh_inf:
            Quad::INFINITY,
            Quad::INFINITY.cosh();
        cosh_neg_inf:
            Quad::INFINITY,
            Quad::NEG_INFINITY.cosh();
        cosh_nan:
            Quad::NAN,
            Quad::NAN.cosh();
    );

    // sinh_cosh tests
    test_all_near!(
        sinh_cosh_pi_sinh:
            Quad::PI.sinh(),
            Quad::PI.sinh_cosh().0;
        sinh_cosh_pi_cosh:
            Quad::PI.cosh(),
            Quad::PI.sinh_cosh().1;
        sinh_cosh_e_sinh:
            Quad::E.sinh(),
            Quad::E.sinh_cosh().0;
        sinh_cosh_e_cosh:
            Quad::E.cosh(),
            Quad::E.sinh_cosh().1;
        sinh_cosh_neg_pi_sinh:
            (-Quad::PI).sinh(),
            (-Quad::PI).sinh_cosh().0;
        sinh_cosh_neg_pi_cosh:
            (-Quad::PI).cosh(),
            (-Quad::PI).sinh_cosh().1;
        sinh_cosh_neg_e_sinh:
            (-Quad::E).sinh(),
            (-Quad::E).sinh_cosh().0;
        sinh_cosh_neg_e_cosh:
            (-Quad::E).cosh(),
            (-Quad::E).sinh_cosh().1;
        sinh_cosh_2_pi_sinh:
            Quad::TAU.sinh(),
            Quad::TAU.sinh_cosh().0;
        sinh_cosh_2_pi_cosh:
            Quad::TAU.cosh(),
            Quad::TAU.sinh_cosh().1;
        sinh_cosh_pi_2_sinh:
            Quad::FRAC_PI_2.sinh(),
            Quad::FRAC_PI_2.sinh_cosh().0;
        sinh_cosh_pi_2_cosh:
            Quad::FRAC_PI_2.cosh(),
            Quad::FRAC_PI_2.sinh_cosh().1;
        sinh_cosh_sqrt_2_sinh:
            Quad::SQRT_2.sinh(),
            Quad::SQRT_2.sinh_cosh().0;
        sinh_cosh_sqrt_2_cosh:
            Quad::SQRT_2.cosh(),
            Quad::SQRT_2.sinh_cosh().1;
        sinh_cosh_1_sqrt_2_sinh:
            Quad::FRAC_1_SQRT_2.sinh(),
            Quad::FRAC_1_SQRT_2.sinh_cosh().0;
        sinh_cosh_1_sqrt_2_cosh:
            Quad::FRAC_1_SQRT_2.cosh(),
            Quad::FRAC_1_SQRT_2.sinh_cosh().1;
        sinh_cosh_small_sinh:
            qd!("0.00001").sinh(),
            qd!("0.00001").sinh_cosh().0;
        sinh_cosh_small_cosh:
            qd!("0.00001").cosh(),
            qd!("0.00001").sinh_cosh().1;
        sinh_cosh_neg_small_sinh:
            qd!("-0.0001").sinh(),
            qd!("-0.0001").sinh_cosh().0;
        sinh_cosh_neg_small_cosh:
            qd!("-0.0001").cosh(),
            qd!("-0.0001").sinh_cosh().1;
        sinh_cosh_150_sinh:
            qd!(150).sinh(),
            qd!(150).sinh_cosh().0;
        sinh_cosh_150_cosh:
            qd!(150).cosh(),
            qd!(150).sinh_cosh().1;
        sinh_cosh_neg_140_sinh:
            qd!(-140).sinh(),
            qd!(-140).sinh_cosh().0;
        sinh_cosh_neg_140_cosh:
            qd!(-140).cosh(),
            qd!(-140).sinh_cosh().1;
    );
    test_all_exact!(
        sinh_cosh_zero_sinh:
            Quad::ZERO,
            Quad::ZERO.sinh_cosh().0;
        sinh_cosh_zero_cosh:
            Quad::ONE,
            Quad::ZERO.sinh_cosh().1;
        sinh_cosh_neg_zero_sinh:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.sinh_cosh().0;
        sinh_cosh_neg_zero_cosh:
            Quad::ONE,
            Quad::NEG_ZERO.sinh_cosh().1;
        sinh_cosh_inf_sinh:
            Quad::INFINITY,
            Quad::INFINITY.sinh_cosh().0;
        sinh_cosh_inf_cosh:
            Quad::INFINITY,
            Quad::INFINITY.sinh_cosh().1;
        sinh_cosh_neg_inf_sinh:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.sinh_cosh().0;
        sinh_cosh_neg_inf_cosh:
            Quad::INFINITY,
            Quad::NEG_INFINITY.sinh_cosh().1;
        sinh_cosh_nan_sinh:
            Quad::NAN,
            Quad::NAN.sinh_cosh().0;
        sinh_cosh_nan_cosh:
            Quad::NAN,
            Quad::NAN.sinh_cosh().1;
    );

    // tanh tests
    test_all_near!(
        tanh_pi:
            qd!("0.99627207622074994426469058001253671189689919080458761436261241597871"),
            Quad::PI.tanh();
        tanh_e:
            qd!("0.99132891580059983779555761569968438292165869798746371676782928137622"),
            Quad::E.tanh();
        tanh_neg_pi:
            qd!("-0.99627207622074994426469058001253671189689919080458761436261241597871"),
            (-Quad::PI).tanh();
        tanh_neg_e:
            qd!("-0.99132891580059983779555761569968438292165869798746371676782928137622"),
            (-Quad::E).tanh();
        tanh_2_pi:
            qd!("0.99999302533961061060510721183234574642771937737571084122455870005561"),
            Quad::TAU.tanh();
        tanh_pi_2:
            qd!("0.91715233566727434637309292144261877536792714860108894534357412429172"),
            Quad::FRAC_PI_2.tanh();
        tanh_sqrt_2:
            qd!("0.88838556158566054495300030572803164902037084848543082042923068315516"),
            Quad::SQRT_2.tanh();
        tanh_1_sqrt_2:
            qd!("0.60885936501391381038594521400112420518839331118402587304740123080607"),
            Quad::FRAC_1_SQRT_2.tanh();
        tanh_150:
            qd!("1.0"),
            qd!(150).tanh();
        tanh_neg_140:
            qd!("-1.0"),
            qd!(-140).tanh();
        tanh_small:
            qd!("0.0000099999999996666666666799999999994603174603393298059955863395863755057"),
            qd!("0.00001").tanh();
        tanh_neg_small:
            qd!("-0.00000000099999999999999999966666666666666666679999999999999999994603174603225"),
            qd!("-0.000000001").tanh();
    );
    test_all_exact!(
        tanh_zero:
            Quad::ZERO,
            Quad::ZERO.tanh();
        tanh_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.tanh();
        tanh_inf:
            Quad::ONE,
            Quad::INFINITY.tanh();
        tanh_neg_inf:
            Quad::NEG_ONE,
            Quad::NEG_INFINITY.tanh();
        tanh_nan:
            Quad::NAN,
            Quad::NAN.tanh();
    );

    // asinh tests
    test_all_near!(
        asinh_pi:
            qd!("1.8622957433108482198883613251826205749026741849615547656128795144236"),
            Quad::PI.asinh();
        asinh_e:
            qd!("1.7253825588523150939450979704048887562745572746729386688142115567082"),
            Quad::E.asinh();
        asinh_neg_pi:
            qd!("-1.8622957433108482198883613251826205749026741849615547656128795144236"),
            (-Quad::PI).asinh();
        asinh_neg_e:
            qd!("-1.7253825588523150939450979704048887562745572746729386688142115567082"),
            (-Quad::E).asinh();
        asinh_2_pi:
            qd!("2.5372975013733611766775071037696746055657169972025193313951566067849"),
            Quad::TAU.asinh();
        asinh_pi_2:
            qd!("1.2334031175112170570731083915452972603561395906198085461437004891214"),
            Quad::FRAC_PI_2.asinh();
        asinh_sqrt_2:
            qd!("1.1462158347805888439003936556740077158109341200078551238025082224159"),
            Quad::SQRT_2.asinh();
        asinh_1_sqrt_2:
            qd!("0.65847894846240835431252317365398422201349098573375823988423612846054"),
            Quad::FRAC_1_SQRT_2.asinh();
        asinh_150:
            qd!("5.7037935855821315576975027994004472827976444064946619017092033475062"),
            qd!(150).asinh();
        asinh_neg_140:
            qd!("-5.6348023580272583991488640099283811389909652342696970652712772047229"),
            qd!(-140).asinh();
    );
    test_all_prec!(
        asinh_small:
            qd!("0.0000099999999998333333333408333333328869047619351438492041119904402889648"),
            qd!("0.00001").asinh(),
            59;
        asinh_neg_small:
            qd!("-0.000099999999833333334083333328869047649429563268341902889677582744003099"),
            qd!("-0.0001").asinh(),
            60;
    );
    test_all_exact!(
        asinh_zero:
            Quad::ZERO,
            Quad::ZERO.asinh();
        asinh_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.asinh();
        asinh_inf:
            Quad::INFINITY,
            Quad::INFINITY.asinh();
        asinh_neg_inf:
            Quad::NEG_INFINITY,
            Quad::NEG_INFINITY.asinh();
        asinh_nan:
            Quad::NAN,
            Quad::NAN.asinh();
    );

    // acosh tests
    test_all_near!(
        acosh_pi:
            qd!("1.811526272460853107021852049305420510220702081057922474861595622974"),
            Quad::PI.acosh();
        acosh_e:
            qd!("1.657454454153077272593828742280534739158392762033676825848582208938"),
            Quad::E.acosh();
        acosh_2_pi:
            qd!("2.5246306599334672302074913165530273494000887001227803690255451707555"),
            Quad::TAU.acosh();
        acosh_pi_2:
            qd!("1.0232274785475505793174956779493038523014056664148620968024210937357"),
            Quad::FRAC_PI_2.acosh();
        acosh_sqrt_2:
            qd!("0.88137358701954302523260932497979230902816032826163541075329560865267"),
            Quad::SQRT_2.acosh();
        acosh_150:
            qd!("5.7037713633599001905278554895391354012923502543623403061858508464653"),
            qd!(150).acosh();
    );
    test_all_prec!(
        acosh_small:
            qd!("0.004472132228228002123128446633028159909034212521519637331062210854491"),
            qd!("1.00001").acosh(),
            59;
    );
    test_all_exact!(
        acosh_neg_pi:
            Quad::NAN,
            (-Quad::PI).acosh();
        acosh_neg_e:
            Quad::NAN,
            (-Quad::E).acosh();
        acosh_1_sqrt_2:
            Quad::NAN,
            Quad::FRAC_1_SQRT_2.acosh();
        acosh_zero:
            Quad::NAN,
            Quad::ZERO.acosh();
        acosh_neg_zero:
            Quad::NAN,
            Quad::NEG_ZERO.acosh();
        acosh_one:
            Quad::ZERO,
            Quad::ONE.acosh();
        acosh_inf:
            Quad::INFINITY,
            Quad::INFINITY.acosh();
        acosh_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.acosh();
        acosh_nan:
            Quad::NAN,
            Quad::NAN.acosh();
    );

    // atanh tests
    test_all_near!(
        atanh_pi_6:
            qd!("0.58128501169472315834806278002976913214059711391275847374959739163012"),
            Quad::FRAC_PI_6.atanh();
        atanh_pi_8:
            qd!("0.41498725684198892721883143584943153659779852210475972513293607997076"),
            Quad::FRAC_PI_8.atanh();
        atanh_1_sqrt_2:
            qd!("0.88137358701954302523260932497979230902816032826163541075329560865386"),
            Quad::FRAC_1_SQRT_2.atanh();
        atanh_99:
            qd!("2.6466524123622461977050606459342686009455526402847362494532304939648"),
            qd!("0.99").atanh();
        atanh_neg_pi_6:
            qd!("-0.58128501169472315834806278002976913214059711391275847374959739163012"),
            (-Quad::FRAC_PI_6).atanh();
        atanh_neg_pi_8:
            qd!("-0.41498725684198892721883143584943153659779852210475972513293607997076"),
            (-Quad::FRAC_PI_8).atanh();
        atanh_neg_1_sqrt_2:
            qd!("-0.88137358701954302523260932497979230902816032826163541075329560865386"),
            (-Quad::FRAC_1_SQRT_2).atanh();
        atanh_neg_99:
            qd!("-2.6466524123622461977050606459342686009455526402847362494532304939648"),
            qd!("-0.99").atanh();
    );
    test_all_exact!(
        atanh_pi:
            Quad::NAN,
            Quad::PI.atanh();
        atanh_neg_pi:
            Quad::NAN,
            (-Quad::PI).atanh();
        atanh_zero:
            Quad::ZERO,
            Quad::ZERO.atanh();
        atanh_neg_zero:
            Quad::NEG_ZERO,
            Quad::NEG_ZERO.atanh();
        atanh_inf:
            Quad::NAN,
            Quad::INFINITY.atanh();
        atanh_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY.atanh();
        atanh_nan:
            Quad::NAN,
            Quad::NAN.atanh();
    );
}
