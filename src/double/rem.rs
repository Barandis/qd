// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;
use std::ops::{Rem, RemAssign};

impl Rem for Double {
    type Output = Double;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Double` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::PI % Double::E;
    /// let expected_x = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < dd!(1e-30));
    ///
    /// let y = Double::PI % -Double::E;
    /// let expected_y = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < dd!(1e-30));
    ///
    /// let z = -Double::PI % Double::E;
    /// let expected_z = dd!("-0.4233108251307480031023559119268");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < dd!(1e-30));
    /// ```
    #[inline]
    fn rem(self, other: Double) -> Double {
        self - other * (self / other).trunc()
    }
}

impl Rem for &Double {
    type Output = Double;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Double` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between two references to `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = &Double::PI % &Double::E;
    /// let expected_x = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < dd!(1e-30));
    ///
    /// let y = &Double::PI % -&Double::E;
    /// let expected_y = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < dd!(1e-30));
    ///
    /// let z = -&Double::PI % &Double::E;
    /// let expected_z = dd!("-0.4233108251307480031023559119268");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < dd!(1e-30));
    /// ```
    #[inline]
    fn rem(self, other: &Double) -> Double {
        *self % *other
    }
}

impl Rem<&Double> for Double {
    type Output = Double;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Double` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = Double::PI % &Double::E;
    /// let expected_x = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < dd!(1e-30));
    ///
    /// let y = Double::PI % -&Double::E;
    /// let expected_y = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < dd!(1e-30));
    ///
    /// let z = -Double::PI % &Double::E;
    /// let expected_z = dd!("-0.4233108251307480031023559119268");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < dd!(1e-30));
    /// ```
    #[inline]
    fn rem(self, other: &Double) -> Double {
        self % *other
    }
}

impl Rem<Double> for &Double {
    type Output = Double;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Double` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between a reference to a `Double`s and a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let x = &Double::PI % Double::E;
    /// let expected_x = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < dd!(1e-30));
    ///
    /// let y = &Double::PI % -Double::E;
    /// let expected_y = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < dd!(1e-30));
    ///
    /// let z = -&Double::PI % Double::E;
    /// let expected_z = dd!("-0.4233108251307480031023559119268");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < dd!(1e-30));
    /// ```
    #[inline]
    fn rem(self, other: Double) -> Double {
        *self % other
    }
}

impl RemAssign for Double {
    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// assigning the result to `self`.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%=` operator between two `Double`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let mut x = Double::PI;
    /// x %= Double::E;
    /// let expected_x = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < dd!(1e-30));
    ///
    /// let mut y = Double::PI;
    /// y %= -Double::E;
    /// let expected_y = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < dd!(1e-30));
    ///
    /// let mut z = -Double::PI;
    /// z %= Double::E;
    /// let expected_z = dd!("-0.4233108251307480031023559119268");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < dd!(1e-30));
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Double) {
        let r = *self % other;
        self.0 = r.0;
        self.1 = r.1;
    }
}

