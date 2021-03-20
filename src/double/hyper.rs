// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
        if self.is_nan() {
            (Double::NAN, Double::NAN)
        } else if self.is_zero() {
            (Double::ZERO, Double::ONE)
        } else if self.abs().0 <= 0.05 {
            let s = self.sinh();
            let c = (Double::ONE + s.sqr()).sqrt();
            (s, c)
        } else {
            let a = self.exp();
            let inv_a = a.recip();
            let s = (a - inv_a).mul_pwr2(0.5);
            let c = (a + inv_a).mul_pwr2(0.5);
            (s, c)
        }
    }

    /// Computes the hyperbolic sine (sinh) of the `Double`.
    /// 
    /// The domain and range of this function are both (-∞, ∞).
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
        if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ZERO
        } else if self.abs().0 > 0.05 {
            let a = self.exp();
            (a - a.recip()).mul_pwr2(0.5)
        } else {
            // The above formula is not accurate enough with very small numbers. Use a
            // Taylor series instead.
            let mut s = self;
            let mut t = self;
            let r = t.sqr();
            let mut m = 1.0;
            let threshold = (self * Double::EPSILON).abs();

            loop {
                m += 2.0;
                t *= r;
                t /= Double::from_mul(m - 1.0, m);
                s += t;
                if t.abs() <= threshold {
                    break;
                }
            }
            s
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
        if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ONE
        } else {
            let a = self.exp();
            (a + a.recip()).mul_pwr2(0.5)
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
        if self.is_nan() {
            Double::NAN
        } else if self.is_zero() {
            Double::ZERO
        } else if self.is_infinite() {
            self.signum() * Double::ONE
        } else if self.abs().0 > 0.05 {
            let a = self.exp();
            let inv_a = a.recip();
            (a - inv_a) / (a + inv_a)
        } else {
            let (s, c) = self.sinh_cosh();
            s / c
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
        if self.is_zero() {
            self
        } else if self.is_infinite() {
            if self.is_sign_positive() {
                Double::INFINITY
            } else {
                Double::NEG_INFINITY
            }
        } else {
            (self + (self.sqr() + Double::ONE).sqrt()).ln()
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
        if self < Double::ONE {
            Double::NAN
        } else if self == Double::ONE {
            Double::ZERO
        } else if self.is_infinite() {
            Double::INFINITY
        } else {
            (self + (self.sqr() - Double::ONE).sqrt()).ln()
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
        if self.abs() >= Double::ONE {
            Double::NAN
        } else {
            ((Double::ONE + self) / (Double::ONE - self)).ln().mul_pwr2(0.5)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sinh_cosh() {
        let (sinh_pi, cosh_pi) = Double::PI.sinh_cosh();
        assert_close!(dd!("11.548739357257748377977334315388"), sinh_pi);
        assert_close!(dd!("11.591953275521520627751752052560"), cosh_pi);

        let (sinh_e, cosh_e) = Double::E.sinh_cosh();
        assert_close!(dd!("7.5441371028169758263418200425165"), sinh_e);
        assert_close!(dd!("7.6101251386622883634186102301134"), cosh_e);
    }

    #[test]
    fn sinh_cosh_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sinh_cosh().0);
        assert_exact!(Double::ONE, Double::ZERO.sinh_cosh().1);
    }

    #[test]
    fn sinh_cosh_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sinh_cosh().0);
        assert_exact!(Double::INFINITY, Double::INFINITY.sinh_cosh().1);

        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.sinh_cosh().0);
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.sinh_cosh().1);
    }

    #[test]
    fn sinh_cosh_nan() {
        assert_exact!(Double::NAN, Double::NAN.sinh_cosh().0);
        assert_exact!(Double::NAN, Double::NAN.sinh_cosh().1);
    }

    #[test]
    fn sinh() {
        assert_close!(dd!("11.548739357257748377977334315388"), Double::PI.sinh());
        assert_close!(dd!("7.5441371028169758263418200425165"), Double::E.sinh());
    }

    #[test]
    fn sinh_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.sinh());
    }

    #[test]
    fn sinh_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.sinh());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.sinh());
    }

    #[test]
    fn sinh_nan() {
        assert_exact!(Double::NAN, Double::NAN.sinh());
    }

    #[test]
    fn cosh() {
        assert_close!(dd!("11.591953275521520627751752052560"), Double::PI.cosh());
        assert_close!(dd!("7.6101251386622883634186102301134"), Double::E.cosh());
    }

    #[test]
    fn cosh_one() {
        assert_exact!(Double::ONE, Double::ZERO.cosh());
    }

    #[test]
    fn cosh_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.cosh());
        assert_exact!(Double::INFINITY, Double::NEG_INFINITY.cosh());
    }

    #[test]
    fn cosh_nan() {
        assert_exact!(Double::NAN, Double::NAN.cosh());
    }

    #[test]
    fn tanh() {
        assert_close!(dd!("0.99627207622074994426469058001254"), Double::PI.tanh());
        assert_close!(dd!("0.99132891580059983779555761569968"), Double::E.tanh());
    }

    #[test]
    fn tanh_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.tanh());
    }

    #[test]
    fn tanh_inf() {
        assert_exact!(Double::ONE, Double::INFINITY.tanh());
        assert_exact!(Double::NEG_ONE, Double::NEG_INFINITY.tanh());
    }

    #[test]
    fn tanh_nan() {
        assert_exact!(Double::NAN, Double::NAN.tanh());
    }

    #[test]
    fn asinh() {
        assert_close!(dd!("1.8622957433108482198883613251826"), Double::PI.asinh());
        assert_close!(dd!("1.7253825588523150939450979704049"), Double::E.asinh());
    }

    #[test]
    fn asin_zero() {
        assert_exact!(Double::ZERO, dd!(0.0).asinh());
        assert_exact!(Double::NEG_ZERO, dd!(-0.0).asinh());
    }

    #[test]
    fn asin_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.asinh());
        assert_exact!(Double::NEG_INFINITY, Double::NEG_INFINITY.asinh());
    }

    #[test]
    fn asin_nan() {
        assert_exact!(Double::NAN, Double::NAN.asinh());
    }

    #[test]
    fn acosh() {
        assert_close!(
            dd!("1.81152627246085310702185204930542"),
            Double::PI.acosh()
        );
        assert_close!(dd!("1.65745445415307727259382874228053"), Double::E.acosh());
    }

    #[test]
    fn acosh_zero() {
        assert_exact!(Double::NAN, Double::ZERO.acosh());
    }

    #[test]
    fn acosh_one() {
        assert_exact!(Double::ZERO, Double::ONE.acosh());
    }

    #[test]
    fn acosh_inf() {
        assert_exact!(Double::INFINITY, Double::INFINITY.acosh());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.acosh());
    }

    #[test]
    fn acosh_nan() {
        assert_exact!(Double::NAN, Double::NAN.acosh());
    }

    #[test]
    fn atanh() {
        assert_close!(
            dd!("0.3297653149566991076178634175552186"),
            Double::PI.recip().atanh()
        );
        assert_close!(
            dd!("0.3859684164526523625353195700175927"),
            Double::E.recip().atanh()
        );
    }

    #[test]
    fn atanh_zero() {
        assert_exact!(Double::ZERO, Double::ZERO.atanh());
    }

    #[test]
    fn atanh_inf() {
        assert_exact!(Double::NAN, Double::INFINITY.atanh());
        assert_exact!(Double::NAN, Double::NEG_INFINITY.atanh());
    }

    #[test]
    fn atanh_nan() {
        assert_exact!(Double::NAN, Double::NAN.atanh());
    }
}
