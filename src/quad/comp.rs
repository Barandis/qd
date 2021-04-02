// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::cmp::Ordering;

impl PartialEq for Quad {
    /// Implements the `==` and `!= operators, testing two `Quad`s for equality and
    /// inequality.
    ///
    /// Equality works exactly the same as it does for system floating-point numbers (`f64`,
    /// etc.), including zero equalling negative zero, `NaN` equalling nothing (including
    /// itself), etc. Notably, equality should be used with care since floating-point
    /// rounding, even with the increased precision of `Quad`s, will still cause some
    /// numbers that should be equal to not be equal.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::PI == Quad::PI);
    /// assert!(Quad::E != Quad::PI);
    /// assert!(qd!(0.0) == qd!(-0.0));
    /// assert!(Quad::NAN != Quad::NAN);
    /// # }
    /// ```
    #[inline]
    fn eq(&self, other: &Quad) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl PartialOrd for Quad {
    /// Implements the `<`, `>`, `<=`, and `>=` operators, testing two `Quad`s for ordering.
    ///
    /// Ordering works the same as it does for system floating-point numbers, including
    /// `NaN` returning false for any of these operators (including when comparing it to
    /// itself).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Quad;
    /// # fn main() {
    /// assert!(Quad::PI > Quad::E);
    /// assert!(qd!(0.0) <= qd!(-0.0));
    /// assert!(!(Quad::NAN >= Quad::NAN));
    /// # }
    /// ```
    ///
    #[inline]
    fn partial_cmp(&self, other: &Quad) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => match self.1.partial_cmp(&other.1) {
                Some(Ordering::Equal) => match self.2.partial_cmp(&other.2) {
                    Some(Ordering::Equal) => self.3.partial_cmp(&other.3),
                    x => x,
                },
                x => x,
            },
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
            Quad::PI == Quad::PI;
        eq_pi_e:
            Quad::PI != Quad::E;
        eq_zero_neg_zero:
            Quad::ZERO == Quad::NEG_ZERO;
        eq_inf_inf:
            Quad::INFINITY == Quad::INFINITY;
        eq_neg_inf_neg_inf:
            Quad::NEG_INFINITY == Quad::NEG_INFINITY;
        eq_inf_neg_inf:
            Quad::INFINITY != Quad::NEG_INFINITY;
        eq_nan_nan:
            Quad::NAN != Quad::NAN;
    );

    // ord tests
    test_all_assert!(
        gt_pi_e:
            Quad::PI > Quad::E;
        gt_pi_pi:
            !(Quad::PI > Quad::PI);
        gt_e_pi:
            !(Quad::E > Quad::PI);
        gte_pi_e:
            Quad::PI >= Quad::E;
        gte_pi_pi:
            Quad::PI >= Quad::PI;
        gte_e_pi:
            !(Quad::E >= Quad::PI);
        lt_pi_e:
            !(Quad::PI < Quad::E);
        lt_pi_pi:
            !(Quad::PI < Quad::PI);
        lt_e_pi:
            Quad::E < Quad::PI;
        lte_pi_e:
            !(Quad::PI <= Quad::E);
        lte_pi_pi:
            Quad::PI <= Quad::PI;
        lte_e_pi:
            Quad::E <= Quad::PI;
        lte_zero_neg_zero:
            Quad::ZERO <= Quad::NEG_ZERO;
        gte_zero_neg_zero:
            Quad::ZERO >= Quad::NEG_ZERO;
        lt_neg_inf_inf:
            Quad::NEG_INFINITY < Quad::INFINITY;
        lte_neg_inf_inf:
            Quad::NEG_INFINITY <= Quad::NEG_INFINITY;
        gte_neg_inf_neg_inf:
            Quad::NEG_INFINITY >= Quad::NEG_INFINITY;
        gt_neg_inf_neg_inf:
            !(Quad::NEG_INFINITY > Quad::NEG_INFINITY);
        lt_nan_nan:
            !(Quad::NAN < Quad::NAN);
        lte_nan_nan:
            !(Quad::NAN <= Quad::NAN);
        gt_nan_nan:
            !(Quad::NAN > Quad::NAN);
        gte_nan_nan:
            !(Quad::NAN >= Quad::NAN);
    );
}
