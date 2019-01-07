// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::basic::*;
use crate::double::DoubleDouble;
use std::f64;

// #region Powers

impl DoubleDouble {
    /// Calculates the square of `self` and returns it as a new `DoubleDouble`.
    ///
    /// This method takes advantage of optimizations in multiplication that are available when the
    /// two numbers being multiplied are the same, so it is more efficient than bare multiplication.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from(3);
    /// assert!(dd.square() == dd * dd); // The left side is faster though
    /// ```
    #[inline]
    pub fn square(self) -> DoubleDouble {
        let (p, e) = two_square(self.0);
        DoubleDouble::from(quick_two_sum(
            p,
            e + 2.0 * self.0 * self.1 + self.1 * self.1,
        ))
    }

    /// Calculates `self`<sup>`n`</sup> and returns it as a new `DoubleDouble`.
    ///
    /// This method uses more-efficient squaring over multiplication where possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from(3);
    /// assert!(dd.powi(3) == 27.0);
    /// ```
    #[inline]
    pub fn powi(self, n: i32) -> DoubleDouble {
        if n == 0 {
            return DoubleDouble::from(1.0);
        }

        let mut r = self.clone();
        let mut s = DoubleDouble::from(1.0);
        let mut i = n.abs();

        if i > 1 {
            while i > 0 {
                if i % 2 == 1 {
                    s *= r;
                }
                i /= 2;
                if i > 0 {
                    r = r.square();
                }
            }
        } else {
            s = r;
        }

        if n < 0 {
            1.0 / s
        } else {
            s
        }
    }

    /// Calculates `self` &times; 2<sup>`n`</sup> and returns it as a new `DoubleDouble`.
    ///
    /// Though this is not an everyday operation, it is often used in more advanced mathematical
    /// calculations (including several within this library). Therefore an implementation that is
    /// much more efficient than calculating it through multiplication and [`powi`](#method.powi) is
    /// offered despite it not being part of the `f64` API.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from(3);
    /// assert!(dd.ldexp(3) == 24.0); // 3 * 2^3
    /// ```
    #[inline]
    pub fn ldexp(self, n: i32) -> DoubleDouble {
        DoubleDouble(self.0 * 2f64.powi(n), self.1 * 2f64.powi(n))
    }
}

// #endregion

// #region Roots

impl DoubleDouble {
    /// Calculates the square root of `self` and returns it as a new `DoubleDouble`.
    ///
    /// # Examples
    ///
    /// ```
    /// use qd::DoubleDouble;
    ///
    /// let dd = DoubleDouble::from(2);
    /// // floating point error is reduced substantially but can't be eliminated,
    /// // so we check to see that the numbers are very close rather than equal
    /// assert!((dd.sqrt() - DoubleDouble::SQRT_2).abs() < DoubleDouble::EPSILON);
    /// ```
    pub fn sqrt(self) -> DoubleDouble {
        if self == 0.0 {
            DoubleDouble::from(0.0)
        } else if self.is_sign_negative() {
            DoubleDouble::from(f64::NAN)
        } else {
            // Strategy: use a method developed by Alan Karp and Peter Markstein at HP
            // https://cr.yp.to/bib/1997/karp.pdf
            //
            // If x is an approximation of sqrt(a), then
            //
            //      sqrt(a) ≈ ax + (a - (ax)^2)x / 2
            //
            // The approximation is accurate to twice the accuracy of x. This can be repeated an
            // arbitrary number of times, but this method when used on double-doubles seems to only
            // require one iteration. (It can be performed with f64 mlutiplication for ax and
            // (...)x, but that proved less accurate with a single iteration and probably requires
            // more.)
            let x = DoubleDouble::from_div(1.0, self.0.sqrt());
            let ax = self * x;
            ax + (self - ax.square()) * x * 0.5
        }
    }
}

// #endregion
