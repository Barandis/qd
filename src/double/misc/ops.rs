// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::core;
use crate::double::Double;
use std::f64;

impl Double {
    /// Calculates the absolute value of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(dd!(3).abs() == dd!(3));
    /// assert!(dd!(-3).abs() == dd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn abs(self) -> Double {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    /// Returns the largest integer value less than or equal to the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.99);
    /// let g = dd!(3.0);
    /// let h = dd!(-3.99);
    ///
    /// assert!(f.floor() == dd!(3));
    /// assert!(g.floor() == dd!(3));
    /// assert!(h.floor() == dd!(-4));
    /// # }
    /// ```
    #[inline]
    pub fn floor(self) -> Double {
        let hi = self.0.floor();

        if (hi - self.0).abs() < f64::EPSILON {
            Double::new(hi, self.1.floor())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the smallest integer value greater than or equal to the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.01);
    /// let g = dd!(4.0);
    /// let h = dd!(-3.01);
    ///
    /// assert!(f.ceil() == dd!(4));
    /// assert!(g.ceil() == dd!(4));
    /// assert!(h.ceil() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn ceil(self) -> Double {
        let hi = self.0.ceil();

        if (hi - self.0).abs() < f64::EPSILON {
            Double::new(hi, self.1.ceil())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the nearest integer value to the `Double`. Half-way cases are rounded away
    /// from `0.0`, per the behavior of `f64`'s `round` method.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(3.5);
    /// let h = dd!(-3.3);
    ///
    /// assert!(f.round() == dd!(3));
    /// assert!(g.round() == dd!(4));
    /// assert!(h.round() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn round(self) -> Double {
        let hi = self.0.round();

        if (hi - self.0).abs() < f64::EPSILON {
            let lo = self.1.round();
            let (a, b) = core::renorm2(hi, lo);
            Double(a, b)
        } else if ((hi - self.0).abs() - 0.5).abs() < f64::EPSILON && self.1 < 0.0 {
            Double(hi - 1.0, 0.0)
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the integer part of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(-3.7);
    ///
    /// assert!(f.trunc() == dd!(3));
    /// assert!(g.trunc() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn trunc(self) -> Double {
        if self.0 >= 0.0 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    /// Returns the fractional part of the `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(-3.7);
    ///
    /// let fdiff = (f.fract() - dd!(0.3)).abs();
    /// let gdiff = (g.fract() - dd!(-0.7)).abs();
    ///
    /// assert!(fdiff < dd!(1e-30));
    /// assert!(gdiff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn fract(self) -> Double {
        self - self.trunc()
    }

    /// Returns a number that represents the sign of the `Double`.
    ///
    /// * `1.0` if the number is positive, including `+0.0` and [`INFINITY`]
    /// * `-1.0` if the number is negative, including `-0.0` and [`NEG_INFINITY`]
    /// *  [`NAN`] if the number is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// assert!(dd!(3.5).signum() == Double::ONE);
    /// assert!(Double::NEG_INFINITY.signum() == -Double::ONE);
    /// assert!(Double::NAN.signum().is_nan());
    /// # }
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    /// [`NEG_INFINITY`]: #associatedconstant.NEG_INFINITY
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    pub fn signum(self) -> Double {
        if self.is_nan() {
            Double::NAN
        } else if self.is_sign_negative() {
            -Double::ONE
        } else {
            Double::ONE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs() {
        assert_exact!((-Double::PI).abs(), Double::PI);
        assert_exact!(Double::PI.abs(), Double::PI);
    }

    #[test]
    fn abs_zero() {
        assert_exact!(Double::ZERO.abs(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.abs(), Double::ZERO);
    }

    #[test]
    fn abs_infinity() {
        assert_exact!(Double::INFINITY.abs(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.abs(), Double::INFINITY);
    }

    #[test]
    fn abs_nan() {
        assert!(Double::NAN.abs().is_nan());
    }

    #[test]
    fn floor() {
        assert_exact!(Double::PI.floor(), dd!(3));
        assert_exact!(Double::E.floor(), dd!(2));
        assert_exact!((-Double::PI).floor(), dd!(-4));
        assert_exact!((-Double::E).floor(), dd!(-3));
        assert_exact!(dd!(2).floor(), dd!(2));
    }

    #[test]
    fn floor_zero() {
        assert_exact!(Double::ZERO.floor(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.floor(), Double::NEG_ZERO);
    }

    #[test]
    fn floor_infinity() {
        assert_exact!(Double::INFINITY.floor(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.floor(), Double::NEG_INFINITY);
    }

    #[test]
    fn floor_nan() {
        assert!(Double::NAN.floor().is_nan());
    }

    #[test]
    fn ceil() {
        assert_exact!(Double::PI.ceil(), dd!(4));
        assert_exact!(Double::E.ceil(), dd!(3));
        assert_exact!((-Double::PI).ceil(), dd!(-3));
        assert_exact!((-Double::E).ceil(), dd!(-2));
        assert_exact!(dd!(2).ceil(), dd!(2));
    }

    #[test]
    fn ceil_zero() {
        assert_exact!(Double::ZERO.ceil(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.ceil(), Double::NEG_ZERO);
    }

    #[test]
    fn ceil_infinity() {
        assert_exact!(Double::INFINITY.ceil(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.ceil(), Double::NEG_INFINITY);
    }

    #[test]
    fn ceil_nan() {
        assert!(Double::NAN.ceil().is_nan());
    }

    #[test]
    fn round() {
        assert_exact!(Double::PI.round(), dd!(3));
        assert_exact!(Double::E.round(), dd!(3));
        assert_exact!((-Double::PI).round(), dd!(-3));
        assert_exact!((-Double::E).round(), dd!(-3));
        assert_exact!(dd!(2).round(), dd!(2));
        assert_exact!(dd!(2.5).round(), dd!(3));
        assert_exact!(dd!(-3.5).round(), dd!(-4));
    }

    #[test]
    fn round_zero() {
        assert_exact!(Double::ZERO.round(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.round(), Double::NEG_ZERO);
    }

    #[test]
    fn round_infinity() {
        assert_exact!(Double::INFINITY.round(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.round(), Double::NEG_INFINITY);
    }

    #[test]
    fn round_nan() {
        assert!(Double::NAN.round().is_nan());
    }

    #[test]
    fn trunc() {
        assert_exact!(Double::PI.trunc(), dd!(3));
        assert_exact!(Double::E.trunc(), dd!(2));
        assert_exact!((-Double::PI).trunc(), dd!(-3));
        assert_exact!((-Double::E).trunc(), dd!(-2));
        assert_exact!(dd!(2).trunc(), dd!(2));
        assert_exact!(dd!(2.5).trunc(), dd!(2));
        assert_exact!(dd!(-3.5).trunc(), dd!(-3));
    }

    #[test]
    fn trunc_zero() {
        assert_exact!(Double::ZERO.trunc(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.trunc(), Double::NEG_ZERO);
    }

    #[test]
    fn trunc_infinity() {
        assert_exact!(Double::INFINITY.trunc(), Double::INFINITY);
        assert_exact!(Double::NEG_INFINITY.trunc(), Double::NEG_INFINITY);
    }

    #[test]
    fn trunc_nan() {
        assert!(Double::NAN.trunc().is_nan());
    }

    #[test]
    fn fract() {
        assert_close!(Double::PI.fract(), Double::PI - dd!(3));
        assert_close!(Double::E.fract(), Double::E - dd!(2));
        assert_close!((-Double::PI).fract(), -Double::PI + dd!(3));
        assert_close!((-Double::E).fract(), -Double::E + dd!(2));
        assert_exact!(dd!(2).fract(), Double::ZERO);
        assert_exact!(dd!(2.5).fract(), dd!(0.5));
        assert_exact!(dd!(-3.5).fract(), dd!(-0.5));
    }

    #[test]
    fn fract_zero() {
        assert_exact!(Double::ZERO.fract(), Double::ZERO);
        assert_exact!(Double::NEG_ZERO.fract(), Double::NEG_ZERO);
    }

    #[test]
    fn fract_infinity() {
        assert_exact!(Double::INFINITY.fract(), Double::NAN);
        assert_exact!(Double::NEG_INFINITY.fract(), Double::NAN);
    }

    #[test]
    fn fract_nan() {
        assert!(Double::NAN.fract().is_nan());
    }

    #[test]
    fn signum() {
        assert_exact!(Double::PI.signum(), Double::ONE);
        assert_exact!(Double::E.signum(), Double::ONE);
        assert_exact!((-Double::PI).signum(), -Double::ONE);
        assert_exact!((-Double::E).signum(), -Double::ONE);
        assert_exact!(dd!(2).signum(), Double::ONE);
        assert_exact!(dd!(2.5).signum(), Double::ONE);
        assert_exact!(dd!(-3.5).signum(), -Double::ONE);
    }

    #[test]
    fn signum_zero() {
        assert_exact!(Double::ZERO.signum(), Double::ONE);
        assert_exact!(Double::NEG_ZERO.signum(), -Double::ONE);
    }

    #[test]
    fn signum_infinity() {
        assert_exact!(Double::INFINITY.signum(), Double::ONE);
        assert_exact!(Double::NEG_INFINITY.signum(), -Double::ONE);
    }

    #[test]
    fn signum_nan() {
        assert!(Double::NAN.signum().is_nan());
    }
}
