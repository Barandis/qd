// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::double::Double;
use std::ops::{Mul, MulAssign};

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for Double {
    type Output = Double;

    /// Multiplies this `Double` by another, producing a new `Double` as a result.
    ///
    /// This implements the `*` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E * Double::PI;
    /// let expected = dd!("8.539734222673567065463550869547");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    fn mul(self, other: Double) -> Double {
        match self.pre_mul(&other) {
            Some(r) => r,
            None => {
                let (p, e) = p::two_prod(self.0, other.0);
                let (a, b) = u::renorm2(p, e + self.0 * other.1 + self.1 * other.0);
                Double(a, b)
            }
        }
    }
}

impl Mul for &Double {
    type Output = Double;

    /// Multiplies a reference to this `Double` by another, producing a new `Double` as a
    /// result.
    ///
    /// This implements the `*` operator between two references to `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::E * &Double::PI;
    /// let expected = dd!("8.539734222673567065463550869547");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn mul(self, other: &Double) -> Double {
        (*self).mul(*other)
    }
}

impl Mul<&Double> for Double {
    type Output = Double;

    /// Multiplies this `Double` by a reference to another, producing a new `Double` as a
    /// result.
    ///
    /// This implements the `*` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::E * &Double::PI;
    /// let expected = dd!("8.539734222673567065463550869547");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn mul(self, other: &Double) -> Double {
        self.mul(*other)
    }
}

impl Mul<Double> for &Double {
    type Output = Double;

    /// Multiplies a reference to this `Double` by another `Double`, producing a new
    /// `Double` as a result.
    ///
    /// This implements the `*` operator between a reference to a `Double` and a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = &Double::E * Double::PI;
    /// let expected = dd!("8.539734222673567065463550869547");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn mul(self, other: Double) -> Double {
        (*self).mul(other)
    }
}

impl MulAssign for Double {
    /// Multiples this `Double` by another one, modifying this one to equal the result.
    ///
    /// This implements the `*=` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::E;
    /// x *= Double::PI;
    /// let expected = dd!("8.539734222673567065463550869547");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: Double) {
        let r = self.mul(other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl MulAssign<&Double> for Double {
    /// Multiples this `Double` by a reference to another one, modifying this one to equal
    /// the result.
    ///
    /// This implements the `*=` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let mut x = Double::E;
    /// x *= &Double::PI;
    /// let expected = dd!("8.539734222673567065463550869547");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    fn mul_assign(&mut self, other: &Double) {
        let r = self.mul(*other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl Double {
    // Precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_mul(&self, other: &Double) -> Option<Double> {
        if self.is_nan() || other.is_nan() {
            Some(Double::NAN)
        } else if self.is_zero() {
            if other.is_infinite() {
                Some(Double::NAN)
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Some(Double::ZERO)
            } else {
                Some(Double::NEG_ZERO)
            }
        } else if self.is_infinite() {
            if other.is_zero() {
                Some(Double::NAN)
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // mul tests
    test_all_near!(
        num_num:
            dd!("8.5397342226735670654635508695465707"),
            Double::PI * Double::E;
        num_ref:
            dd!("8.5397342226735670654635508695465707"),
            Double::PI * &Double::E;
        ref_num:
            dd!("8.5397342226735670654635508695465707"),
            &Double::PI * Double::E;
        ref_ref:
            dd!("8.5397342226735670654635508695465707"),
            &Double::PI * &Double::E;
        num_neg_num:
            dd!("-8.5397342226735670654635508695465707"),
            Double::PI * -Double::E;
        num_neg_ref:
            dd!("-8.5397342226735670654635508695465707"),
            Double::PI * -&Double::E;
        ref_neg_num:
            dd!("-8.5397342226735670654635508695465707"),
            &Double::PI * -Double::E;
        ref_neg_ref:
            dd!("-8.5397342226735670654635508695465707"),
            &Double::PI * -&Double::E;
        num_id:
            Double::PI,
            Double::PI * Double::ONE;
        id_num:
            Double::PI,
            Double::ONE * Double::PI;
        num_small:
            dd!("3.1415926535897932384626433832795009e-30"),
            Double::PI * dd!("1e-30");
        small_num:
            dd!("3.1415926535897932384626433832795009e-30"),
            dd!("1e-30") * Double::PI;
        three_nums:
            dd!("5.9192926991774591936228124210310508"),
            Double::PI * Double::E * Double::LN_2;
        lassoc:
            dd!("5.9192926991774591936228124210310508"),
            (Double::PI * Double::E) * Double::LN_2;
        rassoc:
            dd!("5.9192926991774591936228124210310508"),
            Double::PI * (Double::LN_2 * Double::E);
    );
    test_all_exact!(
        nan_zero:
            Double::NAN,
            Double::NAN * Double::ZERO;
        zero_nan:
            Double::NAN,
            Double::ZERO * Double::NAN;
        inf_zero:
            Double::NAN,
            Double::INFINITY * Double::ZERO;
        zero_inf:
            Double::NAN,
            Double::ZERO * Double::INFINITY;
        inf_neg_zero:
            Double::NAN,
            Double::NEG_INFINITY * Double::ZERO;
        zero_neg_inf:
            Double::NAN,
            Double::ZERO * Double::NEG_INFINITY;

        inf_one:
            Double::INFINITY,
            Double::INFINITY * Double::ONE;
        one_inf:
            Double::INFINITY,
            Double::ONE * Double::INFINITY;
        neg_inf_one:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY * Double::ONE;
        one_neg_inf:
            Double::NEG_INFINITY,
            Double::ONE * Double::NEG_INFINITY;
        inf_inf:
            Double::INFINITY,
            Double::INFINITY * Double::INFINITY;
        inf_neg_inf:
            Double::NEG_INFINITY,
            Double::INFINITY * Double::NEG_INFINITY;
        neg_inf_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY * Double::INFINITY;
        neg_inf_neg_inf:
            Double::INFINITY,
            Double::NEG_INFINITY * Double::NEG_INFINITY;

        nan_one:
            Double::NAN,
            Double::NAN * Double::ONE;
        one_nan:
            Double::NAN,
            Double::ONE * Double::NAN;
    );

    // Assign tests. Assign code delegates to mul code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Double::PI;
            a *= Double::E;
            near!(dd!("8.5397342226735670654635508695465707"), a);
        }
        assign_ref: {
            let mut b = Double::PI;
            b *= &Double::E;
            near!(dd!("8.5397342226735670654635508695465707"), b);
        }
    );
}
