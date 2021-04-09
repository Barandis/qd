// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::common::primitive as p;
use crate::common::utils as u;
use crate::double::Double;
use std::ops::{Sub, SubAssign};

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Double {
    type Output = Double;

    /// Computes $x - y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Double` as the result.
    ///
    /// This implements the binary `-` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::E - Double::PI;
    /// let expected = dd!("-0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    fn sub(self, other: Double) -> Double {
        match self.pre_sub(&other) {
            Some(r) => r,
            None => {
                let (s0, e0) = p::two_diff(self.0, other.0);
                let (s1, e1) = p::two_diff(self.1, other.1);
                let (s2, e2) = p::quick_two_sum(s0, s1 + e0);
                let (a, b) = u::renorm2(s2, e1 + e2);
                Double(a, b)
            }
        }
    }
}

impl Sub for &Double {
    type Output = Double;

    /// Computes $x - y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Double` as the result.
    ///
    /// This implements the binary `-` operator between two references to `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = &Double::E - &Double::PI;
    /// let expected = dd!("-0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn sub(self, other: &Double) -> Double {
        (*self).sub(*other)
    }
}

impl Sub<&Double> for Double {
    type Output = Double;

    /// Computes $x - y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Double` as the result.
    ///
    /// This implements the binary `-` operator between a `Double` and a reference to a
    /// `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::E - &Double::PI;
    /// let expected = dd!("-0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn sub(self, other: &Double) -> Double {
        self.sub(*other)
    }
}

impl Sub<Double> for &Double {
    type Output = Double;

    /// Computes $x - y$, where $x$ is `self` and $y$ is the argument, producing a new
    /// `Double` as the result.
    ///
    /// This implements the binary `-` operator between a reference to a `Double` and a
    /// `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = &Double::E - Double::PI;
    /// let expected = dd!("-0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn sub(self, other: Double) -> Double {
        (*self).sub(other)
    }
}

impl SubAssign for Double {
    /// Computes $x - y$, where $x$ is `self` and $y$ is the argument, assigning the result
    /// to `self`.
    ///
    /// This implements the `-=` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let mut x = Double::E;
    /// x -= Double::PI;
    /// let expected = dd!("-0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: Double) {
        let r = self.sub(other);
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl SubAssign<&Double> for Double {
    /// Computes $x - y$, where $x$ is `self` and $y$ is the argument, assigning the result
    /// to `self`.
    ///
    /// This implements the `-=` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let mut x = Double::E;
    /// x -= &Double::PI;
    /// let expected = dd!("-0.4233108251307480031023559119268");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// ```
    #[inline]
    fn sub_assign(&mut self, other: &Double) {
        let r = self.sub(*other);
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
    fn pre_sub(&self, other: &Double) -> Option<Double> {
        if self.is_nan() || other.is_nan() {
            Some(Double::NAN)
        } else if self.is_infinite() {
            if other.is_infinite() {
                if self.is_sign_positive() {
                    if other.is_sign_positive() {
                        Some(Double::NAN)
                    } else {
                        Some(Double::INFINITY)
                    }
                } else if other.is_sign_negative() {
                    Some(Double::NAN)
                } else {
                    Some(Double::NEG_INFINITY)
                }
            } else if self.is_sign_positive() {
                Some(Double::INFINITY)
            } else {
                Some(Double::NEG_INFINITY)
            }
        } else if other.is_infinite() {
            if other.is_sign_positive() {
                Some(Double::NEG_INFINITY)
            } else {
                Some(Double::INFINITY)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // sub tests
    test_all_near!(
        num_num:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI - Double::E;
        num_ref:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI - &Double::E;
        ref_num:
            dd!("0.42331082513074800310235591192684125"),
            &Double::PI - Double::E;
        ref_ref:
            dd!("0.42331082513074800310235591192684125"),
            &Double::PI - &Double::E;
        num_neg_num:
            dd!("5.8598744820488384738229308546321643"),
            Double::PI - -Double::E;
        num_neg_ref:
            dd!("5.8598744820488384738229308546321643"),
            Double::PI - -&Double::E;
        ref_neg_num:
            dd!("5.8598744820488384738229308546321643"),
            &Double::PI - -Double::E;
        ref_neg_ref:
            dd!("5.8598744820488384738229308546321643"),
            &Double::PI - -&Double::E;
        num_id:
            Double::PI,
            Double::PI - Double::ZERO;
        id_num:
            -Double::PI,
            Double::ZERO - Double::PI;
        num_small:
            dd!("3.1415926535897932384626433832785013"),
            Double::PI - dd!("1e-30");
        small_num:
            dd!("-3.1415926535897932384626433832785013"),
            dd!("1e-30") - Double::PI;
        three_nums:
            dd!("-0.26983635542919730631487620953133551"),
            Double::PI - Double::E - Double::LN_2;
        lassoc:
            dd!("-0.26983635542919730631487620953133551"),
            (Double::PI - Double::E) - Double::LN_2;
        rassoc:
            dd!("5.1667273014888931644056987331739914"),
            Double::PI - (Double::LN_2 - Double::E);
    );
    test_all_exact!(
        inf_one:
            Double::INFINITY,
            Double::INFINITY - Double::ONE;
        one_inf:
            Double::NEG_INFINITY,
            Double::ONE - Double::INFINITY;
        neg_inf_one:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY - Double::ONE;
        one_neg_inf:
            Double::INFINITY,
            Double::ONE - Double::NEG_INFINITY;
        inf_neg_inf:
            Double::INFINITY,
            Double::INFINITY - Double::NEG_INFINITY;
        neg_inf_inf:
            Double::NEG_INFINITY,
            Double::NEG_INFINITY - Double::INFINITY;

        nan_one:
            Double::NAN,
            Double::NAN - Double::ONE;
        one_nan:
            Double::NAN,
            Double::ONE - Double::NAN;
        nan_inf:
            Double::NAN,
            Double::NAN - Double::INFINITY;
        inf_nan:
            Double::NAN,
            Double::INFINITY - Double::NAN;
        inf_inf:
            Double::NAN,
            Double::INFINITY - Double::INFINITY;
        neg_inf_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY - Double::NEG_INFINITY;
    );

    // Assign tests. Assign code delegates to sub code, so there's no need to re-test all
    // of the cases above.
    test_all!(
        assign_num: {
            let mut a = Double::PI;
            a -= Double::E;
            near!(dd!("0.42331082513074800310235591192684125"), a);
        }
        assign_ref: {
            let mut b = Double::PI;
            b -= &Double::E;
            near!(dd!("0.42331082513074800310235591192684125"), b);
        }
    );
}
