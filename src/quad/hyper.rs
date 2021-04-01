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

    #[test]
    fn sinh_cosh() {
        let (sinh_pi, cosh_pi) = Quad::PI.sinh_cosh();
        assert_close!(
            qd!("11.54873935725774837797733431538840968449518906639478945523216336"),
            sinh_pi
        );
        assert_close!(
            qd!("11.59195327552152062775175205256013769577091717620542253821288305"),
            cosh_pi
        );

        let (sinh_e, cosh_e) = Quad::E.sinh_cosh();
        assert_close!(
            qd!("7.544137102816975826341820042516532740294985744301671666369136432"),
            sinh_e
        );
        assert_close!(
            qd!("7.610125138662288363418610230113379165233562792554468102771609974"),
            cosh_e
        );
    }

    #[test]
    fn sinh_cosh_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sinh_cosh().0);
        assert_exact!(Quad::ONE, Quad::ZERO.sinh_cosh().1);
    }

    #[test]
    fn sinh_cosh_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sinh_cosh().0);
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sinh_cosh().1);

        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.sinh_cosh().0);
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.sinh_cosh().1);
    }

    #[test]
    fn sinh_cosh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sinh_cosh().0);
        assert_exact!(Quad::NAN, Quad::NAN.sinh_cosh().1);
    }

    #[test]
    fn sinh() {
        assert_close!(
            qd!("11.54873935725774837797733431538840968449518906639478945523216336"),
            Quad::PI.sinh()
        );
        assert_close!(
            qd!("7.544137102816975826341820042516532740294985744301671666369136432"),
            Quad::E.sinh()
        );
    }

    #[test]
    fn sinh_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.sinh());
    }

    #[test]
    fn sinh_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.sinh());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.sinh());
    }

    #[test]
    fn sinh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.sinh());
    }

    #[test]
    fn cosh() {
        assert_close!(
            qd!("11.59195327552152062775175205256013769577091717620542253821288305"),
            Quad::PI.cosh()
        );
        assert_close!(
            qd!("7.610125138662288363418610230113379165233562792554468102771609974"),
            Quad::E.cosh()
        );
    }

    #[test]
    fn cosh_one() {
        assert_exact!(Quad::ONE, Quad::ZERO.cosh());
    }

    #[test]
    fn cosh_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.cosh());
        assert_exact!(Quad::INFINITY, Quad::NEG_INFINITY.cosh());
    }

    #[test]
    fn cosh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.cosh());
    }

    #[test]
    fn tanh() {
        assert_close!(
            qd!("0.9962720762207499442646905800125367118968991908045876143626124160"),
            Quad::PI.tanh()
        );
        assert_close!(
            qd!("0.9913289158005998377955576156996843829216586979874637167678292814"),
            Quad::E.tanh()
        );
    }

    #[test]
    fn tanh_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.tanh());
    }

    #[test]
    fn tanh_inf() {
        assert_exact!(Quad::ONE, Quad::INFINITY.tanh());
        assert_exact!(Quad::NEG_ONE, Quad::NEG_INFINITY.tanh());
    }

    #[test]
    fn tanh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.tanh());
    }

    #[test]
    fn asinh() {
        assert_close!(
            qd!("1.862295743310848219888361325182620574902674184961554765612879514"),
            Quad::PI.asinh()
        );
        assert_close!(
            qd!("1.725382558852315093945097970404888756274557274672938668814211557"),
            Quad::E.asinh()
        );
    }

    #[test]
    fn asinh_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.asinh());
    }

    #[test]
    fn asinh_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.asinh());
        assert_exact!(Quad::NEG_INFINITY, Quad::NEG_INFINITY.asinh());
    }

    #[test]
    fn asinh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.asinh());
    }

    #[test]
    fn acosh() {
        assert_close!(
            qd!("1.811526272460853107021852049305420510220702081057922474861595623"),
            Quad::PI.acosh()
        );
        assert_close!(
            qd!("1.657454454153077272593828742280534739158392762033676825848582209"),
            Quad::E.acosh()
        );
    }

    #[test]
    fn acosh_zero() {
        assert_exact!(Quad::NAN, Quad::ZERO.acosh());
    }

    #[test]
    fn acosh_inf() {
        assert_exact!(Quad::INFINITY, Quad::INFINITY.acosh());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.acosh());
    }

    #[test]
    fn acosh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.acosh());
    }

    #[test]
    fn atanh() {
        assert_close!(
            qd!("0.3297653149566991076178634175552186042701373911406924144029083548"),
            Quad::PI.recip().atanh()
        );
        assert_close!(
            qd!("0.3859684164526523625353195700175926718961289961812712597770308403"),
            Quad::E.recip().atanh()
        );
    }

    #[test]
    fn atanh_zero() {
        assert_exact!(Quad::ZERO, Quad::ZERO.atanh());
    }

    #[test]
    fn atanh_inf() {
        assert_exact!(Quad::NAN, Quad::INFINITY.atanh());
        assert_exact!(Quad::NAN, Quad::NEG_INFINITY.atanh());
    }

    #[test]
    fn atanh_nan() {
        assert_exact!(Quad::NAN, Quad::NAN.atanh());
    }
}
