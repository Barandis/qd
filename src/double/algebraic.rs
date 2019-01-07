// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

/use crate::basic::*;
use crate::double::DoubleDouble;
use std::f64;

// #region Powers

impl DoubleDouble {
    #[inline]
    pub fn square(self) -> DoubleDouble {
        let (p, e) = two_square(self.0);
        DoubleDouble::from(quick_two_sum(
            p,
            e + 2.0 * self.0 * self.1 + self.1 * self.1,
        ))
    }

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

    #[inline]
    pub fn ldexp(self, exp: i32) -> DoubleDouble {
        DoubleDouble(self.0 * 2f64.powi(exp), self.1 * 2f64.powi(exp))
    }
}

// #endregion

// #region Roots

impl DoubleDouble {
    #[inline]
    pub fn sqrt(self) -> DoubleDouble {
        if self == 0.0 {
            DoubleDouble::from(0.0)
        } else if self.is_sign_negative() {
            DoubleDouble::from(f64::NAN)
        } else {
            let x = 1.0 / self.0.sqrt();
            let ax = self.0 * x;
            DoubleDouble::from_add(ax, (self - ax * ax).0 * x * 0.5)
        }
    }
}

// #endregion
