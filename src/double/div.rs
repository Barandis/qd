// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::double::Double;
use std::ops::{Div, DivAssign};

// Helper function needed to avoid the only place in this arithmetic where Double::from must
// be called on a non-tuple, non-integer number. With the current parsing of floats, calling
// Double::from this way in the basic arithmetic would cause a stack overflow.
#[inline]
fn mul_f64(a: Double, b: f64) -> Double {
    let (p, e) = p::two_prod(a.0, b);
    let (a, b) = u::renorm2(p, e + a.1 * b);
    Double(a, b)
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Double {
    type Output = Double;

    /// Divides this `Double` by another, producing a new `Double` as a result.
    ///
    /// This implements the `/` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::E / Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    fn div(self, other: Double) -> Double {
        match self.pre_div(&other) {
            Some(r) => r,
            None => {
                let q1 = self.0 / other.0;
                let mut r = self - mul_f64(other, q1);

                let q2 = r.0 / other.0;
                r -= mul_f64(other, q2);

                let q3 = r.0 / other.0;

                let (a, b) = u::renorm3(q1, q2, q3);
                Double(a, b)
            }
        }
    }
}

impl Div for &Double {
    type Output = Double;

    /// Divides a reference to this `Double` by another, producing a new `Double` as a result.
    ///
    /// This implements the `/` operator between two references to `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = &Double::E / &Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    fn div(self, other: &Double) -> Double {
        (*self).div(*other)
    }
}

impl Div<&Double> for Double {
    type Output = Double;

    /// Divides this `Double` by a reference to another `Double`, producing a new `Double`
    /// as a result.
    ///
    /// This implements the `/` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::E / &Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn div(self, other: &Double) -> Double {
        self.div(*other)
    }
}

impl Div<Double> for &Double {
    type Output = Double;

    /// Divides a reference to this `Double` by another `Double`, producing a new `Double`
    /// as a result.
    ///
    /// This implements the `/` operator between a reference to a `Double` and a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = &Double::E / Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn div(self, other: Double) -> Double {
        (*self).div(other)
    }
}

