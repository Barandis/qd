// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::renorm2;
use crate::double::Double;
use std::f64;

impl Double {
    /// Calculates the absolute value of the double-double.
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

    /// Returns the largest integer less than or equal to the double-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.99);
    /// let g = dd!(3.0);
    ///
    /// assert!(f.floor() == dd!(3));
    /// assert!(g.floor() == dd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn floor(self) -> Double {
        let hi = self.0.floor();

        if (hi - self.0).abs() < f64::EPSILON {
            Double::norm(hi, self.1.floor())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the smallest integer greater than or equal to the double-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.01);
    /// let g = dd!(4.0);
    ///
    /// assert!(f.ceil() == dd!(4));
    /// assert!(g.ceil() == dd!(4));
    /// # }
    /// ```
    #[inline]
    pub fn ceil(self) -> Double {
        let hi = self.0.ceil();

        if (hi - self.0).abs() < f64::EPSILON {
            Double::norm(hi, self.1.ceil())
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the nearest integer to the double-double. Half-way cases are rounded away
    /// from `0.0`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let f = dd!(3.3);
    /// let g = dd!(-3.3);
    ///
    /// assert!(f.round() == dd!(3));
    /// assert!(g.round() == dd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn round(self) -> Double {
        let hi = self.0.round();

        if (hi - self.0).abs() < f64::EPSILON {
            let lo = self.1.round();
            Double::from(renorm2(hi, lo))
        } else if ((hi - self.0).abs() - 0.5).abs() < f64::EPSILON && self.1 < 0.0 {
            Double(hi - 1.0, 0.0)
        } else {
            Double(hi, 0.0)
        }
    }

    /// Returns the integer part of the double-double.
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

    /// Returns the fractional part of the double-double.
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

    /// Returns a number that represents the sign of the double-double.
    ///
    /// * `1.0` if the double-double is positive, `+0.0`, or [`INFINITY`]
    /// * `-1.0` if the double-double is negative, `-0.0`, or [`NEG_INFINITY`]
    /// *  [`NAN`] if the double-double is [`NAN`]
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
