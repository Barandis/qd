// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::cmp::Ordering;

impl PartialEq for Double {
    /// Calculates whether `self` and the argument are equal to one another.
    ///
    /// This implements the `==` and `!=` operators between `Double`s.
    ///
    /// Equality works exactly the same as it does for system floating-point numbers (`f64`,
    /// etc.), including zero equalling negative zero, `NaN` equalling nothing (including
    /// itself), etc. Notably, equality should be used with care since floating-point
    /// rounding, even with the increased precision of `Double`s, will still cause some
    /// numbers that should be equal to not be equal.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(Double::PI == Double::PI);
    /// assert!(Double::E != Double::PI);
    /// assert!(dd!(0.0) == dd!(-0.0));
    /// assert!(Double::NAN != Double::NAN);
    /// ```
    #[inline]
    fn eq(&self, other: &Double) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd for Double {
    /// Calculates how `self` and the argument should be ordered.
    ///
    /// This implements the `<`, `>`, `<=`, and `>=` operators between two `Double`s.
    ///
    /// Ordering works the same as it does for system floating-point numbers, including
    /// [`NAN`] returning false for any of these operators (including when comparing it to
    /// itself).
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// assert!(Double::PI > Double::E);
    /// assert!(dd!(0.0) <= dd!(-0.0));
    /// assert!(!(Double::NAN >= Double::NAN));
    /// ```
    ///
    /// [`NAN`]: #associatedconstant.NAN
    #[inline]
    fn partial_cmp(&self, other: &Double) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            x => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // eq tests
    test_all_assert!(
        eq_pi_pi:
            Double::PI == Double::PI;
        eq_pi_e:
            Double::PI != Double::E;
        eq_zero_neg_zero:
            Double::ZERO == Double::NEG_ZERO;
        eq_inf_inf:
            Double::INFINITY == Double::INFINITY;
        eq_neg_inf_neg_inf:
            Double::NEG_INFINITY == Double::NEG_INFINITY;
        eq_inf_neg_inf:
            Double::INFINITY != Double::NEG_INFINITY;
        eq_nan_nan:
            Double::NAN != Double::NAN;
    );

    // ord tests
    test_all_assert!(
        gt_pi_e:
            Double::PI > Double::E;
        gt_pi_pi:
            !(Double::PI > Double::PI);
        gt_e_pi:
            !(Double::E > Double::PI);
        gte_pi_e:
            Double::PI >= Double::E;
        gte_pi_pi:
            Double::PI >= Double::PI;
        gte_e_pi:
            !(Double::E >= Double::PI);
        lt_pi_e:
            !(Double::PI < Double::E);
        lt_pi_pi:
            !(Double::PI < Double::PI);
        lt_e_pi:
            Double::E < Double::PI;
        lte_pi_e:
            !(Double::PI <= Double::E);
        lte_pi_pi:
            Double::PI <= Double::PI;
        lte_e_pi:
            Double::E <= Double::PI;
        lte_zero_neg_zero:
            Double::ZERO <= Double::NEG_ZERO;
        gte_zero_neg_zero:
            Double::ZERO >= Double::NEG_ZERO;
        lt_neg_inf_inf:
            Double::NEG_INFINITY < Double::INFINITY;
        lte_neg_inf_inf:
            Double::NEG_INFINITY <= Double::NEG_INFINITY;
        gte_neg_inf_neg_inf:
            Double::NEG_INFINITY >= Double::NEG_INFINITY;
        gt_neg_inf_neg_inf:
            !(Double::NEG_INFINITY > Double::NEG_INFINITY);
        lt_nan_nan:
            !(Double::NAN < Double::NAN);
        lte_nan_nan:
            !(Double::NAN <= Double::NAN);
        gt_nan_nan:
            !(Double::NAN > Double::NAN);
        gte_nan_nan:
            !(Double::NAN >= Double::NAN);
    );
}
