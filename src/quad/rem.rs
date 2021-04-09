// Copyright (c) 2021 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::quad::Quad;
use std::ops::{Rem, RemAssign};

impl Rem for Quad {
    type Output = Quad;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Quad` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::PI % Quad::E;
    /// let expected_x = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < qd!(1e-60));
    ///
    /// let y = Quad::PI % -Quad::E;
    /// let expected_y = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < qd!(1e-60));
    ///
    /// let z = -Quad::PI % Quad::E;
    /// let expected_z = qd!("-0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < qd!(1e-60));
    /// ```
    #[inline]
    fn rem(self, other: Quad) -> Quad {
        self - other * (self / other).trunc()
    }
}

impl Rem for &Quad {
    type Output = Quad;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Quad` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between two references to `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::PI % &Quad::E;
    /// let expected_x = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < qd!(1e-60));
    ///
    /// let y = &Quad::PI % -&Quad::E;
    /// let expected_y = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < qd!(1e-60));
    ///
    /// let z = -&Quad::PI % &Quad::E;
    /// let expected_z = qd!("-0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < qd!(1e-60));
    /// ```
    #[inline]
    fn rem(self, other: &Quad) -> Quad {
        *self % *other
    }
}

impl Rem<&Quad> for Quad {
    type Output = Quad;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Quad` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = Quad::PI % &Quad::E;
    /// let expected_x = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < qd!(1e-60));
    ///
    /// let y = Quad::PI % -&Quad::E;
    /// let expected_y = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < qd!(1e-60));
    ///
    /// let z = -Quad::PI % &Quad::E;
    /// let expected_z = qd!("-0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < qd!(1e-60));
    /// ```
    #[inline]
    fn rem(self, other: &Quad) -> Quad {
        self % *other
    }
}

impl Rem<Quad> for &Quad {
    type Output = Quad;

    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// producing a new `Quad` as the result.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%` operator between a reference to a `Quad` and a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let x = &Quad::PI % Quad::E;
    /// let expected_x = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < qd!(1e-60));
    ///
    /// let y = &Quad::PI % -Quad::E;
    /// let expected_y = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < qd!(1e-60));
    ///
    /// let z = -&Quad::PI % Quad::E;
    /// let expected_z = qd!("-0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < qd!(1e-60));
    /// ```
    #[inline]
    fn rem(self, other: Quad) -> Quad {
        *self % other
    }
}

