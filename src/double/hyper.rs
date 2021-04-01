// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::common as c;
use crate::double::Double;

impl Double {
    /// Simultaneously computes the hyperbolic sine and cosine (sinh and cosh) of the
    /// `Double`.
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
    /// # use qd::Double;
    /// # fn main() {
    /// let (sin_h, cos_h) = dd!(1).sinh_cosh();
    /// let esin = dd!("1.1752011936438014568823818505956");
    /// let ecos = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff1 = (sin_h - esin).abs();
    /// let diff2 = (cos_h - ecos).abs();
    ///
    /// assert!(diff1 < dd!(1e-30));
    /// assert!(diff2 < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`sinh`]: #method.sinh
    /// [`cosh`]: #method.cosh
    pub fn sinh_cosh(self) -> (Double, Double) {
        match self.pre_sinh_cosh() {
            Some(r) => r,
            None => {
                if self.abs().0 <= 0.05 {
                    let s = self.sinh();
                    let c = (Double::ONE + s.sqr()).sqrt();
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

    /// Computes the hyperbolic sine (sinh) of the `Double`.
    ///
    /// The domain and range of this function are both (-∞, ∞). Large values will start to
    /// cause a loss of precision; by the time the number is ±130 or so, precision is down
    /// to 29 digits.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).sinh();
    /// let expected = dd!("1.1752011936438014568823818505956");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn sinh(self) -> Double {
        match self.pre_sinh() {
            Some(r) => r,
            None => {
                if self.abs().0 > 0.05 {
                    let a = self.exp();
                    c::mul_pwr2(a - a.recip(), 0.5)
                } else {
                    // The above formula is not accurate enough with very small numbers. Use
                    // a Taylor series instead.
                    let mut s = self;
                    let mut t = self;
                    let r = t.sqr();
                    let mut m = 1;
                    let threshold = (self * Double::EPSILON).abs();

                    loop {
                        m += 2;
                        t *= r;
                        t /= Double::from((m - 1) * m);
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

    /// Computes the hyperbolic cosine (cosh) of the `Double`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is [1, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).cosh();
    /// let expected = dd!("1.5430806348152437784779056207571");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn cosh(self) -> Double {
        match self.pre_cosh() {
            Some(r) => r,
            None => {
                let a = self.exp();
                c::mul_pwr2(a + a.recip(), 0.5)
            }
        }
    }

    /// Computes the hyperbolic tangent (tanh) of the `Double`.
    ///
    /// The domain of this function is (-∞, ∞), and the range is (-1, 1).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1).tanh();
    /// let expected = dd!("0.76159415595576488811945828260479");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn tanh(self) -> Double {
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

    /// Calculates the inverse hyperbolic sine (sinh<sup>-1</sup>) of the `Double`.
    ///
    /// The domain and range of this function are both (-∞, ∞).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).asinh();
    /// let expected = dd!("1.1947632172871093041119308285191");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    pub fn asinh(self) -> Double {
        match self.pre_asinh() {
            Some(r) => r,
            None => (self + (self.sqr() + Double::ONE).sqrt()).ln(),
        }
    }

    /// Calculates the inverse hyperbolic cosine (cosh<sup>-1</sup>) of the `Double`.
    ///
    /// The domain of the function is [1, ∞) and the range is [0, ∞). Any argument outside
    /// the range will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(1.5).acosh();
    /// let expected = dd!("0.96242365011920689499551782684874");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn acosh(self) -> Double {
        match self.pre_acosh() {
            Some(r) => r,
            None => (self + (self.sqr() - Double::ONE).sqrt()).ln(),
        }
    }

    /// Calculates the inverse hyperbolic tangent (tanh<sup>-1</sup>) of the `Double`.
    ///
    /// The domain of the function is (-1, 1) and the range is (-∞, ∞). Any argument whose
    /// absolute value is greater than or equal to 1 will result in [`NAN`].
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = dd!(0.5).atanh();
    /// let expected = dd!("0.54930614433405484569762261846126");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    pub fn atanh(self) -> Double {
        match self.pre_atanh() {
            Some(r) => r,
            None => c::mul_pwr2(((Double::ONE + self) / (Double::ONE - self)).ln(), 0.5),
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
    fn pre_sinh_cosh(&self) -> Option<(Double, Double)> {
        if self.is_nan() {
            Some((Double::NAN, Double::NAN))
        } else if self.is_zero() {
            Some((Double::ZERO, Double::ONE))
        } else {
            None
        }
    }

    #[inline]
    fn pre_sinh(&self) -> Option<Double> {
        if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(*self)
        } else if self.is_infinite() {
            Some(*self)
        } else {
            None
        }
    }

    #[inline]
    fn pre_cosh(&self) -> Option<Double> {
        if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(Double::ONE)
        } else if self.is_infinite() {
            Some(Double::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_tanh(&self) -> Option<Double> {
        if self.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            Some(Double::ZERO)
        } else if self.is_infinite() {
            Some(self.signum())
        } else {
            None
        }
    }

    #[inline]
    fn pre_asinh(&self) -> Option<Double> {
        if self.is_zero() {
            Some(*self)
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else {
            None
        }
    }

    #[inline]
    fn pre_acosh(&self) -> Option<Double> {
        if *self < Double::ONE {
            Some(Double::NAN)
        } else if *self == Double::ONE {
            Some(Double::ZERO)
        } else if self.is_infinite() {
            Some(Double::INFINITY)
        } else {
            None
        }
    }

    #[inline]
    fn pre_atanh(&self) -> Option<Double> {
        if self.abs() >= Double::ONE {
            Some(Double::NAN)
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
            dd!("11.548739357257748377977334315388404"),
            Double::PI.sinh();
        sinh_e:
            dd!("7.5441371028169758263418200425165246"),
            Double::E.sinh();
        sinh_neg_pi:
            dd!("-11.548739357257748377977334315388404"),
            (-Double::PI).sinh();
        sinh_neg_e:
            dd!("-7.5441371028169758263418200425165246"),
            (-Double::E).sinh();
        sinh_2_pi:
            dd!("267.74489404101651425711744968805619"),
            Double::TAU.sinh();
        sinh_pi_2:
            dd!("2.3012989023072948734630400234344263"),
            Double::FRAC_PI_2.sinh();
        sinh_sqrt_2:
            dd!("1.935066822174356653184359747320181"),
            Double::SQRT_2.sinh();
        sinh_1_sqrt_2:
            dd!("0.76752314512611633163108436606558639"),
            Double::FRAC_1_SQRT_2.sinh();
        sinh_small:
            dd!("0.000010000000000166666666667500000000012"),
            dd!("0.00001").sinh();
        sinh_neg_small:
            dd!("-0.00010000000016666666675000000001984131"),
            dd!("-0.0001").sinh();
    );
    test_all_prec!(
        sinh_150:
            dd!("6.9685479033318984865917096857072841e+64"),
            dd!(150).sinh(),
            29;
        sinh_neg_140:
            dd!("-3.1637158535777926821715122561755732e+60"),
            dd!(-140).sinh(),
            29;
    );
    test_all_exact!(
        sinh_zero:
            Double::ZERO,
            Double::ZERO.sinh();
        sinh_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.sinh();
        sinh_inf:
            Double::INFINITY,
            Double::INFINITY.sinh();
        sinh_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.sinh();
        sinh_nan:
            Double::NAN,
            Double::NAN.sinh();
    );

    // cosh tests
    test_all_near!(
        cosh_pi:
            dd!("11.591953275521520627751752052560135"),
            Double::PI.cosh();
        cosh_e:
            dd!("7.6101251386622883634186102301133709"),
            Double::E.cosh();
        cosh_neg_pi:
            dd!("11.591953275521520627751752052560135"),
            (-Double::PI).cosh();
        cosh_neg_e:
            dd!("7.6101251386622883634186102301133709"),
            (-Double::E).cosh();
        cosh_2_pi:
            dd!("267.74676148374822224593187990099087"),
            Double::TAU.cosh();
        cosh_pi_2:
            dd!("2.5091784786580567820099956432694066"),
            Double::FRAC_PI_2.cosh();
        cosh_sqrt_2:
            dd!("2.178183556608570863989222067820125"),
            Double::SQRT_2.cosh();
        cosh_1_sqrt_2:
            dd!("1.2605918365213561194770417466807639"),
            Double::FRAC_1_SQRT_2.cosh();
        cosh_small:
            dd!("1.0000000000500000000004166666666678"),
            dd!("0.00001").cosh();
        cosh_neg_small:
            dd!("1.0000000050000000041666666680555555"),
            dd!("-0.0001").cosh();
    );
    test_all_prec!(
        cosh_150:
            dd!("6.9685479033318984865917096857072841e+64"),
            dd!(150).cosh(),
            29;
        cosh_neg_140:
            dd!("3.1637158535777926821715122561755732e+60"),
            dd!(-140).cosh(),
            29;
    );
    test_all_exact!(
        cosh_zero:
            Double::ONE,
            Double::ZERO.cosh();
        cosh_neg_zero:
            Double::ONE,
            Double::NEG_ZERO.cosh();
        cosh_inf:
            Double::INFINITY,
            Double::INFINITY.cosh();
        cosh_neg_inf:
            Double::INFINITY,
            Double::NEG_INFINITY.cosh();
        cosh_nan:
            Double::NAN,
            Double::NAN.cosh();
    );

    // sinh_cosh tests
    test_all_near!(
        sinh_cosh_pi_sinh:
            Double::PI.sinh(),
            Double::PI.sinh_cosh().0;
        sinh_cosh_pi_cosh:
            Double::PI.cosh(),
            Double::PI.sinh_cosh().1;
        sinh_cosh_e_sinh:
            Double::E.sinh(),
            Double::E.sinh_cosh().0;
        sinh_cosh_e_cosh:
            Double::E.cosh(),
            Double::E.sinh_cosh().1;
        sinh_cosh_neg_pi_sinh:
            (-Double::PI).sinh(),
            (-Double::PI).sinh_cosh().0;
        sinh_cosh_neg_pi_cosh:
            (-Double::PI).cosh(),
            (-Double::PI).sinh_cosh().1;
        sinh_cosh_neg_e_sinh:
            (-Double::E).sinh(),
            (-Double::E).sinh_cosh().0;
        sinh_cosh_neg_e_cosh:
            (-Double::E).cosh(),
            (-Double::E).sinh_cosh().1;
        sinh_cosh_2_pi_sinh:
            Double::TAU.sinh(),
            Double::TAU.sinh_cosh().0;
        sinh_cosh_2_pi_cosh:
            Double::TAU.cosh(),
            Double::TAU.sinh_cosh().1;
        sinh_cosh_pi_2_sinh:
            Double::FRAC_PI_2.sinh(),
            Double::FRAC_PI_2.sinh_cosh().0;
        sinh_cosh_pi_2_cosh:
            Double::FRAC_PI_2.cosh(),
            Double::FRAC_PI_2.sinh_cosh().1;
        sinh_cosh_sqrt_2_sinh:
            Double::SQRT_2.sinh(),
            Double::SQRT_2.sinh_cosh().0;
        sinh_cosh_sqrt_2_cosh:
            Double::SQRT_2.cosh(),
            Double::SQRT_2.sinh_cosh().1;
        sinh_cosh_1_sqrt_2_sinh:
            Double::FRAC_1_SQRT_2.sinh(),
            Double::FRAC_1_SQRT_2.sinh_cosh().0;
        sinh_cosh_1_sqrt_2_cosh:
            Double::FRAC_1_SQRT_2.cosh(),
            Double::FRAC_1_SQRT_2.sinh_cosh().1;
        sinh_cosh_small_sinh:
            dd!("0.00001").sinh(),
            dd!("0.00001").sinh_cosh().0;
        sinh_cosh_small_cosh:
            dd!("0.00001").cosh(),
            dd!("0.00001").sinh_cosh().1;
        sinh_cosh_neg_small_sinh:
            dd!("-0.0001").sinh(),
            dd!("-0.0001").sinh_cosh().0;
        sinh_cosh_neg_small_cosh:
            dd!("-0.0001").cosh(),
            dd!("-0.0001").sinh_cosh().1;
        sinh_cosh_150_sinh:
            dd!(150).sinh(),
            dd!(150).sinh_cosh().0;
        sinh_cosh_150_cosh:
            dd!(150).cosh(),
            dd!(150).sinh_cosh().1;
        sinh_cosh_neg_140_sinh:
            dd!(-140).sinh(),
            dd!(-140).sinh_cosh().0;
        sinh_cosh_neg_140_cosh:
            dd!(-140).cosh(),
            dd!(-140).sinh_cosh().1;
    );
    test_all_exact!(
        sinh_cosh_zero_sinh:
            Double::ZERO,
            Double::ZERO.sinh_cosh().0;
        sinh_cosh_zero_cosh:
            Double::ONE,
            Double::ZERO.sinh_cosh().1;
        sinh_cosh_neg_zero_sinh:
            Double::NEG_ZERO,
            Double::NEG_ZERO.sinh_cosh().0;
        sinh_cosh_neg_zero_cosh:
            Double::ONE,
            Double::NEG_ZERO.sinh_cosh().1;
        sinh_cosh_inf_sinh:
            Double::INFINITY,
            Double::INFINITY.sinh_cosh().0;
        sinh_cosh_inf_cosh:
            Double::INFINITY,
            Double::INFINITY.sinh_cosh().1;
        sinh_cosh_neg_inf_sinh:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.sinh_cosh().0;
        sinh_cosh_neg_inf_cosh:
            Double::INFINITY,
            Double::NEG_INFINITY.sinh_cosh().1;
        sinh_cosh_nan_sinh:
            Double::NAN,
            Double::NAN.sinh_cosh().0;
        sinh_cosh_nan_cosh:
            Double::NAN,
            Double::NAN.sinh_cosh().1;
    );

    // tanh tests
    test_all_near!(
        tanh_pi:
            dd!("0.99627207622074994426469058001253668"),
            Double::PI.tanh();
        tanh_e:
            dd!("0.99132891580059983779555761569968412"),
            Double::E.tanh();
        tanh_neg_pi:
            dd!("-0.99627207622074994426469058001253668"),
            (-Double::PI).tanh();
        tanh_neg_e:
            dd!("-0.99132891580059983779555761569968412"),
            (-Double::E).tanh();
        tanh_2_pi:
            dd!("0.9999930253396106106051072118323457"),
            Double::TAU.tanh();
        tanh_pi_2:
            dd!("0.91715233566727434637309292144261871"),
            Double::FRAC_PI_2.tanh();
        tanh_sqrt_2:
            dd!("0.888385561585660544953000305:2803144"),
            Double::SQRT_2.tanh();
        tanh_1_sqrt_2:
            dd!("0.60885936501391381038594521400112353"),
            Double::FRAC_1_SQRT_2.tanh();
        tanh_150:
            dd!("1.0"),
            dd!(150).tanh();
        tanh_neg_140:
            dd!("-1.0"),
            dd!(-140).tanh();
        tanh_small:
            dd!("0.0000099999999996666666666799999999994698"),
            dd!("0.00001").tanh();
        tanh_neg_small:
            dd!("-0.000099999999666666667999999994603174675"),
            dd!("-0.0001").tanh();
    );
    test_all_exact!(
        tanh_zero:
            Double::ZERO,
            Double::ZERO.tanh();
        tanh_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.tanh();
        tanh_inf:
            Double::ONE,
            Double::INFINITY.tanh();
        tanh_neg_inf:
            Double::NEG_ONE,
            Double::NEG_INFINITY.tanh();
        tanh_nan:
            Double::NAN,
            Double::NAN.tanh();
    );

    // asinh tests
    test_all_near!(
        asinh_pi:
            dd!("1.8622957433108482198883613251826208"),
            Double::PI.asinh();
        asinh_e:
            dd!("1.7253825588523150939450979704048887"),
            Double::E.asinh();
        asinh_neg_pi:
            dd!("-1.8622957433108482198883613251826208"),
            (-Double::PI).asinh();
        asinh_neg_e:
            dd!("-1.7253825588523150939450979704048887"),
            (-Double::E).asinh();
        asinh_2_pi:
            dd!("2.537297501373361176677507103769674"),
            Double::TAU.asinh();
        asinh_pi_2:
            dd!("1.2334031175112170570731083915452966"),
            Double::FRAC_PI_2.asinh();
        asinh_sqrt_2:
            dd!("1.1462158347805888439003936556740084"),
            Double::SQRT_2.asinh();
        asinh_1_sqrt_2:
            dd!("0.65847894846240835431252317365398395"),
            Double::FRAC_1_SQRT_2.asinh();
        asinh_150:
            dd!("5.703793585582131557697502799400447"),
            dd!(150).asinh();
    );
    test_all_prec!(
        asinh_neg_140:
            dd!("-5.6348023580272583991488640099283842"),
            dd!(-140).asinh(),
            28;
        asinh_small:
            dd!("0.0000099999999998333333333408333333328982"),
            dd!("0.00001").asinh(),
            26;
        asinh_neg_small:
            dd!("-0.000099999999833333334083333328869047707"),
            dd!("-0.0001").asinh(),
            28;
    );
    test_all_exact!(
        asinh_zero:
            Double::ZERO,
            Double::ZERO.asinh();
        asinh_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.asinh();
        asinh_inf:
            Double::INFINITY,
            Double::INFINITY.asinh();
        asinh_neg_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY.asinh();
        asinh_nan:
            Double::NAN,
            Double::NAN.asinh();
    );

    // acosh tests
    test_all_near!(
        acosh_pi:
            dd!("1.8115262724608531070218520493054203"),
            Double::PI.acosh();
        acosh_e:
            dd!("1.6574544541530772725938287422805339"),
            Double::E.acosh();
        acosh_2_pi:
            dd!("2.5246306599334672302074913165530261"),
            Double::TAU.acosh();
        acosh_pi_2:
            dd!("1.0232274785475505793174956779493034"),
            Double::FRAC_PI_2.acosh();
        acosh_sqrt_2:
            dd!("0.88137358701954302523260932497979278"),
            Double::SQRT_2.acosh();
        acosh_150:
            dd!("5.7037713633599001905278554895391333"),
            dd!(150).acosh();
    );
    test_all_prec!(
        acosh_small:
            dd!("0.0044721322282280021231284466331917651"),
            dd!("1.00001").acosh(),
            28;
    );
    test_all_exact!(
        acosh_neg_pi:
            Double::NAN,
            (-Double::PI).acosh();
        acosh_neg_e:
            Double::NAN,
            (-Double::E).acosh();
        acosh_1_sqrt_2:
            Double::NAN,
            Double::FRAC_1_SQRT_2.acosh();
        acosh_zero:
            Double::NAN,
            Double::ZERO.acosh();
        acosh_neg_zero:
            Double::NAN,
            Double::NEG_ZERO.acosh();
        acosh_one:
            Double::ZERO,
            Double::ONE.acosh();
        acosh_inf:
            Double::INFINITY,
            Double::INFINITY.acosh();
        acosh_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.acosh();
        acosh_nan:
            Double::NAN,
            Double::NAN.acosh();
    );

    // atanh tests
    test_all_near!(
        atanh_pi_6:
            dd!("0.58128501169472315834806278002976886"),
            Double::FRAC_PI_6.atanh();
        atanh_pi_8:
            dd!("0.41498725684198892721883143584943144"),
            Double::FRAC_PI_8.atanh();
        atanh_1_sqrt_2:
            dd!("0.88137358701954302523260932497979124"),
            Double::FRAC_1_SQRT_2.atanh();
        atanh_99:
            dd!("2.6466524123622461977050606459342783"),
            dd!("0.99").atanh();
        atanh_neg_pi_6:
            dd!("-0.58128501169472315834806278002976886"),
            (-Double::FRAC_PI_6).atanh();
        atanh_neg_pi_8:
            dd!("-0.41498725684198892721883143584943144"),
            (-Double::FRAC_PI_8).atanh();
        atanh_neg_1_sqrt_2:
            dd!("-0.88137358701954302523260932497979124"),
            (-Double::FRAC_1_SQRT_2).atanh();
        atanh_neg_99:
            dd!("-2.6466524123622461977050606459342783"),
            dd!("-0.99").atanh();
    );
    test_all_exact!(
        atanh_pi:
            Double::NAN,
            Double::PI.atanh();
        atanh_neg_pi:
            Double::NAN,
            (-Double::PI).atanh();
        atanh_zero:
            Double::ZERO,
            Double::ZERO.atanh();
        atanh_neg_zero:
            Double::NEG_ZERO,
            Double::NEG_ZERO.atanh();
        atanh_inf:
            Double::NAN,
            Double::INFINITY.atanh();
        atanh_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY.atanh();
        atanh_nan:
            Double::NAN,
            Double::NAN.atanh();
    );
}