impl RemAssign<&Double> for Double {
    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// assigning the result to `self`.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%=` operator between a `Double` and a reference to a `Double`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{dd, Double};
    /// let mut x = Double::PI;
    /// x %= &Double::E;
    /// let expected_x = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < dd!(1e-30));
    ///
    /// let mut y = Double::PI;
    /// y %= -&Double::E;
    /// let expected_y = dd!("0.4233108251307480031023559119268");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < dd!(1e-30));
    ///
    /// let mut z = -Double::PI;
    /// z %= &Double::E;
    /// let expected_z = dd!("-0.4233108251307480031023559119268");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < dd!(1e-30));
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Double) {
        let r = *self % *other;
        self.0 = r.0;
        self.1 = r.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // rem tests
    test_all_near!(
        num_num:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI % Double::E;
        num_ref:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI % &Double::E;
        ref_num:
            dd!("0.42331082513074800310235591192684125"),
            &Double::PI % Double::E;
        ref_ref:
            dd!("0.42331082513074800310235591192684125"),
            &Double::PI % &Double::E;
        num_neg_num:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI % -Double::E;
        num_neg_ref:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI % -&Double::E;
        ref_neg_num:
            dd!("0.42331082513074800310235591192684125"),
            &Double::PI % -Double::E;
        ref_neg_ref:
            dd!("0.42331082513074800310235591192684125"),
            &Double::PI % -&Double::E;
        neg_num_num:
            dd!("-0.42331082513074800310235591192684125"),
            -Double::PI % Double::E;
        neg_num_ref:
            dd!("-0.42331082513074800310235591192684125"),
            -Double::PI % &Double::E;
        neg_ref_num:
            dd!("-0.42331082513074800310235591192684125"),
            -&Double::PI % Double::E;
        neg_ref_ref:
            dd!("-0.42331082513074800310235591192684125"),
            -&Double::PI % &Double::E;
        neg_num_neg_num:
            dd!("-0.42331082513074800310235591192684125"),
            -Double::PI % -Double::E;
        neg_num_neg_ref:
            dd!("-0.42331082513074800310235591192684125"),
            -Double::PI % -&Double::E;
        neg_ref_neg_num:
            dd!("-0.42331082513074800310235591192684125"),
            -&Double::PI % -Double::E;
        neg_ref_neg_ref:
            dd!("-0.42331082513074800310235591192684125"),
            -&Double::PI % -&Double::E;
        num_id:
            dd!("0.1415926535897932384626433832795028"),
            Double::PI % Double::ONE;
        id_num:
            Double::ONE,
            Double::ONE % Double::PI;
        three_nums:
            dd!("0.42331082513074800310235591192684125"),
            Double::PI % Double::E % Double::LN_2;
        lassoc:
            dd!("0.42331082513074800310235591192684125"),
            (Double::PI % Double::E) % Double::LN_2;
        rassoc:
            dd!("0.36900393135001200079371489744679573"),
            Double::PI % (Double::LN_2 % Double::E);
    );
    test_all_exact!(
        nan_zero:
            Double::NAN,
            Double::NAN % Double::ZERO;
        zero_nan:
            Double::NAN,
            Double::ZERO % Double::NAN;
        inf_zero:
            Double::NAN,
            Double::INFINITY % Double::ZERO;
        zero_inf:
            Double::NAN,
            Double::ZERO % Double::INFINITY;
        neg_inf_zero:
            Double::NAN,
            Double::NEG_INFINITY % Double::ZERO;
        zero_neg_inf:
            Double::NAN,
            Double::ZERO % Double::NEG_INFINITY;

        inf_one:
            Double::NAN,
            Double::INFINITY % Double::ONE;
        one_inf:
            Double::NAN,
            Double::ONE % Double::INFINITY;
        neg_inf_one:
            Double::NAN,
            Double::NEG_INFINITY % Double::ONE;
        one_neg_inf:
            Double::NAN,
            Double::ONE % Double::NEG_INFINITY;
        inf_inf:
            Double::NAN,
            Double::INFINITY % Double::INFINITY;
        inf_neg_inf:
            Double::NAN,
            Double::INFINITY % Double::NEG_INFINITY;
        neg_inf_inf:
            Double::NAN,
            Double::NEG_INFINITY % Double::INFINITY;
        neg_inf_neg_inf:
            Double::NAN,
            Double::NEG_INFINITY % Double::NEG_INFINITY;

        nan_nan:
            Double::NAN,
            Double::NAN % Double::NAN;
        nan_one:
            Double::NAN,
            Double::NAN % Double::ONE;
        one_nan:
            Double::NAN,
            Double::ONE % Double::NAN;
    );

    test_all!(
        assign_num: {
            let mut a = Double::PI;
            a %= Double::E;
            near!(dd!("0.42331082513074800310235591192684125"), a);
        }
        assign_ref: {
            let mut b = Double::PI;
            b %= &Double::E;
            near!(dd!("0.42331082513074800310235591192684125"), b);
        }
    );
}