impl DivAssign for Double {
    /// Divides this `Double` by another, modifying this one to equal the result.
    ///
    /// This implements the `/=` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let mut x = Double::E;
    /// x /= Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn div_assign(&mut self, other: Double) {
        let r = self.div(other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl DivAssign<&Double> for Double {
    /// Divides this `Double` by a reference to another, modifying this one to equal the
    /// result.
    ///
    /// This implements the `/=` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let mut x = Double::E;
    /// x /= &Double::PI;
    /// let expected = dd!("0.8652559794322650872177747896461");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn div_assign(&mut self, other: &Double) {
        let r = self.div(*other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl Double {
    // precalc functions
    //
    // This series of functions returns `Some` with a value that is to be returned, if it
    // turns out that the function doesn't have to be calculated because a shortcut result
    // is known. They return `None` if the value has to be calculated normally.
    //
    // This keeps the public functions from being mucked up with code that does validation
    // rather than calculation.

    #[inline]
    fn pre_div(&self, other: &Double) -> Option<Double> {
        if self.is_nan() || other.is_nan() {
            Some(Double::NAN)
        } else if other.is_zero() {
            if self.is_zero() {
                Some(Double::NAN)
            } else if self.is_sign_negative() == other.is_sign_positive() {
                Some(Double::NEG_INFINITY)
            } else {
                Some(Double::INFINITY)
            }
        } else if self.is_infinite() {
            if other.is_infinite() {
                Some(Double::NAN)
            } else if self.is_sign_positive() == other.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if self.is_sign_positive() == other.is_sign_positive() {
                Some(Double::ZERO)
            } else {
                Some(Double::NEG_ZERO)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // div tests
    test_all_near!(
        num_num:
            dd!("1.1557273497909217179100931833126961"),
            Double::PI / Double::E;
        num_ref:
            dd!("1.1557273497909217179100931833126961"),
            Double::PI / &Double::E;
        ref_num:
            dd!("1.1557273497909217179100931833126961"),
            &Double::PI / Double::E;
        ref_ref:
            dd!("1.1557273497909217179100931833126961"),
            &Double::PI / &Double::E;
        num_neg_num:
            dd!("-1.1557273497909217179100931833126961"),
            Double::PI / -Double::E;
        num_neg_ref:
            dd!("-1.1557273497909217179100931833126961"),
            Double::PI / -&Double::E;
        ref_neg_num:
            dd!("-1.1557273497909217179100931833126961"),
            &Double::PI / -Double::E;
        ref_neg_ref:
            dd!("-1.1557273497909217179100931833126961"),
            &Double::PI / -&Double::E;
        num_id:
            Double::PI,
            Double::PI / Double::ONE;
        id_num:
            Double::FRAC_1_PI,
            Double::ONE / Double::PI;
        num_small:
            dd!("3141592653589793238462643383279.5039"),
            Double::PI / dd!("1e-30");
        small_num:
            dd!("3.1830988618379067153776752674502853e-31"),
            dd!("1e-30") / Double::PI;
        three_nums:
            dd!("1.6673621161631071223063639072253465"),
            Double::PI / Double::E / Double::LN_2;
        lassoc:
            dd!("1.6673621161631071223063639072253465"),
            (Double::PI / Double::E) / Double::LN_2;
        rassoc:
            dd!("12.320232213560921976987672083576714"),
            Double::PI / (Double::LN_2 / Double::E);
    );
    test_all_exact!(
        zero_inf:
            Double::ZERO,
            Double::ZERO / Double::INFINITY;
        zero_neg_inf:
            Double::NEG_ZERO,
            Double::ZERO / Double::NEG_INFINITY;
        inf_zero:
            Double::INFINITY,
            Double::INFINITY / Double::ZERO;
        neg_inf_zero:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY / Double::ZERO;
        nan_zero:
            Double::NAN,
            Double::NAN / Double::ZERO;
        zero_nan:
            Double::NAN,
            Double::ZERO / Double::NAN;
        zero_zero:
            Double::NAN,
            Double::ZERO / Double::ZERO;

        one_inf:
            Double::ZERO,
            Double::ONE / Double::INFINITY;
        one_neg_inf:
            Double::NEG_ZERO,
            Double::ONE / Double::NEG_INFINITY;
        inf_one:
            Double::INFINITY,
            Double::INFINITY / Double::ONE;
        neg_inf_one:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY / Double::ONE;
        inf_inf:
            Double::NAN,
            Double::INFINITY / Double::INFINITY;
        inf_neg_inf:
            Double::NAN,
            Double::INFINITY / Double::NEG_INFINITY;
        neg_inf_inf:
            Double::NAN,
            Double::NEG_INFINITY / Double::INFINITY;
        neg_inf_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY / Double::NEG_INFINITY;
        one_zero:
            Double::INFINITY,
            Double::ONE / Double::ZERO;
        neg_one_zero:
            Double::NEG_INFINITY,
            Double::NEG_ONE / Double::ZERO;

        nan_one:
            Double::NAN,
            Double::NAN / Double::ONE;
        one_nan:
            Double::NAN,
            Double::ONE / Double::NAN;
    );

    // Assign tests. Assign code delegates to div code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Double::PI;
            a /= Double::E;
            near!(dd!("1.1557273497909217179100931833126961"), a);
        }
        assign_ref: {
            let mut b = Double::PI;
            b /= &Double::E;
            near!(dd!("1.1557273497909217179100931833126961"), b);
        }
    );

    test!(chain_tens: {
        let mut value = Double::LN_2;
        let ten = dd!(10);

        near!("6.9314718055994530941723212145818e-1", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-2", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-3", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-4", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-5", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-6", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-7", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-8", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-9", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-10", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-11", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-12", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-13", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-14", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-15", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-16", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-17", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-18", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-19", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-20", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-21", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-22", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-23", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-24", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-25", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-26", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-27", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-28", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-29", value);
        value /= ten;
        near!("6.9314718055994530941723212145818e-30", value);
    });
}
