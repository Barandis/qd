// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::basic::renorm4;
use crate::quad::Quad;

impl Quad {
    /// Calculates the absolute value of the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(3).abs() == qd!(3));
    /// assert!(qd!(-3).abs() == qd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn abs(self) -> Quad {
        if self.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    /// Returns the largest integer less than or equal to the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.99);
    /// let g = qd!(3.0);
    ///
    /// assert!(f.floor() == qd!(3));
    /// assert!(g.floor() == qd!(3));
    /// # }
    /// ```
    #[inline]
    pub fn floor(self) -> Quad {
        let a = self.0.floor();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;

        if a == self.0 {
            b = self.1.floor();
            if b == self.1 {
                c = self.2.floor();
                if c == self.2 {
                    d = self.3.floor();
                }
            }
            Quad::from(renorm4(a, b, c, d))
        } else {
            Quad(a, b, c, d)
        }
    }

    /// Returns the smallest integer greater than or equal to the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.01);
    /// let g = qd!(4.0);
    ///
    /// assert!(f.ceil() == qd!(4));
    /// assert!(g.ceil() == qd!(4));
    /// # }
    /// ```
    #[inline]
    pub fn ceil(self) -> Quad {
        let a = self.0.ceil();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut d = 0.0;

        if a == self.0 {
            b = self.1.ceil();
            if b == self.1 {
                c = self.2.ceil();
                if c == self.2 {
                    d = self.3.ceil();
                }
            }
            Quad::from(renorm4(a, b, c, d))
        } else {
            Quad(a, b, c, d)
        }
    }

    /// Returns the nearest integer to the quad-double. Half-way cases are rounded away from `0.0`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.3);
    /// let g = qd!(-3.3);
    ///
    /// assert!(f.round() == qd!(3));
    /// assert!(g.round() == qd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn round(self) -> Quad {
        let a = self.0.round();
        if a == self.0 {
            let b = self.1.round();
            if b == self.1 {
                let c = self.2.round();
                if c == self.2 {
                    let d = self.3.round();
                    Quad::from(renorm4(a, b, c, d))
                } else {
                    if (c - self.2).abs() == 0.5 && self.3 < 0.0 {
                        Quad(a, b, c - 1.0, 0.0)
                    } else {
                        Quad(a, b, c, 0.0)
                    }
                }
            } else {
                if (b - self.1).abs() == 0.5 && self.2 < 0.0 {
                    Quad(a, b - 1.0, 0.0, 0.0)
                } else {
                    Quad(a, b, 0.0, 0.0)
                }
            }
        } else {
            if (a - self.0).abs() == 0.5 && self.1 < 0.0 {
                Quad(a - 1.0, 0.0, 0.0, 0.0)
            } else {
                Quad(a, 0.0, 0.0, 0.0)
            }
        }
    }

    /// Returns the integer part of the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.3);
    /// let g = qd!(-3.7);
    ///
    /// assert!(f.trunc() == qd!(3));
    /// assert!(g.trunc() == qd!(-3));
    /// # }
    /// ```
    #[inline]
    pub fn trunc(self) -> Quad {
        if self.0 >= 0.0 {
            self.floor()
        } else {
            self.ceil()
        }
    }

    /// Returns the fractional part of the quad-double.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// let f = qd!(3.3);
    /// let g = qd!(-3.7);
    ///
    /// let fdiff = (f.fract() - qd!(0.3)).abs();
    /// let gdiff = (g.fract() - qd!(-0.7)).abs();
    ///
    /// assert!(fdiff < qd!(1e-60));
    /// assert!(gdiff < qd!(1e-60));
    /// # }
    /// ```
    #[inline]
    pub fn fract(self) -> Quad {
        self - self.trunc()
    }

    /// Returns a number that represents the sign of the quad-double.
    ///
    /// * `1.0` if the quad-double is positive, `+0.0`, or [`INFINITY`]
    /// * `-1.0` if the quad-double is negative, `-0.0`, or [`NEG_INFINITY`]
    /// *  [`NAN`] if the quad-double is [`NAN`]
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(qd!(3.5).signum() == Quad::ONE);
    /// assert!(Quad::NEG_INFINITY.signum() == -Quad::ONE);
    /// assert!(Quad::NAN.signum().is_nan());
    /// # }
    /// ```
    ///
    /// [`INFINITY`]: #associatedconstant.INFINITY
    /// [`NEG_INFINITY`]: #associatedconstant.NEG_INFINITY
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    pub fn signum(self) -> Quad {
        if self.is_nan() {
            Quad::NAN
        } else if self.is_sign_negative() {
            -Quad::ONE
        } else {
            Quad::ONE
        }
    }
}