impl RemAssign for Quad {
    /// Computes the remainder of $x \div y$, where $x$ is `self` and $y$ is the argument,
    /// assigning the result to `self`.
    ///
    /// The result is given the same sign as the dividend (`self`), no matter the sign of
    /// the divisor. This is consistent with the way `f64` behaves.
    ///
    /// This implements the `%=` operator between two `Quad`s.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::PI;
    /// x %= Quad::E;
    /// let expected_x = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < qd!(1e-60));
    ///
    /// let mut y = Quad::PI;
    /// y %= -Quad::E;
    /// let expected_y = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < qd!(1e-60));
    ///
    /// let mut z = -Quad::PI;
    /// z %= Quad::E;
    /// let expected_z = qd!("-0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < qd!(1e-60));
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: Quad) {
        let r = *self % other;
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

impl RemAssign<&Quad> for Quad {
    /// Divides this `Quad` by a reference to another, modifying this one to equal the
    /// remainder. This operation uses floored division.
    ///
    /// This implements the `%=` operator between a `Quad` and a reference to a `Quad`.
    ///
    /// # Examples
    /// ```
    /// # use qd::{qd, Quad};
    /// let mut x = Quad::PI;
    /// x %= &Quad::E;
    /// let expected_x = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_x = (x - expected_x).abs();
    /// assert!(delta_x < qd!(1e-60));
    ///
    /// let mut y = Quad::PI;
    /// y %= -&Quad::E;
    /// let expected_y = qd!("0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_y = (y - expected_y).abs();
    /// assert!(delta_y < qd!(1e-60));
    ///
    /// let mut z = -Quad::PI;
    /// z %= &Quad::E;
    /// let expected_z = qd!("-0.423310825130748003102355911926840386439922305675146246007976965");
    ///
    /// let delta_z = (z - expected_z).abs();
    /// assert!(delta_z < qd!(1e-60));
    /// ```
    #[inline]
    fn rem_assign(&mut self, other: &Quad) {
        let r = *self % *other;
        self.0 = r.0;
        self.1 = r.1;
        self.2 = r.2;
        self.3 = r.3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // rem tests
    test_all_near!(
        num_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI % Quad::E;
        num_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI % &Quad::E;
        ref_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI % Quad::E;
        ref_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI % &Quad::E;
        num_neg_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI % -Quad::E;
        num_neg_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI % -&Quad::E;
        ref_neg_num:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI % -Quad::E;
        ref_neg_ref:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            &Quad::PI % -&Quad::E;
        neg_num_num:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -Quad::PI % Quad::E;
        neg_num_ref:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -Quad::PI % &Quad::E;
        neg_ref_num:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -&Quad::PI % Quad::E;
        neg_ref_ref:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -&Quad::PI % &Quad::E;
        neg_num_neg_num:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -Quad::PI % -Quad::E;
        neg_num_neg_ref:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -Quad::PI % -&Quad::E;
        neg_ref_neg_num:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -&Quad::PI % -Quad::E;
        neg_ref_neg_ref:
            qd!("-0.42331082513074800310235591192684038643992230567514624600797696458298"),
            -&Quad::PI % -&Quad::E;
        num_id:
            qd!("0.14159265358979323846264338327950288419716939937510582097494459230689"),
            Quad::PI % Quad::ONE;
        id_num:
            Quad::ONE,
            Quad::ONE % Quad::PI;
        three_nums:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            Quad::PI % Quad::E % Quad::LN_2;
        lassoc:
            qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"),
            (Quad::PI % Quad::E) % Quad::LN_2;
        rassoc:
            qd!("0.36900393135001200079371489744679661189516886193408480449222455433266"),
            Quad::PI % (Quad::LN_2 % Quad::E);
    );
    test_all_exact!(
        nan_zero:
            Quad::NAN,
            Quad::NAN % Quad::ZERO;
        zero_nan:
            Quad::NAN,
            Quad::ZERO % Quad::NAN;
        inf_zero:
            Quad::NAN,
            Quad::INFINITY % Quad::ZERO;
        zero_inf:
            Quad::NAN,
            Quad::ZERO % Quad::INFINITY;
        neg_inf_zero:
            Quad::NAN,
            Quad::NEG_INFINITY % Quad::ZERO;
        zero_neg_inf:
            Quad::NAN,
            Quad::ZERO % Quad::NEG_INFINITY;

        inf_one:
            Quad::NAN,
            Quad::INFINITY % Quad::ONE;
        one_inf:
            Quad::NAN,
            Quad::ONE % Quad::INFINITY;
        neg_inf_one:
            Quad::NAN,
            Quad::NEG_INFINITY % Quad::ONE;
        one_neg_inf:
            Quad::NAN,
            Quad::ONE % Quad::NEG_INFINITY;
        inf_inf:
            Quad::NAN,
            Quad::INFINITY % Quad::INFINITY;
        inf_neg_inf:
            Quad::NAN,
            Quad::INFINITY % Quad::NEG_INFINITY;
        neg_inf_inf:
            Quad::NAN,
            Quad::NEG_INFINITY % Quad::INFINITY;
        neg_inf_neg_inf:
            Quad::NAN,
            Quad::NEG_INFINITY % Quad::NEG_INFINITY;

        nan_nan:
            Quad::NAN,
            Quad::NAN % Quad::NAN;
        nan_one:
            Quad::NAN,
            Quad::NAN % Quad::ONE;
        one_nan:
            Quad::NAN,
            Quad::ONE % Quad::NAN;
    );

    test_all!(
        assign_num: {
            let mut a = Quad::PI;
            a %= Quad::E;
            near!(qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"), a);
        }
        assign_ref: {
            let mut b = Quad::PI;
            b %= &Quad::E;
            near!(qd!("0.42331082513074800310235591192684038643992230567514624600797696458298"), b);
        }
    );
}
